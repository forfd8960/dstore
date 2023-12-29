use std::{collections::HashMap, sync::Arc};

use crate::{
    errors::KvError,
    pb::pb::{commond_request::RequestData, CommandResponse, Get, HGet, HSet, Kv, Set},
    storage::{MemTable, Storage},
};

pub struct StoreServer {
    data_store: Arc<dyn Storage>,
}

impl StoreServer {
    pub fn new() -> Self {
        let memtable = MemTable::new(100);
        Self {
            data_store: Arc::new(memtable),
        }
    }

    pub fn dispatch(&self, req: RequestData) -> Result<CommandResponse, KvError> {
        match req {
            RequestData::Get(get_req) => self.get(get_req),
            RequestData::Set(set_req) => self.set(set_req),
            RequestData::Hget(hget_req) => self.hget(hget_req),
            RequestData::Hset(hset_req) => self.hset(hset_req),
        }
    }

    pub fn get(&self, get: Get) -> Result<CommandResponse, KvError> {
        let value = self.data_store.get(&get.key)?;
        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![value],
        })
    }

    pub fn set(&self, set: Set) -> Result<CommandResponse, KvError> {
        let kv = set.kv.unwrap();
        let res = self.data_store.set(&kv.key, &kv.value)?;

        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![res.unwrap()],
        })
    }

    pub fn hget(&self, hget: HGet) -> Result<CommandResponse, KvError> {
        let res = self.data_store.hget(&hget.key, &hget.field)?;
        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![res.unwrap()],
        })
    }

    pub fn hset(&self, hset: HSet) -> Result<CommandResponse, KvError> {
        let data = hset.data.unwrap();
        let mut m = HashMap::new();
        for field_v in data.field_values {
            m.insert(field_v.key, field_v.value);
        }

        let res = self.data_store.hset(&data.key, m)?;
        Ok(CommandResponse {
            status: 0,
            message: res.to_string(),
            pairs: vec![],
        })
    }
}

impl Clone for StoreServer {
    fn clone(&self) -> Self {
        Self {
            data_store: Arc::clone(&self.data_store),
        }
    }
}
