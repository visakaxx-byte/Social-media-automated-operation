use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub platform: String,
    pub username: String,
    pub profile_path: String,
    pub proxy: Option<String>,
    pub fingerprint_preset: Option<String>,
    pub status: AccountStatus,
    pub last_active: Option<DateTime<Utc>>,
    pub health_score: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    NeedsVerification,
}

impl AccountStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AccountStatus::Active => "active",
            AccountStatus::Inactive => "inactive",
            AccountStatus::Suspended => "suspended",
            AccountStatus::NeedsVerification => "needs_verification",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(AccountStatus::Active),
            "inactive" => Some(AccountStatus::Inactive),
            "suspended" => Some(AccountStatus::Suspended),
            "needs_verification" => Some(AccountStatus::NeedsVerification),
            _ => None,
        }
    }
}
