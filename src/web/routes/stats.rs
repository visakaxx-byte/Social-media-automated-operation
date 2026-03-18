use crate::db::DbOperations;
use crate::models::{AccountStatus, TaskStatus};
use crate::web::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub accounts: AccountStats,
    pub tasks: TaskStats,
    pub contents: ContentStats,
}

#[derive(Debug, Serialize)]
pub struct AccountStats {
    pub total: usize,
    pub active: usize,
    pub inactive: usize,
    pub suspended: usize,
}

#[derive(Debug, Serialize)]
pub struct TaskStats {
    pub total: usize,
    pub pending: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
}

#[derive(Debug, Serialize)]
pub struct ContentStats {
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub scheduler_running: bool,
    pub active_accounts: usize,
    pub pending_tasks: usize,
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<StatsResponse>, StatusCode> {
    let accounts = state
        .db
        .list_accounts()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tasks = state
        .db
        .list_tasks(None)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let contents = state
        .db
        .list_contents()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let account_stats = AccountStats {
        total: accounts.len(),
        active: accounts
            .iter()
            .filter(|a| a.status == AccountStatus::Active)
            .count(),
        inactive: accounts
            .iter()
            .filter(|a| a.status == AccountStatus::Inactive)
            .count(),
        suspended: accounts
            .iter()
            .filter(|a| a.status == AccountStatus::Suspended)
            .count(),
    };

    let task_stats = TaskStats {
        total: tasks.len(),
        pending: tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .count(),
        running: tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Running)
            .count(),
        completed: tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .count(),
        failed: tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Failed)
            .count(),
    };

    let content_stats = ContentStats {
        total: contents.len(),
    };

    Ok(Json(StatsResponse {
        accounts: account_stats,
        tasks: task_stats,
        contents: content_stats,
    }))
}

pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<StatusResponse>, StatusCode> {
    let scheduler_running = if let Some(scheduler) = state.get_scheduler().await {
        scheduler.is_running().await
    } else {
        false
    };

    let active_accounts = state.account_manager.get_active_accounts().await.len();

    let pending_tasks = state
        .db
        .list_tasks(Some(TaskStatus::Pending))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .len();

    Ok(Json(StatusResponse {
        scheduler_running,
        active_accounts,
        pending_tasks,
    }))
}
