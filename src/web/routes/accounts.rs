use crate::db::DbOperations;
use crate::models::{Account, AccountStatus};
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
pub struct CreateAccountRequest {
    pub platform: String,
    pub username: String,
    pub proxy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: String,
    pub platform: String,
    pub username: String,
    pub status: String,
    pub health_score: i32,
    pub last_active: Option<String>,
    pub created_at: String,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            platform: account.platform,
            username: account.username,
            status: account.status.as_str().to_string(),
            health_score: account.health_score,
            last_active: account.last_active.map(|dt| dt.to_rfc3339()),
            created_at: account.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<AccountResponse>>, StatusCode> {
    let accounts = state
        .db
        .list_accounts()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<AccountResponse> = accounts.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AccountResponse>, StatusCode> {
    let account = state
        .db
        .get_account(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(account.into()))
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, StatusCode> {
    let account = Account {
        id: Uuid::new_v4().to_string(),
        platform: req.platform.clone(),
        username: req.username.clone(),
        profile_path: format!("./profiles/{}_{}", req.platform, req.username),
        proxy: req.proxy,
        fingerprint_preset: None,
        status: AccountStatus::Inactive,
        last_active: None,
        health_score: 100,
        created_at: Utc::now(),
    };

    state
        .db
        .create_account(&account)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(account.into()))
}

pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAccountRequest>,
) -> Result<Json<AccountResponse>, StatusCode> {
    if let Some(status_str) = req.status {
        let status = AccountStatus::from_str(&status_str).ok_or(StatusCode::BAD_REQUEST)?;

        state
            .db
            .update_account_status(&id, status)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let account = state
        .db
        .get_account(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(account.into()))
}

pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    state
        .db
        .delete_account(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
