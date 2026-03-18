use crate::config::Settings;
use crate::db::{Database, DbOperations};
use crate::models::{Task, TaskStatus};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

pub struct TaskScheduler {
    db: Arc<Database>,
    settings: Settings,
    running: Arc<RwLock<bool>>,
}

impl TaskScheduler {
    pub fn new(db: Arc<Database>, settings: Settings) -> Self {
        Self {
            db,
            settings,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the scheduler
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Err(anyhow::anyhow!("Scheduler is already running"));
        }
        *running = true;
        drop(running);

        tracing::info!("Task scheduler started");

        let check_interval = self.settings.scheduler.check_interval;
        let mut ticker = interval(Duration::from_secs(check_interval));

        loop {
            ticker.tick().await;

            // Check if we should stop
            if !*self.running.read().await {
                break;
            }

            // Process pending tasks
            if let Err(e) = self.process_pending_tasks().await {
                tracing::error!("Error processing tasks: {}", e);
            }
        }

        tracing::info!("Task scheduler stopped");
        Ok(())
    }

    /// Stop the scheduler
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("Stopping task scheduler...");
    }

    /// Process pending tasks
    async fn process_pending_tasks(&self) -> Result<()> {
        // Get all pending tasks
        let tasks = self.db.list_tasks(Some(TaskStatus::Pending))?;

        tracing::debug!("Found {} pending tasks", tasks.len());

        for task in tasks {
            // Check if task should be executed now
            if self.should_execute_task(&task) {
                tracing::info!("Executing task: {} ({})", task.id, task.task_type.as_str());

                // Update task status to running
                self.db.update_task_status(&task.id, TaskStatus::Running)?;

                // Execute task (this would be handled by TaskExecutor)
                // For now, just mark as completed
                // In production, this would spawn a task to execute asynchronously

                // Simulate task execution
                tokio::time::sleep(Duration::from_millis(100)).await;

                // Mark as completed
                self.db.update_task_status(&task.id, TaskStatus::Completed)?;

                tracing::info!("Task completed: {}", task.id);
            }
        }

        Ok(())
    }

    /// Check if a task should be executed now
    fn should_execute_task(&self, task: &Task) -> bool {
        // If no scheduled time, execute immediately
        if task.scheduled_at.is_none() {
            return true;
        }

        // Check if scheduled time has passed
        if let Some(scheduled_at) = task.scheduled_at {
            return Utc::now() >= scheduled_at;
        }

        false
    }

    /// Get scheduler status
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Schedule a task for execution
    pub async fn schedule_task(&self, task_id: &str, scheduled_at: chrono::DateTime<Utc>) -> Result<()> {
        // Update task scheduled_at time
        // This would require adding a method to DbOperations
        tracing::info!("Scheduled task {} for {}", task_id, scheduled_at);
        Ok(())
    }

    /// Get pending task count
    pub async fn get_pending_count(&self) -> Result<usize> {
        let tasks = self.db.list_tasks(Some(TaskStatus::Pending))?;
        Ok(tasks.len())
    }

    /// Get running task count
    pub async fn get_running_count(&self) -> Result<usize> {
        let tasks = self.db.list_tasks(Some(TaskStatus::Running))?;
        Ok(tasks.len())
    }
}
