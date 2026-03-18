use crate::db::{Database, DbOperations};
use crate::models::{Account, AccountStatus};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AccountManager {
    db: Arc<Database>,
    // Cache of active accounts
    active_accounts: Arc<RwLock<HashMap<String, Account>>>,
}

impl AccountManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            active_accounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load all active accounts into cache
    pub async fn load_active_accounts(&self) -> Result<()> {
        let accounts = self.db.list_accounts()?;
        let mut cache = self.active_accounts.write().await;

        for account in accounts {
            if account.status == AccountStatus::Active {
                cache.insert(account.id.clone(), account);
            }
        }

        tracing::info!("Loaded {} active accounts", cache.len());
        Ok(())
    }

    /// Get an account by ID
    pub async fn get_account(&self, account_id: &str) -> Result<Option<Account>> {
        // Try cache first
        {
            let cache = self.active_accounts.read().await;
            if let Some(account) = cache.get(account_id) {
                return Ok(Some(account.clone()));
            }
        }

        // Fall back to database
        self.db.get_account(account_id)
    }

    /// Update account status
    pub async fn update_account_status(&self, account_id: &str, status: AccountStatus) -> Result<()> {
        self.db.update_account_status(account_id, status.clone())?;

        // Update cache
        let mut cache = self.active_accounts.write().await;
        if let Some(account) = cache.get_mut(account_id) {
            account.status = status.clone();
        }

        // If status changed to active, add to cache
        if status == AccountStatus::Active {
            if let Some(account) = self.db.get_account(account_id)? {
                cache.insert(account_id.to_string(), account);
            }
        } else {
            // Remove from cache if not active
            cache.remove(account_id);
        }

        Ok(())
    }

    /// Get all active accounts
    pub async fn get_active_accounts(&self) -> Vec<Account> {
        let cache = self.active_accounts.read().await;
        cache.values().cloned().collect()
    }

    /// Check account health
    pub async fn check_account_health(&self, account_id: &str) -> Result<i32> {
        if let Some(account) = self.get_account(account_id).await? {
            Ok(account.health_score)
        } else {
            Err(anyhow::anyhow!("Account not found: {}", account_id))
        }
    }

    /// Update account health score
    pub async fn update_health_score(&self, account_id: &str, delta: i32) -> Result<()> {
        if let Some(mut account) = self.get_account(account_id).await? {
            account.health_score = (account.health_score + delta).clamp(0, 100);

            // Update in database (would need a new method in DbOperations)
            tracing::info!(
                "Updated health score for account {}: {}",
                account_id,
                account.health_score
            );

            // Update cache
            let mut cache = self.active_accounts.write().await;
            if let Some(cached_account) = cache.get_mut(account_id) {
                cached_account.health_score = account.health_score;
            }
        }

        Ok(())
    }

    /// Get account count by status
    pub async fn get_account_count_by_status(&self, status: AccountStatus) -> Result<usize> {
        let accounts = self.db.list_accounts()?;
        Ok(accounts.iter().filter(|a| a.status == status).count())
    }
}
