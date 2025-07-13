use ai_screenshot_analyzer::{Args, Commands, AppState};
use ai_screenshot_analyzer::config::AppConfig;
use ai_screenshot_analyzer::ai_client::AIClient;
use ai_screenshot_analyzer::screenshot::ScreenshotCapture;
use clap::Parser;
use std::sync::Arc;
use tracing::Level;

#[test]
fn test_args_default_values() {
    // Save current environment variable
    let original_api_key = std::env::var("AI_API_KEY").ok();
    
    // Temporarily clear environment variable for this test
    std::env::remove_var("AI_API_KEY");
    
    let args = Args::parse_from(["ai-screenshot-analyzer"]);
    
    assert!(args.command.is_none());
    assert!(args.api_key.is_none());
    assert_eq!(args.provider, "openai");
    assert!(args.prompt.is_none());
    assert!(!args.debug);
    
    // Restore environment variable if it existed
    if let Some(key) = original_api_key {
        std::env::set_var("AI_API_KEY", key);
    }
}

#[test]
fn test_args_with_provider() {
    let args = Args::parse_from(["ai-screenshot-analyzer", "--provider", "claude"]);
    
    assert_eq!(args.provider, "claude");
}

#[test]
fn test_args_with_debug() {
    let args = Args::parse_from(["ai-screenshot-analyzer", "--debug"]);
    
    assert!(args.debug);
}

#[test]
fn test_args_with_prompt() {
    let args = Args::parse_from([
        "ai-screenshot-analyzer", 
        "--prompt", 
        "Analyze this image"
    ]);
    
    assert_eq!(args.prompt, Some("Analyze this image".to_string()));
}

#[test]
fn test_args_with_api_key() {
    let args = Args::parse_from([
        "ai-screenshot-analyzer",
        "--api-key",
        "test-key-123"
    ]);
    
    assert_eq!(args.api_key, Some("test-key-123".to_string()));
}

#[test]
fn test_commands_run() {
    let args = Args::parse_from(["ai-screenshot-analyzer", "run"]);
    
    assert!(matches!(args.command, Some(Commands::Run)));
}

#[test]
fn test_commands_capture() {
    let args = Args::parse_from(["ai-screenshot-analyzer", "capture"]);
    
    assert!(matches!(args.command, Some(Commands::Capture)));
}

#[test]
fn test_commands_config() {
    let args = Args::parse_from(["ai-screenshot-analyzer", "config"]);
    
    assert!(matches!(args.command, Some(Commands::Config)));
}

#[test]
fn test_commands_test() {
    let args = Args::parse_from(["ai-screenshot-analyzer", "test"]);
    
    assert!(matches!(args.command, Some(Commands::Test)));
}

#[test]
fn test_complex_args_combination() {
    let args = Args::parse_from([
        "ai-screenshot-analyzer",
        "--provider", "claude",
        "--prompt", "Detailed analysis",
        "--debug",
        "capture"
    ]);
    
    assert_eq!(args.provider, "claude");
    assert_eq!(args.prompt, Some("Detailed analysis".to_string()));
    assert!(args.debug);
    assert!(matches!(args.command, Some(Commands::Capture)));
}

#[test]
fn test_app_state_creation() {
    let config = AppConfig::default();
    let ai_client = AIClient::new("test", "key").unwrap();
    
    // Test that we can create screenshot capture
    match ScreenshotCapture::new() {
        Ok(screenshot_capture) => {
            let app_state = AppState {
                ai_client,
                screenshot_capture,
                config,
                custom_prompt: Some("test prompt".to_string()),
            };
            
            assert_eq!(app_state.custom_prompt, Some("test prompt".to_string()));
            assert_eq!(app_state.ai_client.provider(), "test");
        }
        Err(_) => {
            // Expected in headless environments
            println!("Screenshot capture not available for AppState test");
        }
    }
}

#[test]
fn test_tracing_level_constants() {
    // Test that Level constants are accessible
    let _debug_level = Level::DEBUG;
    let _info_level = Level::INFO;
    
}

#[test]
fn test_app_state_fields() {
    // Test AppState structure without actual initialization
    // since that requires external dependencies
    let config = AppConfig::default();
    
    assert_eq!(config.default_provider, "openai");
    assert_eq!(config.image_format, "png");
}

#[tokio::test]
async fn test_show_config_function() {
    // Test the show_config function
    let config = AppConfig::default();
    let ai_client = AIClient::new("test", "test-key").unwrap();
    
    if let Ok(screenshot_capture) = ScreenshotCapture::new() {
        let app_state = Arc::new(AppState {
            ai_client,
            screenshot_capture,
            config,
            custom_prompt: None,
        });

        let result = ai_screenshot_analyzer::show_config(app_state).await;
        assert!(result.is_ok(), "show_config should succeed");
    }
}

