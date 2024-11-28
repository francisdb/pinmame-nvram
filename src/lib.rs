mod model;

use crate::model::{Checksum16, Encoding, Endian, Nibble, NvramMap, Score};
use include_dir::{include_dir, Dir};
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

static MAPS: Dir = include_dir!("pinmame-nvram-maps");

fn de_nibble(length: usize, buff: &[u8], nibble: &Nibble) -> io::Result<Vec<u8>> {
    if nibble == &Nibble::High && length % 2 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be even when reading the high nibble",
        ));
    }
    // TODO make this more efficient
    let resulting_length = (length + 1) / 2;
    let mut result = vec![0; resulting_length];
    let mut buffer = buff.to_owned();
    if length % 2 != 0 {
        // prepend 0
        buffer = vec![0].into_iter().chain(buffer).collect();
    };
    let mut iter = buffer.into_iter();
    for b in result.iter_mut() {
        // if uneven the high byte should be 0
        let high = iter.next().unwrap();
        let low = iter.next().unwrap();
        *b = match nibble {
            Nibble::High => (high & 0xF0) | ((low & 0xF0) >> 4),
            Nibble::Low => ((high & 0x0F) << 4) | (low & 0x0F),
        };
    }
    Ok(result)
}

fn do_nibble(length: usize, buff: &[u8], nibble: &Nibble) -> io::Result<Vec<u8>> {
    if nibble == &Nibble::High && length % 2 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be even when writing the high nibble",
        ));
    }
    if nibble == &Nibble::Low && length % 2 != 0 && buff[0] > 0x0F {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "When writing the low nibble for an uneven length the first nibble should be 0",
        ));
    }
    if length < buff.len() * 2 - 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be at least twice the length of the buffer minus 1",
        ));
    }
    if length > buff.len() * 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Length should be at most twice the length of the buffer",
        ));
    }
    let mut result = Vec::with_capacity(length);
    for b in buff.iter() {
        match nibble {
            Nibble::High => {
                result.push(b & 0xF0);
                result.push((b & 0x0F) << 4);
            }
            Nibble::Low => {
                result.push((b & 0xF0) >> 4);
                result.push(b & 0x0F);
            }
        }
    }
    // remove the first byte if the length is uneven
    if length % 2 != 0 {
        result.remove(0);
    }
    Ok(result)
}

