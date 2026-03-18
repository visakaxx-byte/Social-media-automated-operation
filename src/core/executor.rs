use crate::core::AccountManager;
use crate::db::{Database, DbOperations};
use crate::models::{Content, Task, TaskStatus, TaskType};
use crate::platforms::{PlatformAdapter, PlatformFactory};
use anyhow::Result;
use std::sync::Arc;

pub struct TaskExecutor {
    db: Arc<Database>,
    account_manager: Arc<AccountManager>,
}

impl TaskExecutor {
    pub fn new(db: Arc<Database>, account_manager: Arc<AccountManager>) -> Self {
        Self {
            db,
            account_manager,
        }
    }

    /// Execute a task
    pub async fn execute_task(&self, task: &Task) -> Result<()> {
        tracing::info!("Executing task: {} ({})", task.id, task.task_type.as_str());

        // Update task status to running
        self.db.update_task_status(&task.id, TaskStatus::Running)?;

        // Execute based on task type
        let result = match task.task_type {
            TaskType::Post => self.execute_post_task(task).await,
            TaskType::Like => self.execute_like_task(task).await,
            TaskType::Comment => self.execute_comment_task(task).await,
            TaskType::Follow => self.execute_follow_task(task).await,
        };

        // Update task status based on result
        match result {
            Ok(_) => {
                self.db.update_task_status(&task.id, TaskStatus::Completed)?;
                tracing::info!("Task completed successfully: {}", task.id);
            }
            Err(e) => {
                self.db.update_task_status(&task.id, TaskStatus::Failed)?;
                tracing::error!("Task failed: {} - {}", task.id, e);

                // Update account health score
                self.account_manager
                    .update_health_score(&task.account_id, -5)
                    .await?;
            }
        }

        Ok(())
    }

    /// Execute a post task
    async fn execute_post_task(&self, task: &Task) -> Result<()> {
        // Get account
        let account = self
            .account_manager
            .get_account(&task.account_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Account not found: {}", task.account_id))?;

        // Get content
        let content_id = task
            .content_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No content ID for post task"))?;

        let content = self
            .db
            .get_content(content_id)?
            .ok_or_else(|| anyhow::anyhow!("Content not found: {}", content_id))?;

        // Get platform adapter
        let adapter = PlatformFactory::create(&account.platform)?;

        // Publish content
        let post_id = adapter.post(&content).await?;

        tracing::info!("Published post: {} on {}", post_id, account.platform);

        // Update account health score (positive)
        self.account_manager
            .update_health_score(&task.account_id, 1)
            .await?;

        Ok(())
    }

    /// Execute a like task
    async fn execute_like_task(&self, task: &Task) -> Result<()> {
        let account = self
            .account_manager
            .get_account(&task.account_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Account not found: {}", task.account_id))?;

        // Extract target ID from metadata
        let target_id = task
            .metadata
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No target ID in metadata"))?;

        let adapter = PlatformFactory::create(&account.platform)?;
        adapter.like(target_id).await?;

        tracing::info!("Liked post: {} on {}", target_id, account.platform);

        Ok(())
    }

    /// Execute a comment task
    async fn execute_comment_task(&self, task: &Task) -> Result<()> {
        let account = self
            .account_manager
            .get_account(&task.account_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Account not found: {}", task.account_id))?;

        // Parse metadata for target_id and comment_text
        // In production, metadata would be JSON with structured data
        let metadata = task
            .metadata
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No metadata for comment task"))?;

        // Simplified: assume metadata is "target_id:comment_text"
        let parts: Vec<&str> = metadata.split(':').collect();
        if parts.len() < 2 {
            return Err(anyhow::anyhow!("Invalid metadata format"));
        }

        let target_id = parts[0];
        let comment_text = parts[1];

        let adapter = PlatformFactory::create(&account.platform)?;
        adapter.comment(target_id, comment_text).await?;

        tracing::info!("Commented on post: {} on {}", target_id, account.platform);

        Ok(())
    }

    /// Execute a follow task
    async fn execute_follow_task(&self, task: &Task) -> Result<()> {
        let account = self
            .account_manager
            .get_account(&task.account_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Account not found: {}", task.account_id))?;

        let user_id = task
            .metadata
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No user ID in metadata"))?;

        let adapter = PlatformFactory::create(&account.platform)?;
        adapter.follow(user_id).await?;

        tracing::info!("Followed user: {} on {}", user_id, account.platform);

        Ok(())
    }
}
