use std::sync::Arc;

use crate::storage::{MemTable, Storage};

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
}

impl Clone for StoreServer {
    fn clone(&self) -> Self {
        Self {
            data_store: Arc::clone(&self.data_store),
        }
    }
}
