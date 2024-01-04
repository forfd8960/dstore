use std::collections::HashMap;

use crate::errors::KvError;
use crate::pb::pb;

mod memory;
pub use memory::MemTable;

pub trait Storage {
    fn get(&self, key: &str) -> Result<pb::Kv, KvError>;
    fn set(&self, key: &str, value: &str) -> Result<Option<pb::Kv>, KvError>;
    fn hget(&self, key: &str, field: &str) -> Result<Option<pb::Kv>, KvError>;
    fn hset(&self, key: &str, m: HashMap<String, String>) -> Result<i64, KvError>;
    fn sadd(&self, key: &str, values: Vec<String>) -> Result<i64, KvError>;
    fn smembers(&self, key: &str) -> Result<Vec<String>, KvError>;
    fn lpush(&self, key: &str, elements: Vec<String>) -> Result<i64, KvError>;
    fn lpop(&self, key: &str, count: i64) -> Result<Vec<String>, KvError>;
    fn lrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>, KvError>;
}
