use dashmap::DashMap;

use super::Storage;
use crate::errors::KvError;
use crate::pb::pb;

pub struct MemTable {
    simple_k_v: DashMap<String, String>,
    data_map: DashMap<String, DashMap<String, String>>,
}

impl MemTable {
    pub fn new(cap: i64) -> Self {
        Self {
            simple_k_v: DashMap::with_capacity(cap as usize),
            data_map: DashMap::with_capacity(cap as usize),
        }
    }
}

impl Storage for MemTable {
    fn get(&self, key: &str) -> Result<Option<pb::Kv>, KvError> {
        let result = self.simple_k_v.get(key);
        match result {
            Some(kv) => Ok(Some(pb::Kv {
                key: kv.key().to_string(),
                value: kv.value().to_string(),
            })),
            None => Err(KvError::NotFound(key.to_string())),
        }
    }
}
