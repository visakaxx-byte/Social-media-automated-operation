use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_type: TaskType,
    pub account_id: String,
    pub content_id: Option<String>,
    pub status: TaskStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub priority: i32,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Post,
    Like,
    Comment,
    Follow,
}

impl TaskType {
    pub fn as_str(&self) -> &str {
        match self {
            TaskType::Post => "post",
            TaskType::Like => "like",
            TaskType::Comment => "comment",
            TaskType::Follow => "follow",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "post" => Some(TaskType::Post),
            "like" => Some(TaskType::Like),
            "comment" => Some(TaskType::Comment),
            "follow" => Some(TaskType::Follow),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(TaskStatus::Pending),
            "running" => Some(TaskStatus::Running),
            "completed" => Some(TaskStatus::Completed),
            "failed" => Some(TaskStatus::Failed),
            "cancelled" => Some(TaskStatus::Cancelled),
            _ => None,
        }
    }
}
