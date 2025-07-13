pub mod config;
pub mod screenshot;
pub mod ai_client;
pub mod daemon;
pub mod ui;
pub mod permissions;

use ai_client::AIClient;
use config::AppConfig;
use screenshot::ScreenshotCapture;
use std::sync::Arc;
use anyhow::Result;

pub struct AppState {
    pub ai_client: AIClient,
    pub screenshot_capture: ScreenshotCapture,
    pub config: AppConfig,
    pub custom_prompt: Option<String>,
}

// Re-export types from main.rs
pub use crate::main_types::{Args, Commands};

// Re-export main functions
pub async fn capture_once(state: Arc<AppState>) -> Result<()> {
    ui::print_header();
    daemon::handle_screenshot_request(state).await
}

pub async fn show_config(state: Arc<AppState>) -> Result<()> {
    println!("📋 Configuration:");
    println!("├── Screenshots Directory: {}", state.config.screenshots_dir.display());
    println!("├── Image Format: {}", state.config.image_format);
    println!("├── JPEG Quality: {}", state.config.jpeg_quality);
    println!("├── Max Image Size: {} MB", state.config.max_image_size_mb);
    println!("└── AI Provider: {}", state.ai_client.provider());
    Ok(())
}

pub async fn test_ai_connection(state: Arc<AppState>) -> Result<()> {
    ui::print_status("🧪 Testing AI connection...");
    
    // Create a simple test image (1x1 pixel)
    let test_image = image::RgbImage::new(1, 1);
    let mut buffer = Vec::new();
    test_image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
    
    match state.ai_client.analyze_image(&buffer, "Test connection").await {
        Ok(_) => {
            ui::print_success("✅ AI connection successful!");
            Ok(())
        }
        Err(e) => {
            ui::print_error(&format!("❌ AI connection failed: {}", e));
            Err(e)
        }
    }
}

// Module for main types to avoid circular dependency
pub mod main_types {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[command(subcommand)]
        pub command: Option<Commands>,
        
        /// API key for AI service
        #[arg(long, env = "AI_API_KEY")]
        pub api_key: Option<String>,
        
        /// AI provider (openai, claude, gemini)
        #[arg(long, default_value = "openai")]
        pub provider: String,
        
        /// Custom prompt for AI analysis
        #[arg(long)]
        pub prompt: Option<String>,
        
        /// Enable debug logging
        #[arg(long)]
        pub debug: bool,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        /// Run the screenshot analyzer daemon
        Run,
        /// Capture and analyze a single screenshot
        Capture,
        /// Show configuration
        Config,
        /// Test AI connection
        Test,
    }
}