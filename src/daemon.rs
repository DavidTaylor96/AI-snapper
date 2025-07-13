use anyhow::Result;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::{AppState, ui};

pub async fn run_daemon(state: Arc<AppState>) -> Result<()> {
    ui::print_header();
    
    // Initialize global hotkey manager
    let manager = GlobalHotKeyManager::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize hotkey manager: {}", e))?;
    
    // Create hotkey (Ctrl+Shift+A)
    let hotkey = HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::KeyA,
    );
    
    // Register hotkey
    manager.register(hotkey)
        .map_err(|e| anyhow::anyhow!("Failed to register hotkey: {}", e))?;
    
    info!("🚀 AI Screenshot Analyzer is running");
    println!("Press Ctrl+Shift+A to capture and analyze screenshot");
    println!("Press Ctrl+C to exit");
    
    let state = Arc::clone(&state);
    
    // Main event loop
    loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.id == hotkey.id() {
                let state_clone = Arc::clone(&state);
                tokio::spawn(async move {
                    if let Err(e) = handle_screenshot_request(state_clone).await {
                        error!("Screenshot analysis failed: {}", e);
                        ui::print_error(&format!("❌ Analysis failed: {}", e));
                    }
                });
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

pub async fn handle_screenshot_request(state: Arc<AppState>) -> Result<()> {
    ui::print_status("📸 Capturing screenshot...");
    
    // Capture screenshot
    let screenshot_data = state.screenshot_capture.capture().await?;
    
    ui::print_status("🤖 Analyzing with AI...");
    
    // Create progress indicator
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Processing with AI...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    
    // Analyze with AI
    let prompt = state.custom_prompt.as_deref()
        .unwrap_or("Analyze this screenshot in detail. Describe what you see, including any text, UI elements, data, or important information. Be comprehensive and specific.");
    
    let analysis = state.ai_client.analyze_image(&screenshot_data, prompt).await?;
    
    pb.finish_and_clear();
    
    // Display results
    ui::print_analysis_result(&analysis);
    
    Ok(())
}

