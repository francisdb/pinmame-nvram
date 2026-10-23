//! Expansion of `repeat` templates (file format 0.9).
//!
//! A descriptor in an array can carry `"repeat": {"count": n, "step": s}`, which
//! stands for `n` copies of it. Copy `i` (0-based) has `i * s` added to its
//! `start` and `checksum` addresses, and to the `start` of its nested
//! descriptors (the `initials`, `score`, ... of a high score or mode champion).
//! `{#}` in `label` and `short_label` is replaced by the 1-based copy number.
//!
//! see https://github.com/tomlogic/pinball-memory-maps?tab=readme-ov-file#descriptors

use serde_json::{Map, Value};

/// Replace every templated entry in every array of the map by its copies.
pub(crate) fn expand_repeats(value: &mut Value) {
    match value {
        Value::Object(map) => map.values_mut().for_each(expand_repeats),
        Value::Array(array) => {
            let expanded = std::mem::take(array)
                .into_iter()
                .flat_map(|mut entry| {
                    expand_repeats(&mut entry);
                    expand_entry(entry)
                })
                .collect();
            *array = expanded;
        }
        _ => {}
    }
}

fn expand_entry(entry: Value) -> Vec<Value> {
    let Value::Object(mut template) = entry else {
        return vec![entry];
    };
    let Some(repeat) = template.remove("repeat") else {
        return vec![Value::Object(template)];
    };
    let count = repeat.get("count").and_then(Value::as_u64).unwrap_or(1);
    let step = repeat.get("step").and_then(Value::as_u64).unwrap_or(0);
    (0..count)
        .map(|i| {
            let mut copy = template.clone();
            let shift = i * step;
            // a mirror without `start` has an implicit start of 0
            if !copy.contains_key("start") && copy.contains_key("offsets") {
                copy.insert("start".to_string(), Value::from(0u64));
            }
            shift_address(&mut copy, "start", shift);
            shift_address(&mut copy, "checksum", shift);
            for nested in copy.values_mut() {
                if let Value::Object(nested) = nested {
                    shift_address(nested, "start", shift);
                }
            }
            for key in ["label", "short_label"] {
                if let Some(Value::String(label)) = copy.get_mut(key) {
                    *label = label.replace("{#}", &(i + 1).to_string());
                }
            }
            Value::Object(copy)
        })
        .collect()
}

/// Add `shift` to an address that is either a number or a `0x` hex string,
/// keeping its representation.
fn shift_address(map: &mut Map<String, Value>, key: &str, shift: u64) {
    let Some(address) = map.get_mut(key) else {
        return;
    };
    match address {
        Value::Number(n) => {
            if let Some(n) = n.as_u64() {
                *address = Value::from(n + shift);
            }
        }
        Value::String(s) => {
            if let Some(hex) = s.strip_prefix("0x")
                && let Ok(n) = u64::from_str_radix(hex, 16)
            {
                *s = format!("0x{:X}", n + shift);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn test_expand_mode_champions() {
        let mut map = json!({
            "mode_champions": [
                {"label": "First", "score": {"start": 10, "encoding": "bcd"}},
                {
                    "label": "Masters #{#}",
                    "short_label": "M #{#}",
                    "initials": {"start": "0x1ED1", "encoding": "ch", "length": 3},
                    "score": {"start": 7892, "encoding": "bcd", "length": 6},
                    "repeat": {"count": 2, "step": 9}
                }
            ]
        });
        expand_repeats(&mut map);
        assert_eq!(
            map,
            json!({
                "mode_champions": [
                    {"label": "First", "score": {"start": 10, "encoding": "bcd"}},
                    {
                        "label": "Masters #1",
                        "short_label": "M #1",
                        "initials": {"start": "0x1ED1", "encoding": "ch", "length": 3},
                        "score": {"start": 7892, "encoding": "bcd", "length": 6}
                    },
                    {
                        "label": "Masters #2",
                        "short_label": "M #2",
                        "initials": {"start": "0x1EDA", "encoding": "ch", "length": 3},
                        "score": {"start": 7901, "encoding": "bcd", "length": 6}
                    }
                ]
            })
        );
    }

    #[test]
    fn test_expand_validation() {
        let mut map = json!({
            "_metadata": {"validation": {
                "checksum": [{
                    "label": "Audit {#}", "start": "0x10", "length": 4,
                    "checksum": "0x80", "complement": true,
                    "repeat": {"count": 2, "step": 4}
                }],
                "mirror": [{
                    "label": "Copy", "length": 2, "offsets": [0, 8],
                    "repeat": {"count": 2, "step": 16}
                }]
            }}
        });
        expand_repeats(&mut map);
        assert_eq!(
            map,
            json!({
                "_metadata": {"validation": {
                    "checksum": [
                        {"label": "Audit 1", "start": "0x10", "length": 4, "checksum": "0x80", "complement": true},
                        {"label": "Audit 2", "start": "0x14", "length": 4, "checksum": "0x84", "complement": true}
                    ],
                    "mirror": [
                        {"label": "Copy", "start": 0, "length": 2, "offsets": [0, 8]},
                        {"label": "Copy", "start": 16, "length": 2, "offsets": [0, 8]}
                    ]
                }}
            })
        );
    }
}
