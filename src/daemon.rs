use anyhow::Result;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::{AppState, ui, permissions};

pub async fn run_daemon(state: Arc<AppState>) -> Result<()> {
    ui::print_header();
    
    // Final permission verification
    if !permissions::verify_permissions() {
        error!("❌ Required permissions not available");
        println!("⚠️  Some permissions may still be missing.");
        println!("💡 If hotkeys don't work, please check System Preferences → Security & Privacy → Privacy");
        println!("   and ensure Terminal/your app has both Accessibility and Screen Recording permissions.");
        println!("");
    }
    
    info!("Initializing global hotkey manager...");
    debug!("AppState initialized with AI provider: {}", state.ai_client.provider());
    
    // Initialize global hotkey manager
    let manager = GlobalHotKeyManager::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize hotkey manager: {}", e))?;
    
    info!("Creating hotkey Cmd+Shift+2...");
    
    // Create hotkey (Cmd+Shift+2)
    let hotkey = HotKey::new(
        Some(Modifiers::META | Modifiers::SHIFT),
        Code::Digit2,
    );
    
    info!("Hotkey created with ID: {:?}", hotkey.id());
    
    // Register hotkey
    info!("Registering hotkey...");
    manager.register(hotkey)
        .map_err(|e| anyhow::anyhow!("Failed to register hotkey: {}", e))?;
    
    info!("✅ Hotkey registered successfully");
    info!("🚀 AI Screenshot Analyzer is running");
    println!("Press Cmd+Shift+2 to capture and analyze screenshot");
    println!("Press Ctrl+C to exit");
    
    let state = Arc::clone(&state);
    let mut event_count = 0;
    
    // Main event loop - use blocking recv with timeout for better event handling
    debug!("Starting main event loop with timeout of 100ms");
    loop {
        match GlobalHotKeyEvent::receiver().recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(event) => {
                event_count += 1;
                info!("🔥 Hotkey event received! Event #{}, ID: {:?}, Expected ID: {:?}", 
                      event_count, event.id, hotkey.id());
                
                if event.id == hotkey.id() {
                    info!("✅ Hotkey ID matches! Starting screenshot capture...");
                    let state_clone = Arc::clone(&state);
                    tokio::spawn(async move {
                        if let Err(e) = handle_screenshot_request(state_clone).await {
                            error!("Screenshot analysis failed: {}", e);
                            ui::print_error(&format!("❌ Analysis failed: {}", e));
                        }
                    });
                } else {
                    warn!("❌ Hotkey ID mismatch! Received: {:?}, Expected: {:?}", event.id, hotkey.id());
                }
            }
            Err(e) => {
                debug!("Hotkey recv timeout or error: {:?}", e);
                debug!("Event loop iteration completed, continuing...");
                // Continue loop on timeout, exit on disconnect
                if format!("{:?}", e).contains("Disconnected") {
                    error!("Hotkey event receiver disconnected!");
                    return Err(anyhow::anyhow!("Hotkey event receiver disconnected"));
                }
            }
        }
    }
}

pub async fn handle_screenshot_request(state: Arc<AppState>) -> Result<()> {
    info!("🚀 Starting screenshot capture and analysis...");
    ui::print_status("📸 Capturing screenshot...");
    
    // Capture screenshot
    debug!("About to call screenshot capture with provider: {}", state.ai_client.provider());
    info!("Calling screenshot capture...");
    let screenshot_data = match state.screenshot_capture.capture().await {
        Ok(data) => {
            info!("✅ Screenshot captured successfully, size: {} bytes", data.len());
            data
        }
        Err(e) => {
            error!("❌ Screenshot capture failed: {}", e);
            return Err(e);
        }
    };
    
    ui::print_status("🤖 Analyzing with AI...");
    
    // Create progress indicator
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Processing with AI...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    
    // Analyze with AI
    let prompt = state.custom_prompt.as_deref()
        .unwrap_or("Analyze this screenshot in detail. Describe what you see, including any text, UI elements, data, or important information. Be comprehensive and specific.");
    debug!("Using prompt: {}", prompt);
    debug!("Screenshot data size: {} bytes", screenshot_data.len());
    
    debug!("About to send image to {} for analysis", state.ai_client.provider());
    info!("Sending image to AI for analysis...");
    let analysis = match state.ai_client.analyze_image(&screenshot_data, prompt).await {
        Ok(result) => {
            info!("✅ AI analysis completed successfully");
            result
        }
        Err(e) => {
            error!("❌ AI analysis failed: {}", e);
            pb.finish_and_clear();
            return Err(e);
        }
    };
    
    pb.finish_and_clear();
    
    // Display results
    info!("Displaying analysis results...");
    ui::print_analysis_result(&analysis);
    
    info!("✅ Screenshot analysis completed successfully");
    Ok(())
}