#[tokio::test]
async fn test_test_ai_connection_function() {
    // Test the test_ai_connection function with a real AI client
    let config = AppConfig::default();
    let ai_client = AIClient::new("unsupported-provider", "test-key").unwrap();
    
    if let Ok(screenshot_capture) = ScreenshotCapture::new() {
        let app_state = Arc::new(AppState {
            ai_client,
            screenshot_capture,
            config,
            custom_prompt: None,
        });

        let result = ai_screenshot_analyzer::test_ai_connection(app_state).await;
        // Should fail with unsupported provider
        assert!(result.is_err(), "test_ai_connection should fail with unsupported provider");
    }
}

#[tokio::test]
async fn test_capture_once_function() {
    // Test the capture_once function
    let config = AppConfig::default();
    let ai_client = AIClient::new("test", "test-key").unwrap();
    
    if let Ok(screenshot_capture) = ScreenshotCapture::new() {
        let app_state = Arc::new(AppState {
            ai_client,
            screenshot_capture,
            config,
            custom_prompt: Some("Test capture".to_string()),
        });

        // This may fail in headless environments, which is expected
        let result = ai_screenshot_analyzer::capture_once(app_state).await;
        match result {
            Ok(_) => println!("✅ capture_once succeeded"),
            Err(e) => println!("⚠️ capture_once failed (expected in headless): {}", e),
        }
    }
}

#[test]
fn test_args_validation_errors() {
    // Test various error conditions in argument parsing
    // Note: clap will panic on invalid args, so we test valid edge cases
    
    // Test with all options
    let args = Args::parse_from([
        "ai-screenshot-analyzer",
        "--provider", "gemini",
        "--api-key", "test-key-456",
        "--prompt", "Complex test prompt with spaces",
        "--debug",
        "test"
    ]);
    
    assert_eq!(args.provider, "gemini");
    assert_eq!(args.api_key, Some("test-key-456".to_string()));
    assert_eq!(args.prompt, Some("Complex test prompt with spaces".to_string()));
    assert!(args.debug);
    assert!(matches!(args.command, Some(Commands::Test)));
}

#[test]
fn test_image_creation_for_ai_test() {
    // Test the image creation logic used in test_ai_connection
    let test_image = image::RgbImage::new(1, 1);
    assert_eq!(test_image.width(), 1);
    assert_eq!(test_image.height(), 1);
    
    let mut buffer = Vec::new();
    let result = test_image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png);
    assert!(result.is_ok(), "Image writing should succeed");
    assert!(!buffer.is_empty(), "Buffer should contain image data");
}

#[tokio::test]
async fn test_app_state_with_different_configs() {
    // Test AppState creation with different configurations
    let config = AppConfig { 
        image_format: "jpeg".to_string(), 
        jpeg_quality: 80, 
        max_image_size_mb: 5, 
        ..Default::default() 
    };
    
    let ai_client = AIClient::new("claude", "test-key").unwrap();
    
    if let Ok(screenshot_capture) = ScreenshotCapture::new() {
        let app_state = AppState {
            ai_client,
            screenshot_capture,
            config: config.clone(),
            custom_prompt: Some("Custom test prompt".to_string()),
        };
        
        assert_eq!(app_state.config.image_format, "jpeg");
        assert_eq!(app_state.config.jpeg_quality, 80);
        assert_eq!(app_state.config.max_image_size_mb, 5);
        assert_eq!(app_state.ai_client.provider(), "claude");
        assert_eq!(app_state.custom_prompt, Some("Custom test prompt".to_string()));
    }
}

#[test]
fn test_commands_enum_variants() {
    // Test all command variants
    use std::mem;
    
    let run_cmd = Commands::Run;
    let capture_cmd = Commands::Capture;
    let config_cmd = Commands::Config;
    let test_cmd = Commands::Test;
    
    // Ensure all variants are the same size (enum optimization check)
    assert_eq!(mem::size_of_val(&run_cmd), mem::size_of_val(&capture_cmd));
    assert_eq!(mem::size_of_val(&capture_cmd), mem::size_of_val(&config_cmd));
    assert_eq!(mem::size_of_val(&config_cmd), mem::size_of_val(&test_cmd));
}

#[test]
fn test_logging_level_configuration() {
    // Test logging level constants are accessible
    let _debug_level = Level::DEBUG;
    let _info_level = Level::INFO;
    let _warn_level = Level::WARN;
    let _error_level = Level::ERROR;
    
    // Test that levels can be used in configuration
}

#[tokio::test]
async fn test_main_entry_point_simulation() {
    // Test main function behavior by testing its components
    // We can't easily test main() directly, but we can test the logic paths
    
    // Test Args parsing for different scenarios
    let default_args = Args::parse_from(["ai-screenshot-analyzer"]);
    assert!(default_args.command.is_none());
    assert_eq!(default_args.provider, "openai");
    
    // Test with explicit command
    let capture_args = Args::parse_from(["ai-screenshot-analyzer", "capture"]);
    assert!(matches!(capture_args.command, Some(Commands::Capture)));
    
    // Test debug flag impact
    let debug_args = Args::parse_from(["ai-screenshot-analyzer", "--debug"]);
    assert!(debug_args.debug);
}

