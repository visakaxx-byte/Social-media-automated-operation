use crate::models::{Account, Content};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Post data structure for platform operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub media_urls: Vec<String>,
    pub tags: Vec<String>,
    pub url: Option<String>,
}

/// Login result
#[derive(Debug, Clone)]
pub enum LoginResult {
    Success,
    NeedsVerification(String), // Verification type (captcha, sms, etc.)
    Failed(String),             // Error message
}

/// Platform adapter trait
/// Each social media platform implements this trait
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Get platform name
    fn platform_name(&self) -> &str;

    /// Login to the platform
    async fn login(&self, account: &Account, password: &str) -> Result<LoginResult>;

    /// Check if currently logged in
    async fn check_login_status(&self) -> Result<bool>;

    /// Publish content
    async fn post(&self, content: &Content) -> Result<String>;

    /// Like a post
    async fn like(&self, target_id: &str) -> Result<()>;

    /// Comment on a post
    async fn comment(&self, target_id: &str, text: &str) -> Result<()>;

    /// Follow a user
    async fn follow(&self, user_id: &str) -> Result<()>;

    /// Unfollow a user
    async fn unfollow(&self, user_id: &str) -> Result<()>;

    /// Get feed posts
    async fn get_feed(&self, limit: usize) -> Result<Vec<Post>>;

    /// Search for posts by keyword
    async fn search(&self, keyword: &str, limit: usize) -> Result<Vec<Post>>;

    /// Get user's posts
    async fn get_user_posts(&self, user_id: &str, limit: usize) -> Result<Vec<Post>>;
}
