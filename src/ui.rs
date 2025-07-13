use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io;

pub fn print_header() {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        SetForegroundColor(Color::Cyan),
        Print("🤖 AI Screenshot Analyzer\n"),
        Print("━━━━━━━━━━━━━━━━━━━━━━━━━━\n"),
        ResetColor
    )
    .ok();
}

pub fn print_status(message: &str) {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!("{}\n", message)),
        ResetColor
    )
    .ok();
}

pub fn print_success(message: &str) {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print(format!("{}\n", message)),
        ResetColor
    )
    .ok();
}

pub fn print_error(message: &str) {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Red),
        Print(format!("{}\n", message)),
        ResetColor
    )
    .ok();
}

pub fn print_analysis_result(analysis: &str) {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print("💡 Analysis Result:\n"),
        ResetColor,
        SetForegroundColor(Color::White),
        Print("─".repeat(50)),
        Print("\n"),
        Print(format!("{}\n", analysis)),
        Print("─".repeat(50)),
        Print("\n"),
        ResetColor
    )
    .ok();
}
