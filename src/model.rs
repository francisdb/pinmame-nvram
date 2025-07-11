use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::fmt;

pub const DEFAULT_LENGTH: usize = 1;
pub const DEFAULT_SCALE: i32 = 1;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MemoryLayoutType {
    Ram,
    NVRam,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Strings>,
    pub label: String,
    pub address: HexOrInteger,
    pub size: HexOrInteger,
    #[serde(rename = "type")]
    pub type_: MemoryLayoutType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
}

impl MemoryLayout {
    pub(crate) fn nibble(&self) -> Nibble {
        self.nibble.unwrap_or(Nibble::Both)
    }
}

/// A platform defines the native memory layout
///
/// The nvram map will contain physical addresses instead of nvram file addresses if
/// the platform is defined.
#[derive(Serialize, Deserialize)]
pub struct Platform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Strings>,
    pub cpu: String,
    pub endian: Endian,
    pub memory_layout: Vec<MemoryLayout>,
}

impl Platform {
    // TODO this should probably take a memory address instead
    pub(crate) fn nibble(&self, memory_layout_type: MemoryLayoutType) -> Nibble {
        self.layout(memory_layout_type)
            .nibble
            .unwrap_or(Nibble::Both)
    }

    // TODO this should probably take a memory address instead
    pub(crate) fn offset(&self, memory_layout_type: MemoryLayoutType) -> u64 {
        (&self.layout(memory_layout_type).address).into()
    }

    pub(crate) fn layout(&self, memory_layout_type: MemoryLayoutType) -> &MemoryLayout {
        if let Some(layout) = self
            .memory_layout
            .iter()
            .find(|l| l.type_ == memory_layout_type)
        {
            layout
        } else {
            panic!("Memory layout not found for {memory_layout_type:?}");
        }
    }
}

