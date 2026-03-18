// Common test utilities and helpers

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use tempfile::TempDir;

/// Create a temporary test database
pub fn create_test_db() -> Result<(Connection, TempDir)> {
    let temp_dir = tempfile::tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let conn = Connection::open(&db_path)?;

    // Initialize schema
    social_auto::db::init_database(&conn)?;

    Ok((conn, temp_dir))
}

/// Create a test configuration
pub fn create_test_config() -> social_auto::config::Settings {
    use social_auto::config::*;
    use std::collections::HashMap;

    Settings {
        system: SystemConfig {
            max_concurrent_tasks: 2,
            task_timeout: 60,
            log_level: "debug".to_string(),
            data_dir: "./test_data".to_string(),
        },
        browser: BrowserConfig {
            headless: true,
            chrome_path: None,
            user_data_dir: "./test_profiles".to_string(),
            window_size: [1920, 1080],
        },
        stealth: StealthConfig {
            enabled: true,
            randomize_fingerprint: true,
            block_webrtc: true,
            timezone: "Asia/Shanghai".to_string(),
        },
        scheduler: SchedulerConfig {
            check_interval: 10,
            retry_max: 2,
            retry_backoff: vec![10, 30, 60],
        },
        rate_limits: HashMap::new(),
    }
}

/// Generate a random test string
pub fn random_string(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
