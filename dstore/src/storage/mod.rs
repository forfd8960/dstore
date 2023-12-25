use crate::errors::KvError;
use crate::pb::pb;

mod memory;
pub use memory::MemTable;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Option<pb::Kv>, KvError>;
}
