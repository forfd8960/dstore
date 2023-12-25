use dashmap::DashMap;

use super::Storage;
use crate::errors::KvError;
use crate::pb::pb;

pub struct MemTable {
    dashmap: DashMap<String, DashMap<String, String>>,
}

impl Storage for MemTable {
    fn get(&self, key: &str) -> Result<Option<pb::Kv>, KvError> {
        Ok(Some(pb::Kv {
            key: "Hello".to_string(),
            value: "World!".to_string(),
        }))
    }
}
