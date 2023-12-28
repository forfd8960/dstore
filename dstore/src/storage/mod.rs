use std::collections::HashMap;

use crate::errors::KvError;
use crate::pb::pb;

mod memory;
pub use memory::MemTable;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Option<pb::Kv>, KvError>;
    fn set(&self, key: &str, value: &str) -> Result<Option<pb::Kv>, KvError>;
    fn hget(&self, key: &str, field: &str) -> Result<Option<pb::Kv>, KvError>;
    fn hset(&self, key: &str, m: HashMap<String, String>) -> Result<i64, KvError>;
}
