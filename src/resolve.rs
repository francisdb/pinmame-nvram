use crate::checksum::{verify_checksum8, verify_checksum16};
use crate::encoding::{Location, read_bcd, read_ch, read_exact_at, read_int, read_wpc_rtc};
use crate::model::{
    Checksum8, Checksum16, DEFAULT_LENGTH, DEFAULT_SCALE, Encoding, Endian, GlobalSettings,
    GlobalSettingsImpl, MemoryLayout, MemoryLayoutType, Nibble, Null, Platform,
};
use crate::{dips, open_nvram, read_platform};
use serde_json::{Map, Number, Value};
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek};
use std::path::Path;

pub fn resolve(nv_path: &Path) -> io::Result<Option<Value>> {
    let map: Option<Value> = open_nvram(nv_path)?;
    let result = if let Some(map) = &map {
        // TODO how can we do this without cloning the whole object?
        let global_settings: GlobalSettingsImpl =
            serde_json::from_value(map.clone()).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse global settings: {e}"),
                )
            })?;

        let platform: Platform = read_platform(global_settings.platform())?;

        let mut rom = OpenOptions::new().read(true).open(nv_path)?;
        match resolve_recursive(map, &global_settings, &platform, &mut rom) {
            Ok(resolved) => Some(resolved),
            Err(e) => {
                return Err(io::Error::new(
                    e.kind(),
                    format!("Failed to resolve: {}: {}", nv_path.display(), e),
                ));
            }
        }
    } else {
        None
    };
    Ok(result)
}

fn resolve_recursive<T: Read + Seek, S: GlobalSettings>(
    value: &Value,
    global_settings: &S,
    platform: &Platform,
    rom: &mut T,
) -> io::Result<Value> {
    let result: Value = match value {
        Value::Object(map) => {
            // println!("{:?}", map);
            // println!("{:?}", map.get("encoding"));
            if let Some(encoding) = map.get("encoding") {
                let encoding: Encoding = serde_json::from_value(encoding.clone())?;
                let value = resolve_value(rom, map, encoding, global_settings, platform);
                let warning = match &value {
                    Ok(value) => {
                        // FIXME if the value is an enum we need to validate the raw value
                        //   instead of the enum value
                        validate_range(map, value)
                    }
                    Err(e) => Some(format!("Failed to resolve: {e}")),
                };
                let mut resolved_map = Map::new();
                if let Ok(value) = value {
                    resolved_map.insert("value".to_string(), value);
                }
                if let Some(label) = map.get("label") {
                    // maybe we should instead remove all properties related to the encoding
                    resolved_map.insert("label".to_string(), label.clone());
                }
                if let Some(warning) = warning {
                    resolved_map.insert("warning".to_string(), Value::String(warning));
                }
                Value::Object(resolved_map)
            } else {
                let mut resolved_map = Map::new();
                for (key, value) in map.iter() {
                    if key.eq("checksum16") {
                        let checksum_result = resolve_checksum16(
                            platform.endian,
                            rom,
                            value,
                            platform.offset(MemoryLayoutType::NVRam),
                        )?;
                        resolved_map.insert(key.clone(), checksum_result);
                    } else if key.eq("checksum8") {
                        let checksum_result = resolve_checksum8(rom, value)?;
                        resolved_map.insert(key.clone(), checksum_result);
                    } else if key.eq("_fileformat") || !key.starts_with('_') {
                        resolved_map.insert(
                            key.clone(),
                            resolve_recursive(value, global_settings, platform, rom)?,
                        );
                    }
                }
                Value::Object(resolved_map)
            }
        }
        Value::Array(array) => {
            let resolved_array: Vec<Value> = array
                .iter()
                .map(|v| resolve_recursive(v, global_settings, platform, rom))
                .collect::<Result<Vec<_>, _>>()?;
            Value::Array(resolved_array)
        }
        other => other.clone(),
    };
    Ok(result)
}

