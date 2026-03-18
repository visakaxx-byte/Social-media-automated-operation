use crate::db::DbOperations;
use crate::models::{Content, ContentType};
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
pub struct CreateContentRequest {
    pub content_type: String,
    pub platform: String,
    pub title: Option<String>,
    pub body: String,
    pub media_paths: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ContentResponse {
    pub id: String,
    pub content_type: String,
    pub platform: String,
    pub title: Option<String>,
    pub body: String,
    pub media_paths: Vec<String>,
    pub tags: Vec<String>,
    pub used_count: i32,
    pub created_at: String,
}

impl From<Content> for ContentResponse {
    fn from(content: Content) -> Self {
        Self {
            id: content.id,
            content_type: content.content_type.as_str().to_string(),
            platform: content.platform,
            title: content.title,
            body: content.body,
            media_paths: content.media_paths,
            tags: content.tags,
            used_count: content.used_count,
            created_at: content.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_contents(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContentResponse>>, StatusCode> {
    let contents = state
        .db
        .list_contents()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<ContentResponse> = contents.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

pub async fn get_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ContentResponse>, StatusCode> {
    let content = state
        .db
        .get_content(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(content.into()))
}

pub async fn create_content(
    State(state): State<AppState>,
    Json(req): Json<CreateContentRequest>,
) -> Result<Json<ContentResponse>, StatusCode> {
    let content_type = ContentType::from_str(&req.content_type).ok_or(StatusCode::BAD_REQUEST)?;

    let content = Content {
        id: Uuid::new_v4().to_string(),
        content_type,
        platform: req.platform,
        title: req.title,
        body: req.body,
        media_paths: req.media_paths,
        tags: req.tags,
        source: None,
        created_at: Utc::now(),
        used_count: 0,
    };

    state
        .db
        .create_content(&content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(content.into()))
}

pub async fn delete_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    state
        .db
        .delete_content(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
