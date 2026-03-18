mod settings;

pub use settings::{Settings, SystemConfig, BrowserConfig, StealthConfig, SchedulerConfig, RateLimits};

use anyhow::Result;
use std::path::Path;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Settings> {
    let content = std::fs::read_to_string(path)?;
    let settings: Settings = serde_yaml::from_str(&content)?;
    Ok(settings)
}
