use std::collections::HashMap;
use std::sync::Arc;

use common::async_trait::async_trait;
use domain::models::Backlog;
use ports::{BacklogRepository, PortError};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct HashMapDb(Arc<Mutex<HashMap<u8, Backlog>>>);

impl Default for HashMapDb {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

#[async_trait]
impl BacklogRepository for HashMapDb {
    async fn get(&self) -> Result<Option<Backlog>, PortError> {
        let db = self.0.clone();
        let db = db.lock().await;
        Ok(db.get(&0).cloned())
    }

    async fn save(&self, backlog: Backlog) -> Result<(), PortError> {
        let db = self.0.clone();
        let mut db = db.lock().await;
        db.insert(0, backlog);
        Ok(())
    }
}
