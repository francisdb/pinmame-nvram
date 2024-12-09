mod encoding;
mod model;
pub mod resolve;

use crate::encoding::{read_bcd, read_ch, read_int, read_wpc_rtc, write_bcd, write_ch, Location};
use crate::model::{Checksum16, Encoding, Endian, NvramMap, Score, StateOrStateList};
use include_dir::{include_dir, Dir, File};
use serde::de;
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

static MAPS: Dir = include_dir!("$OUT_DIR/maps.brotli");

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
        let map_opt: Option<NvramMap> = open_nvram(nv_path)?;
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

    // TODO we probably want to remove this
    pub fn read_replay_score(&mut self) -> io::Result<Option<u64>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_replay_score(&mut file, &self.map)
    }

    pub fn read_game_state(&mut self) -> io::Result<Option<HashMap<String, String>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_game_state(&mut file, &self.map)
    }

    pub fn dip_switches_len(&self) -> usize {
        // TODO get the number of dip switches from the map
        // centaur
        // 32 default switches
        // 3 additional switches for the sound board reverb effect
        32 + 3
    }

    /// Get the value of a dip switch
    /// # Arguments
    /// * `number` - The number of the dip switch to get, 1-based!
    /// # Returns
    /// * `Ok(true)` if the dip switch is ON
    /// * `Ok(false)` if the dip switch is OFF
    /// * `Err(io::Error)` if the dip switch number is out of range or an IO error occurred
    pub fn get_dip_switch(&self, number: usize) -> io::Result<bool> {
        let switch_count = self.dip_switches_len();
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        get_dip_switch(&mut file, switch_count, number)
    }

    /// Set a dip switch to on or off
    /// # Arguments
    /// * `number` - The number of the dip switch to set, 1-based!
    /// * `on` - `true` to set the dip switch to ON, `false` to set it to OFF
    /// # Returns
    /// * `Ok(())` if the dip switch was set successfully
    /// * `Err(io::Error)` if the dip switch number is out of range or an IO error occurred
    pub fn set_dip_switch(&self, number: usize, on: bool) -> io::Result<()> {
        let switch_count = self.dip_switches_len();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.nv_path)?;
        set_dip_switch(&mut file, switch_count, number, on)
    }
}

trait ReadRoms {
    fn roms(&self) -> Vec<String>;
}

impl ReadRoms for NvramMap {
    fn roms(&self) -> Vec<String> {
        // TODO avoid the clone
        self._roms.clone()
    }
}

impl ReadRoms for Value {
    fn roms(&self) -> Vec<String> {
        // find "_roms" property and read as string
        self["_roms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
    }
}

fn open_nvram<T: ReadRoms + de::DeserializeOwned>(nv_path: &Path) -> io::Result<Option<T>> {
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
    find_map(&rom_name)
}

fn find_map<T: ReadRoms + de::DeserializeOwned>(rom_name: &String) -> io::Result<Option<T>> {
    let map_name = format!("{}.json.brotli", rom_name);
    if let Some(map_file) = MAPS.get_file(&map_name) {
        let map: T = read_compressed_map(map_file)?;
        // check that the rom name is in the map
        let roms = map.roms();
        if !roms.contains(rom_name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Map for {}.nv.json found but {} not in _roms list: {}",
                    rom_name,
                    rom_name,
                    roms.join(", ")
                ),
            ));
        }
        return Ok(Some(map));
    }
    for entry in MAPS.files() {
        let map: T = read_compressed_map(entry)?;
        if map.roms().contains(rom_name) {
            return Ok(Some(map));
        }
    }
    Ok(None)
}

