use crate::pb::pb;
use crate::{errors::KvError, storage::Storage};

pub fn get(data_store: &impl Storage, key: &str) -> Result<Option<pb::Kv>, KvError> {
    let result = data_store.get(key)?;
    Ok(result)
}

pub fn set(data_store: &impl Storage, key: &str, value: &str) -> Result<Option<pb::Kv>, KvError> {
    let result = data_store.set(key, value)?;
    Ok(result)
}
