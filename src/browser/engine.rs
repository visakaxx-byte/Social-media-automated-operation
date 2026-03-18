use crate::config::BrowserConfig;
use crate::browser::stealth::StealthPatch;
use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig as ChromeConfig};
use chromiumoxide::Page;
use std::path::PathBuf;

pub struct BrowserEngine {
    browser: Browser,
    _config: BrowserConfig,
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

        let chrome_config = builder.build()
            .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?;
        let (browser, _handler) = Browser::launch(chrome_config).await?;

        // Handler will be dropped, browser will continue to work
        // In production, you may want to keep the handler alive

        Ok(Self { browser, _config: config })
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

        page.evaluate(stealth_script.as_str()).await?;

        Ok(page)
    }

    pub async fn close(mut self) -> Result<()> {
        self.browser.close().await?;
        Ok(())
    }
}

