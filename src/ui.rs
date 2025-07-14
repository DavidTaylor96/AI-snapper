use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use std::io;

pub fn print_header() {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        SetForegroundColor(Color::Cyan),
        Print("🤖 AI Screenshot Analyzer - ChatGPT Edition\n"),
        Print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"),
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
    // Simple, clean formatting for the analysis result
    let lines: Vec<&str> = analysis.lines().collect();
    let mut in_code_block = false;
    
    for line in lines {
        if line.trim().starts_with("┌─ CODE SOLUTION") {
            // Code block header - make it bright and noticeable
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Green),
                Print(line),
                Print("\n"),
                ResetColor
            ).ok();
        } else if line.trim().starts_with("└─") {
            // Code block footer
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Green),
                Print(line),
                Print("\n"),
                ResetColor
            ).ok();
        } else if line.trim().starts_with("```") {
            if !in_code_block {
                // Starting code block
                execute!(
                    io::stdout(),
                    SetForegroundColor(Color::Yellow),
                    Print(line),
                    Print("\n"),
                    ResetColor
                ).ok();
                in_code_block = true;
            } else {
                // Ending code block
                execute!(
                    io::stdout(),
                    SetForegroundColor(Color::Yellow),
                    Print(line),
                    Print("\n"),
                    ResetColor
                ).ok();
                in_code_block = false;
            }
        } else if in_code_block {
            // Code content - bright white on black for visibility
            execute!(
                io::stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::Black),
                Print(line),
                Print("\n"),
                ResetColor
            ).ok();
        } else if line.trim().starts_with("─") {
            // Separator lines
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Blue),
                Print(line),
                Print("\n"),
                ResetColor
            ).ok();
        } else if line.contains("🤖 ChatGPT Analysis") {
            // Header
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Cyan),
                Print(line),
                Print("\n"),
                ResetColor
            ).ok();
        } else {
            // Regular text
            execute!(
                io::stdout(),
                SetForegroundColor(Color::White),
                Print(line),
                Print("\n"),
                ResetColor
            ).ok();
        }
    }
    
    // Add copy instruction
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("\n💡 Tip: Select and copy code between the ``` markers\n"),
        ResetColor
    ).ok();
}