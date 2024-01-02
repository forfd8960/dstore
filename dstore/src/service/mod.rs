use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    errors::KvError,
    pb::pb::{commond_request::RequestData, CommandResponse, Get, HGet, HSet, SAdd, SMembers, Set},
    storage::{MemTable, Storage},
};

pub struct StoreServer {
    server_inner: Arc<ServerInner>,
}

struct ServerInner {
    data_store: Box<dyn Storage + Send + Sync>,
}

impl StoreServer {
    pub fn new() -> Self {
        let data_store: Box<dyn Storage + Send + Sync> = Box::new(MemTable::new(100));
        Self {
            server_inner: Arc::new(ServerInner {
                data_store: data_store,
            }),
        }
    }

    pub fn dispatch(&self, req: RequestData) -> Result<CommandResponse, KvError> {
        match req {
            RequestData::Get(get_req) => self.get(get_req),
            RequestData::Set(set_req) => self.set(set_req),
            RequestData::Hget(hget_req) => self.hget(hget_req),
            RequestData::Hset(hset_req) => self.hset(hset_req),
            RequestData::Sadd(sadd_req) => self.sadd(sadd_req),
            RequestData::Smembers(smembers_req) => self.smembers(smembers_req),
        }
    }

    pub fn get(&self, get: Get) -> Result<CommandResponse, KvError> {
        let value = self.server_inner.data_store.get(&get.key)?;
        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![value],
        })
    }

    pub fn set(&self, set: Set) -> Result<CommandResponse, KvError> {
        let kv = set.kv.unwrap();
        let res = self.server_inner.data_store.set(&kv.key, &kv.value)?;

        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![res.unwrap()],
        })
    }

    pub fn hget(&self, hget: HGet) -> Result<CommandResponse, KvError> {
        let res = self.server_inner.data_store.hget(&hget.key, &hget.field)?;
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

        let res = self.server_inner.data_store.hset(&data.key, m)?;
        Ok(CommandResponse {
            status: 0,
            message: res.to_string(),
            pairs: vec![],
        })
    }

    pub fn sadd(&self, sadd_req: SAdd) -> Result<CommandResponse, KvError> {
        let res = self
            .server_inner
            .data_store
            .sadd(&sadd_req.key, sadd_req.values)?;
        Ok(CommandResponse {
            status: 0,
            message: res.to_string(),
            pairs: vec![],
        })
    }

    pub fn smembers(&self, smembers_req: SMembers) -> Result<CommandResponse, KvError> {
        let res = self.server_inner.data_store.smembers(&smembers_req.key)?;
        Ok(CommandResponse {
            status: 0,
            message: format!("{:?}", res),
            pairs: vec![],
        })
    }
}

impl Clone for StoreServer {
    fn clone(&self) -> Self {
        Self {
            server_inner: Arc::clone(&self.server_inner),
        }
    }
}
