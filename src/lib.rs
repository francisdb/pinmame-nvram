pub mod checksum;
pub mod dips;
mod encoding;
mod index;
mod model;
pub mod resolve;

use crate::checksum::{ChecksumMismatch, update_all_checksum16, verify_all_checksum16};
use crate::dips::{get_dip_switch, set_dip_switch, validate_dip_switch_range};
use crate::encoding::{Location, read_bcd, read_ch, read_int, read_wpc_rtc, write_bcd, write_ch};
use crate::index::get_index_map;
use crate::model::{
    DEFAULT_LENGTH, DEFAULT_SCALE, Descriptor, Encoding, Endian, GlobalSettings, MemoryLayoutType,
    Nibble, NvramMap, Platform, StateOrStateList,
};
use include_dir::{Dir, File, include_dir};
use serde::de;
use serde::de::DeserializeOwned;
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, Write};
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
    pub initials: Option<String>,
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
    pub platform: Platform,
    pub nv_path: PathBuf,
}

impl Nvram {
    /// Open a NVRAM file from the embedded maps
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Nvram))` if the file was found and a map was found for the ROM
    /// * `Ok(None)` if the file was found but no map was found for the ROM
    pub fn open(nv_path: &Path) -> io::Result<Option<Nvram>> {
        let map_opt: Option<NvramMap> = open_nvram(nv_path)?;
        if let Some(map) = map_opt {
            // find the platform from the map
            let platform = read_platform(&map._metadata.platform)?;
            Ok(Some(Nvram {
                map,
                platform,
                nv_path: nv_path.to_path_buf(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Open a NVRAM file from the file system using the local maps
    /// Expects the `pinmame-nvram-maps` folder to exist in the current working directory
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Nvram))` if the file was found and a map was found for the ROM
    /// * `Ok(None)` if the file was found but no map was found for the ROM
    pub fn open_local(nv_path: &Path) -> io::Result<Option<Nvram>> {
        let map_opt: Option<NvramMap> = open_nvram_local(nv_path)?;
        //let platform = todo!("Determine platform from map");
        if let Some(map) = map_opt {
            let platform = read_platform_local(map.platform())?;
            Ok(Some(Nvram {
                map,
                platform,
                nv_path: nv_path.to_path_buf(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn read_highscores(&mut self) -> io::Result<Vec<HighScore>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_highscores(
            self.platform.endian,
            self.platform.nibble(MemoryLayoutType::NVRam),
            self.platform.offset(MemoryLayoutType::NVRam),
            &self.map,
            &mut file,
        )
    }

    pub fn clear_highscores(&mut self) -> io::Result<()> {
        // re-open the file in write mode
        let mut rw_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.nv_path)?;
        clear_highscores(
            &mut rw_file,
            self.platform.nibble(MemoryLayoutType::NVRam),
            self.platform.offset(MemoryLayoutType::NVRam),
            &self.map,
        )?;
        update_all_checksum16(&mut rw_file, &self.map, &self.platform)
    }

    pub fn read_mode_champions(&mut self) -> io::Result<Option<Vec<ModeChampion>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_mode_champions(
            &mut file,
            self.platform.endian,
            self.platform.nibble(MemoryLayoutType::NVRam),
            self.platform.offset(MemoryLayoutType::NVRam),
            &self.map,
        )
    }

    pub fn read_last_game(&mut self) -> io::Result<Option<Vec<LastGamePlayer>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_last_game(
            &mut file,
            self.platform.endian,
            self.platform.nibble(MemoryLayoutType::NVRam),
            self.platform.offset(MemoryLayoutType::NVRam),
            &self.map,
        )
    }

    pub fn verify_all_checksum16(&mut self) -> io::Result<Vec<ChecksumMismatch<u16>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        verify_all_checksum16(&mut file, &self.map, &self.platform)
    }

    // TODO we probably want to remove this
    pub fn read_replay_score(&mut self) -> io::Result<Option<u64>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_replay_score(
            &mut file,
            self.platform.endian,
            self.platform.nibble(MemoryLayoutType::NVRam),
            self.platform.offset(MemoryLayoutType::NVRam),
            &self.map,
        )
    }

    pub fn read_game_state(&mut self) -> io::Result<Option<HashMap<String, String>>> {
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        read_game_state(
            &mut file,
            self.platform.endian,
            self.platform.nibble(MemoryLayoutType::NVRam),
            self.platform.offset(MemoryLayoutType::NVRam),
            &self.map,
        )
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
        validate_dip_switch_range(self.dip_switches_len(), number)?;
        let mut file = OpenOptions::new().read(true).open(&self.nv_path)?;
        get_dip_switch(&mut file, number)
    }

    /// Set a dip switch to on or off
    /// # Arguments
    /// * `number` - The number of the dip switch to set, 1-based!
    /// * `on` - `true` to set the dip switch to ON, `false` to set it to OFF
    /// # Returns
    /// * `Ok(())` if the dip switch was set successfully
    /// * `Err(io::Error)` if the dip switch number is out of range or an IO error occurred
    pub fn set_dip_switch(&self, number: usize, on: bool) -> io::Result<()> {
        validate_dip_switch_range(self.dip_switches_len(), number)?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.nv_path)?;
        set_dip_switch(&mut file, number, on)
    }
}

