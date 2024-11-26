mod model;

use crate::model::{Checksum16, Endian, NvramMap};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

fn read_ch<A: Read + Seek>(stream: &mut A, location: u64, length: usize) -> io::Result<String> {
    stream.seek(SeekFrom::Start(location))?;
    let mut buff = vec![0; length];
    stream.read_exact(&mut buff)?;
    // TODO is utf8 the right encoding? I would expect ASCII which is a subset of UTF8
    String::from_utf8(buff.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_ch<A: Write + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    value: String,
) -> io::Result<()> {
    stream.seek(SeekFrom::Start(location))?;
    // if buffer contains non-ASCII characters, fail
    if !value.is_ascii() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("String is not ASCII: {}", value),
        ));
    }
    let buff = value.as_bytes().to_vec();
    if buff.len() > length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("String is too long: {} > {}", buff.len(), length),
        ));
    }
    stream.write_all(&buff)?;
    Ok(())
}

/// Read a binary coded decimal number from the nvram file
/// https://en.wikipedia.org/wiki/Binary-coded_decimal
///
/// # Arguments
///
/// * `nvram_file` - The file to read from
/// * `location` - The location in the file to start reading from
/// * `length` - The number of bytes to read
fn read_bcd<A: Read + Seek>(stream: &mut A, location: u64, length: usize) -> io::Result<u64> {
    stream.seek(SeekFrom::Start(location))?;
    let mut buff = vec![0; length];
    stream.read_exact(&mut buff)?;
    let mut score = 0;
    for i in 0..length {
        score *= 100;
        score += (buff[i] & 0x0F) as u64;
        score += ((buff[i] & 0xF0) >> 4) as u64 * 10;
    }
    Ok(score)
}

fn write_bcd<A: Write + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    value: u64,
) -> io::Result<()> {
    stream.seek(SeekFrom::Start(location))?;
    let mut buff = vec![0; length];
    let mut score = value;
    for i in 0..length {
        buff[i] = ((score % 10) + ((score / 10) << 4)) as u8;
        score /= 100;
    }
    stream.write_all(&buff)?;
    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct HighScore {
    pub label: Option<String>,
    pub short_label: Option<String>,
    pub initials: String,
    pub score: u64,
}

pub struct Nvram {
    pub map: NvramMap,
    pub nv_path: PathBuf,
}

impl Nvram {
    pub fn open(nv_path: &Path) -> io::Result<Option<Nvram>> {
        // get the rom name from the file name
        let rom_name = nv_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_string();
        // check if file exists
        if !nv_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {:?}", nv_path),
            ));
        }
        // can we read this rom file?
        // TODO some of the files can read multiple roms, we need an index
        if !PathBuf::from(format!("pinmame-nvram-maps/{}.nv.json", rom_name)).exists() {
            return Ok(None);
        }

        let map_file = File::open(format!("pinmame-nvram-maps/{}.nv.json", rom_name))?;
        let map: NvramMap = serde_json::from_reader(map_file)?;
        Ok(Some(Nvram {
            map,
            nv_path: nv_path.to_path_buf(),
        }))
    }

    pub fn read_highscores(&mut self) -> io::Result<Vec<HighScore>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_highscores(&self.map, &mut file)
    }

    pub fn clear_highscores(&mut self) -> io::Result<()> {
        // re-open the file in write mode
        let mut rw_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.nv_path)?;
        clear_highscores(&mut rw_file, &self.map)?;
        update_all_checksum16(&mut rw_file, &self.map)
    }

    pub fn verify_checksum16(&mut self) -> io::Result<Vec<ChecksumMismatch<u16>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        verify_all_checksum16(&mut file, &self.map)
    }
}

fn read_highscores<T: Read + Seek>(
    map: &NvramMap,
    mut nvram_file: &mut T,
) -> io::Result<Vec<HighScore>> {
    let scores: Result<Vec<HighScore>, io::Error> = map
        .high_scores
        .iter()
        .map(|hs| read_highscore(&mut nvram_file, hs))
        .collect();
    scores
}

