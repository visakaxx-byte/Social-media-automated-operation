use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub system: SystemConfig,
    pub browser: BrowserConfig,
    pub stealth: StealthConfig,
    pub scheduler: SchedulerConfig,
    pub rate_limits: HashMap<String, RateLimits>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemConfig {
    pub max_concurrent_tasks: usize,
    pub task_timeout: u64,
    pub log_level: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BrowserConfig {
    pub headless: bool,
    pub chrome_path: Option<String>,
    pub user_data_dir: String,
    pub window_size: [u32; 2],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StealthConfig {
    pub enabled: bool,
    pub randomize_fingerprint: bool,
    pub block_webrtc: bool,
    pub timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchedulerConfig {
    pub check_interval: u64,
    pub retry_max: u32,
    pub retry_backoff: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimits {
    pub posts_per_day: u32,
    pub likes_per_hour: u32,
    pub comments_per_hour: u32,
    pub follows_per_day: u32,
}
