use std::collections::HashMap;

use crate::pb::pb;
use crate::{errors::KvError, storage::Storage};

pub fn get(data_store: &impl Storage, key: &str) -> Result<pb::Kv, KvError> {
    let result = data_store.get(key)?;
    Ok(result)
}

pub fn set(data_store: &impl Storage, key: &str, value: &str) -> Result<Option<pb::Kv>, KvError> {
    let result = data_store.set(key, value)?;
    Ok(result)
}

pub fn hget(data_store: &impl Storage, key: &str, field: &str) -> Result<Option<pb::Kv>, KvError> {
    let result = data_store.hget(key, field)?;
    Ok(result)
}

pub fn hset(
    data_store: &impl Storage,
    key: &str,
    m: HashMap<String, String>,
) -> Result<i64, KvError> {
    let result = data_store.hset(key, m)?;
    Ok(result)
}
