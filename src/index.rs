use crate::{read_compressed_json, MAPS};
use lazy_static::lazy_static;
use serde_json::Value;
use std::io;
use std::sync::RwLock;

pub(crate) struct MapIndex {
    map: Value,
}

impl MapIndex {
    fn new() -> io::Result<Self> {
        let index_path = "index.json.brotli";
        let index_file = MAPS.get_file(index_path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", index_path),
            )
        })?;
        let map: Value = read_compressed_json(index_file)?;
        Ok(MapIndex { map })
    }

    pub(crate) fn get(&self, rom_name: &String) -> Option<&Value> {
        if rom_name == "_note" {
            return None;
        }
        self.map.get(rom_name)
    }
}

lazy_static! {
    static ref INDEX_MAP: RwLock<MapIndex> = RwLock::new(MapIndex::new().unwrap());
}

pub(crate) fn get_index_map() -> io::Result<std::sync::RwLockReadGuard<'static, MapIndex>> {
    Ok(INDEX_MAP.read().unwrap())
}
