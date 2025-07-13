use anyhow::Result;
use tracing::{info, debug};

pub fn check_and_request_permissions() -> Result<bool> {
    info!("🔐 Checking macOS permissions...");
    
    // Check if we already have the required permissions
    let screen_recording_granted = check_screen_recording_permission();
    let accessibility_granted = check_accessibility_permission();
    
    debug!("Screen recording permission: {}", screen_recording_granted);
    debug!("Accessibility permission: {}", accessibility_granted);
    
    if screen_recording_granted && accessibility_granted {
        info!("✅ All required permissions are already granted");
        return Ok(true);
    }
    
    // Show permission setup guide for missing permissions
    println!("\n🔐 Permission Setup Guide");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("The AI Screenshot Analyzer needs the following permissions:");
    println!("");
    
    if !screen_recording_granted {
        println!("1. 🖥️  SCREEN RECORDING - To capture screenshots");
        println!("   → System Preferences → Security & Privacy → Privacy → Screen Recording");
        println!("   → Add and enable your Terminal app");
        println!("");
    } else {
        println!("1. ✅ SCREEN RECORDING - Already granted");
        println!("");
    }
    
    if !accessibility_granted {
        println!("2. ♿ ACCESSIBILITY - To detect global hotkeys (Cmd+Shift+2)");
        println!("   → System Preferences → Security & Privacy → Privacy → Accessibility");
        println!("   → Add and enable your Terminal app");
        println!("");
    } else {
        println!("2. ✅ ACCESSIBILITY - Already granted");
        println!("");
    }
    
    if !screen_recording_granted || !accessibility_granted {
        println!("💡 TIP: Permission dialogs may appear when you first use these features.");
        println!("💡 TIP: If hotkeys don't work, check Accessibility permissions.");
        println!("💡 TIP: If screenshots fail, check Screen Recording permissions.");
        println!("");
        
        // Try to open system preferences to help the user
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            println!("🚀 Opening System Preferences for you...");
            let _ = Command::new("open")
                .arg("/System/Library/PreferencePanes/Security.prefPane")
                .spawn();
        }
        
        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
    }
    
    info!("✅ Continuing with permission setup complete");
    Ok(true)
}

fn check_screen_recording_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        // Try to check screen recording permission by attempting to get display info
        // This is a lightweight check that doesn't require additional dependencies
        let output = Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .arg("-json")
            .output();
            
        match output {
            Ok(result) => {
                // If we can run system_profiler successfully, we likely have screen recording permission
                // This is not 100% accurate but good enough for a basic check
                result.status.success() && !result.stdout.is_empty()
            }
            Err(_) => false,
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // On non-macOS systems, assume permission is granted
        true
    }
}

fn check_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        // Check if we can query accessibility permissions
        // This is a basic check - a more robust solution would use CoreFoundation APIs
        let output = Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to get name of every process")
            .output();
            
        match output {
            Ok(result) => {
                // If osascript can access System Events without error, we likely have accessibility permission
                result.status.success()
            }
            Err(_) => false,
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // On non-macOS systems, assume permission is granted
        true
    }
}

pub fn verify_permissions() -> bool {
    // Check both permissions
    let screen_recording = check_screen_recording_permission();
    let accessibility = check_accessibility_permission();
    
    debug!("Permission verification - Screen recording: {}, Accessibility: {}", 
           screen_recording, accessibility);
    
    screen_recording && accessibility
}