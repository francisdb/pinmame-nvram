use crate::checksum::{verify_checksum8, verify_checksum16};
use crate::encoding::{
    Location, read_bcd, read_bool, read_ch, read_exact_at, read_int, read_wpc_rtc,
};
use crate::model::{
    Checksum8, Checksum16, DEFAULT_INVERT, DEFAULT_LENGTH, DEFAULT_SCALE, Encoding, Endian,
    GlobalSettings, GlobalSettingsImpl, MemoryLayout, MemoryLayoutType, Nibble, Null, Platform,
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
                let resolved = resolve_value(rom, map, encoding, global_settings, platform);
                let warning = match &resolved {
                    Ok((_, range_value)) => validate_range(map, *range_value),
                    Err(e) => Some(format!("Failed to resolve: {e}")),
                };
                let mut resolved_map = Map::new();
                if let Ok((value, _)) = resolved {
                    resolved_map.insert("value".to_string(), value);
                }
                if let Some(label) = map.get("label") {
                    // maybe we should instead remove all properties related to the encoding
                    resolved_map.insert("label".to_string(), label.clone());
                }
                // Surface the map's unit annotation on resolved numeric fields
                // so downstream consumers can render time-like / distance-like
                // scores with the right formatting (e.g. `units: "seconds"`
                // -> `mm:ss.dd`) without hardcoding per-table knowledge.
                // `scale` is intentionally not propagated: `value` is already
                // post-scaled, so exposing it would just invite double-scaling
                // bugs. See issue #124.
                if let Some(units) = map.get("units") {
                    resolved_map.insert("units".to_string(), units.clone());
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

/// Validate the raw numeric value against the descriptor's min and max range.
/// If the value is out of range a warning is returned.
///
/// `value` is the raw number the range applies to: the integer for `int`/`bcd`
/// encodings, or the selected index for `enum` encodings. Encodings that do not
/// produce a number pass `None` and are never range-checked (they also never
/// carry min/max).
fn validate_range(map: &Map<String, Value>, value: Option<u64>) -> Option<String> {
    let (Some(min), Some(max)) = (map.get("min"), map.get("max")) else {
        return None;
    };
    let number_value = value?;
    // min and max are unscaled values so we need to unscale the value first
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

/// Resolve a descriptor to its display value plus the raw numeric value that
/// range validation should be applied to.
///
/// For most numeric encodings the raw value matches the display value, but for
/// `enum` the display value is a string label while the range (`min`/`max`)
/// applies to the underlying index. Encodings that do not produce a number
/// return `None` as the raw value and are never range-checked.
fn resolve_value<T: Read + Seek, U: GlobalSettings>(
    rom: &mut T,
    descriptor: &Map<String, Value>,
    encoding: Encoding,
    global_settings: &U,
    platform: &Platform,
) -> io::Result<(Value, Option<u64>)> {
    // we only have access to nvram files and cannot access the RAM.
    let nvram_layout = platform.layout(MemoryLayoutType::NVRam);
    let nibble = nvram_layout.nibble();
    let endian = platform.endian;
    let length = descriptor
        .get("length")
        .map_or(DEFAULT_LENGTH, |v| v.as_u64().unwrap() as usize);
    // The raw numeric value the descriptor's min/max range applies to, if any.
    let mut range_value: Option<u64> = None;
    let value = match encoding {
        Encoding::Int => {
            let scale = descriptor
                .get("scale")
                .and_then(|s| s.as_number())
                .cloned()
                .unwrap_or(Number::from(DEFAULT_SCALE));
            let location = location_in_nvram_file(nvram_layout, descriptor, length)?;
            let value = read_int(rom, endian, nibble, location, &scale)?;
            range_value = Some(value);
            Value::Number(value.into())
        }
        Encoding::Enum => {
            let location = location_in_nvram_file(nvram_layout, descriptor, length)?;
            let index =
                read_int(rom, endian, nibble, location, &Number::from(DEFAULT_SCALE))? as usize;
            let values = descriptor.get("values").unwrap().as_array().unwrap();
            match values.get(index) {
                Some(enum_value) => {
                    // The range (min/max) constrains the raw index, not the label.
                    range_value = Some(index as u64);
                    enum_value.clone()
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Index {} out of bounds for enum with {} values",
                            index,
                            values.len()
                        ),
                    ));
                }
            }
        }
        Encoding::Bcd => {
            let location = location_in_nvram_file(nvram_layout, descriptor, length)?;
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
            range_value = Some(value);
            Value::Number(value.into())
        }
        Encoding::Ch => {
            let location = location_in_nvram_file(nvram_layout, descriptor, length)?;
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
                location,
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
        Encoding::Bool => {
            let location = location_in_nvram_file(nvram_layout, descriptor, length)?;
            let invert = descriptor
                .get("invert")
                .and_then(|v| v.as_bool())
                .unwrap_or(DEFAULT_INVERT);
            let bool_value = read_bool(rom, nibble, endian, location, invert)?;
            Value::Bool(bool_value)
        }
    };
    Ok((value, range_value))
}

/// Resolve the location of a value in the NVRAM file from a descriptor.
///
/// A descriptor either has a single `start` address (a contiguous run of
/// `length` bytes) or an `offsets` array listing the address of each byte
/// (used by platforms that map 8-bit NVRAM on a wider bus, so the bytes are
/// not adjacent in the file). Both forms are CPU addresses and are translated
/// to file offsets by subtracting the NVRAM layout base address.
fn location_in_nvram_file(
    nvram_layout: &MemoryLayout,
    descriptor: &Map<String, Value>,
    length: usize,
) -> io::Result<Location> {
    match descriptor.get("offsets") {
        Some(offsets) => {
            let base: u64 = (&nvram_layout.address).into();
            let offsets = offsets
                .as_array()
                .unwrap()
                .iter()
                .map(|o| {
                    let address = json_hex_or_int(o)?;
                    if address < base {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Value is stored outside the NVRAM",
                        ))
                    } else {
                        Ok(address - base)
                    }
                })
                .collect::<io::Result<Vec<u64>>>()?;
            Ok(Location::Scattered { offsets })
        }
        None => {
            let start = start_in_nvram_file(nvram_layout, descriptor)?;
            Ok(Location::Continuous { start, length })
        }
    }
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

    fn range_map() -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("min".to_string(), Value::from(0));
        map.insert("max".to_string(), Value::from(31));
        map
    }

    #[test]
    fn test_validate_range_in_range() {
        assert_eq!(validate_range(&range_map(), Some(15)), None);
    }

    #[test]
    fn test_validate_range_out_of_range() {
        assert_eq!(
            validate_range(&range_map(), Some(255)),
            Some("Value out of range: 0 ≤ 255 ≤ 31".to_string())
        );
    }

    #[test]
    fn test_validate_range_no_raw_value_is_skipped() {
        // encodings that do not produce a number pass None and are not checked
        assert_eq!(validate_range(&range_map(), None), None);
    }

    #[test]
    fn test_validate_range_no_bounds() {
        // a descriptor without min/max is never range-checked
        assert_eq!(validate_range(&Map::new(), Some(255)), None);
    }

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
    fn test_resolve_propagates_units() -> io::Result<()> {
        // Lord of the Rings has a Destroy Ring Champion score annotated as
        // `units: "seconds"` in the map. We want it present on the resolved
        // value so downstream consumers can render time-like scores as
        // mm:ss.dd without per-table knowledge (see #124).
        //
        // `scale` is deliberately not propagated; `value` is already post-
        // scaled by resolve_value, so exposing it would invite double-scaling
        // bugs in callers.
        let resolved = resolve(Path::new("testdata/lotr.nv"))?.expect("lotr.nv should resolve");
        let drc = resolved
            .get("mode_champions")
            .and_then(|v| v.as_array())
            .and_then(|a| {
                a.iter().find(|e| {
                    e.get("label") == Some(&Value::String("Destroy Ring Champion".to_string()))
                })
            })
            .expect("Destroy Ring Champion entry");
        let score = drc.get("score").expect("score object");
        assert_eq!(
            score.get("units").and_then(|v| v.as_str()),
            Some("seconds"),
            "units annotation should be on the resolved score"
        );
        assert!(
            score.get("scale").is_none(),
            "scale should NOT be propagated - value is already post-scaled"
        );
        Ok(())
    }

    #[test]
    fn test_missing_test_nvrams() -> io::Result<()> {
        let excludes = ["_note"];

        let missing_nvrams_path = Path::new("testdata/aaa_missing_nvrams.txt");
        let missing_nvrams_content = std::fs::read_to_string(missing_nvrams_path)?;

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
                missing.push(rom.as_str());
            }
        }
        // assert equals on a tx file format
        let missing = missing.join("\n");
        let expected_missing = missing_nvrams_content
            .replace("\r\n", "\n")
            .trim()
            .to_string();
        assert_eq!(missing, expected_missing);
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

    /// Warnings of this kind are tolerated wholesale by the ratchet: the value
    /// is simply not part of the dumped NVRAM region, which is common and
    /// expected, so we do not enumerate them in the expected-warnings file.
    const OUTSIDE_NVRAM_WARNING: &str = "Value is stored outside the NVRAM";

    fn collect_warnings(rom: &str, value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                if let Some(Value::String(warning)) = map.get("warning") {
                    if !warning.contains(OUTSIDE_NVRAM_WARNING) {
                        let label = map.get("label").and_then(|l| l.as_str()).unwrap_or("(no label)");
                        out.push(format!("{rom} | {label} | {warning}"));
                    }
                }
                for v in map.values() {
                    collect_warnings(rom, v, out);
                }
            }
            Value::Array(array) => {
                for v in array {
                    collect_warnings(rom, v, out);
                }
            }
            _ => {}
        }
    }

    /// Ratchet: the set of resolve warnings (excluding "outside NVRAM") must
    /// match testdata/aaa_expected_warnings.txt exactly. A new unexpected
    /// warning fails here even when added together with a fresh golden
    /// .nv.json, and a fixed warning must be removed from the list on purpose.
    ///
    /// The failure message is directional: warnings present now but missing
    /// from the file are flagged as regressions to investigate, while warnings
    /// in the file that no longer occur are flagged for removal (e.g. after an
    /// upstream map fix).
    #[test]
    fn test_expected_warnings() -> io::Result<()> {
        let test_dir = testdir!();
        let mut actual = Vec::new();
        for entry in std::fs::read_dir("testdata")? {
            let nvram_path = entry?.path();
            if nvram_path.extension().and_then(|e| e.to_str()) != Some("nv") {
                continue;
            }
            // The display name keeps the original file stem (e.g. the
            // "-default" suffix), while resolving needs the rom-named path.
            let rom = nvram_path.file_stem().unwrap().to_str().unwrap().to_string();
            let path = path_for_test(&test_dir, &nvram_path)?;
            if let Some(value) = resolve(&path)? {
                collect_warnings(&rom, &value, &mut actual);
            }
        }
        let actual: std::collections::BTreeSet<String> = actual.into_iter().collect();

        let expected_content = std::fs::read_to_string("testdata/aaa_expected_warnings.txt")?;
        let expected: std::collections::BTreeSet<String> = expected_content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| l.to_string())
            .collect();

        let new: Vec<&String> = actual.difference(&expected).collect();
        let resolved: Vec<&String> = expected.difference(&actual).collect();

        if !new.is_empty() || !resolved.is_empty() {
            let mut msg = String::from(
                "resolve warnings differ from testdata/aaa_expected_warnings.txt\n",
            );
            if !new.is_empty() {
                msg.push_str(
                    "\nNEW unexpected warnings - a regression to investigate, \
                     or add these lines to the file if intended:\n",
                );
                for w in &new {
                    msg.push_str(&format!("  + {w}\n"));
                }
            }
            if !resolved.is_empty() {
                msg.push_str(
                    "\nExpected warnings that no longer occur - remove these lines \
                     from the file:\n",
                );
                for w in &resolved {
                    msg.push_str(&format!("  - {w}\n"));
                }
            }
            panic!("{msg}");
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