/// Validate the value against the min and max range
/// If the value is out of range a warning is returned
fn validate_range(map: &Map<String, Value>, value: &Value) -> Option<String> {
    if let (Some(min), Some(max)) = (map.get("min"), map.get("max")) {
        // TODO might be better to do this check earlier before the scaling is applied
        // min and max are unscaled values so we need to unscale the value first
        let Some(number_value) = value.as_u64() else {
            return Some(format!("Value {value} is not an unsigned int"));
        };
        let unscaled_value = if let Some(scale) = map.get("scale") {
            let scale = scale.as_u64().unwrap();
            number_value / scale
        } else {
            number_value
        };

        let min = min.as_u64().unwrap();
        let max = max.as_u64().unwrap();
        if unscaled_value < min || unscaled_value > max {
            return Some(format!(
                "Value out of range: {min} ≤ {unscaled_value} ≤ {max}"
            ));
        }
    }
    None
}

fn resolve_checksum16<T: Read + Seek>(
    endian: Endian,
    rom: &mut T,
    value: &Value,
    offset: u64,
) -> io::Result<Value> {
    // go over the checksum16 array and verify the checksum
    let mut checksum_result: Vec<Value> = Vec::new();
    for checksum in value.as_array().unwrap() {
        let checksum16: Checksum16 = serde_json::from_value(checksum.clone())?;
        let checksum_failure = verify_checksum16(rom, &checksum16, endian, offset)?;
        let mut map = Map::new();
        if let Some(label) = checksum.get("label") {
            map.insert("label".to_string(), label.clone());
        }
        if let Some(checksum_failure) = checksum_failure {
            map.insert("value".to_string(), Value::String("mismatch".to_string()));
            map.insert(
                "checksum_mismatch_expected".to_string(),
                Value::Number(checksum_failure.expected.into()),
            );
            map.insert(
                "checksum_mismatch_calculated".to_string(),
                Value::Number(checksum_failure.calculated.into()),
            );
            checksum_result.push(Value::Object(map));
        } else {
            map.insert("value".to_string(), Value::String("valid".to_string()));
            checksum_result.push(Value::Object(map));
        }
    }
    Ok(Value::Array(checksum_result))
}

fn resolve_checksum8<T: Read + Seek>(rom: &mut T, value: &Value) -> io::Result<Value> {
    // go over the checksum16 array and verify the checksum
    let mut checksum_result: Vec<Value> = Vec::new();
    for checksum in value.as_array().unwrap() {
        let checksum8: Checksum8 = serde_json::from_value(checksum.clone())?;
        let checksum_failure = verify_checksum8(rom, &checksum8)?;
        let mut map = Map::new();
        if let Some(label) = checksum.get("label") {
            map.insert("label".to_string(), label.clone());
        }
        if let Some(checksum_failure) = checksum_failure {
            map.insert("value".to_string(), Value::String("mismatch".to_string()));
            map.insert(
                "checksum_mismatch_expected".to_string(),
                Value::Number(checksum_failure.expected.into()),
            );
            map.insert(
                "checksum_mismatch_calculated".to_string(),
                Value::Number(checksum_failure.calculated.into()),
            );
            checksum_result.push(Value::Object(map));
        } else {
            map.insert("value".to_string(), Value::String("valid".to_string()));
            checksum_result.push(Value::Object(map));
        }
    }
    Ok(Value::Array(checksum_result))
}

