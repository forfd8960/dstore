use crate::pb::pb;
use crate::{errors::KvError, storage::Storage};

pub fn get(data_store: &impl Storage, key: &str) -> Result<Option<pb::Kv>, KvError> {
    let result = data_store.get(key)?;
    Ok(result)
}

#[derive(Debug, Clone)]
pub enum Cmd {
    Get(String),
    Set(String, String),
}
