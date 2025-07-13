use anyhow::Result;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::{AppState, ui, permissions, hotkey_monitor::HotkeyMonitor};

pub async fn run_daemon(state: Arc<AppState>) -> Result<()> {
    ui::print_header();
    
    // Final permission verification
    if !permissions::verify_permissions() {
        error!("❌ Required permissions not available");
        println!("⚠️  Some permissions may still be missing.");
        println!("💡 If hotkeys don't work, please check System Preferences → Security & Privacy → Privacy");
        println!("   and ensure Terminal/your app has both Accessibility and Screen Recording permissions.");
        println!();
    }
    
    info!("🚀 AI Screenshot Analyzer is running");
    println!("Press Ctrl+Alt+Space to capture and analyze screenshot");
    println!("Press Ctrl+C to exit");
    
    // Initialize and start hotkey monitoring using global-hotkey
    let mut monitor = HotkeyMonitor::new();
    monitor.start_monitoring(Arc::clone(&state))?;
    
    info!("✅ Hotkey monitoring started successfully");
    
    // Keep the main thread alive and responsive to Ctrl+C
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        
        // Check if monitoring is still active
        if !monitor.is_monitoring() {
            info!("⚠️ Hotkey monitoring stopped");
            break;
        }
    }
    
    Ok(())
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

