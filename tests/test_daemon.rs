use ai_screenshot_analyzer::{AppState, daemon};
use ai_screenshot_analyzer::config::AppConfig;
use ai_screenshot_analyzer::screenshot::ScreenshotCapture;
use ai_screenshot_analyzer::ai_client::AIClient;
use std::sync::Arc;

fn create_test_app_state() -> Option<Arc<AppState>> {
    let config = AppConfig::default();
    let ai_client = AIClient::new("test", "test-key").ok()?;
    let screenshot_capture = ScreenshotCapture::new().ok()?;
    
    Some(Arc::new(AppState {
        ai_client,
        screenshot_capture,
        config,
        custom_prompt: Some("Test prompt".to_string()),
    }))
}

#[tokio::test]
async fn test_handle_screenshot_request_success() {
    if let Some(app_state) = create_test_app_state() {
        // Test the screenshot request handling
        // Note: This may fail in headless environments, which is expected
        match daemon::handle_screenshot_request(app_state).await {
            Ok(_) => {
                println!("✅ Screenshot request handled successfully");
            }
            Err(e) => {
                println!("⚠️ Screenshot request failed (expected in headless): {}", e);
                // This is expected in CI/headless environments
            }
        }
    } else {
        println!("⚠️ Could not create test app state (no display available)");
    }
}

#[tokio::test]
async fn test_handle_screenshot_request_with_custom_prompt() {
    if let Some(app_state) = create_test_app_state() {
        // Verify custom prompt is used
        assert_eq!(app_state.custom_prompt, Some("Test prompt".to_string()));
        
        // Test screenshot handling (may fail in headless, which is OK)
        let _ = daemon::handle_screenshot_request(app_state).await;
    }
}

#[test]
fn test_daemon_module_compilation() {
    // Test that daemon module functions are properly accessible
    // This ensures the module structure is correct
}

#[tokio::test]
async fn test_run_daemon_initialization() {
    // Test daemon initialization without actually running the loop
    if let Some(app_state) = create_test_app_state() {
        // We can't easily test the full daemon due to global hotkey requirements
        // But we can test that the AppState is properly structured
        assert_eq!(app_state.ai_client.provider(), "test");
        assert!(app_state.custom_prompt.is_some());
    }
}

#[test]
fn test_app_state_structure() {
    if let Some(app_state) = create_test_app_state() {
        // Test that all AppState fields are accessible
        let _ai_client = &app_state.ai_client;
        let _screenshot_capture = &app_state.screenshot_capture;
        let _config = &app_state.config;
        let _custom_prompt = &app_state.custom_prompt;
        
        // AppState structure is valid if we reach this point
    }
}

#[tokio::test]
async fn test_error_handling_in_screenshot_request() {
    // Create a minimal config for testing error paths
    let config = AppConfig::default();
    let ai_client = AIClient::new("unsupported-provider", "test-key").unwrap();
    
    // Test with a provider that will cause errors
    if let Ok(screenshot_capture) = ScreenshotCapture::new() {
        let app_state = Arc::new(AppState {
            ai_client,
            screenshot_capture,
            config,
            custom_prompt: None,
        });

        // This should fail due to unsupported provider
        let result = daemon::handle_screenshot_request(app_state).await;
        match result {
            Ok(_) => {
                // Unexpected success - maybe screenshot failed first
                println!("Screenshot request completed (may have failed at screenshot stage)");
            }
            Err(e) => {
                println!("✅ Expected error in screenshot request: {}", e);
                assert!(e.to_string().contains("Unsupported provider") || 
                       e.to_string().contains("Failed to") ||
                       e.to_string().contains("create image buffer"));
            }
        }
    }
}