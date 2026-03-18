// Configuration management unit tests

use anyhow::Result;
use social_auto::config::Settings;

mod common;

#[test]
fn test_load_default_config() -> Result<()> {
    let settings = Settings::load("config.yaml")?;

    assert_eq!(settings.system.max_concurrent_tasks, 3);
    assert_eq!(settings.system.task_timeout, 300);
    assert_eq!(settings.browser.headless, true);
    assert_eq!(settings.stealth.enabled, true);

    Ok(())
}

#[test]
fn test_config_validation() -> Result<()> {
    let settings = common::create_test_config();

    assert!(settings.system.max_concurrent_tasks > 0);
    assert!(settings.system.task_timeout > 0);
    assert!(settings.scheduler.check_interval > 0);
    assert!(settings.browser.window_size[0] > 0);
    assert!(settings.browser.window_size[1] > 0);

    Ok(())
}