fn read_ch<A: Read + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    mask: Option<u64>,
    char_map: &Option<String>,
    nibble: &Option<Nibble>,
) -> io::Result<String> {
    stream.seek(SeekFrom::Start(location))?;

    let mut buff = vec![0; length];
    stream.read_exact(&mut buff)?;

    if let Some(nibble) = nibble {
        let result = de_nibble(length, &buff, nibble)?;
        // filter out zero bytes
        let result = result.into_iter().filter(|&b| b != 0).collect::<Vec<u8>>();
        return String::from_utf8(result.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
    }

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
    nibble: &Option<Nibble>,
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

    if let Some(nibble) = nibble {
        buff = do_nibble(length, &buff, nibble)?;
    }

    stream.write_all(&buff)?;
    Ok(())
}

pub enum Location {
    Continuous { start: u64, length: usize },
    Discontinuous { offsets: Vec<u64> },
}

/// Read a binary coded decimal number from the nvram file
/// https://en.wikipedia.org/wiki/Binary-coded_decimal
///
/// # Arguments
///
/// * `nvram_file` - The file to read from
/// * `location` - The location in the file to start reading from
/// * `length` - The number of bytes to read
fn read_bcd<A: Read + Seek>(
    stream: &mut A,
    location: Location,
    nibble: &Option<Nibble>,
    scale: u64,
    endian: Endian,
) -> io::Result<u64> {
    let mut buff = match location {
        Location::Continuous { start, length } => {
            stream.seek(SeekFrom::Start(start))?;
            let mut buff = vec![0; length];
            stream.read_exact(&mut buff)?;
            buff
        }
        Location::Discontinuous { offsets } => {
            let mut buff = vec![0; offsets.len()];
            for offset in offsets.iter() {
                stream.seek(SeekFrom::Start(*offset))?;
                let mut byte = [0; 1];
                stream.read_exact(&mut byte)?;
                buff.push(byte[0]);
            }
            buff
        }
    };

    if endian == Endian::Little {
        buff.reverse();
    }

    if let Some(nibble) = nibble {
        buff = de_nibble(buff.len(), &buff, nibble)?;
    }

    let mut score = 0;
    for item in buff.iter() {
        score *= 100;
        score += cap_bcd(item & 0x0F) as u64;
        score += cap_bcd((item & 0xF0) >> 4) as u64 * 10;
    }
    Ok(score * scale)
}

/// Ignore nibbles 0xA to 0xF (0xF = blank on Dracula/Wild Fyre) (prefix)
fn cap_bcd(value: u8) -> u8 {
    if value > 9 {
        0
    } else {
        value
    }
}

fn write_bcd<A: Write + Seek>(
    stream: &mut A,
    location: u64,
    length: usize,
    nibble: &Option<Nibble>,
    value: u64,
) -> io::Result<()> {
    stream.seek(SeekFrom::Start(location))?;
    // the nibble function will validate the length
    let buff_len = if nibble.is_some() {
        (length + 1) / 2
    } else {
        length
    };
    let mut buff = vec![0; buff_len];
    let mut score = value;
    for b in buff.iter_mut() {
        *b = ((score % 10) + ((score / 10) << 4)) as u8;
        score /= 100;
    }

    if let Some(nibble) = nibble {
        buff = do_nibble(length, &buff, nibble)?;
    }

    stream.write_all(&buff)?;
    Ok(())
}

fn read_int<T: Read + Seek>(
    nvram_file: &mut &mut T,
    endian: Endian,
    start: u64,
    length: usize,
) -> io::Result<u64> {
    nvram_file.seek(SeekFrom::Start(start))?;
    let mut buff = vec![0; length];
    nvram_file.read_exact(&mut buff)?;
    let score = match endian {
        Endian::Big => buff
            .iter()
            .fold(0u64, |acc, &x| acc.wrapping_shl(8).wrapping_add(x as u64)),
        Endian::Little => buff
            .iter()
            .rev()
            .fold(0u64, |acc, &x| acc.wrapping_shl(8).wrapping_add(x as u64)),
    };
    Ok(score)
}

/// A special type for a real-time clock value from a WPC game,
/// stored as a sequence of 7 bytes:
/// * two-byte year (2015 is 0x07 0xDF),
/// * month (1-12),
/// * day of month (1-31),
/// * day of the week (0-6, 0=Sunday),
/// * hour (0-23)
/// * minute (0-59).
///
fn read_wpc_rtc<T: Read + Seek>(
    nvram_file: &mut &mut T,
    start: u64,
    length: usize,
) -> io::Result<String> {
    nvram_file.seek(SeekFrom::Start(start))?;
    let mut buff = vec![0; length];
    nvram_file.read_exact(&mut buff)?;
    let year = (buff[0] as u16) << 8 | buff[1] as u16;
    let month = buff[2];
    let day = buff[3];
    let _dow = buff[4];
    let hour = buff[5];
    let minute = buff[6];
    // output as "YYYY-MM-DD HH:MM"
    Ok(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hour, minute
    ))
}

#[derive(Debug, PartialEq)]
pub struct HighScore {
    pub label: Option<String>,
    pub short_label: Option<String>,
    pub initials: String,
    pub score: u64,
}

#[derive(Debug, PartialEq)]
// probably one of both, score or timestamp
pub struct ModeChampion {
    pub label: Option<String>,
    pub short_label: Option<String>,
    pub initials: String,
    pub score: Option<u64>,
    pub suffix: Option<String>,
    pub timestamp: Option<String>,
}

/// Score of the last game played
/// For each player that played during the last game, the score is stored.
#[derive(Debug, PartialEq)]
pub struct LastGamePlayer {
    pub score: u64,
    pub label: Option<String>,
}

/// Main interface to read and write data from a NVRAM file
pub struct Nvram {
    pub map: NvramMap,
    pub nv_path: PathBuf,
}