fn resolve_value<T: Read + Seek, U: GlobalSettings>(
    rom: &mut T,
    descriptor: &Map<String, Value>,
    encoding: Encoding,
    global_settings: &U,
    platform: &Platform,
) -> io::Result<Value> {
    // we only have access to nvram files and cannot access the RAM.
    let nvram_layout = platform.layout(MemoryLayoutType::NVRam);
    let nibble = nvram_layout.nibble();
    let endian = platform.endian;
    let length = descriptor
        .get("length")
        .map_or(DEFAULT_LENGTH, |v| v.as_u64().unwrap() as usize);
    let value = match encoding {
        Encoding::Int => {
            let scale = descriptor
                .get("scale")
                .and_then(|s| s.as_number())
                .cloned()
                .unwrap_or(Number::from(DEFAULT_SCALE));
            let start = start_in_nvram_file(nvram_layout, descriptor)?;
            let value = read_int(rom, endian, nibble, start, length, &scale)?;
            Value::Number(value.into())
        }
        Encoding::Enum => {
            let start = start_in_nvram_file(nvram_layout, descriptor)?;
            let index = read_int(
                rom,
                endian,
                nibble,
                start,
                length,
                &Number::from(DEFAULT_SCALE),
            )? as usize;
            let values = descriptor.get("values").unwrap().as_array().unwrap();
            let enum_value = values.get(index);
            return if let Some(enum_value) = enum_value {
                Ok(enum_value.clone())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Index {} out of bounds for enum with {} values",
                        index,
                        values.len()
                    ),
                ))
            };
        }
        Encoding::Bcd => {
            let location = match descriptor.get("offsets") {
                None => {
                    let start = start_in_nvram_file(nvram_layout, descriptor)?;
                    Location::Continuous { start, length }
                }
                Some(offsets) => {
                    let offsets: Vec<u64> = offsets
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(json_hex_or_int)
                        .collect::<Result<Vec<_>, _>>()?;
                    Location::Scattered { offsets }
                }
            };
            let scale = descriptor
                .get("scale")
                .and_then(|s| s.as_number())
                .cloned()
                .unwrap_or(Number::from(DEFAULT_SCALE));
            // how can we avoid the clone() here?
            let nibble: Nibble = descriptor
                .get("nibble")
                .map(|n| serde_json::from_value(n.clone()).unwrap())
                .unwrap_or(nibble);

            let value = read_bcd(rom, location, nibble, &scale, endian)?;
            Value::Number(value.into())
        }
        Encoding::Ch => {
            let start = start_in_nvram_file(nvram_layout, descriptor)?;
            let mask = descriptor.get("mask").map(json_hex_or_int).transpose()?;
            let nibble = descriptor
                .get("nibble")
                .map(|n| serde_json::from_value(n.clone()).unwrap())
                .unwrap_or(nibble);
            let null: Option<Null> = descriptor
                .get("null")
                .map(|n| serde_json::from_value(n.clone()).unwrap());
            let value = read_ch(
                rom,
                start,
                length,
                mask,
                global_settings.char_map(),
                nibble,
                null,
            )?;
            Value::String(value)
        }
        Encoding::WpcRtc => {
            let start = start_in_nvram_file(nvram_layout, descriptor)?;
            let value = read_wpc_rtc(rom, start, length)?;
            Value::String(value)
        }
        Encoding::Bits => {
            let value = "Bits encoding not implemented".to_string();
            Value::String(value)
        }
        Encoding::Raw => {
            let start = start_in_nvram_file(nvram_layout, descriptor)?;
            let mut buff = vec![0; length];
            read_exact_at(rom, start, &mut buff)?;
            Value::Array(buff.iter().map(|b| Value::Number((*b).into())).collect())
        }
        Encoding::Dipsw => {
            let offsets = descriptor
                .get("offsets")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(json_hex_or_int)
                .collect::<io::Result<Vec<_>>>()?;

            let mut dips = Vec::new();
            for offset in offsets {
                let dip_on = dips::get_dip_switch(rom, offset as usize)?;
                dips.push(dip_on);
            }
            // convert the bits to a number, always msb first
            let mut value = 0;
            for dip in dips.iter() {
                value = (value << 1) | if *dip { 1 } else { 0 };
            }

            let values = descriptor
                .get("values")
                .expect("Missing values for dip switch");
            let index = value as usize;
            match values {
                Value::Array(array) => array.get(index).unwrap_or(&Value::Null).clone(),
                Value::String(value_reference) => {
                    match global_settings.value(value_reference, index) {
                        Some(value) => Value::String(value),
                        None => Value::Null,
                    }
                }
                _ => {
                    panic!("Unexpected dip switch values type: {values:?}");
                }
            }
        }
    };
    Ok(value)
}

fn start_in_nvram_file(
    nvram_layout: &MemoryLayout,
    descriptor: &Map<String, Value>,
) -> io::Result<u64> {
    let start_native = descriptor.get("start").map(json_hex_or_int).transpose()?;
    if let Some(s) = start_native {
        let offset: u64 = (&nvram_layout.address).into();
        if s < offset {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Value is stored outside the NVRAM",
            ))
        } else {
            Ok(s - offset)
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing start value for NVRAM encoding",
        ))
    }
}

