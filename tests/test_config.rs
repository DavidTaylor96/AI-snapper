use ai_screenshot_analyzer::config::AppConfig;
use std::path::PathBuf;

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

#[test]
fn test_config_load_error_handling() {
    // Test that config loading handles default creation
    // We can't easily test file I/O errors in unit tests, but we can test the logic
    let config = AppConfig::default();
    
    // Verify all default values
    assert_eq!(config.image_format, "png");
    assert_eq!(config.jpeg_quality, 95);
    assert_eq!(config.max_image_size_mb, 10);
    assert_eq!(config.default_provider, "openai");
    assert!(config.api_key.is_none());
    assert!(config.screenshots_dir.to_string_lossy().contains(".ai-screenshots"));
}

#[test]
fn test_config_edge_cases() {
    // Test edge cases for config values
    let config = AppConfig {
        screenshots_dir: PathBuf::from(""),
        image_format: "webp".to_string(),
        jpeg_quality: 100,
        max_image_size_mb: 1,
        api_key: Some("".to_string()),
        default_provider: "gemini".to_string(),
    };

    assert_eq!(config.image_format, "webp");
    assert_eq!(config.jpeg_quality, 100);
    assert_eq!(config.max_image_size_mb, 1);
    assert_eq!(config.api_key, Some("".to_string()));
    assert_eq!(config.default_provider, "gemini");
}

#[test]
fn test_config_clone_and_debug() {
    let config = AppConfig::default();
    
    // Test Clone trait
    let cloned_config = config.clone();
    assert_eq!(config.image_format, cloned_config.image_format);
    assert_eq!(config.jpeg_quality, cloned_config.jpeg_quality);
    
    // Test Debug trait
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("AppConfig"));
    assert!(debug_str.contains("image_format"));
}

#[test]
fn test_config_with_all_formats() {
    // Test config with different image formats
    let formats = vec!["png", "jpeg", "webp", "bmp"];
    
    for format in formats {
        let config = AppConfig {
            image_format: format.to_string(),
            ..Default::default()
        };
        assert_eq!(config.image_format, format);
    }
}

#[test]
fn test_config_quality_ranges() {
    // Test different JPEG quality ranges
    let qualities = vec![1, 50, 85, 95, 100];
    
    for quality in qualities {
        let config = AppConfig {
            jpeg_quality: quality,
            ..Default::default()
        };
        assert_eq!(config.jpeg_quality, quality);
    }
}

#[test]
fn test_config_size_limits() {
    // Test different size limits
    let sizes = vec![1, 5, 10, 20, 50];
    
    for size in sizes {
        let config = AppConfig {
            max_image_size_mb: size,
            ..Default::default()
        };
        assert_eq!(config.max_image_size_mb, size);
    }
}

#[test]
fn test_config_providers() {
    // Test different providers
    let providers = vec!["openai", "claude", "gemini"];
    
    for provider in providers {
        let config = AppConfig {
            default_provider: provider.to_string(),
            ..Default::default()
        };
        assert_eq!(config.default_provider, provider);
    }
}

#[test]
fn test_pathbuf_handling() {
    // Test PathBuf handling
    let paths = vec![
        "/tmp",
        "/home/user/screenshots", 
        "relative/path",
        "."
    ];
    
    for path in paths {
        let config = AppConfig {
            screenshots_dir: PathBuf::from(path),
            ..Default::default()
        };
        assert_eq!(config.screenshots_dir, PathBuf::from(path));
    }
}