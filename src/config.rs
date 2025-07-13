use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub screenshots_dir: PathBuf,
    pub image_format: String,
    pub jpeg_quality: u8,
    pub max_image_size_mb: u64,
    pub api_key: Option<String>,
    pub default_provider: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let screenshots_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".ai-screenshots");

        Self {
            screenshots_dir,
            image_format: "png".to_string(),
            jpeg_quality: 95,
            max_image_size_mb: 10,
            api_key: None,
            default_provider: "openai".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ai-screenshot-analyzer");

        let config_file = config_dir.join("config.toml");

        if config_file.exists() {
            let config_str = std::fs::read_to_string(&config_file)?;
            let config: AppConfig = toml::from_str(&config_str)?;
            Ok(config)
        } else {
            let config = AppConfig::default();

            // Create config directory
            std::fs::create_dir_all(&config_dir)?;

            // Save default config
            let config_str = toml::to_string_pretty(&config)?;
            std::fs::write(&config_file, config_str)?;

            Ok(config)
        }
    }
}