/// Descriptor for a single value in the NVRAM.
/// Describing a section of the .nv file and how to interpret it
///
/// see https://github.com/tomlogic/pinmame-nvram-maps?tab=readme-ov-file#descriptors
#[derive(Serialize, Deserialize)]
pub struct Descriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Strings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<HexOrInteger>,
    pub encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<StringOrNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ValuesOrReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_values: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Number>,
    // can be negative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<HexOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endian: Option<Endian>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<HexOrInteger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<Null>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Checksum16 {
    pub start: HexOrInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<HexOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Checksum8 {
    pub start: HexOrInteger,
    pub end: HexOrInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<u64>,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Strings>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValuesOrReference {
    // TODO adjust both according to result of https://github.com/tomlogic/pinmame-nvram-maps/issues/83
    Values(Vec<Value>),
    Reference(String),
}

// "enum": An enumerated type where the byte at start is used as an index into a list of strings provided in values.
// "int": A (possibly) multibyte integer, where each byte is multiplied by a power of 256. The byte sequence 0x12 0x34 would translate to the decimal value 4660.
// "bits": Same decoding as "int", but used to sum select integers from the list in values.
// "bcd": A binary-coded decimal value, where each byte represents two decimal digits of a number. The byte sequence 0x12 0x34 would translate to the decimal value 1234.
// "ch": A sequence of 7-bit ASCII characters. If the JSON file has a _char_map key, use bytes from the NV file as indexes into that string instead of interpreting them as 7-bit ASCII.
// "raw": A series of raw bytes, useful for extracting data yet to be decoded or that requires custom processing.
// "wpc_rtc"
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    Enum,
    Int,
    Bits,
    Bcd,
    Ch,
    Raw,
    #[serde(rename = "wpc_rtc")]
    WpcRtc,
    /// Dip switches
    Dipsw,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Null {
    /// Ignore (skip over) null bytes.
    Ignore,
    /// A null can shorten the string, but won't be present for strings that fill the allotted space.
    Truncate,
    /// Null bytes are always present and terminate the string.
    Terminate,
}

#[derive(Serialize, Deserialize)]
pub struct ModeChampion {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<Descriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<Descriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Descriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter: Option<Descriptor>,
    #[serde(rename = "nth time", skip_serializing_if = "Option::is_none")]
    pub nth_time: Option<Descriptor>,
}

#[derive(Serialize, Deserialize)]
pub struct HighScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<Descriptor>,
    pub score: Descriptor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum HexOrInteger {
    Hex(HexString),
    Integer(i64),
}
impl From<&HexOrInteger> for u64 {
    fn from(h: &HexOrInteger) -> u64 {
        match h {
            HexOrInteger::Hex(h) => h.value,
            HexOrInteger::Integer(i) => *i as u64,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Nibble {
    Both,
    High,
    Low,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Endian {
    Big,
    Little,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Strings {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Adjustments {
    Anonymous(Vec<Descriptor>),
    Named(HashMap<String, Descriptor>),
}

#[derive(Debug, PartialEq)]
pub struct HexString {
    pub value: u64,
    pub serialized: String,
}

impl From<&HexString> for u64 {
    fn from(h: &HexString) -> u64 {
        h.value
    }
}

impl Serialize for HexString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.serialized.as_str())
    }
}

impl<'de> Deserialize<'de> for HexString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            let value = u64::from_str_radix(&s[2..], 16).unwrap();
            Ok(HexString {
                value,
                serialized: s,
            })
        } else {
            Err(serde::de::Error::custom("invalid hex string"))
        }
    }
}

impl fmt::Display for HexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.serialized, self.value)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StateOrStateList {
    State(Box<Descriptor>),
    StateList(Vec<Descriptor>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(u64),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditOrNote {
    Audit(Box<Descriptor>),
    Note(String),
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Strings>,
    pub copyright: Strings,
    pub license: String,
    pub platform: String,
    pub version: Number,
    pub roms: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct NvramMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Strings>,
    pub _fileformat: f64,
    pub _metadata: Metadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _todo: Option<Strings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _ramsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _history: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_played: Option<Descriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_game: Option<Vec<Descriptor>>,
    pub high_scores: Vec<HighScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_champions: Option<Vec<ModeChampion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_mode_champions: Option<Vec<ModeChampion>>,
    // keys are normally numbers except for notes, which as value are strings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audits: Option<HashMap<String, HashMap<String, AuditOrNote>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<HashMap<String, Adjustments>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum8: Option<Vec<Checksum8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum16: Option<Vec<Checksum16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_state: Option<HashMap<String, StateOrStateList>>,
    /// TODO for now we don't do anything with this, so we keep it generic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_state: Option<Value>,
    /// TODO this HashMap<String, StateOrStateList>should probably be removed as it is an adjustment and only used in ww_l5.nv.json
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_score: Option<Descriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyin_high_scores: Option<Vec<HighScore>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dip_switches: Option<HashMap<String, Descriptor>>,
}

impl NvramMap {
    pub fn char_map(&self) -> &Option<String> {
        &self._metadata.char_map
    }
}

pub trait GlobalSettings {
    fn char_map(&self) -> &Option<String>;
    fn value(&self, key: &str, index: usize) -> Option<String>;
    fn platform(&self) -> &str;
}

impl GlobalSettings for NvramMap {
    fn char_map(&self) -> &Option<String> {
        self.char_map()
    }
    fn value(&self, key: &str, index: usize) -> Option<String> {
        self._metadata
            .values
            .as_ref()?
            .get(key)?
            .get(index)
            .cloned()
    }
    fn platform(&self) -> &str {
        &self._metadata.platform
    }
}

#[derive(Serialize, Deserialize)]
pub struct GlobalSettingsImpl {
    pub _metadata: Metadata,
}

impl GlobalSettings for GlobalSettingsImpl {
    fn char_map(&self) -> &Option<String> {
        &self._metadata.char_map
    }

    fn value(&self, key: &str, index: usize) -> Option<String> {
        self._metadata
            .values
            .as_ref()?
            .get(key)?
            .get(index)
            .cloned()
    }

    fn platform(&self) -> &str {
        &self._metadata.platform
    }
}

// test: read file write file and compare

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::path::Path;
    use walkdir::WalkDir;

    #[test]
    fn read_all_nvram_maps() {
        // read all ../pinmame-nvram-maps/*.json recursively
        let maps_path = Path::new("pinmame-nvram-maps").join("maps");
        let mut found_any = false;
        for entry in WalkDir::new(maps_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if file_name.ends_with(".nv.json") {
                found_any = true;
                let json = std::fs::read_to_string(path).unwrap();
                let nvram_map: NvramMap = serde_json::from_str(&json)
                    .unwrap_or_else(|e| panic!("Failed reading {path:?}: {e}"));
                let json2 = serde_json::to_string_pretty(&nvram_map).unwrap();

                // read json as Value to compare without formatting
                let json_obj: Value = serde_json::from_str(&json).unwrap();
                let json_obj2: Value = serde_json::from_str(&json2).unwrap();

                assert_eq!(json_obj, json_obj2, "Failed for {path:?}");
            }
        }
        assert!(found_any, "No nvram map files found");
    }

    #[test]
    fn read_all_platforms() {
        // read all ../pinmame-nvram-maps/platforms/*.json recursively
        let platforms_path = Path::new("pinmame-nvram-maps").join("platforms");
        let mut found_any = false;
        for entry in WalkDir::new(platforms_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if file_name.ends_with(".json") {
                found_any = true;
                let json = std::fs::read_to_string(path).unwrap();
                let platform: Platform = serde_json::from_str(&json)
                    .unwrap_or_else(|e| panic!("Failed reading {file_name}: {e}"));
                let json2 = serde_json::to_string_pretty(&platform).unwrap();

                // read json as Value to compare without formatting
                let json_obj: Value = serde_json::from_str(&json).unwrap();
                let json_obj2: Value = serde_json::from_str(&json2).unwrap();

                assert_eq!(json_obj, json_obj2, "Failed for {}", file_name);
            }
        }
        assert!(found_any, "No platform files found");
    }
}