fn json_hex_or_int(s: &Value) -> io::Result<u64> {
    match s {
        // TODO deduplicate
        Value::String(s) => {
            if s.starts_with("0x") || s.starts_with("0X") {
                u64::from_str_radix(&s[2..], 16)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            } else {
                panic!("Not implemented: int from string {s}")
            }
        }

        Value::Number(n) => {
            // TODO proper error handling
            Ok(n.as_u64().unwrap())
        }
        other => todo!("Start not implemented: {:?}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use testdir::testdir;

    #[test]
    fn test_resolve() -> io::Result<()> {
        let path = Path::new("testdata/hs_l4.nv");
        let map: Option<Value> = resolve(path)?;
        assert!(map.is_some(), "Failed to resolve: {path:?}");

        // let json = serde_json::to_string_pretty(&map.unwrap())?;
        // assert_eq!("{}", json);
        Ok(())
    }

    #[test]
    fn test_missing_test_nvrams() -> io::Result<()> {
        let excludes = ["_note"];

        // Temporarily disable these rom game names if you can't find the nvram
        let expected: [&str; 188] = [
            "algar_l1ff",
            "alpok_b6",
            "alpok_f6",
            "alpok_f6ff",
            "alpok_l2",
            "alpok_l2ff",
            "alpok_l6ff",
            "amaz3afp",
            "amazn2fp",
            "amazn3fp",
            "amazonh2",
            "amazonh3",
            "arena_fp",
            "arenaafp",
            "arenaffp",
            "arenagfp",
            "badgrffp",
            "badgrgfp",
            "badgrlfp",
            "bguns_la",
            "bighosfp",
            "bighsffp",
            "bighsgfp",
            "blkou_f1",
            "blkou_f1ff",
            "blkou_l1ff",
            "blkou_t1",
            "blkou_t1ff",
            "bonebffp",
            "bonebgfp",
            "bonebsfp",
            "bop_d7",
            "bop_d8",
            "bountgfp",
            "bounthfp",
            "br_d4",
            "cftbl_d4",
            "comet_l4",
            "congo_20",
            "dh_dx2",
            "diamnffp",
            "diamngfp",
            "diamonfp",
            "dm_dx4",
            "dm_h6c",
            "dm_lx4c",
            "drac_d1",
            "dw_d2",
            "eatpm_l2",
            "esha_la1",
            "excalbfp",
            "excalffp",
            "excalgfp",
            "fh_d9",
            "fh_d9b",
            "fh_l9b",
            "flash_l1ff",
            "flash_l2",
            "flash_t1",
            "flash_t1ff",
            "frpwr_l2",
            "frpwr_l2ff",
            "frpwr_l6",
            "frpwr_l6ff",
            "frpwr_t6",
            "frpwr_t6ff",
            "fs_dx5",
            "fs_sp2",
            "fs_sp2d",
            "ft_d5",
            "ft_d6",
            "ft_l5p",
            "genesffp",
            "genesgfp",
            "genesifp",
            "genesis",
            "gi_d9",
            "gldwgffp",
            "gldwggfp",
            "goldwgfp",
            "grand_l4",
            "grgar_c1",
            "grgar_l1ff",
            "grgar_t1",
            "grgar_t1ff",
            "gw_d5",
            "gw_l5c",
            "hd_d1",
            "hd_d3",
            "hlywdhfp",
            "hlywhffp",
            "hlywhgfp",
            "hotshffp",
            "hotshgfp",
            "hotshtfp",
            "hs_l3",
            "hurr_d2",
            "ij_d7",
            "jb_101r",
            "jd_d1",
            "jd_d7",
            "jy_12c",
            "lzbal_l2ff",
            "lzbal_l2sp",
            "lzbal_l2spff",
            "lzbal_t2",
            "lzbal_t2ff",
            "milln_l3",
            "mntcr2fp",
            "mntcrafp",
            "mntcrffp",
            "mntcrfmfp",
            "mntcrgfp",
            "mntcrgmfp",
            "mntcrmfp",
            "mntecrfp",
            "nmovesfp",
            "pop_dx5",
            "pz_d3",
            "pz_f5",
            "raven",
            "ravenafp",
            "ravenfp",
            "ravengfp",
            "robowffp",
            "robowrfp",
            "rock_efp",
            "rock_enc",
            "rockegfp",
            "rockfp",
            "rockgfp",
            "rs_l6c",
            "rvrbt_l3",
            "scrpn_l1ff",
            "scrpn_t1",
            "scrpn_t1ff",
            "sprbrafp",
            "sprbreak",
            "sprbrffp",
            "sprbrgfp",
            "sprbrkfp",
            "sprbrsfp",
            "sttng_d7",
            "sttng_dx",
            "sttng_l7c",
            "sttng_x7",
            "t2_d8",
            "t2_l81",
            "t2_l82",
            "t2_l83",
            "t2_l84",
            "taf_d5",
            "taf_d6",
            "taf_d7",
            "taf_i4",
            "taf_l5c",
            "tafg_dx3",
            "tagteam",
            "tagtem2f",
            "tagtemfp",
            "tagtmgfp",
            "tmwrp_l2ff",
            "tmwrp_l3",
            "tmwrp_l3ff",
            "tmwrp_t2",
            "tmwrp_t2ff",
            "tom_13c",
            "triplgfp",
            "triplyf1",
            "triplyfp",
            "trizn_l1ff",
            "trizn_t1",
            "trizn_t1ff",
            "ts_dx5",
            "txsecffp",
            "txsecgfp",
            "txsectfp",
            "tz_93",
            "victr101",
            "victr11",
            "victr12",
            "victrffp",
            "victrgfp",
            "victryfp",
            "wcs_d2",
            "wcs_l3c",
            "ww_d5",
            "ww_lh6c",
        ];

        let index = Path::new("pinmame-nvram-maps").join("index.json");
        let testdata = Path::new("testdata");
        let index: Value = serde_json::from_str(&std::fs::read_to_string(index)?)?;
        let mut missing = Vec::new();
        for (rom, _) in index.as_object().unwrap() {
            if excludes.contains(&rom.as_str()) {
                continue;
            }
            let expected = testdata.join(format!("{rom}.nv"));
            if !expected.exists() {
                missing.push(rom);
            }
        }
        assert_eq!(missing, expected);
        Ok(())
    }

    #[test]
    fn test_resolve_all() -> io::Result<()> {
        // any nvram that contains - in the file name needs to be renamed first
        let test_dir = testdir!();

        let excludes = [/*"taf_l7"*/];

        for entry in std::fs::read_dir("testdata")? {
            let entry = entry?;
            let nvram_path = entry.path();
            if nvram_path.extension().unwrap() == "nv" {
                let path = path_for_test(&test_dir, &nvram_path)?;
                if excludes.contains(&path.file_stem().unwrap().to_str().unwrap()) {
                    println!("Skipping: {path:?}");
                    continue;
                }
                let map = resolve(&path)?;

                if let Some(map) = &map {
                    let json_path = &nvram_path.with_extension("nv.json");
                    // Enable this to regenerate the json files
                    // let json = serde_json::to_string_pretty(&map)?;
                    // std::fs::write(json_path, json)?;
                    if json_path.exists() {
                        let expected = std::fs::read_to_string(json_path)?;
                        let actual = serde_json::to_string_pretty(&map)?;
                        assert_eq!(expected, actual, "Mismatch: {json_path:?}");
                    } else {
                        // Enable this to regenerate the missing json files
                        // let json = serde_json::to_string_pretty(&map)?;
                        // std::fs::write(&json_path, json)?;
                        panic!("Expected file not found: {json_path:?}");
                    }
                } else {
                    panic!("Failed to resolve: {path:?}");
                }
            }
        }
        Ok(())
    }

    fn path_for_test(test_dir: &Path, nvram_path: &PathBuf) -> io::Result<PathBuf> {
        let path = if nvram_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains('-')
        {
            // take the file name and remove the - and everything after it
            let rom_name = nvram_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split('-')
                .next()
                .unwrap();
            let new_path = test_dir.join(rom_name).with_extension("nv");
            std::fs::copy(nvram_path, &new_path)?;
            new_path
        } else {
            nvram_path.clone()
        };
        Ok(path)
    }
}
