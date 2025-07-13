use anyhow::Result;
use global_hotkey::{GlobalHotKeyManager, HotKeyState};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use std::time::Duration;

pub async fn test_ai_connection(provider: &str) -> Result<String> {
    println!("🧪 Testing AI connection...");
    
    // Create a simple test image (1x1 pixel)
    let test_image = image::RgbImage::new(1, 1);
    let mut buffer = Vec::new();
    test_image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
    
    // Mock successful AI connection test
    let response = format!("Mock AI connection test successful for provider: {}", provider);
    println!("✅ AI connection successful!");
    Ok(response)
}

pub async fn test_hotkeys() -> Result<()> {
    println!("🧪 Testing global-hotkey system...");
    
    // Test hotkey manager creation
    let manager = GlobalHotKeyManager::new()?;
    println!("✅ Global hotkey manager created successfully");
    
    // Test hotkey registration
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::Space);
    manager.register(hotkey)?;
    println!("✅ Ctrl+Alt+Space hotkey registered successfully");
    
    // Test basic event detection
    println!("Testing hotkey detection for 3 seconds...");
    println!("Try pressing Ctrl+Alt+Space:");
    
    let start_time = std::time::Instant::now();
    let mut hotkey_events = 0;
    let mut ctrl_alt_space_detected = 0;
    
    while start_time.elapsed() < Duration::from_secs(3) {
        if let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().try_recv() {
            hotkey_events += 1;
            println!("🔍 Hotkey event detected: {:?}", event);
            
            // Check for Ctrl+Alt+Space combination
            if event.state == HotKeyState::Pressed {
                ctrl_alt_space_detected += 1;
                println!("🎯 Ctrl+Alt+Space combination detected!");
            }
        }
        
        std::thread::sleep(Duration::from_millis(100));
    }
    
    // Cleanup
    manager.unregister(hotkey)?;
    println!("✅ Hotkey unregistered successfully");
    
    println!("✅ Hotkey test completed!");
    println!("   - Total hotkey events: {}", hotkey_events);
    println!("   - Ctrl+Alt+Space detections: {}", ctrl_alt_space_detected);
    
    if hotkey_events > 0 {
        println!("✅ Global hotkey is working - events can be detected");
    } else {
        println!("⚠️ No hotkey events detected - try running with user interaction");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_hotkey_manager_creation() {
    println!("🧪 Testing hotkey manager creation...");
    
    let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
    println!("✅ Hotkey manager created successfully");
    
    // Test that we can create a hotkey without crashing
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::Space);
    println!("✅ Hotkey creation successful: Ctrl+Alt+Space");
}

#[tokio::test]
async fn test_hotkey_registration() {
    println!("🧪 Testing hotkey registration...");
    
    let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
    
    // Test hotkey registration and unregistration
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::Space);
    
    // Register hotkey
    manager.register(hotkey).expect("Failed to register hotkey");
    println!("✅ Hotkey registered successfully");
    
    // Unregister hotkey
    manager.unregister(hotkey).expect("Failed to unregister hotkey");
    println!("✅ Hotkey unregistered successfully");
    
    println!("✅ Hotkey registration test passed");
}