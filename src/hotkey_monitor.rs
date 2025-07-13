use anyhow::Result;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};
use tracing::{debug, info, warn};

use crate::{AppState, ui};

static IS_MONITORING: AtomicBool = AtomicBool::new(false);

pub struct HotkeyMonitor {
    is_running: Arc<AtomicBool>,
    manager: Option<GlobalHotKeyManager>,
    hotkey: Option<HotKey>,
}

impl HotkeyMonitor {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            manager: None,
            hotkey: None,
        }
    }
    
    pub fn start_monitoring(&mut self, state: Arc<AppState>) -> Result<()> {
        if IS_MONITORING.load(Ordering::SeqCst) {
            warn!("Hotkey monitoring is already running");
            return Ok(());
        }
        
        // Create global hotkey manager
        let manager = GlobalHotKeyManager::new()?;
        
        // Define hotkey: Ctrl+Alt+Space (easy to press, uncommon on macOS)
        let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::Space);
        
        // Register the hotkey
        manager.register(hotkey)?;
        
        info!("🎹 Starting global hotkey monitoring (Ctrl+Alt+Space)");
        info!("💡 Global hotkey registered successfully");
        
        // Store manager and hotkey for cleanup
        self.manager = Some(manager);
        self.hotkey = Some(hotkey);
        
        IS_MONITORING.store(true, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);
        
        let is_running = Arc::clone(&self.is_running);
        
        // Start event listener thread
        thread::spawn(move || {
            let mut last_activation = std::time::Instant::now();
            let debounce_time = Duration::from_millis(1000);
            
            info!("🔄 Global hotkey event listener started");
            
            while is_running.load(Ordering::SeqCst) && IS_MONITORING.load(Ordering::SeqCst) {
                if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                    debug!("Global hotkey event received: {:?}", event);
                    
                    if event.state == HotKeyState::Pressed {
                        let now = std::time::Instant::now();
                        let time_since_last = now.duration_since(last_activation);
                        
                        if time_since_last >= debounce_time {
                            last_activation = now;
                            info!("🔥 Global hotkey triggered: Ctrl+Alt+Space");
                            
                            // Trigger screenshot analysis
                            let state_clone = Arc::clone(&state);
                            tokio::spawn(async move {
                                if let Err(e) = handle_hotkey_trigger(state_clone).await {
                                    tracing::error!("Hotkey trigger failed: {}", e);
                                }
                            });
                        }
                    }
                }
                
                thread::sleep(Duration::from_millis(10)); // Small sleep to prevent busy waiting
            }
            
            info!("🛑 Global hotkey event listener stopped");
        });
        
        Ok(())
    }
    
    pub fn stop_monitoring(&mut self) {
        info!("🛑 Stopping global hotkey monitoring");
        IS_MONITORING.store(false, Ordering::SeqCst);
        self.is_running.store(false, Ordering::SeqCst);
        
        // Unregister hotkey if it exists
        if let (Some(manager), Some(hotkey)) = (&self.manager, &self.hotkey) {
            if let Err(e) = manager.unregister(*hotkey) {
                warn!("Failed to unregister hotkey: {}", e);
            } else {
                info!("🎹 Global hotkey unregistered successfully");
            }
        }
        
        self.manager = None;
        self.hotkey = None;
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