use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};
use tracing::{debug, info, warn};

use crate::{ui, AppState};

static IS_MONITORING: AtomicBool = AtomicBool::new(false);

pub struct HotkeyMonitor {
    is_running: Arc<AtomicBool>,
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
        }
    }

    pub fn start_monitoring(&mut self, state: Arc<AppState>) -> Result<()> {
        if IS_MONITORING.load(Ordering::SeqCst) {
            warn!("Hotkey monitoring is already running");
            return Ok(());
        }

        info!("🎹 Starting device_query hotkey monitoring (Cmd+Shift+Space)");

        IS_MONITORING.store(true, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);

        let is_running = Arc::clone(&self.is_running);

        // Start device_query monitoring thread
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut last_activation = std::time::Instant::now();
            let debounce_time = Duration::from_millis(1000);

            info!("🔄 Device query hotkey listener started");
            debug!("📋 Monitoring for hotkey: Cmd+Shift+Space (Meta+Shift+Space)");

            while is_running.load(Ordering::SeqCst) && IS_MONITORING.load(Ordering::SeqCst) {
                // Log periodic status to confirm monitoring is active
                static mut LAST_STATUS_LOG: Option<std::time::Instant> = None;
                let now = std::time::Instant::now();
                unsafe {
                    if LAST_STATUS_LOG.map_or(true, |last| {
                        now.duration_since(last) >= Duration::from_secs(30)
                    }) {
                        debug!("🔍 Hotkey monitoring active - polling keys...");
                        LAST_STATUS_LOG = Some(now);
                    }
                }

                let keys: Vec<Keycode> = device_state.get_keys();

                if !keys.is_empty() {
                    debug!("🎹 Keys detected: {:?}", keys);

                    // Check for Cmd+Shift+Space combination
                    let space_pressed = keys.contains(&Keycode::Space);
                    let meta_pressed =
                        keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::RMeta); // Cmd key on macOS
                    let shift_pressed =
                        keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift);

                    if space_pressed && meta_pressed && shift_pressed {
                        debug!("⬇️ Hotkey combination detected, checking debounce...");
                        let time_since_last = now.duration_since(last_activation);
                        debug!(
                            "⏱️ Time since last activation: {:?} (debounce: {:?})",
                            time_since_last, debounce_time
                        );

                        if time_since_last >= debounce_time {
                            last_activation = now;
                            info!("🔥 Global hotkey triggered: Cmd+Shift+Space - starting screenshot capture");

                            // Trigger screenshot analysis
                            let state_clone = Arc::clone(&state);
                            tokio::spawn(async move {
                                if let Err(e) = handle_hotkey_trigger(state_clone).await {
                                    tracing::error!("Hotkey trigger failed: {}", e);
                                }
                            });
                        } else {
                            debug!("⚡ Hotkey press ignored due to debounce (too soon after last activation)");
                        }
                    }
                }

                thread::sleep(Duration::from_millis(100));
            }

            info!("🛑 Device query hotkey listener stopped");
        });

        Ok(())
    }

    pub fn stop_monitoring(&mut self) {
        info!("🛑 Stopping device_query hotkey monitoring");
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
