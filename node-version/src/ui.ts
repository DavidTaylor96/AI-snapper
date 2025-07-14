import chalk from 'chalk';
import ora from 'ora';

export function printHeader(): void {
    console.clear();
    console.log(chalk.cyan('🤖 AI Screenshot Analyzer - Node.js Edition'));
    console.log(chalk.cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'));
}

export function printStatus(message: string): void {
    console.log(chalk.yellow(message));
}

export function printSuccess(message: string): void {
    console.log(chalk.green(message));
}

export function printError(message: string): void {
    console.log(chalk.red(message));
}

export function printAnalysisResult(analysis: string): void {
    // Simple, clean formatting for the analysis result
    const lines = analysis.split('\n');
    let inCodeBlock = false;
    
    for (const line of lines) {
        if (line.trim().startsWith('┌─ CODE SOLUTION')) {
            // Code block header - make it bright and noticeable
            console.log(chalk.green(line));
        } else if (line.trim().startsWith('└─')) {
            // Code block footer
            console.log(chalk.green(line));
        } else if (line.trim().startsWith('```')) {
            if (!inCodeBlock) {
                // Starting code block
                console.log(chalk.yellow(line));
                inCodeBlock = true;
            } else {
                // Ending code block
                console.log(chalk.yellow(line));
                inCodeBlock = false;
            }
        } else if (inCodeBlock) {
            // Code content - bright white on black for visibility
            console.log(chalk.bgBlack.white(line));
        } else if (line.trim().startsWith('─')) {
            // Separator lines
            console.log(chalk.blue(line));
        } else if (line.includes('🤖 ChatGPT Analysis')) {
            // Header
            console.log(chalk.cyan(line));
        } else {
            // Regular text
            console.log(chalk.white(line));
        }
    }
    
    // Add copy instruction
    console.log(chalk.gray('\n💡 Tip: Select and copy code between the ``` markers'));
}

export function createSpinner(message: string): any {
    return ora(message).start();
}

export function updateSpinner(spinner: any, message: string): void {
    spinner.text = message;
}

export function stopSpinner(spinner: any, success: boolean = true): void {
    if (success) {
        spinner.succeed();
    } else {
        spinner.fail();
    }
}