impl Nvram {
    /// Open a NVRAM file
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Nvram))` if the file was found and a map was found for the ROM
    /// * `Ok(None)` if the file was found but no map was found for the ROM
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

    pub fn read_mode_champions(&mut self) -> io::Result<Option<Vec<ModeChampion>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_mode_champions(&mut file, &self.map)
    }

    pub fn read_last_game(&mut self) -> io::Result<Option<Vec<LastGamePlayer>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_last_game(&mut file, &self.map)
    }

    pub fn verify_all_checksum16(&mut self) -> io::Result<Vec<ChecksumMismatch<u16>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        verify_all_checksum16(&mut file, &self.map)
    }

    pub fn read_replay_score(&mut self) -> io::Result<Option<u64>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_replay_score(&mut file, &self.map)
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
        .map(|hs| read_highscore(&mut nvram_file, hs, &map._char_map, map.endianness()))
        .collect();
    scores
}

fn read_highscore<T: Read + Seek>(
    mut nvram_file: &mut T,
    hs: &model::HighScore,
    char_map: &Option<String>,
    endian: Endian,
) -> io::Result<HighScore> {
    let mut initials = "".to_string();
    if let Some(map_initials) = &hs.initials {
        initials = read_ch(
            &mut nvram_file,
            (&map_initials.start).into(),
            map_initials.length as usize,
            map_initials.mask.as_ref().map(|m| m.into()),
            char_map,
            &map_initials.nibble,
        )?;
    }
    let score = match &hs.score.encoding {
        Encoding::Bcd => {
            let location = match &hs.score.offsets.as_ref() {
                None => Location::Continuous {
                    start: hs.score.start.as_ref().unwrap().into(),
                    length: hs.score.length.unwrap_or(0) as usize,
                },
                Some(offsets) => Location::Discontinuous {
                    offsets: offsets.iter().map(|o| o.into()).collect(),
                },
            };
            read_bcd(
                &mut nvram_file,
                location,
                &hs.score.nibble,
                hs.score.scale.unwrap_or(1u64),
                endian,
            )?
        }
        Encoding::Int => {
            if let Some(map_score_start) = &hs.score.start {
                read_int(
                    &mut nvram_file,
                    endian,
                    map_score_start.into(),
                    hs.score.length.unwrap_or(0) as usize,
                )?
            } else {
                todo!("Int requires start")
            }
        }
        other => todo!("Encoding not implemented: {:?}", other),
    };

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
                &map_initials.nibble,
            )?;
        }
        if let Some(map_score_start) = &hs.score.start {
            write_bcd(
                &mut nvram_file,
                map_score_start.into(),
                hs.score.length.unwrap_or(0) as usize,
                &hs.score.nibble,
                0,
            )?;
        }
    }
    Ok(())
}

fn read_mode_champion<T: Read + Seek>(
    mut nvram_file: &mut T,
    mc: &model::ModeChampion,
    char_map: &Option<String>,
    endian: Endian,
) -> io::Result<ModeChampion> {
    let initials = read_ch(
        &mut nvram_file,
        (&mc.initials.start).into(),
        mc.initials.length as usize,
        mc.initials.mask.as_ref().map(|m| m.into()),
        char_map,
        &mc.initials.nibble,
    )?;
    let score = if let Some(score) = &mc.score.as_ref() {
        match &score.encoding {
            Encoding::Bcd => {
                let location = location_for(score);
                let result = read_bcd(
                    &mut nvram_file,
                    location,
                    &score.nibble,
                    score.scale.unwrap_or(1u64),
                    endian,
                )?;
                Some(result)
            }
            Encoding::Int => {
                if let Some(map_score_start) = &score.start {
                    let result = read_int(
                        &mut nvram_file,
                        endian,
                        map_score_start.into(),
                        score.length.unwrap_or(0) as usize,
                    )?;
                    Some(result)
                } else {
                    todo!("Int requires start")
                }
            }
            other => todo!("Encoding not implemented: {:?}", other),
        }
    } else {
        None
    };

    let timestamp = mc
        .timestamp
        .as_ref()
        .map(|ts| match &ts.encoding {
            Encoding::WpcRtc => {
                read_wpc_rtc(&mut nvram_file, (&ts.start).into(), ts.length as usize)
            }
            other => todo!("Timestamp encoding not implemented: {:?}", other),
        })
        .transpose()?;

    Ok(ModeChampion {
        label: Some(mc.label.clone()),
        short_label: mc.short_label.clone(),
        initials,
        score,
        suffix: mc.score.as_ref().and_then(|s| s.suffix.clone()),
        timestamp,
    })
}

fn read_last_game_player<T: Read + Seek>(
    mut nvram_file: &mut T,
    lg: &model::LastGamePlayer,
    endian: Endian,
) -> io::Result<LastGamePlayer> {
    let score = match &lg.encoding {
        Encoding::Int => read_int(
            &mut nvram_file,
            endian,
            (&lg.start).into(),
            lg.length as usize,
        )?,
        Encoding::Bcd => read_bcd(
            &mut nvram_file,
            Location::Continuous {
                start: (&lg.start).into(),
                length: lg.length as usize,
            },
            &lg.nibble,
            1,
            endian,
        )?,
        other => todo!("Encoding not implemented: {:?}", other),
    };
    Ok(LastGamePlayer {
        score,
        label: lg.label.clone(),
    })
}

fn read_last_game<T: Read + Seek>(
    mut nvram_file: &mut T,
    map: &NvramMap,
) -> io::Result<Option<Vec<LastGamePlayer>>> {
    if let Some(lg) = &map.last_game {
        let last_games: Result<Vec<LastGamePlayer>, io::Error> = lg
            .iter()
            .map(|lg| read_last_game_player(&mut nvram_file, lg, map.endianness()))
            .collect();
        Ok(Some(last_games?))
    } else {
        Ok(None)
    }
}

fn read_mode_champions<T: Read + Seek>(
    mut nvram_file: &mut T,
    map: &NvramMap,
) -> io::Result<Option<Vec<ModeChampion>>> {
    if let Some(mode_champions) = &map.mode_champions {
        let champions: Result<Vec<ModeChampion>, io::Error> = mode_champions
            .iter()
            .map(|mc| read_mode_champion(&mut nvram_file, mc, &map._char_map, map.endianness()))
            .collect();
        Ok(Some(champions?))
    } else {
        Ok(None)
    }
}

fn read_replay_score<T: Read + Seek>(
    mut nvram_file: &mut T,
    map: &NvramMap,
) -> io::Result<Option<u64>> {
    if let Some(replay_score) = &map.replay_score {
        match &replay_score.encoding {
            Encoding::Int => {
                if let Some(map_score_start) = &replay_score.start {
                    let score = read_int(
                        &mut nvram_file,
                        map.endianness(),
                        map_score_start.into(),
                        replay_score.length.unwrap_or(0) as usize,
                    )?;
                    Ok(Some(score))
                } else {
                    todo!("Int requires start")
                }
            }
            Encoding::Bcd => {
                let location = location_for(replay_score);
                let score = read_bcd(
                    &mut nvram_file,
                    location,
                    &replay_score.nibble,
                    replay_score.scale.unwrap_or(1u64),
                    map.endianness(),
                )?;
                Ok(Some(score))
            }
            other => todo!("Encoding not implemented: {:?}", other),
        }
    } else {
        Ok(None)
    }
}

fn location_for(score: &Score) -> Location {
    match score.offsets.as_ref() {
        None => Location::Continuous {
            start: score.start.as_ref().unwrap().into(),
            length: score.length.unwrap_or(0) as usize,
        },
        Some(offsets) => Location::Discontinuous {
            offsets: offsets.iter().map(|o| o.into()).collect(),
        },
    }
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
        let location = Location::Continuous {
            start: 0,
            length: 5,
        };
        let score = read_bcd(&mut cursor, location, &None, 1, Endian::Big)?;
        Ok(assert_eq!(score, 1_234_567_890))
    }

    #[test]
    fn test_read_ch() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x41, 0x42, 0x43, 0x44, 0x45]);
        let score = read_ch(&mut cursor, 0x0000, 5, None, &None, &None)?;
        Ok(assert_eq!(score, "ABCDE"))
    }

    #[test]
    fn test_read_ch_with_charmap() -> io::Result<()> {
        let char_map = Some("???????????ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        let mut cursor = io::Cursor::new(vec![0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
        let score = read_ch(&mut cursor, 0x0000, 5, None, &char_map, &None)?;
        Ok(assert_eq!(score, "ABCDE"))
    }

    #[test]
    fn test_write_ch() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        write_ch(&mut cursor, 0x0000, 5, "ABCDE".to_string(), &None, &None)?;
        Ok(assert_eq!(
            cursor.into_inner(),
            vec![0x41, 0x42, 0x43, 0x44, 0x45]
        ))
    }

    #[test]
    fn test_write_ch_with_charmap() -> io::Result<()> {
        let char_map = Some("???????????ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
        let mut cursor = io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        write_ch(
            &mut cursor,
            0x0000,
            5,
            "ABCDE".to_string(),
            &char_map,
            &None,
        )?;
        Ok(assert_eq!(
            cursor.into_inner(),
            vec![0x0B, 0x0C, 0x0D, 0x0E, 0x0F]
        ))
    }

    #[test]
    fn test_read_ch_with_nibble() -> io::Result<()> {
        // Nibble: where the sequence 0x04 0x01 0x04 0x02 0x04 0x03
        // translates to 0x41 0x42 0x43 which is the string "ABC"
        let mut cursor = io::Cursor::new(vec![0x04, 0x01, 0x04, 0x02, 0x04, 0x03]);
        let score = read_ch(&mut cursor, 0x0000, 6, None, &None, &Some(Nibble::Low))?;
        Ok(assert_eq!(score, "ABC"))
    }

    #[test]
    fn test_do_nibble_even() {
        let buff = vec![0x41, 0x42, 0x43];
        let result = do_nibble(6, &buff, &Nibble::Low).unwrap();
        assert_eq!(result, vec![0x04, 0x01, 0x04, 0x02, 0x04, 0x03]);
    }

    #[test]
    fn test_do_nibble_uneven() {
        let buff = vec![0x01, 0x42, 0x43];
        let result = do_nibble(5, &buff, &Nibble::Low).unwrap();
        assert_eq!(result, vec![0x01, 0x04, 0x02, 0x04, 0x03]);
    }

    #[test]
    #[should_panic(
        expected = "When writing the low nibble for an uneven length the first nibble should be 0"
    )]
    fn test_do_nibble_uneven_fail_drop() {
        let buff = vec![0x11, 0x42, 0x43];
        do_nibble(5, &buff, &Nibble::Low).unwrap();
    }

    #[test]
    #[should_panic(expected = "Length should be at most twice the length of the buffer")]
    fn test_do_nibble_uneven_fail_length() {
        let buff = vec![0x01, 0x42];
        do_nibble(6, &buff, &Nibble::Low).unwrap();
    }

    #[test]
    #[should_panic(expected = "Length should be at least twice the length of the buffer minus 1")]
    fn test_do_nibble_uneven_fail_length2() {
        let buff = vec![0x01, 0x42, 0x43];
        do_nibble(2, &buff, &Nibble::Low).unwrap();
    }

    #[test]
    fn test_de_nibble_even() {
        let buff = vec![0x40, 0x10, 0x40, 0x20, 0x40, 0x30];
        let result = de_nibble(6, &buff, &Nibble::High).unwrap();
        assert_eq!(result, vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn test_de_nibble_uneven() {
        let buff = vec![0x04, 0x01, 0x04, 0x02, 0x04];
        let result = de_nibble(5, &buff, &Nibble::Low).unwrap();
        assert_eq!(result, vec![0x04, 0x14, 0x24]);
    }

    #[test]
    fn test_find_map() -> io::Result<()> {
        let map = find_map(&"afm_113b".to_string())?;
        Ok(assert_eq!(true, map.is_some()))
    }
}