fn read_compressed_map<T: de::DeserializeOwned>(map_file: &File) -> io::Result<T> {
    let mut cursor = io::Cursor::new(map_file.contents());
    let reader = brotli::Decompressor::new(&mut cursor, 4096);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
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
                Some(offsets) => Location::Scattered {
                    offsets: offsets.iter().map(|o| o.into()).collect(),
                },
            };
            read_bcd(
                &mut nvram_file,
                location,
                &hs.score.nibble,
                hs.score.scale.as_ref().unwrap_or(&Number::from(1u64)),
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
                    score.scale.as_ref().unwrap_or(&Number::from(1)),
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
            &Number::from(1),
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

fn read_game_state_item<T: Read + Seek>(
    mut nvram_file: &mut T,
    state: &model::State,
    char_map: &Option<String>,
    endian: Endian,
) -> io::Result<String> {
    match &state.encoding {
        Encoding::Ch => read_ch(
            &mut nvram_file,
            (&state.start).into(),
            state.length.unwrap_or(0),
            state.mask.as_ref().map(|m| m.into()),
            char_map,
            &state.nibble,
        ),
        Encoding::Int => {
            let score = read_int(
                &mut nvram_file,
                endian,
                (&state.start).into(),
                state.length.unwrap_or(0),
            )?;
            Ok(score.to_string())
        }
        Encoding::Bcd => {
            let score = read_bcd(
                &mut nvram_file,
                Location::Continuous {
                    start: (&state.start).into(),
                    length: state.length.unwrap_or(0),
                },
                &state.nibble,
                &Number::from(1),
                endian,
            )?;
            Ok(score.to_string())
        }
        Encoding::Bits => Ok("Bits encoding not implemented".to_string()),
        other => todo!("Encoding not implemented: {:?}", other),
    }
}

fn read_game_state<T: Read + Seek>(
    mut nvram_file: &mut T,
    map: &NvramMap,
) -> io::Result<Option<HashMap<String, String>>> {
    if let Some(game_state) = &map.game_state {
        // map the hashmap values to a new hashmap with the values read from the nvram file
        let state: Result<HashMap<String, String>, io::Error> = game_state
            .iter()
            .flat_map(|(key, v)| match v {
                StateOrStateList::State(s) => {
                    let r =
                        read_game_state_item(&mut nvram_file, s, &map._char_map, map.endianness())
                            .map(|r| (key.clone(), r));
                    vec![r]
                }
                StateOrStateList::StateList(sl) => sl
                    .iter()
                    .enumerate()
                    .map(|(index, s)| {
                        let compund_key = format!("{}.{}", key, index);
                        read_game_state_item(&mut nvram_file, s, &map._char_map, map.endianness())
                            .map(|r| (compund_key, r))
                    })
                    .collect(),
            })
            .collect();

        Ok(Some(state?))
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
                    replay_score.scale.as_ref().unwrap_or(&Number::from(1)),
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
        Some(offsets) => Location::Scattered {
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
    endian: Endian,
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
        .map(|cs| verify_checksum16(&mut nvram_file, cs, *endian))
        .filter_map(|r| r.transpose())
        .collect()
}

fn update_checksum16<T: Read + Seek + Write>(
    nvram_file: &mut T,
    checksum16: &Checksum16,
    endian: Endian,
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
        .try_for_each(|cs| update_checksum16(&mut nvram_file, cs, *endian))
}

/// Number of bytes appended to the NVRAM for dip switches
///
/// PinMAME has a maximum of 10 banks with 8 switches each
/// Somehow only 6 bytes are written to the nvram file
/// https://github.com/vpinball/pinmame/blob/f14bbc89c48d0ecb0d44d4be7a694730cfbf24e1/src/wpc/core.c#L2303-L2309
const DIP_SWITCH_BYTES: i64 = 6;

pub fn get_dip_switch<T: Read + Seek>(
    nvram_file: &mut T,
    switch_count: usize,
    number: usize,
) -> io::Result<bool> {
    validate_dip_switch_range(switch_count, number)?;
    let index = number - 1;
    let register = index / 8;
    let bit = index % 8;
    let mut buff = [0; 1];
    nvram_file.seek(SeekFrom::End(-DIP_SWITCH_BYTES + register as i64))?;
    nvram_file.read_exact(&mut buff)?;
    Ok((buff[0] & (1 << bit)) != 0)
}

pub fn set_dip_switch<T: Read + Write + Seek>(
    nvram_file: &mut T,
    switch_count: usize,
    number: usize,
    on: bool,
) -> io::Result<()> {
    validate_dip_switch_range(switch_count, number)?;
    let index = number - 1;
    let register = index / 8;
    let bit = index % 8;
    // write single byte with value
    let mut buff = [0; 1];
    nvram_file.seek(SeekFrom::End(-DIP_SWITCH_BYTES + register as i64))?;
    nvram_file.read_exact(&mut buff)?;
    if on {
        buff[0] |= 1 << bit;
    } else {
        buff[0] &= !(1 << bit);
    }
    nvram_file.seek(SeekFrom::End(-DIP_SWITCH_BYTES + register as i64))?;
    nvram_file.write_all(&buff)
}

fn validate_dip_switch_range(switch_count: usize, number: usize) -> io::Result<()> {
    const MAX_SWITCH_COUNT: i64 = DIP_SWITCH_BYTES * 8;
    if switch_count > (MAX_SWITCH_COUNT) as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Switch count {} out of range, expected 1-{}",
                switch_count, MAX_SWITCH_COUNT
            ),
        ));
    }
    if number < 1 || number > switch_count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Dip switch #{} out of range, expected 1-{}",
                number, switch_count
            ),
        ));
    }
    Ok(())
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
        assert_eq!(true, nvram.is_none());
        Ok(())
    }

    #[test]
    fn test_checksum16() -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open("testdata/dm_lx4.nv")?;
        let nvram = Nvram::open(Path::new("testdata/dm_lx4.nv"))?.unwrap();
        let checksum_failures = verify_all_checksum16(&mut file, &nvram.map)?;
        assert_eq!(Vec::<ChecksumMismatch<u16>>::new(), checksum_failures);
        Ok(())
    }

    #[test]
    fn test_find_map() -> io::Result<()> {
        let map: Option<Value> = find_map(&"afm_113b".to_string())?;
        assert_eq!(true, map.is_some());
        Ok(())
    }

    #[test]
    fn test_dip_switches() -> io::Result<()> {
        // cursor with 8 bytes, we always have 6 dip switch bytes
        let switch_count = 32;
        let mut cursor = io::Cursor::new(vec![0; 8]);
        let switch1 = get_dip_switch(&mut cursor, switch_count, 1)?;
        let switch2 = get_dip_switch(&mut cursor, switch_count, 32)?;

        // switches are OFF
        assert_eq!(false, switch1);
        assert_eq!(false, switch2);
        set_dip_switch(&mut cursor, switch_count, 1, true)?;
        set_dip_switch(&mut cursor, switch_count, 32, true)?;

        // switches are both ON
        let switch1 = get_dip_switch(&mut cursor, switch_count, 1)?;
        let switch2 = get_dip_switch(&mut cursor, switch_count, 32)?;
        assert_eq!(true, switch1);
        assert_eq!(true, switch2);

        // the switch data should be written to the cursor
        let mut buff = [0; 6];
        cursor.seek(SeekFrom::End(-6))?;
        cursor.read_exact(&mut buff)?;
        assert_eq!([1, 0, 0, 128, 0, 0], buff);

        // other data in the cursor should not be changed
        let mut buff = [0; 2];
        cursor.seek(SeekFrom::Start(0))?;
        cursor.read_exact(&mut buff)?;
        assert_eq!([0, 0], buff);
        Ok(())
    }

    #[test]
    fn test_dip_switch_out_of_range() -> io::Result<()> {
        let mut cursor = io::Cursor::new(vec![0; 8]);
        let result = get_dip_switch(&mut cursor, 16, 17);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Dip switch #17 out of range, expected 1-16"
        ));
        let result = set_dip_switch(&mut cursor, 16, 0, true);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Dip switch #0 out of range, expected 1-16"
        ));
        let result = set_dip_switch(&mut cursor, 8 * 6 + 1, 1, true);
        println!("{:?}", result);
        assert!(matches!(
            result,
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput && e.to_string() == "Switch count 49 out of range, expected 1-48"
        ));
        Ok(())
    }
}
