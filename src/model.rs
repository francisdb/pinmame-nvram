use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Checksum16 {
    pub start: HexOrInteger,
    pub end: HexOrInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Checksum8 {
    pub start: HexOrInteger,
    pub end: HexOrInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<u64>,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct Adjustment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _note: Option<String>,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_label: Option<String>,
    pub start: HexOrInteger,
    pub encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<StringOrNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_values: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    // can be negative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Audit {
    pub label: String,
    pub start: String,
    pub encoding: String,
    pub length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Score {
    // TODO when is this None? How do we then read it?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<HexOrInteger>,
    pub encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<HexOrInteger>>,
}

#[derive(Serialize, Deserialize)]
pub struct ModeChampion {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_label: Option<String>,
    pub initials: Initials,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<Score>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<LastGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter: Option<LastGame>,
    #[serde(rename = "nth time", skip_serializing_if = "Option::is_none")]
    pub nth_time: Option<LastGame>,
}

#[derive(Serialize, Deserialize)]
pub struct HighScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<Initials>,
    pub score: Score,
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

#[derive(Serialize, Deserialize)]
pub struct LastGame {
    pub start: HexOrInteger,
    pub encoding: String,
    pub length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Nibble {
    High,
    Low,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Endian {
    Big,
    Little,
}

#[derive(Serialize, Deserialize)]
pub struct Initials {
    pub start: HexOrInteger,
    pub encoding: String,
    pub length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nibble: Option<Nibble>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<HexOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _note: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Notes {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Adjustments {
    Anonymous(Vec<Adjustment>),
    Named(HashMap<String, Adjustment>),
}

#[derive(Debug, PartialEq)]
pub struct HexString {
    pub value: u64,
    pub serialized: String,
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

enum IntegerOrFloat {
    Integer(i64),
    Float(f64),
}

impl Serialize for IntegerOrFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IntegerOrFloat::Integer(i) => serializer.serialize_i64(*i),
            IntegerOrFloat::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

impl<'de> Deserialize<'de> for IntegerOrFloat {
    fn deserialize<D>(deserializer: D) -> Result<IntegerOrFloat, D::Error>
    where
        D: Deserializer<'de>,
    {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(IntegerOrFloat::Integer(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(IntegerOrFloat::Float(f))
                } else {
                    Err(serde::de::Error::custom("invalid number"))
                }
            }
            _ => Err(serde::de::Error::custom("expected number")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip_serializing_if = "Option::is_none")]
    _note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    _note2: Option<String>,
    encoding: String,
    label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nibble: Option<Nibble>,
    start: HexOrInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask: Option<HexString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endian: Option<Endian>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<IntegerOrFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_values: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StateOrStateList {
    State(Box<State>),
    StateList(Vec<State>),
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
    Audit(Audit),
    Note(String),
}

#[derive(Serialize, Deserialize)]
pub struct NvramMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _notes: Option<Notes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _todo: Option<Notes>,
    pub _copyright: String,
    pub _license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _endian: Option<Endian>,
    // TODO remove these cases from original database
    #[deprecated = "use _endian instead"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endian: Option<Endian>,
    pub _roms: Vec<String>,
    pub _fileformat: f64,
    pub _version: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _ramsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _history: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _char_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_played: Option<LastGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_game: Option<Vec<LastGame>>,
    pub high_scores: Vec<HighScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_champions: Option<Vec<ModeChampion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_mode_champions: Option<Vec<ModeChampion>>,
    // keys are nomrally numbers except for notes, which as value are strings
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_score: Option<Score>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyin_high_scores: Option<Vec<HighScore>>,
}

// test: read file write file and compare

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;

    #[test]
    fn read_all_nvram_maps() {
        // read all ../pinmame-nvram-maps/*.json
        for file in std::fs::read_dir("pinmame-nvram-maps").unwrap() {
            let file = file.unwrap();
            let path = file.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if file_name.ends_with(".nv.json") {
                // println!("Reading {}", file_name);
                let json = std::fs::read_to_string(path).unwrap();
                let nvram_map: NvramMap = serde_json::from_str(&json).unwrap();
                let json2 = serde_json::to_string_pretty(&nvram_map).unwrap();

                // read json as Value to compare without formatting
                let json_obj: Value = serde_json::from_str(&json).unwrap();
                let json_obj2: Value = serde_json::from_str(&json2).unwrap();

                assert_eq!(json_obj, json_obj2, "Failed for {}", file_name);
            }
        }
    }
}
