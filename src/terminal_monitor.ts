// src/terminal_monitor.ts
import * as readline from 'readline';
import { AppState } from './main';
import { printStatus, printAnalysisResult } from './ui';

export class TerminalMonitor {
    private rl: readline.Interface | null = null;
    private isProcessing: boolean = false;
    private lastCommand: string = '';

    constructor() {
        // Enable raw mode for single keypress detection if needed
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(true);
        }
    }

    async startMonitoring(state: AppState, mode: 'keypress' | 'command' = 'keypress'): Promise<void> {
        if (mode === 'keypress') {
            await this.startKeypressMode(state);
        } else {
            await this.startCommandMode(state);
        }
    }

    /**
     * Mode 1: Single keypress trigger (Space/Enter)
     * No special permissions required!
     */
    private async startKeypressMode(state: AppState): Promise<void> {
        console.log('📌 Terminal Controls:');
        console.log('  [Space]  → Capture & Analyze');
        console.log('  [Enter]  → Capture & Analyze');
        console.log('  [s]      → Solve coding problem');
        console.log('  [e]      → Explain what\'s on screen');
        console.log('  [q]      → Ask custom question');
        console.log('  [h]      → Show this help');
        console.log('  [Ctrl+C] → Exit\n');
        console.log('Ready! Press Space or Enter to capture...\n');

        // Set up stdin for raw keypress input
        process.stdin.setEncoding('utf8');
        process.stdin.resume();

        process.stdin.on('data', async (key: string) => {
            // Handle Ctrl+C
            if (key === '\u0003') {
                console.log('\n👋 Goodbye!');
                process.exit();
            }

            // Ignore input while processing
            if (this.isProcessing) {
                return;
            }

            // Handle different keypresses
            switch(key) {
                case ' ':  // Space
                case '\r': // Enter
                case '\n': // Newline
                    await this.triggerCapture(state);
                    break;
                
                case 's':
                case 'S':
                    await this.triggerCapture(state, 'Analyze this coding problem and provide a complete solution.');
                    break;
                
                case 'e':
                case 'E':
                    await this.triggerCapture(state, 'Explain what you see in this image clearly and concisely.');
                    break;
                
                case 'q':
                case 'Q':
                    await this.askQuestion(state);
                    break;
                
                case 'h':
                case 'H':
                    this.showHelp();
                    break;
                
                case 'c':
                case 'C':
                    console.clear();
                    this.showHelp();
                    break;
                
                default:
                    // Ignore other keys
                    break;
            }
        });
    }

    /**
     * Mode 2: Command-based input (type commands + Enter)
     */
    private async startCommandMode(state: AppState): Promise<void> {
        this.rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            prompt: '📸 > '
        });

        console.log('📌 Terminal Commands:');
        console.log('  capture / c     → Capture & Analyze');
        console.log('  solve / s       → Solve coding problem');
        console.log('  explain / e     → Explain screen content');
        console.log('  ask <question>  → Ask specific question');
        console.log('  repeat / r      → Repeat last capture');
        console.log('  clear           → Clear screen');
        console.log('  help / h        → Show this help');
        console.log('  exit / quit     → Exit\n');
        console.log('Or just press Enter to capture!\n');

        this.rl.prompt();

        this.rl.on('line', async (input: string) => {
            const command = input.trim().toLowerCase();
            
            if (this.isProcessing) {
                console.log('⏳ Still processing previous capture...');
                this.rl!.prompt();
                return;
            }

            // Handle commands
            if (!command || command === 'capture' || command === 'c') {
                await this.triggerCapture(state);
            } else if (command === 'solve' || command === 's') {
                await this.triggerCapture(state, 'Analyze this coding problem and provide a complete solution.');
            } else if (command === 'explain' || command === 'e') {
                await this.triggerCapture(state, 'Explain what you see in this image clearly and concisely.');
            } else if (command.startsWith('ask ')) {
                const question = input.substring(4).trim();
                await this.triggerCapture(state, question);
            } else if (command === 'repeat' || command === 'r') {
                if (this.lastCommand) {
                    await this.triggerCapture(state, this.lastCommand);
                } else {
                    await this.triggerCapture(state);
                }
            } else if (command === 'clear') {
                console.clear();
                this.showHelp();
            } else if (command === 'help' || command === 'h') {
                this.showHelp();
            } else if (command === 'exit' || command === 'quit') {
                console.log('👋 Goodbye!');
                process.exit();
            } else {
                console.log(`❓ Unknown command: ${command}`);
            }

            this.rl!.prompt();
        });

        this.rl.on('close', () => {
            console.log('\n👋 Goodbye!');
            process.exit();
        });
    }

    /**
     * Mode 3: Interactive question mode
     */
    private async askQuestion(state: AppState): Promise<void> {
        // Temporarily switch to line input mode
        process.stdin.setRawMode(false);
        
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });

        rl.question('❓ What would you like to know? ', async (question: string) => {
            rl.close();
            
            // Switch back to raw mode
            if (process.stdin.isTTY) {
                process.stdin.setRawMode(true);
            }
            
            if (question.trim()) {
                await this.triggerCapture(state, question.trim());
            }
        });
    }

    private async triggerCapture(state: AppState, customPrompt?: string): Promise<void> {
        if (this.isProcessing) {
            return;
        }

        this.isProcessing = true;
        this.lastCommand = customPrompt || '';

        console.log('\n' + '─'.repeat(50));
        printStatus('📸 Capturing screenshot...');

        try {
            const screenshotData = await state.screenshotCapture.capture();
            
            printStatus('🤖 Analyzing with AI...');
            
            const question = customPrompt || state.customQuestion || state.customPrompt;
            const analysis = await state.aiClient.analyzeImage(screenshotData, question);
            
            printAnalysisResult(analysis);
            console.log('─'.repeat(50) + '\n');
            console.log('✅ Ready for next capture (press Space/Enter)\n');
        } catch (error) {
            console.error('❌ Capture failed:', error);
        } finally {
            this.isProcessing = false;
        }
    }

    private showHelp(): void {
        console.log('\n📌 Quick Controls:');
        console.log('  [Space/Enter] → Capture');
        console.log('  [s] → Solve  [e] → Explain  [q] → Question');
        console.log('  [h] → Help   [c] → Clear    [Ctrl+C] → Exit\n');
    }

    stopMonitoring(): void {
        if (this.rl) {
            this.rl.close();
        }
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(false);
        }
        process.stdin.pause();
    }
}

