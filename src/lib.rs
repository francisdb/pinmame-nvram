mod model;

use crate::model::{Checksum16, Endian, NvramMap};
use include_dir::{include_dir, Dir};
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

static MAPS: Dir = include_dir!("pinmame-nvram-maps");

fn read_ch<A: Read + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    mask: Option<u64>,
    char_map: &Option<String>,
) -> io::Result<String> {
    stream.seek(SeekFrom::Start(location))?;
    let mut buff = vec![0; length];
    stream.read_exact(&mut buff)?;
    // apply mask if set
    if let Some(mask) = mask {
        for b in buff.iter_mut() {
            *b &= mask as u8;
        }
    }
    // if char_map is set, convert the buffer to a string
    if let Some(char_map) = char_map {
        let mut result = String::new();
        for b in buff.iter() {
            result.push(char_map.chars().nth(*b as usize).unwrap_or('?'));
        }
        return Ok(result);
    }

    String::from_utf8(buff.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_ch<A: Write + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    value: String,
    char_map: &Option<String>,
) -> io::Result<()> {
    stream.seek(SeekFrom::Start(location))?;
    // if buffer contains non-ASCII characters, fail
    if !value.is_ascii() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("String is not ASCII: {}", value),
        ));
    }
    let mut buff = value.as_bytes().to_vec();
    if let Some(char_map) = char_map {
        for b in buff.iter_mut() {
            let idx = char_map.find(*b as char).unwrap_or(0);
            *b = idx as u8;
        }
    }
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
    for item in buff.iter() {
        score *= 100;
        score += (item & 0x0F) as u64;
        score += ((item & 0xF0) >> 4) as u64 * 10;
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
    for b in buff.iter_mut() {
        *b = ((score % 10) + ((score / 10) << 4)) as u8;
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

        let map_opt = find_map(&rom_name)?;
        Ok(map_opt.map(|map| Nvram {
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

    pub fn verify_all_checksum16(&mut self) -> io::Result<Vec<ChecksumMismatch<u16>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        verify_all_checksum16(&mut file, &self.map)
    }
}

fn find_map(rom_name: &String) -> io::Result<Option<NvramMap>> {
    let map_name = format!("{}.nv.json", rom_name);
    if let Some(map_file) = MAPS.get_file(&map_name) {
        let cursor = io::Cursor::new(map_file.contents());
        let map = serde_json::from_reader(cursor)?;
        return Ok(Some(map));
    }
    // Preferably we would have a pre-filtered MAPS, see
    // https://github.com/Michael-F-Bryan/include_dir/issues/81
    for entry in MAPS
        .files()
        .filter(|entry| entry.path().extension().unwrap_or(OsStr::new("")) == "json")
    {
        let cursor = io::Cursor::new(entry.contents());
        let map: NvramMap = serde_json::from_reader(cursor)?;
        if map._roms.contains(rom_name) {
            return Ok(Some(map));
        }
    }
    Ok(None)
}

fn read_highscores<T: Read + Seek>(
    map: &NvramMap,
    mut nvram_file: &mut T,
) -> io::Result<Vec<HighScore>> {
    let scores: Result<Vec<HighScore>, io::Error> = map
        .high_scores
        .iter()
        .map(|hs| read_highscore(&mut nvram_file, hs, &map._char_map))
        .collect();
    scores
}

fn read_highscore<T: Read + Seek>(
    mut nvram_file: &mut T,
    hs: &model::HighScore,
    char_map: &Option<String>,
) -> io::Result<HighScore> {
    let mut initials = "???".to_string();
    if let Some(map_initials) = &hs.initials {
        initials = read_ch(
            &mut nvram_file,
            (&map_initials.start).into(),
            map_initials.length as usize,
            map_initials.mask.as_ref().map(|m| m.into()),
            char_map,
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
                &map._char_map,
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
    let endian = map._endian.as_ref().unwrap();
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
    let endian = map._endian.as_ref().unwrap();
    map.checksum16
        .iter()
        .flatten()
        .try_for_each(|cs| update_checksum16(&mut nvram_file, cs, endian))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs::File;
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
    fn test_read_bcd() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x12, 0x34, 0x56, 0x78, 0x90]);
        let score = read_bcd(&mut cursor, 0x0000, 5)?;
        Ok(assert_eq!(score, 1_234_567_890))
    }

    #[test]
    fn test_read_ch() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x41, 0x42, 0x43, 0x44, 0x45]);
        let score = read_ch(&mut cursor, 0x0000, 5, None, &None)?;
        Ok(assert_eq!(score, "ABCDE"))
    }

    #[test]
    fn test_read_ch_with_charmap() -> io::Result<()> {
        let char_map = Some("???????????ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        let mut cursor = io::Cursor::new(vec![0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
        let score = read_ch(&mut cursor, 0x0000, 5, None, &char_map)?;
        Ok(assert_eq!(score, "ABCDE"))
    }

    #[test]
    fn test_write_ch() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        write_ch(&mut cursor, 0x0000, 5, "ABCDE".to_string(), &None)?;
        Ok(assert_eq!(
            cursor.into_inner(),
            vec![0x41, 0x42, 0x43, 0x44, 0x45]
        ))
    }

    #[test]
    fn test_write_ch_with_charmap() -> io::Result<()> {
        let char_map = Some("???????????ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        let mut cursor = io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        write_ch(&mut cursor, 0x0000, 5, "ABCDE".to_string(), &char_map)?;
        Ok(assert_eq!(
            cursor.into_inner(),
            vec![0x0B, 0x0C, 0x0D, 0x0E, 0x0F]
        ))
    }

    #[test]
    fn test_find_map() -> io::Result<()> {
        let map = find_map(&"afm_113b".to_string())?;
        Ok(assert_eq!(true, map.is_some()))
    }
}
