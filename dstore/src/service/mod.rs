use std::{collections::HashMap, sync::Arc};

use crate::{
    errors::KvError,
    pb::pb::{
        commond_request::RequestData, CommandResponse, Get, HGet, HSet, LPop, LPush, LRange, SAdd,
        SMembers, Scard, Set, Value,
    },
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
            RequestData::Lpush(lpush_req) => self.lpush(lpush_req),
            RequestData::Lpop(lpop_req) => self.lpop(lpop_req),
            RequestData::Lrange(lrange_req) => self.lrange(lrange_req),
            RequestData::Scard(scard_req) => self.scard(scard_req),
        }
    }

    pub fn get(&self, get: Get) -> Result<CommandResponse, KvError> {
        let value = self.server_inner.data_store.get(&get.key)?;
        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![value],
            values: vec![],
        })
    }

    pub fn set(&self, set: Set) -> Result<CommandResponse, KvError> {
        let kv = set.kv.unwrap();
        let res = self.server_inner.data_store.set(&kv.key, &kv.value)?;

        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![res.unwrap()],
            values: vec![],
        })
    }

    pub fn hget(&self, hget: HGet) -> Result<CommandResponse, KvError> {
        let res = self.server_inner.data_store.hget(&hget.key, &hget.field)?;
        Ok(CommandResponse {
            status: 0,
            message: "OK".to_string(),
            pairs: vec![res.unwrap()],
            values: vec![],
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
            values: vec![],
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
            values: vec![],
        })
    }

    pub fn smembers(&self, smembers_req: SMembers) -> Result<CommandResponse, KvError> {
        let res = self.server_inner.data_store.smembers(&smembers_req.key)?;
        Ok(CommandResponse {
            status: 0,
            message: format!("{:?}", res),
            pairs: vec![],
            values: Vec::from_iter(res.iter().map(|x| Value { val: x.to_string() })),
        })
    }

    pub fn scard(&self, smembers_req: Scard) -> Result<CommandResponse, KvError> {
        let res = self.server_inner.data_store.scard(&smembers_req.key)?;
        Ok(CommandResponse {
            status: 0,
            message: format!("{:?}", res),
            pairs: vec![],
            values: vec![Value {
                val: res.to_string(),
            }],
        })
    }

    pub fn lpush(&self, lpush_req: LPush) -> Result<CommandResponse, KvError> {
        let res = self
            .server_inner
            .data_store
            .lpush(&lpush_req.key, lpush_req.elements)?;
        Ok(CommandResponse {
            status: 0,
            message: format!("{:?}", res),
            pairs: vec![],
            values: vec![],
        })
    }

    pub fn lpop(&self, lpop_req: LPop) -> Result<CommandResponse, KvError> {
        let res = self
            .server_inner
            .data_store
            .lpop(&lpop_req.key, lpop_req.count)?;
        Ok(CommandResponse {
            status: 0,
            message: "".to_string(),
            pairs: vec![],
            values: Vec::from_iter(res.iter().map(|x| Value { val: x.to_string() })),
        })
    }

    pub fn lrange(&self, lrange_req: LRange) -> Result<CommandResponse, KvError> {
        let res = self.server_inner.data_store.lrange(
            &lrange_req.key,
            lrange_req.start,
            lrange_req.stop,
        )?;
        Ok(CommandResponse {
            status: 0,
            message: "".to_string(),
            pairs: vec![],
            values: Vec::from_iter(res.iter().map(|x| Value { val: x.to_string() })),
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