/**
 * Mode 4: Countdown Timer Mode
 * Automatically captures every N seconds
 */
export class TimerMonitor {
    private interval: NodeJS.Timeout | null = null;
    private countdown: number = 0;
    private isProcessing: boolean = false;

    async startMonitoring(state: AppState, intervalSeconds: number = 5): Promise<void> {
        console.log(`⏱️  Auto-capture mode: Every ${intervalSeconds} seconds`);
        console.log('Press [p] to pause/resume, [n] for next capture now, [Ctrl+C] to exit\n');

        // Set up keypress handling for pause/resume
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(true);
        }
        process.stdin.setEncoding('utf8');
        process.stdin.resume();

        let isPaused = false;

        process.stdin.on('data', async (key: string) => {
            if (key === '\u0003') { // Ctrl+C
                this.stopMonitoring();
                process.exit();
            } else if (key === 'p' || key === 'P') {
                isPaused = !isPaused;
                console.log(isPaused ? '⏸️  Paused' : '▶️  Resumed');
            } else if (key === 'n' || key === 'N') {
                if (!this.isProcessing) {
                    await this.capture(state);
                }
            }
        });

        // Start countdown timer
        this.countdown = intervalSeconds;
        
        this.interval = setInterval(async () => {
            if (isPaused || this.isProcessing) {
                return;
            }

            this.countdown--;
            
            // Update countdown display
            process.stdout.write(`\r⏱️  Next capture in: ${this.countdown}s  `);
            
            if (this.countdown <= 0) {
                await this.capture(state);
                this.countdown = intervalSeconds;
            }
        }, 1000);
    }

    private async capture(state: AppState): Promise<void> {
        if (this.isProcessing) return;
        
        this.isProcessing = true;
        console.log('\n📸 Auto-capturing...');
        
        try {
            const screenshotData = await state.screenshotCapture.capture();
            const analysis = await state.aiClient.analyzeImage(screenshotData);
            printAnalysisResult(analysis);
        } catch (error) {
            console.error('❌ Auto-capture failed:', error);
        } finally {
            this.isProcessing = false;
        }
    }

    stopMonitoring(): void {
        if (this.interval) {
            clearInterval(this.interval);
            this.interval = null;
        }
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(false);
        }
        process.stdin.pause();
    }
}

/**
 * Mode 5: Watch Mode - Monitor clipboard
 * Works by watching for a specific text pattern in clipboard
 */
export class ClipboardMonitor {
    private lastClipboard: string = '';
    private checkInterval: NodeJS.Timeout | null = null;

    async startMonitoring(state: AppState): Promise<void> {
        console.log('📋 Clipboard trigger mode activated!');
        console.log('Copy the text "analyze" to trigger a screenshot');
        console.log('Copy "analyze: <question>" to ask a specific question\n');

        // Note: This would require the 'clipboardy' package
        // npm install clipboardy
        const clipboardy = require('clipboardy');

        this.checkInterval = setInterval(async () => {
            try {
                const currentClip = await clipboardy.read();
                
                if (currentClip !== this.lastClipboard) {
                    this.lastClipboard = currentClip;
                    
                    if (currentClip.toLowerCase().startsWith('analyze')) {
                        const parts = currentClip.split(':');
                        const question = parts.length > 1 ? parts[1].trim() : undefined;
                        
                        console.log('📋 Clipboard trigger detected!');
                        await this.triggerCapture(state, question);
                        
                        // Clear clipboard to prevent re-triggering
                        await clipboardy.write('');
                    }
                }
            } catch (error) {
                // Ignore clipboard read errors
            }
        }, 500);
    }

    private async triggerCapture(state: AppState, question?: string): Promise<void> {
        printStatus('📸 Capturing screenshot...');
        
        try {
            const screenshotData = await state.screenshotCapture.capture();
            const analysis = await state.aiClient.analyzeImage(screenshotData, question);
            printAnalysisResult(analysis);
        } catch (error) {
            console.error('❌ Capture failed:', error);
        }
    }

    stopMonitoring(): void {
        if (this.checkInterval) {
            clearInterval(this.checkInterval);
            this.checkInterval = null;
        }
    }
}