use crate::core::{AccountManager, TaskScheduler};
use crate::db::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub account_manager: Arc<AccountManager>,
    pub scheduler: Arc<RwLock<Option<Arc<TaskScheduler>>>>,
}

impl AppState {
    pub fn new(db: Arc<Database>, account_manager: Arc<AccountManager>) -> Self {
        Self {
            db,
            account_manager,
            scheduler: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_scheduler(&self, scheduler: Arc<TaskScheduler>) {
        let mut s = self.scheduler.write().await;
        *s = Some(scheduler);
    }

    pub async fn get_scheduler(&self) -> Option<Arc<TaskScheduler>> {
        let s = self.scheduler.read().await;
        s.clone()
    }
}
