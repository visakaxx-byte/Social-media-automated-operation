use crate::db::DbOperations;
use crate::models::{Task, TaskStatus, TaskType};
use crate::web::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub task_type: String,
    pub account_id: String,
    pub content_id: Option<String>,
    pub metadata: Option<String>,
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub task_type: String,
    pub account_id: String,
    pub content_id: Option<String>,
    pub status: String,
    pub scheduled_at: Option<String>,
    pub executed_at: Option<String>,
    pub retry_count: i32,
    pub priority: i32,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            task_type: task.task_type.as_str().to_string(),
            account_id: task.account_id,
            content_id: task.content_id,
            status: task.status.as_str().to_string(),
            scheduled_at: task.scheduled_at.map(|dt| dt.to_rfc3339()),
            executed_at: task.executed_at.map(|dt| dt.to_rfc3339()),
            retry_count: task.retry_count,
            priority: task.priority,
        }
    }
}

pub async fn list_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<TaskResponse>>, StatusCode> {
    let tasks = state
        .db
        .list_tasks(None)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<TaskResponse> = tasks.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let task = state
        .db
        .get_task(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task.into()))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let task_type = TaskType::from_str(&req.task_type).ok_or(StatusCode::BAD_REQUEST)?;

    let scheduled_at = if let Some(dt_str) = req.scheduled_at {
        Some(
            chrono::DateTime::parse_from_rfc3339(&dt_str)
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let task = Task {
        id: Uuid::new_v4().to_string(),
        task_type,
        account_id: req.account_id,
        content_id: req.content_id,
        status: TaskStatus::Pending,
        scheduled_at,
        executed_at: None,
        retry_count: 0,
        priority: 0,
        metadata: req.metadata,
    };

    state
        .db
        .create_task(&task)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(task.into()))
}

pub async fn cancel_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, StatusCode> {
    state
        .db
        .update_task_status(&id, TaskStatus::Cancelled)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = state
        .db
        .get_task(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task.into()))
}
