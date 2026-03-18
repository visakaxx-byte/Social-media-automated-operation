use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: String,
    pub content_type: ContentType,
    pub platform: String,
    pub title: Option<String>,
    pub body: String,
    pub media_paths: Vec<String>,
    pub tags: Vec<String>,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
    pub used_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Image,
    Video,
    Mixed,
}

impl ContentType {
    pub fn as_str(&self) -> &str {
        match self {
            ContentType::Text => "text",
            ContentType::Image => "image",
            ContentType::Video => "video",
            ContentType::Mixed => "mixed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "text" => Some(ContentType::Text),
            "image" => Some(ContentType::Image),
            "video" => Some(ContentType::Video),
            "mixed" => Some(ContentType::Mixed),
            _ => None,
        }
    }
}