fn open_nvram<T: DeserializeOwned>(nv_path: &Path) -> io::Result<Option<T>> {
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
            format!("File not found: {nv_path:?}"),
        ));
    }
    find_map(&rom_name)
}

fn open_nvram_local<T: DeserializeOwned>(nv_path: &Path) -> io::Result<Option<T>> {
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
            format!("File not found: {nv_path:?}"),
        ));
    }
    find_map_local(&rom_name)
}

fn read_platform<T: DeserializeOwned>(platform_name: &str) -> io::Result<T> {
    let platform_file_name = format!("{platform_name}.json.brotli");
    let compressed_platform_path = Path::new("platforms").join(platform_file_name);

    let map_file = MAPS.get_file(&compressed_platform_path).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "File not found: {}",
                compressed_platform_path.to_string_lossy()
            ),
        )
    })?;
    read_compressed_json(map_file)
}

fn read_platform_local<T: DeserializeOwned>(platform_name: &str) -> io::Result<T> {
    let platform_file_name = format!("{platform_name}.json");
    let platform_path = Path::new("pinmame-nvram-maps")
        .join("platforms")
        .join(platform_file_name);
    if !platform_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {platform_path:?}"),
        ));
    }
    let platform_file = OpenOptions::new().read(true).open(&platform_path)?;
    let platform = serde_json::from_reader(platform_file)?;
    Ok(platform)
}

fn find_map<T: DeserializeOwned>(rom_name: &String) -> io::Result<Option<T>> {
    match get_index_map()?.get(rom_name) {
        Some(map_path) => {
            let compressed_map_path = format!("{}.brotli", map_path.as_str().unwrap());
            let map_file = MAPS.get_file(&compressed_map_path).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {compressed_map_path}"),
                )
            })?;
            let map: T = read_compressed_json(map_file)?;
            Ok(Some(map))
        }
        None => Ok(None),
    }
}

fn find_map_local<T: DeserializeOwned>(rom_name: &String) -> io::Result<Option<T>> {
    let index_file = Path::new("pinmame-nvram-maps").join("index.json");
    if !index_file.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {index_file:?}"),
        ));
    }
    let index_file = OpenOptions::new().read(true).open(&index_file)?;
    let map: Value = serde_json::from_reader(index_file)?;
    match map.get(rom_name) {
        Some(map_path) => {
            let map_file = Path::new("pinmame-nvram-maps").join(map_path.as_str().unwrap());
            if !map_file.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {map_file:?}"),
                ));
            }
            let map_file = OpenOptions::new().read(true).open(&map_file)?;
            let map: T = serde_json::from_reader(map_file)?;
            Ok(Some(map))
        }
        None => Ok(None),
    }
}