fn read_highscore<T: Read + Seek>(
    mut nvram_file: &mut T,
    hs: &model::HighScore,
) -> io::Result<HighScore> {
    let mut initials = "???".to_string();
    if let Some(map_initials) = &hs.initials {
        initials = read_ch(
            &mut nvram_file,
            (&map_initials.start).into(),
            map_initials.length as usize,
        )?;
    }
    let mut score = 0;
    if let Some(map_score_start) = &hs.score.start {
        score = read_bcd(
            &mut nvram_file,
            map_score_start.into(),
            hs.score.length.unwrap_or(0) as usize,
        )?;
    }
    Ok(HighScore {
        label: hs.label.clone(),
        short_label: hs.short_label.clone(),
        initials,
        score,
    })
}

fn clear_highscores<T: Write + Seek>(mut nvram_file: &mut T, map: &NvramMap) -> io::Result<()> {
    for hs in &map.high_scores {
        if let Some(map_initials) = &hs.initials {
            write_ch(
                &mut nvram_file,
                (&map_initials.start).into(),
                map_initials.length as usize,
                "AAA".to_string(),
            )?;
        }
        if let Some(map_score_start) = &hs.score.start {
            write_bcd(
                &mut nvram_file,
                map_score_start.into(),
                hs.score.length.unwrap_or(0) as usize,
                0,
            )?;
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct ChecksumMismatch<T> {
    label: Option<String>,
    expected: T,
    calculated: T,
}

/// checksum16: An array of memory regions protected by a 16-bit checksum. The last two bytes of
/// the range are set so that adding all other bytes in the range results in a value of 0xFFFF.
fn verify_checksum16<T: Read + Seek>(
    nvram_file: &mut T,
    checksum16: &Checksum16,
    endian: &Endian,
) -> io::Result<Option<ChecksumMismatch<u16>>> {
    let start: u64 = (&checksum16.start).into();
    let end: u64 = (&checksum16.end).into();
    let length = (1 + end - start) as usize;

    nvram_file.seek(SeekFrom::Start(start))?;
    let mut buff = vec![0; length];
    nvram_file.read_exact(&mut buff)?;

    let stored_sum = match endian {
        Endian::Big => (buff.pop().unwrap() as u16) + ((buff.pop().unwrap() as u16) << 8),
        Endian::Little => ((buff.pop().unwrap() as u16) << 8) + buff.pop().unwrap() as u16,
    };

    // adding sum + all other bytes should result in 0xFFFF
    let calc_sum: u16 = 0xFFFFu16 - buff.iter().fold(0u16, |acc, &x| acc.wrapping_add(x as u16));
    if calc_sum != stored_sum {
        return Ok(Some(ChecksumMismatch {
            label: checksum16.label.clone(),
            expected: stored_sum,
            calculated: calc_sum,
        }));
    }
    Ok(None)
}

fn verify_all_checksum16<T: Read + Seek>(
    mut nvram_file: &mut T,
    map: &NvramMap,
) -> io::Result<Vec<ChecksumMismatch<u16>>> {
    let endian = map._endian.as_ref().or(map.endian.as_ref()).unwrap();
    map.checksum16
        .iter()
        .flatten()
        .map(|cs| verify_checksum16(&mut nvram_file, cs, endian))
        .filter_map(|r| r.transpose())
        .collect()
}

fn update_checksum16<T: Read + Seek + Write>(
    nvram_file: &mut T,
    checksum16: &Checksum16,
    endian: &Endian,
) -> io::Result<()> {
    let start: u64 = (&checksum16.start).into();
    let end: u64 = (&checksum16.end).into();
    let length = (1 + end - start) as usize;

    nvram_file.seek(SeekFrom::Start(start))?;
    let mut buff = vec![0; length - 2];
    nvram_file.read_exact(&mut buff)?;

    // adding sum + all other bytes should result in 0xFFFF
    let calc_sum: u16 = 0xFFFFu16 - buff.iter().fold(0u16, |acc, &x| acc.wrapping_add(x as u16));

    // push the calculated sum to the end of the buffer
    match endian {
        Endian::Big => {
            buff.push((calc_sum >> 8) as u8);
            buff.push((calc_sum & 0xFF) as u8);
        }
        Endian::Little => {
            buff.push((calc_sum & 0xFF) as u8);
            buff.push((calc_sum >> 8) as u8);
        }
    }

    nvram_file.seek(SeekFrom::Start(start))?;
    nvram_file.write_all(&buff)?;

    Ok(())
}

fn update_all_checksum16<T: Read + Seek + Write>(
    mut nvram_file: &mut T,
    map: &NvramMap,
) -> io::Result<()> {
    let endian = map._endian.as_ref().or(map.endian.as_ref()).unwrap();
    map.checksum16
        .iter()
        .flatten()
        .map(|cs| update_checksum16(&mut nvram_file, cs, endian))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn test_not_found() {
        let nvram = Nvram::open(Path::new("does_not_exist.nv"));
        assert!(matches!(
            nvram,
            Err(ref e) if e.kind() == io::ErrorKind::NotFound && e.to_string() == "File not found: \"does_not_exist.nv\""
        ));
    }

    #[test]
    fn test_no_map() -> io::Result<()> {
        let dir = testdir!();
        let test_file = dir.join("unknown_rom.nv");
        let _ = File::create(&test_file)?;
        let nvram = Nvram::open(&test_file)?;
        Ok(assert_eq!(true, nvram.is_none()))
    }

    #[test]
    fn test_demolition_man() -> io::Result<()> {
        let mut nvram = Nvram::open(Path::new("testdata/dm_lx4.nv"))?.unwrap();
        let scores = nvram.read_highscores()?;
        let expected = Vec::from([
            HighScore {
                label: Some("Grand Champion".to_string()),
                short_label: Some("GC".to_string()),
                initials: "TED".to_string(),
                score: 1_250_000_000,
            },
            HighScore {
                label: Some("First Place".to_string()),
                short_label: Some("1st".to_string()),
                initials: "WAG".to_string(),
                score: 950_000_000,
            },
            HighScore {
                label: Some("Second Place".to_string()),
                short_label: Some("2nd".to_string()),
                initials: "DEN".to_string(),
                score: 800_000_000,
            },
            HighScore {
                label: Some("Third Place".to_string()),
                short_label: Some("3rd".to_string()),
                initials: "DTW".to_string(),
                score: 650_000_000,
            },
            HighScore {
                label: Some("Fourth Place".to_string()),
                short_label: Some("4th".to_string()),
                initials: "HEY".to_string(),
                score: 500_000_000,
            },
        ]);

        Ok(assert_eq!(expected, scores))
    }

    #[test]
    fn test_checksum16() -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open("testdata/dm_lx4.nv")?;
        let nvram = Nvram::open(Path::new("testdata/dm_lx4.nv"))?.unwrap();
        let checksum_failures = verify_all_checksum16(&mut file, &nvram.map)?;
        Ok(assert_eq!(
            Vec::<ChecksumMismatch<u16>>::new(),
            checksum_failures
        ))
    }

    #[test]
    fn test_clear_demolition_man_scores() -> io::Result<()> {
        let dir = testdir!();
        let test_file = dir.join("dm_lx4.nv");
        // copy the test file to the test directory
        std::fs::copy("testdata/dm_lx4.nv", &test_file)?;
        let mut nvram = Nvram::open(&test_file)?.unwrap();
        nvram.clear_highscores()?;
        let scores = nvram.read_highscores()?;
        let expected = Vec::from([
            HighScore {
                label: Some("Grand Champion".to_string()),
                short_label: Some("GC".to_string()),
                initials: "AAA".to_string(),
                score: 0,
            },
            HighScore {
                label: Some("First Place".to_string()),
                short_label: Some("1st".to_string()),
                initials: "AAA".to_string(),
                score: 0,
            },
            HighScore {
                label: Some("Second Place".to_string()),
                short_label: Some("2nd".to_string()),
                initials: "AAA".to_string(),
                score: 0,
            },
            HighScore {
                label: Some("Third Place".to_string()),
                short_label: Some("3rd".to_string()),
                initials: "AAA".to_string(),
                score: 0,
            },
            HighScore {
                label: Some("Fourth Place".to_string()),
                short_label: Some("4th".to_string()),
                initials: "AAA".to_string(),
                score: 0,
            },
        ]);

        assert_eq!(expected, scores);

        let checksum_failures = nvram.verify_checksum16()?;
        Ok(assert_eq!(
            Vec::<ChecksumMismatch<u16>>::new(),
            checksum_failures
        ))
    }
}
