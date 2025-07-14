use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};
use tracing::{debug, info, warn, error};
use tokio::sync::mpsc;

use crate::{ui, AppState};

static IS_MONITORING: AtomicBool = AtomicBool::new(false);
static LAST_TRIGGER_TIME: AtomicU64 = AtomicU64::new(0);

pub struct HotkeyMonitor {
    is_running: Arc<AtomicBool>,
    trigger_sender: Option<mpsc::UnboundedSender<()>>,
}

impl Default for HotkeyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl HotkeyMonitor {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            trigger_sender: None,
        }
    }

    pub fn start_monitoring(&mut self, state: Arc<AppState>) -> Result<()> {
        if IS_MONITORING.load(Ordering::SeqCst) {
            warn!("Hotkey monitoring is already running");
            return Ok(());
        }

        // Create a channel for communication between the thread and async runtime
        let (trigger_sender, mut trigger_receiver) = mpsc::unbounded_channel::<()>();
        self.trigger_sender = Some(trigger_sender.clone());

        // Test device_query availability first
        let device_state = DeviceState::new();
        let initial_keys = device_state.get_keys();
        info!("🔧 Device query initialized, current keys: {:?}", initial_keys);

        info!("🎹 Starting enhanced hotkey monitoring (Cmd+Shift+Space)");
        info!("🔍 Detected platform: {}", std::env::consts::OS);

        IS_MONITORING.store(true, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);

        let is_running = Arc::clone(&self.is_running);

        // Start the async handler task
        let state_for_handler = Arc::clone(&state);
        tokio::spawn(async move {
            while let Some(_) = trigger_receiver.recv().await {
                if let Err(e) = handle_hotkey_trigger(Arc::clone(&state_for_handler)).await {
                    error!("Hotkey trigger failed: {}", e);
                }
            }
        });

        // Enhanced monitoring thread with better error handling
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let debounce_time = Duration::from_millis(500); // Reduced debounce time
            let poll_interval = Duration::from_millis(50); // Faster polling
            
            // Track key states for better edge detection
            let mut last_keys: Vec<Keycode> = Vec::new();
            let mut combo_start_time: Option<std::time::Instant> = None;
            let mut status_log_interval = std::time::Instant::now();

            info!("🔄 Enhanced hotkey listener started");
            debug!("📋 Monitoring hotkey: Cmd+Shift+Space with edge detection");

            while is_running.load(Ordering::SeqCst) && IS_MONITORING.load(Ordering::SeqCst) {
                let now = std::time::Instant::now();
                
                // Periodic status logging
                if now.duration_since(status_log_interval) >= Duration::from_secs(30) {
                    debug!("🔍 Hotkey monitoring active - enhanced polling...");
                    status_log_interval = now;
                }

                // Get current key state
                let current_keys: Vec<Keycode> = device_state.get_keys();

                // Detect key state changes
                let keys_changed = current_keys != last_keys;
                
                if keys_changed && !current_keys.is_empty() {
                    debug!("🎹 Key state changed: {:?}", current_keys);
                }

                // Check for our specific combination
                let space_pressed = current_keys.contains(&Keycode::Space);
                let meta_pressed = current_keys.contains(&Keycode::LMeta) 
                    || current_keys.contains(&Keycode::RMeta)
                    || current_keys.contains(&Keycode::Command); // Add Command key variant
                let shift_pressed = current_keys.contains(&Keycode::LShift) 
                    || current_keys.contains(&Keycode::RShift);

                // Enhanced detection logic
                let combo_active = space_pressed && meta_pressed && shift_pressed;
                let combo_was_active = last_keys.contains(&Keycode::Space) 
                    && (last_keys.contains(&Keycode::LMeta) 
                        || last_keys.contains(&Keycode::RMeta)
                        || last_keys.contains(&Keycode::Command)) // Add Command key variant
                    && (last_keys.contains(&Keycode::LShift) 
                        || last_keys.contains(&Keycode::RShift));

                // Detect combo activation (edge detection)
                if combo_active && !combo_was_active {
                    debug!("⬇️ Hotkey combo started (edge detected)");
                    combo_start_time = Some(now);
                } else if combo_active && combo_start_time.is_some() {
                    // Combo is being held - check if held long enough
                    let hold_duration = now.duration_since(combo_start_time.unwrap());
                    if hold_duration >= Duration::from_millis(100) {
                        debug!("⏱️ Hotkey combo held for {:?}, checking debounce...", hold_duration);
                        
                        // Check debounce
                        let last_trigger = LAST_TRIGGER_TIME.load(Ordering::SeqCst);
                        let last_trigger_instant = std::time::UNIX_EPOCH + Duration::from_millis(last_trigger);
                        let system_time = std::time::SystemTime::now();
                        
                        let should_trigger = if last_trigger == 0 {
                            true
                        } else {
                            system_time.duration_since(last_trigger_instant)
                                .map(|d| d >= debounce_time)
                                .unwrap_or(true)
                        };

                        if should_trigger {
                            let current_time = system_time.duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default().as_millis() as u64;
                            LAST_TRIGGER_TIME.store(current_time, Ordering::SeqCst);
                            
                            info!("🔥 Global hotkey triggered: Cmd+Shift+Space (enhanced detection)");
                            
                            // Reset combo tracking
                            combo_start_time = None;

                            // Send trigger signal through channel
                            if let Err(e) = trigger_sender.send(()) {
                                error!("Failed to send hotkey trigger: {}", e);
                            }
                        } else {
                            debug!("⚡ Hotkey trigger ignored due to debounce");
                        }
                    }
                } else if !combo_active && combo_was_active {
                    debug!("⬆️ Hotkey combo released");
                    combo_start_time = None;
                }

                // Alternative detection method for debugging
                if keys_changed && current_keys.len() >= 3 {
                    let key_names: Vec<String> = current_keys.iter()
                        .map(|k| format!("{:?}", k))
                        .collect();
                    debug!("🔍 Multiple keys pressed: {}", key_names.join("+"));
                    
                    // Check for common macOS variations
                    let has_cmd = current_keys.iter().any(|k| matches!(k, 
                        Keycode::LMeta | Keycode::RMeta | Keycode::Command));
                    let has_shift = current_keys.iter().any(|k| matches!(k, 
                        Keycode::LShift | Keycode::RShift));
                    let has_space = current_keys.contains(&Keycode::Space);
                    
                    if has_cmd && has_shift && has_space {
                        debug!("🎯 Detected Cmd+Shift+Space pattern with alternative detection");
                        // Since we detected it here, let's also trigger it
                        info!("🔥 Global hotkey triggered via alternative detection: Cmd+Shift+Space");
                        
                        // Check debounce for this alternative detection too
                        let last_trigger = LAST_TRIGGER_TIME.load(Ordering::SeqCst);
                        let system_time = std::time::SystemTime::now();
                        
                        let should_trigger = if last_trigger == 0 {
                            true
                        } else {
                            let last_trigger_instant = std::time::UNIX_EPOCH + Duration::from_millis(last_trigger);
                            system_time.duration_since(last_trigger_instant)
                                .map(|d| d >= debounce_time)
                                .unwrap_or(true)
                        };

                        if should_trigger {
                            let current_time = system_time.duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default().as_millis() as u64;
                            LAST_TRIGGER_TIME.store(current_time, Ordering::SeqCst);

                            // Send trigger signal through channel
                            if let Err(e) = trigger_sender.send(()) {
                                error!("Failed to send hotkey trigger: {}", e);
                            }
                        }
                    }
                }

                last_keys = current_keys;
                thread::sleep(poll_interval);
            }

            info!("🛑 Enhanced hotkey listener stopped");
        });

        Ok(())
    }

    pub fn stop_monitoring(&mut self) {
        info!("🛑 Stopping enhanced hotkey monitoring");
        IS_MONITORING.store(false, Ordering::SeqCst);
        self.is_running.store(false, Ordering::SeqCst);
        self.trigger_sender = None;
    }

    pub fn is_monitoring(&self) -> bool {
        IS_MONITORING.load(Ordering::SeqCst)
    }

    // Test method to verify hotkey detection
    pub fn test_key_detection(&self) -> Result<()> {
        info!("🧪 Testing key detection capabilities...");
        
        let device_state = DeviceState::new();
        
        println!("Press and hold Cmd+Shift+Space for 3 seconds to test detection...");
        println!("Press Ctrl+C to cancel test");
        
        let start_time = std::time::Instant::now();
        let test_duration = Duration::from_secs(10);
        
        while start_time.elapsed() < test_duration {
            let keys = device_state.get_keys();
            
            if !keys.is_empty() {
                let key_names: Vec<String> = keys.iter()
                    .map(|k| format!("{:?}", k))
                    .collect();
                println!("Keys detected: {}", key_names.join("+"));
                
                let space_pressed = keys.contains(&Keycode::Space);
                let meta_pressed = keys.contains(&Keycode::LMeta) 
                    || keys.contains(&Keycode::RMeta)
                    || keys.contains(&Keycode::Command); // Add Command key variant
                let shift_pressed = keys.contains(&Keycode::LShift) 
                    || keys.contains(&Keycode::RShift);
                
                if space_pressed && meta_pressed && shift_pressed {
                    println!("✅ SUCCESS: Cmd+Shift+Space detected!");
                    return Ok(());
                }
            }
            
            thread::sleep(Duration::from_millis(100));
        }
        
        println!("❌ Test completed - Cmd+Shift+Space not detected");
        println!("This suggests the hotkey detection has issues on your system");
        
        Ok(())
    }
}

impl Drop for HotkeyMonitor {
    fn drop(&mut self) {
        self.stop_monitoring();
    }
}

async fn handle_hotkey_trigger(state: Arc<AppState>) -> Result<()> {
    info!("🚀 Processing hotkey trigger - starting screenshot capture");

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

    let analysis = state
        .ai_client
        .analyze_image(&screenshot_data, prompt)
        .await?;

    pb.finish_and_clear();

    // Display results
    ui::print_analysis_result(&analysis);

    info!("✅ Screenshot analysis completed successfully");

    Ok(())
}