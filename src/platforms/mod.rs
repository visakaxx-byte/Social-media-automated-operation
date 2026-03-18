mod platform_trait;
mod xiaohongshu;
mod twitter;

pub use platform_trait::{PlatformAdapter, Post, LoginResult};
pub use xiaohongshu::XiaohongshuAdapter;
pub use twitter::TwitterAdapter;

use anyhow::Result;
use std::collections::HashMap;

/// Platform adapter factory
pub struct PlatformFactory;

impl PlatformFactory {
    pub fn create(platform: &str) -> Result<Box<dyn PlatformAdapter>> {
        match platform.to_lowercase().as_str() {
            "xiaohongshu" => Ok(Box::new(XiaohongshuAdapter::new())),
            "twitter" => Ok(Box::new(TwitterAdapter::new())),
            _ => Err(anyhow::anyhow!("Unsupported platform: {}", platform)),
        }
    }

    pub fn supported_platforms() -> Vec<&'static str> {
        vec!["xiaohongshu", "twitter"]
    }
}
