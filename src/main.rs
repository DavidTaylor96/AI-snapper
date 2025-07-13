use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing::{debug, Level};

mod config;
mod screenshot;
mod ai_client;
mod daemon;
mod ui;
mod permissions;

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

#[derive(Subcommand, Debug)]
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
    // Load .env file if it exists
    debug!("Loading .env file if present...");
    dotenv::dotenv().ok();
    
    debug!("Parsing command line arguments...");
    let args = Args::parse();
    debug!("Args parsed - provider: {}, debug: {}, command: {:?}", args.provider, args.debug, args.command);
    
    // Initialize logging
    if args.debug {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        debug!("Debug logging enabled");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .init();
        debug!("Info logging enabled");
    }
    
    // Check and request permissions first
    debug!("Checking system permissions...");
    if !permissions::check_and_request_permissions()? {
        return Err(anyhow::anyhow!("Required permissions not granted. Please grant accessibility and screen recording permissions and try again."));
    }
    
    // Load configuration
    debug!("Loading application configuration...");
    let config = AppConfig::load()?;
    debug!("Configuration loaded successfully");
    debug!("Resolving API key from args or config...");
    let api_key = args.api_key.or(config.api_key.clone())
        .ok_or_else(|| anyhow::anyhow!("API key required. Set AI_API_KEY environment variable or use --api-key"))?;
    debug!("API key resolved (length: {} chars)", api_key.len());
    
    // Initialize components
    debug!("Initializing AI client with provider: {}", args.provider);
    let ai_client = AIClient::new(&args.provider, &api_key)?;
    debug!("Initializing screenshot capture...");
    let screenshot_capture = ScreenshotCapture::new()?;
    debug!("Creating application state...");
    let app_state = Arc::new(AppState {
        ai_client,
        screenshot_capture,
        config,
        custom_prompt: args.prompt.clone(),
    });
    debug!("AppState created with custom prompt: {:?}", args.prompt);
    
    debug!("Executing command: {:?}", args.command);
    match args.command {
        Some(Commands::Run) => {
            debug!("Starting daemon mode");
            daemon::run_daemon(app_state).await
        },
        Some(Commands::Capture) => {
            debug!("Starting single capture mode");
            capture_once(app_state).await
        },
        Some(Commands::Config) => {
            debug!("Showing configuration");
            show_config(app_state).await
        },
        Some(Commands::Test) => {
            debug!("Testing AI connection");
            test_ai_connection(app_state).await
        },
        None => {
            debug!("No command specified, defaulting to daemon mode");
            daemon::run_daemon(app_state).await
        },
    }
}

pub struct AppState {
    pub ai_client: AIClient,
    pub screenshot_capture: ScreenshotCapture,
    pub config: AppConfig,
    pub custom_prompt: Option<String>,
}


pub async fn capture_once(state: Arc<AppState>) -> Result<()> {
    debug!("Single capture mode initiated");
    ui::print_header();
    daemon::handle_screenshot_request(state).await
}

pub async fn show_config(state: Arc<AppState>) -> Result<()> {
    debug!("Displaying configuration to user");
    println!("📋 Configuration:");
    println!("├── Screenshots Directory: {}", state.config.screenshots_dir.display());
    println!("├── Image Format: {}", state.config.image_format);
    println!("├── JPEG Quality: {}", state.config.jpeg_quality);
    println!("├── Max Image Size: {} MB", state.config.max_image_size_mb);
    println!("└── AI Provider: {}", state.ai_client.provider());
    Ok(())
}

pub async fn test_ai_connection(state: Arc<AppState>) -> Result<()> {
    debug!("Testing AI connection with provider: {}", state.ai_client.provider());
    ui::print_status("🧪 Testing AI connection...");
    
    // Create a simple test image (1x1 pixel)
    debug!("Creating 1x1 test image for connection test");
    let test_image = image::RgbImage::new(1, 1);
    let mut buffer = Vec::new();
    test_image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
    
    debug!("Sending test image to AI service...");
    match state.ai_client.analyze_image(&buffer, "Test connection").await {
        Ok(response) => {
            debug!("AI connection test successful, response length: {} chars", response.len());
            ui::print_success("✅ AI connection successful!");
            Ok(())
        }
        Err(e) => {
            debug!("AI connection test failed: {}", e);
            ui::print_error(&format!("❌ AI connection failed: {}", e));
            Err(e)
        }
    }
}