fn read_compressed_json<T: de::DeserializeOwned>(map_file: &File) -> io::Result<T> {
    let mut cursor = io::Cursor::new(map_file.contents());
    let reader = brotli::Decompressor::new(&mut cursor, 4096);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

fn read_highscores<T: Read + Seek>(
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    map: &NvramMap,
    mut nvram_file: &mut T,
) -> io::Result<Vec<HighScore>> {
    let scores: Result<Vec<HighScore>, io::Error> = map
        .high_scores
        .iter()
        .map(|hs| read_highscore(&mut nvram_file, hs, endian, nibble, offset, map))
        .collect();
    scores
}

fn read_highscore<T: Read + Seek, S: GlobalSettings>(
    mut nvram_file: &mut T,
    hs: &model::HighScore,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    global_settings: &S,
) -> io::Result<HighScore> {
    let mut initials = "".to_string();
    if let Some(map_initials) = &hs.initials {
        initials = read_ch(
            &mut nvram_file,
            u64::from(
                map_initials
                    .start
                    .as_ref()
                    .expect("missing start for ch encoding"),
            ) - offset,
            map_initials.length.expect("missing length for ch encoding"),
            map_initials.mask.as_ref().map(|m| m.into()),
            global_settings.char_map(),
            map_initials.nibble.unwrap_or(nibble),
            map_initials.null,
        )?;
    }

    let score = read_descriptor_to_u64(&mut nvram_file, &hs.score, endian, nibble, offset)?;

    Ok(HighScore {
        label: hs.label.clone(),
        short_label: hs.short_label.clone(),
        initials,
        score,
    })
}

fn clear_highscores<T: Write + Seek>(
    mut nvram_file: &mut T,
    nibble: Nibble,
    offset: u64,
    map: &NvramMap,
) -> io::Result<()> {
    for hs in &map.high_scores {
        if let Some(map_initials) = &hs.initials {
            write_ch(
                &mut nvram_file,
                u64::from(
                    map_initials
                        .start
                        .as_ref()
                        .expect("missing start for ch encoding"),
                ) - offset,
                map_initials.length.expect("missing length for ch encoding"),
                "AAA".to_string(),
                map.char_map(),
                &map_initials.nibble.or(Some(nibble)),
            )?;
        }
        if let Some(map_score_start) = &hs.score.start {
            write_bcd(
                &mut nvram_file,
                u64::from(map_score_start) - offset,
                hs.score.length.unwrap_or(0),
                hs.score.nibble.unwrap_or(nibble),
                0,
            )?;
        }
    }
    Ok(())
}

fn read_mode_champion<T: Read + Seek, S: GlobalSettings>(
    mut nvram_file: &mut T,
    mc: &model::ModeChampion,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    global_settings: &S,
) -> io::Result<ModeChampion> {
    let initials = mc
        .initials
        .as_ref()
        .map(|initials| {
            read_ch(
                &mut nvram_file,
                u64::from(
                    initials
                        .start
                        .as_ref()
                        .expect("missing start for ch encoding"),
                ) - offset,
                initials.length.expect("missing start for ch encoding"),
                initials.mask.as_ref().map(|m| m.into()),
                global_settings.char_map(),
                initials.nibble.unwrap_or(nibble),
                initials.null,
            )
        })
        .transpose()?;
    let score = if let Some(score) = &mc.score {
        let result = read_descriptor_to_u64(&mut nvram_file, score, endian, nibble, offset)?;
        Some(result)
    } else {
        None
    };

    let timestamp = mc
        .timestamp
        .as_ref()
        .map(|ts| read_descriptor_to_rtc_string(&mut nvram_file, ts))
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
    descriptor: &Descriptor,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
) -> io::Result<LastGamePlayer> {
    let score = read_descriptor_to_u64(&mut nvram_file, descriptor, endian, nibble, offset)?;
    Ok(LastGamePlayer {
        score,
        label: descriptor.label.clone(),
    })
}

fn read_last_game<T: Read + Seek>(
    mut nvram_file: &mut T,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    map: &NvramMap,
) -> io::Result<Option<Vec<LastGamePlayer>>> {
    if let Some(lg) = &map.last_game {
        // this is the old location of the last game scores
        // TODO remove once all maps have been updated
        let last_games: Result<Vec<LastGamePlayer>, io::Error> = lg
            .iter()
            .map(|lg| read_last_game_player(&mut nvram_file, lg, endian, nibble, offset))
            .collect();
        Ok(Some(last_games?))
    } else if let Some(game_state) = &map.game_state {
        if let Some(scores) = game_state.get("scores") {
            let scores: Result<Vec<LastGamePlayer>, io::Error> = match scores {
                StateOrStateList::StateList(sl) => sl
                    .iter()
                    .map(|s| read_last_game_player(&mut nvram_file, s, endian, nibble, offset))
                    .collect(),
                StateOrStateList::State(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Scores is not a StateList",
                    ));
                }
            };
            Ok(Some(scores?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn read_mode_champions<T: Read + Seek>(
    mut nvram_file: &mut T,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    map: &NvramMap,
) -> io::Result<Option<Vec<ModeChampion>>> {
    if let Some(mode_champions) = &map.mode_champions {
        let champions: Result<Vec<ModeChampion>, io::Error> = mode_champions
            .iter()
            .map(|mc| read_mode_champion(&mut nvram_file, mc, endian, nibble, offset, map))
            .collect();
        Ok(Some(champions?))
    } else {
        Ok(None)
    }
}

fn read_replay_score<T: Read + Seek>(
    mut nvram_file: &mut T,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    map: &NvramMap,
) -> io::Result<Option<u64>> {
    if let Some(descriptor) = &map.replay_score {
        let value = read_descriptor_to_u64(&mut nvram_file, descriptor, endian, nibble, offset)?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

fn read_game_state<T: Read + Seek>(
    mut nvram_file: &mut T,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    map: &NvramMap,
) -> io::Result<Option<HashMap<String, String>>> {
    if let Some(game_state) = &map.game_state {
        // map the hashmap values to a new hashmap with the values read from the nvram file
        let state: Result<HashMap<String, String>, io::Error> = game_state
            .iter()
            .flat_map(|(key, v)| match v {
                StateOrStateList::State(s) => {
                    let r =
                        read_descriptor_to_string(&mut nvram_file, s, endian, nibble, offset, map)
                            .map(|r| (key.clone(), r));
                    vec![r]
                }
                StateOrStateList::StateList(sl) => sl
                    .iter()
                    .enumerate()
                    .map(|(index, s)| {
                        let compund_key = format!("{key}.{index}");
                        read_descriptor_to_string(&mut nvram_file, s, endian, nibble, offset, map)
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

fn read_descriptor_to_string<T: Read + Seek, S: GlobalSettings>(
    mut nvram_file: &mut T,
    descriptor: &Descriptor,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
    global_settings: &S,
) -> io::Result<String> {
    match descriptor.encoding {
        Encoding::Ch => match &descriptor.start {
            Some(start) => read_ch(
                &mut nvram_file,
                u64::from(start) - offset,
                descriptor.length.unwrap_or(DEFAULT_LENGTH),
                descriptor.mask.as_ref().map(|m| m.into()),
                global_settings.char_map(),
                descriptor.nibble.unwrap_or(nibble),
                None,
            ),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Ch descriptor requires start",
            )),
        },
        Encoding::Int => match &descriptor.start {
            Some(start) => {
                let start = u64::from(start);
                if start < offset {
                    return Ok("Value is stored outside the NVRAM".to_string());
                }
                let score = read_int(
                    &mut nvram_file,
                    endian,
                    nibble,
                    start - offset,
                    descriptor.length.unwrap_or(DEFAULT_LENGTH),
                    descriptor
                        .scale
                        .as_ref()
                        .unwrap_or(&Number::from(DEFAULT_SCALE)),
                )?;
                Ok(score.to_string())
            }
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Int descriptor requires start",
            )),
        },
        Encoding::Bcd => {
            let location = match location_for(descriptor, offset)? {
                LocateResult::OutsideNVRAM => {
                    return Ok("Value is stored outside the NVRAM".to_string());
                }
                LocateResult::Located(loc) => loc,
            };
            let score = read_bcd(
                &mut nvram_file,
                location,
                descriptor.nibble.unwrap_or(nibble),
                descriptor
                    .scale
                    .as_ref()
                    .unwrap_or(&Number::from(DEFAULT_SCALE)),
                endian,
            )?;
            Ok(score.to_string())
        }
        Encoding::Bits => Ok("Bits encoding not implemented".to_string()),
        other => todo!("Encoding not implemented: {:?}", other),
    }
}

fn read_descriptor_to_u64<T: Read + Seek>(
    mut nvram_file: &mut T,
    descriptor: &Descriptor,
    endian: Endian,
    nibble: Nibble,
    offset: u64,
) -> io::Result<u64> {
    match descriptor.encoding {
        Encoding::Bcd => {
            let location = match location_for(descriptor, offset)? {
                LocateResult::OutsideNVRAM => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Descriptor points outside NVRAM",
                    ));
                }
                LocateResult::Located(loc) => loc,
            };
            read_bcd(
                &mut nvram_file,
                location,
                descriptor.nibble.unwrap_or(nibble),
                descriptor
                    .scale
                    .as_ref()
                    .unwrap_or(&Number::from(DEFAULT_SCALE)),
                endian,
            )
        }
        Encoding::Int => {
            if let Some(start) = &descriptor.start {
                read_int(
                    &mut nvram_file,
                    endian,
                    descriptor.nibble.unwrap_or(nibble),
                    u64::from(start) - offset,
                    descriptor.length.unwrap_or(DEFAULT_LENGTH),
                    descriptor
                        .scale
                        .as_ref()
                        .unwrap_or(&Number::from(DEFAULT_SCALE)),
                )
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Int descriptor requires start",
                ))
            }
        }
        other => todo!("Encoding not implemented: {:?}", other),
    }
}

fn read_descriptor_to_rtc_string<T: Read + Seek>(
    mut nvram_file: &mut T,
    ts: &Descriptor,
) -> io::Result<String> {
    match &ts.encoding {
        Encoding::WpcRtc => read_wpc_rtc(
            &mut nvram_file,
            ts.start
                .as_ref()
                .expect("missing start for wpc_rtc encoding")
                .into(),
            ts.length.expect("missing length for wpc_rtc encoding"),
        ),
        other => todo!("Timestamp encoding not implemented: {:?}", other),
    }
}

enum LocateResult {
    OutsideNVRAM,
    Located(Location),
}

fn location_for(descriptor: &Descriptor, offset: u64) -> io::Result<LocateResult> {
    match &descriptor.offsets {
        None => match &descriptor.start {
            Some(start) => {
                let start = u64::from(start);
                if start < offset {
                    return Ok(LocateResult::OutsideNVRAM);
                }
                Ok(LocateResult::Located(Location::Continuous {
                    start: start - offset,
                    length: descriptor.length.unwrap_or(DEFAULT_LENGTH),
                }))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Descriptor without offsets requires start",
            )),
        },
        Some(offsets) => Ok(LocateResult::Located(Location::Scattered {
            offsets: offsets.iter().map(|o| u64::from(o) - offset).collect(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
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
    fn test_find_map() -> io::Result<()> {
        let map: Option<Value> = find_map(&"afm_113b".to_string())?;
        assert_eq!(true, map.is_some());
        Ok(())
    }
}
