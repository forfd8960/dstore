use std::collections::{HashMap, HashSet};

use dashmap::DashMap;

use super::Storage;
use crate::errors::KvError;
use crate::pb::pb;

#[derive(Debug, Clone)]
pub struct MemTable {
    simple_k_v: DashMap<String, String>,
    data_map: DashMap<String, DashMap<String, String>>,
    simple_set: DashMap<String, HashSet<String>>,
    list: DashMap<String, Vec<String>>,
}

impl MemTable {
    pub fn new(cap: i64) -> Self {
        Self {
            simple_k_v: DashMap::with_capacity(cap as usize),
            data_map: DashMap::with_capacity(cap as usize),
            simple_set: DashMap::with_capacity(cap as usize),
            list: DashMap::with_capacity(cap as usize),
        }
    }
}

impl Storage for MemTable {
    fn get(&self, key: &str) -> Result<pb::Kv, KvError> {
        let result = self.simple_k_v.get(key);
        match result {
            Some(kv) => Ok(pb::Kv {
                key: kv.key().to_string(),
                value: kv.value().to_string(),
            }),
            None => Err(KvError::NotFound(key.to_string())),
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<Option<pb::Kv>, KvError> {
        let result = self.simple_k_v.insert(key.to_string(), value.to_string());
        match result {
            Some(_) => Ok(Some(pb::Kv {
                key: key.to_string(),
                value: value.to_string(),
            })),
            None => Err(KvError::NotFound(key.to_string())),
        }
    }

    fn sadd(&self, key: &str, values: Vec<String>) -> Result<i64, KvError> {
        let result = self.simple_set.get_mut(key);
        let length = values.len();
        match result {
            Some(mut set) => {
                for v in values {
                    set.insert(v);
                }
            }
            None => {
                self.simple_set
                    .insert(key.to_string(), HashSet::from_iter(values));
            }
        }
        Ok(length as i64)
    }

    // smembers key
    /*
    127.0.0.1:6379> smembers set2
    1) "hello"
    2) "world"
    3) "good"
    4) "morning"
    5) "happy"
    6) "new"
    7) "year"
    127.0.0.1:6379> SMEMBERS no-set
    (empty array)
    */
    fn smembers(&self, key: &str) -> Result<Vec<String>, KvError> {
        let mut members = vec![];
        let result = self.simple_set.get(key);
        match result {
            Some(set) => {
                for v in set.iter() {
                    members.push(v.clone());
                }
            }
            None => {}
        }

        Ok(members)
    }

    fn scard(&self, key: &str) -> Result<i64, KvError> {
        let result = self.simple_set.get(key);
        match result {
            Some(set) => Ok(set.len() as i64),
            None => Ok(0),
        }
    }

    fn hget(&self, key: &str, field: &str) -> Result<Option<pb::Kv>, KvError> {
        let result = self.data_map.get(key);
        match result {
            Some(m) => {
                let value = m.get(field);
                match value {
                    Some(v) => Ok(Some(pb::Kv {
                        key: key.to_string(),
                        value: v.to_string(),
                    })),
                    None => Err(KvError::NotFound(field.to_string())),
                }
            }
            None => Err(KvError::NotFound(key.to_string())),
        }
    }

    fn hset(&self, key: &str, m: HashMap<String, String>) -> Result<i64, KvError> {
        let d: DashMap<String, String> = DashMap::new();
        for (k, v) in m.iter() {
            d.insert(k.to_string(), v.to_string());
        }
        let len = d.len();
        self.data_map.insert(key.to_string(), d);
        Ok(len as i64)
    }

    fn lpush(&self, key: &str, elements: Vec<String>) -> Result<i64, KvError> {
        let result = self.list.get_mut(key);
        match result {
            Some(mut l) => {
                for e in elements {
                    l.insert(0, e);
                }
                Ok(l.len() as i64)
            }
            None => {
                self.list.insert(key.to_string(), elements.clone());
                Ok(elements.len() as i64)
            }
        }
    }

    fn lpop(&self, key: &str, count: i64) -> Result<Vec<String>, KvError> {
        let result = self.list.get_mut(key);
        match result {
            Some(mut l) => {
                if count <= 1 {
                    let u = l.drain(..1);
                    return Ok(u.collect());
                }

                if count as usize >= l.len() {
                    let u = l.drain(..);
                    return Ok(u.collect());
                }

                let u = l.drain(..count as usize);
                return Ok(u.collect());
            }
            None => Ok(vec![]),
        }
    }

    fn lrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>, KvError> {
        let result = self.list.get(key);
        match result {
            Some(l) => {
                let start1 = start as usize;
                if start1 > l.len() {
                    return Ok(vec![]);
                }

                let mut end = (stop + 1) as usize;
                if end > l.len() {
                    end = l.len();
                }

                if start1 > end {
                    return Ok(vec![]);
                }

                Ok(Vec::from_iter(l[start1..end].iter().map(|x| x.to_string())))
            }
            None => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MemTable;
    use crate::storage::Storage;

    #[test]
    fn test_lpush() {
        let mem_table = MemTable::new(1000);
        let res = mem_table.lpush("test1", vec!["data1".to_string(), "data2".to_string()]);
        assert_eq!(true, res.is_ok());
        assert_eq!(2 as i64, res.unwrap());

        let data = mem_table.lpop("test1", 1);
        assert_eq!(true, data.is_ok());
        assert_eq!(vec!["data1".to_string()], data.unwrap());
    }

    #[test]
    fn test_lrange() {
        let mem_table = MemTable::new(1000);
        let res = mem_table.lpush(
            "test1",
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(true, res.is_ok());
        assert_eq!(3 as i64, res.unwrap());

        let data = mem_table.lrange("test1", 0, 0);
        assert_eq!(true, data.is_ok());
        assert_eq!(vec!["a".to_string()], data.unwrap());

        let data1 = mem_table.lrange("test1", 0, 6);
        assert_eq!(true, data1.is_ok());
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            data1.unwrap()
        );

        let data2 = mem_table.lrange("test1", 0, 1);
        assert_eq!(true, data2.is_ok());
        assert_eq!(vec!["a".to_string(), "b".to_string()], data2.unwrap());

        let data3 = mem_table.lrange("test1", 0, 2);
        assert_eq!(true, data3.is_ok());
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            data3.unwrap()
        );
    }
}
