use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};
use tracing::{debug, info, warn};

use crate::{AppState, ui};

static IS_MONITORING: AtomicBool = AtomicBool::new(false);

pub struct HotkeyMonitor {
    is_running: Arc<AtomicBool>,
}

impl HotkeyMonitor {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn start_monitoring(&self, state: Arc<AppState>) -> Result<()> {
        if IS_MONITORING.load(Ordering::SeqCst) {
            warn!("Hotkey monitoring is already running");
            return Ok(());
        }
        
        IS_MONITORING.store(true, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);
        
        let is_running = Arc::clone(&self.is_running);
        
        info!("🎹 Starting hotkey monitoring (Cmd+Shift+S)");
        info!("💡 Monitoring thread will check for key combinations every 100ms");
        
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut last_activation = std::time::Instant::now();
            let debounce_time = Duration::from_millis(1000); // Prevent double-triggers
            let mut iteration_count = 0;
            
            info!("🔄 Hotkey monitoring thread started");
            
            while is_running.load(Ordering::SeqCst) && IS_MONITORING.load(Ordering::SeqCst) {
                iteration_count += 1;
                
                // Periodic heartbeat for debugging
                if iteration_count % 100 == 0 {
                    debug!("Hotkey monitor heartbeat - iteration {}", iteration_count);
                }
                
                let keys: Vec<Keycode> = device_state.get_keys();
                
                if !keys.is_empty() {
                    debug!("Keys detected: {:?}", keys);
                    
                    // Check for Cmd+Shift+S combination (Meta key on macOS, LWin/RWin on Windows/Linux)
                    let cmd_pressed = keys.contains(&Keycode::LMeta) || 
                                     keys.contains(&Keycode::RMeta);
                    let shift_pressed = keys.contains(&Keycode::LShift) || 
                                       keys.contains(&Keycode::RShift);
                    let s_pressed = keys.contains(&Keycode::S);
                    
                    let now = std::time::Instant::now();
                    let time_since_last = now.duration_since(last_activation);
                    
                    if cmd_pressed && shift_pressed && s_pressed && time_since_last >= debounce_time {
                        last_activation = now;
                        info!("🔥 Hotkey combination detected: Cmd+Shift+S");
                        
                        // Trigger screenshot analysis
                        let state_clone = Arc::clone(&state);
                        tokio::spawn(async move {
                            if let Err(e) = handle_hotkey_trigger(state_clone).await {
                                tracing::error!("Hotkey trigger failed: {}", e);
                            }
                        });
                    }
                }
                
                thread::sleep(Duration::from_millis(100));
            }
            
            info!("🛑 Hotkey monitoring thread stopped");
        });
        
        Ok(())
    }
    
    pub fn stop_monitoring(&self) {
        info!("🛑 Stopping hotkey monitoring");
        IS_MONITORING.store(false, Ordering::SeqCst);
        self.is_running.store(false, Ordering::SeqCst);
    }
    
    pub fn is_monitoring(&self) -> bool {
        IS_MONITORING.load(Ordering::SeqCst)
    }
}

impl Drop for HotkeyMonitor {
    fn drop(&mut self) {
        self.stop_monitoring();
    }
}

async fn handle_hotkey_trigger(state: Arc<AppState>) -> Result<()> {
    info!("🚀 Processing hotkey trigger - starting screenshot capture");
    
    // Use ui module functions directly
    
    ui::print_status("📸 Capturing screenshot...");
    
    // Capture screenshot
    let screenshot_data = state.screenshot_capture.capture().await?;
    
    ui::print_status("🤖 Analyzing with AI...");
    
    // Create progress indicator
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Processing with AI...");
    pb.enable_steady_tick(Duration::from_millis(100));
    
    // Analyze with AI
    let prompt = state.custom_prompt.as_deref()
        .unwrap_or("Analyze this screenshot in detail. Describe what you see, including any text, UI elements, data, or important information. Be comprehensive and specific.");
    
    let analysis = state.ai_client.analyze_image(&screenshot_data, prompt).await?;
    
    pb.finish_and_clear();
    
    // Display results
    ui::print_analysis_result(&analysis);
    
    info!("✅ Screenshot analysis completed successfully");
    
    Ok(())
}