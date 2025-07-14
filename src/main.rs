use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing::info;

mod ai_client;
mod config;
mod hotkey_monitor;
mod screenshot;
mod ui;

use ai_client::AIClient;
use config::AppConfig;
use hotkey_monitor::HotkeyMonitor;
use screenshot::ScreenshotCapture;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// API key for AI service
    #[arg(long, env = "AI_API_KEY")]
    api_key: Option<String>,

    /// AI provider (openai, claude, gemini)
    #[arg(long, default_value = "openai")]
    provider: String,

    /// Custom prompt for AI analysis
    #[arg(long)]
    prompt: Option<String>,

    /// Ask a specific question about the screenshot
    #[arg(long, short)]
    question: Option<String>,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the screenshot analyzer daemon
    Run,
    /// Capture and analyze a single screenshot
    Capture,
    /// Show configuration
    Config,
    /// Test AI connection
    Test,
    /// Debug hotkey detection (NEW)
    TestHotkey,
    /// Solve coding problem on screen
    Solve,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.debug { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(log_level).init();

    // For hotkey test, we don't need full initialization
    if matches!(args.command, Some(Commands::TestHotkey)) {
        return test_hotkey_detection().await;
    }

    // Load configuration for other commands
    let config = AppConfig::load()?;
    let api_key = args.api_key.or(config.api_key.clone()).ok_or_else(|| {
        anyhow::anyhow!("API key required. Set AI_API_KEY environment variable or use --api-key")
    })?;

    // Initialize components - provider parameter is ignored now (always uses OpenAI)
    let ai_client = AIClient::new("openai", &api_key)?;
    let screenshot_capture = ScreenshotCapture::new()?;
    let app_state = Arc::new(AppState {
        ai_client,
        screenshot_capture,
        config,
        custom_question: args.question,
        custom_prompt: args.prompt,
    });

    match args.command {
        Some(Commands::Run) => run_daemon(app_state).await,
        Some(Commands::Capture) => capture_once(app_state).await,
        Some(Commands::Config) => show_config(app_state).await,
        Some(Commands::Test) => test_ai_connection(app_state).await,
        Some(Commands::TestHotkey) => unreachable!(), // Handled above
        Some(Commands::Solve) => solve_coding_problem(app_state).await,
        None => run_daemon(app_state).await,
    }
}

struct AppState {
    ai_client: AIClient,
    screenshot_capture: ScreenshotCapture,
    config: AppConfig,
    custom_question: Option<String>,
    custom_prompt: Option<String>,
}

async fn run_daemon(state: Arc<AppState>) -> Result<()> {
    ui::print_header();

    info!("🚀 AI Screenshot Analyzer is running");
    println!("Press Cmd+Shift+Space to capture and analyze screenshot");
    if let Some(question) = &state.custom_question {
        println!("📝 Active question: {}", question);
    }
    println!("Press Ctrl+C to exit");

    // Initialize and start hotkey monitoring
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

async fn capture_once(state: Arc<AppState>) -> Result<()> {
    ui::print_header();

    ui::print_status("📸 Capturing screenshot...");

    // Capture screenshot
    let screenshot_data = state.screenshot_capture.capture().await?;

    ui::print_status("🤖 Analyzing with AI...");

    // Create progress indicator
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Processing with AI...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Use the question if provided, otherwise use custom prompt or default
    let question_to_ask = state.custom_question.as_deref()
        .or(state.custom_prompt.as_deref());

    let analysis = state
        .ai_client
        .analyze_image(&screenshot_data, question_to_ask)
        .await?;

    pb.finish_and_clear();

    // Display results
    ui::print_analysis_result(&analysis);

    Ok(())
}

async fn show_config(state: Arc<AppState>) -> Result<()> {
    println!("📋 Configuration:");
    println!(
        "├── Screenshots Directory: {}",
        state.config.screenshots_dir.display()
    );
    println!("├── Image Format: {}", state.config.image_format);
    println!("├── JPEG Quality: {}", state.config.jpeg_quality);
    println!("├── Max Image Size: {} MB", state.config.max_image_size_mb);
    println!("└── AI Provider: {}", state.ai_client.provider());
    Ok(())
}

async fn test_ai_connection(state: Arc<AppState>) -> Result<()> {
    ui::print_status("🧪 Testing AI connection...");

    // Create a simple test image (1x1 pixel)
    let test_image = image::RgbImage::new(1, 1);
    let mut buffer = Vec::new();
    test_image.write_to(
        &mut std::io::Cursor::new(&mut buffer),
        image::ImageOutputFormat::Png,
    )?;

    match state
        .ai_client
        .analyze_image(&buffer, Some("Test connection"))
        .await
    {
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

// NEW: Hotkey detection test function
async fn test_hotkey_detection() -> Result<()> {
    ui::print_header();
    
    println!("🧪 Hotkey Detection Test");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");
    println!("This will test if your system can detect the Cmd+Shift+Space combination.");
    println!();
    
    // Check platform
    println!("🔍 Platform: {}", std::env::consts::OS);
    
    // Test basic device_query functionality
    println!("📋 Testing device_query library...");
    
    let monitor = HotkeyMonitor::new();
    monitor.test_key_detection()?;
    
    Ok(())
}

// NEW: Solve coding problem function
async fn solve_coding_problem(state: Arc<AppState>) -> Result<()> {
    ui::print_header();
    
    println!("🧩 Coding Problem Solver");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");
    
    ui::print_status("📸 Capturing screen for coding problem...");

    // Capture screenshot
    let screenshot_data = state.screenshot_capture.capture().await?;

    ui::print_status("🤖 Analyzing and solving...");

    // Create progress indicator
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Solving coding problem...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Use a specific prompt for solving coding problems
    let solve_prompt = "This appears to be a coding challenge or problem. Please:\n\
                       1. Briefly explain what the problem asks for\n\
                       2. Provide a complete, working solution\n\
                       3. Include any edge cases the solution handles\n\
                       Keep it concise and focus on the solution.";

    let analysis = state
        .ai_client
        .analyze_image(&screenshot_data, Some(solve_prompt))
        .await?;

    pb.finish_and_clear();

    // Display results
    ui::print_analysis_result(&analysis);

    Ok(())
}