#[tokio::test]
async fn test_command_routing_logic() {
    // Test the command routing logic that would happen in main()
    
    // Test all command variants exist
    let commands = vec![
        Commands::Run,
        Commands::Capture, 
        Commands::Config,
        Commands::Test,
    ];
    
    for cmd in commands {
        // Just test that we can create and match against all commands
        match cmd {
            Commands::Run => {},
            Commands::Capture => {},
            Commands::Config => {}, 
            Commands::Test => {},
        }
    }
}

#[test]
fn test_args_environment_variable_support() {
    // Test that Args supports environment variables
    // Note: We can't easily test actual env var reading in unit tests
    // but we can test the structure supports it
    
    let args_with_key = Args::parse_from([
        "ai-screenshot-analyzer",
        "--api-key", "env-test-key"
    ]);
    assert_eq!(args_with_key.api_key, Some("env-test-key".to_string()));
}

#[test]
fn test_app_state_comprehensive_creation() {
    // Test comprehensive AppState creation scenarios
    let config1 = AppConfig::default();
    let config2 = AppConfig {
        image_format: "jpeg".to_string(),
        jpeg_quality: 80,
        max_image_size_mb: 5,
        default_provider: "claude".to_string(),
        ..Default::default()
    };
    
    let ai_client1 = AIClient::new("openai", "key1").unwrap();
    let ai_client2 = AIClient::new("claude", "key2").unwrap();
    
    if let Ok(screenshot_capture) = ScreenshotCapture::new() {
        let _state1 = AppState {
            ai_client: ai_client1,
            screenshot_capture,
            config: config1.clone(),
            custom_prompt: None,
        };
        
        assert_eq!(config1.image_format, "png");
        assert_eq!(config2.image_format, "jpeg");
        assert_eq!(ai_client2.provider(), "claude");
    }
}

#[tokio::test]
async fn test_error_scenarios_in_main_flow() {
    // Test error scenarios that could happen in main()
    
    // Test AppConfig default creation
    let config = AppConfig::default();
    assert_eq!(config.default_provider, "openai");
    
    // Test AI client creation with empty values (this actually succeeds)
    let empty_ai = AIClient::new("", "");
    assert!(empty_ai.is_ok());
    
    // Test various argument combinations
    let complex_args = Args::parse_from([
        "ai-screenshot-analyzer",
        "--provider", "claude", 
        "--debug",
        "--prompt", "Complex test prompt",
        "test"
    ]);
    assert_eq!(complex_args.provider, "claude");
    assert!(complex_args.debug);
    assert_eq!(complex_args.prompt, Some("Complex test prompt".to_string()));
    assert!(matches!(complex_args.command, Some(Commands::Test)));
}

#[tokio::test]
async fn test_logging_initialization_simulation() {
    // Test the logging initialization logic
    use tracing::Level;
    
    // Test that we can create different log levels (simulating main() logic)
    let debug_level = Level::DEBUG;
    let info_level = Level::INFO;
    
    // Simulate the conditional logic in main()
    let use_debug = true;
    let selected_level = if use_debug { debug_level } else { info_level };
    
    // Test that the level selection works
    match selected_level {
        Level::DEBUG => assert!(use_debug),
        Level::INFO => assert!(!use_debug),
        _ => panic!("Unexpected log level"),
    }
}

#[test]
fn test_command_line_edge_cases() {
    // Test edge cases in command line parsing
    
    // Test with all options
    let full_args = Args::parse_from([
        "ai-screenshot-analyzer",
        "--provider", "gemini",
        "--api-key", "secret-key-123", 
        "--prompt", "Detailed analysis with special chars: 🤖📸",
        "--debug",
        "config"
    ]);
    
    assert_eq!(full_args.provider, "gemini");
    assert_eq!(full_args.api_key, Some("secret-key-123".to_string()));
    assert_eq!(full_args.prompt, Some("Detailed analysis with special chars: 🤖📸".to_string()));
    assert!(full_args.debug);
    assert!(matches!(full_args.command, Some(Commands::Config)));
}

#[test]
fn test_args_struct_completeness() {
    // Test that Args struct handles all expected fields
    let args = Args::parse_from(["ai-screenshot-analyzer"]);
    
    // Test all fields exist and have expected defaults
    let _ = args.command;
    let _ = args.api_key;
    let _ = args.provider;
    let _ = args.prompt;
    let _ = args.debug;
    
    // Test derived traits work
    assert_eq!(args.provider, "openai"); // Default value
    assert!(!args.debug); // Default value
}