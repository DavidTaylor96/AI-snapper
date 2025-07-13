use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing::Level;

mod config;
mod screenshot;
mod ai_client;
mod daemon;
mod ui;

use config::AppConfig;
use screenshot::ScreenshotCapture;
use ai_client::AIClient;

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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    if args.debug {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .init();
    }
    
    // Load configuration
    let config = AppConfig::load()?;
    let api_key = args.api_key.or(config.api_key.clone())
        .ok_or_else(|| anyhow::anyhow!("API key required. Set AI_API_KEY environment variable or use --api-key"))?;
    
    // Initialize components
    let ai_client = AIClient::new(&args.provider, &api_key)?;
    let screenshot_capture = ScreenshotCapture::new()?;
    let app_state = Arc::new(AppState {
        ai_client,
        screenshot_capture,
        config,
        custom_prompt: args.prompt,
    });
    
    match args.command {
        Some(Commands::Run) => daemon::run_daemon(app_state).await,
        Some(Commands::Capture) => capture_once(app_state).await,
        Some(Commands::Config) => show_config(app_state).await,
        Some(Commands::Test) => test_ai_connection(app_state).await,
        None => daemon::run_daemon(app_state).await,
    }
}

pub struct AppState {
    pub ai_client: AIClient,
    pub screenshot_capture: ScreenshotCapture,
    pub config: AppConfig,
    pub custom_prompt: Option<String>,
}


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


