use crate::config::BrowserConfig;
use crate::browser::stealth::StealthPatch;
use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig as ChromeConfig};
use chromiumoxide::Page;
use std::path::PathBuf;

pub struct BrowserEngine {
    browser: Browser,
    config: BrowserConfig,
}

impl BrowserEngine {
    pub async fn new(config: BrowserConfig) -> Result<Self> {
        let mut builder = ChromeConfig::builder();

        // Set headless mode
        if !config.headless {
            builder = builder.with_head();
        }

        // Set window size
        builder = builder.window_size(config.window_size[0], config.window_size[1]);

        // Set user data directory
        let user_data_dir = PathBuf::from(&config.user_data_dir);
        builder = builder.user_data_dir(user_data_dir);

        // Set Chrome path if specified
        if let Some(chrome_path) = &config.chrome_path {
            builder = builder.chrome_executable(PathBuf::from(chrome_path));
        }

        let chrome_config = builder.build()?;
        let (browser, mut handler) = Browser::launch(chrome_config).await?;

        // Spawn handler task
        tokio::spawn(async move {
            loop {
                if handler.next().await.is_none() {
                    break;
                }
            }
        });

        Ok(Self { browser, config })
    }

    pub async fn new_page(&self) -> Result<Page> {
        let page = self.browser.new_page("about:blank").await?;
        Ok(page)
    }

    pub async fn new_page_with_stealth(&self) -> Result<Page> {
        let page = self.new_page().await?;

        // Generate and apply stealth patches
        let fingerprint = StealthPatch::generate_random_fingerprint();
        let stealth_script = StealthPatch::generate_stealth_script(&fingerprint);

        page.evaluate(&stealth_script).await?;

        Ok(page)
    }

    pub async fn close(&self) -> Result<()> {
        self.browser.close().await?;
        Ok(())
    }
}

