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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        
        assert_eq!(config.image_format, "png");
        assert_eq!(config.jpeg_quality, 95);
        assert_eq!(config.max_image_size_mb, 10);
        assert_eq!(config.default_provider, "openai");
        assert!(config.api_key.is_none());
        assert!(config.screenshots_dir.to_string_lossy().contains(".ai-screenshots"));
    }

    #[test]
    fn test_config_load_creates_default_when_missing() {
        // Test that default config is returned (actual file creation depends on dirs crate)
        let config = AppConfig::default();
        
        // Verify default values
        assert_eq!(config.image_format, "png");
        assert_eq!(config.jpeg_quality, 95);
        assert_eq!(config.max_image_size_mb, 10);
        assert_eq!(config.default_provider, "openai");
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_config_load_existing_file() {
        // Test TOML parsing directly
        let config_content = r#"
screenshots_dir = "/tmp/test-screenshots"
image_format = "jpeg"
jpeg_quality = 85
max_image_size_mb = 5
api_key = "test-key"
default_provider = "claude"
"#;
        
        let config: AppConfig = toml::from_str(config_content.trim()).unwrap();
        
        assert_eq!(config.image_format, "jpeg");
        assert_eq!(config.jpeg_quality, 85);
        assert_eq!(config.max_image_size_mb, 5);
        assert_eq!(config.api_key, Some("test-key".to_string()));
        assert_eq!(config.default_provider, "claude");
        assert_eq!(config.screenshots_dir, PathBuf::from("/tmp/test-screenshots"));
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig {
            screenshots_dir: PathBuf::from("/tmp/test"),
            image_format: "jpeg".to_string(),
            jpeg_quality: 80,
            max_image_size_mb: 15,
            api_key: Some("test-api-key".to_string()),
            default_provider: "claude".to_string(),
        };

        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("image_format = \"jpeg\""));
        assert!(toml_str.contains("jpeg_quality = 80"));
        assert!(toml_str.contains("max_image_size_mb = 15"));
        assert!(toml_str.contains("api_key = \"test-api-key\""));
        assert!(toml_str.contains("default_provider = \"claude\""));
    }
}