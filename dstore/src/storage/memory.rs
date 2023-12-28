use std::collections::HashMap;

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

    fn set(&self, key: &str, value: &str) -> Result<Option<pb::Kv>, KvError> {
        let result = self.simple_k_v.insert(key.to_string(), value.to_string());
        match result {
            Some(_) => Ok(Some(pb::Kv {
                key: key.to_string(),
                value: value.to_string(),
            })),
            None => Err(KvError::NotFound(key.to_string())),
        }
    }

    fn hget(&self, key: &str, field: &str) -> Result<Option<pb::Kv>, KvError> {
        let result = self.data_map.get(key);
        match result {
            Some(m) => {
                let value = m.get(field);
                match value {
                    Some(v) => Ok(Some(pb::Kv {
                        key: key.to_string(),
                        value: v.to_string(),
                    })),
                    None => Err(KvError::NotFound(key.to_string())),
                }
            }
            None => Err(KvError::NotFound(key.to_string())),
        }
    }

    fn hset(&self, key: &str, m: HashMap<String, String>) -> Result<i64, KvError> {
        let d: DashMap<String, String> = DashMap::new();
        for (k, v) in m.iter() {
            d.insert(k.to_string(), v.to_string());
        }
        let len = d.len();
        self.data_map.insert(key.to_string(), d);
        Ok(len as i64)
    }
}
