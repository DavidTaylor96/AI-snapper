"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.ClipboardMonitor = exports.TimerMonitor = exports.TerminalMonitor = void 0;
// src/terminal_monitor.ts
const readline = __importStar(require("readline"));
const ui_1 = require("./ui");
class TerminalMonitor {
    constructor() {
        this.rl = null;
        this.isProcessing = false;
        this.lastCommand = '';
        // Enable raw mode for single keypress detection if needed
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(true);
        }
    }
    async startMonitoring(state, mode = 'keypress') {
        if (mode === 'keypress') {
            await this.startKeypressMode(state);
        }
        else {
            await this.startCommandMode(state);
        }
    }
    /**
     * Mode 1: Single keypress trigger (Space/Enter)
     * No special permissions required!
     */
    async startKeypressMode(state) {
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
        process.stdin.on('data', async (key) => {
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
            switch (key) {
                case ' ': // Space
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
    async startCommandMode(state) {
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
        this.rl.on('line', async (input) => {
            const command = input.trim().toLowerCase();
            if (this.isProcessing) {
                console.log('⏳ Still processing previous capture...');
                this.rl.prompt();
                return;
            }
            // Handle commands
            if (!command || command === 'capture' || command === 'c') {
                await this.triggerCapture(state);
            }
            else if (command === 'solve' || command === 's') {
                await this.triggerCapture(state, 'Analyze this coding problem and provide a complete solution.');
            }
            else if (command === 'explain' || command === 'e') {
                await this.triggerCapture(state, 'Explain what you see in this image clearly and concisely.');
            }
            else if (command.startsWith('ask ')) {
                const question = input.substring(4).trim();
                await this.triggerCapture(state, question);
            }
            else if (command === 'repeat' || command === 'r') {
                if (this.lastCommand) {
                    await this.triggerCapture(state, this.lastCommand);
                }
                else {
                    await this.triggerCapture(state);
                }
            }
            else if (command === 'clear') {
                console.clear();
                this.showHelp();
            }
            else if (command === 'help' || command === 'h') {
                this.showHelp();
            }
            else if (command === 'exit' || command === 'quit') {
                console.log('👋 Goodbye!');
                process.exit();
            }
            else {
                console.log(`❓ Unknown command: ${command}`);
            }
            this.rl.prompt();
        });
        this.rl.on('close', () => {
            console.log('\n👋 Goodbye!');
            process.exit();
        });
    }
    /**
     * Mode 3: Interactive question mode
     */
    async askQuestion(state) {
        // Temporarily switch to line input mode
        process.stdin.setRawMode(false);
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        rl.question('❓ What would you like to know? ', async (question) => {
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
    async triggerCapture(state, customPrompt) {
        if (this.isProcessing) {
            return;
        }
        this.isProcessing = true;
        this.lastCommand = customPrompt || '';
        console.log('\n' + '─'.repeat(50));
        (0, ui_1.printStatus)('📸 Capturing screenshot...');
        try {
            const screenshotData = await state.screenshotCapture.capture();
            (0, ui_1.printStatus)('🤖 Analyzing with AI...');
            const question = customPrompt || state.customQuestion || state.customPrompt;
            const analysis = await state.aiClient.analyzeImage(screenshotData, question);
            (0, ui_1.printAnalysisResult)(analysis);
            console.log('─'.repeat(50) + '\n');
            console.log('✅ Ready for next capture (press Space/Enter)\n');
        }
        catch (error) {
            console.error('❌ Capture failed:', error);
        }
        finally {
            this.isProcessing = false;
        }
    }
    showHelp() {
        console.log('\n📌 Quick Controls:');
        console.log('  [Space/Enter] → Capture');
        console.log('  [s] → Solve  [e] → Explain  [q] → Question');
        console.log('  [h] → Help   [c] → Clear    [Ctrl+C] → Exit\n');
    }
    stopMonitoring() {
        if (this.rl) {
            this.rl.close();
        }
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(false);
        }
        process.stdin.pause();
    }
}
exports.TerminalMonitor = TerminalMonitor;
/**
 * Mode 4: Countdown Timer Mode
 * Automatically captures every N seconds
 */
class TimerMonitor {
    constructor() {
        this.interval = null;
        this.countdown = 0;
        this.isProcessing = false;
    }
    async startMonitoring(state, intervalSeconds = 5) {
        console.log(`⏱️  Auto-capture mode: Every ${intervalSeconds} seconds`);
        console.log('Press [p] to pause/resume, [n] for next capture now, [Ctrl+C] to exit\n');
        // Set up keypress handling for pause/resume
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(true);
        }
        process.stdin.setEncoding('utf8');
        process.stdin.resume();
        let isPaused = false;
        process.stdin.on('data', async (key) => {
            if (key === '\u0003') { // Ctrl+C
                this.stopMonitoring();
                process.exit();
            }
            else if (key === 'p' || key === 'P') {
                isPaused = !isPaused;
                console.log(isPaused ? '⏸️  Paused' : '▶️  Resumed');
            }
            else if (key === 'n' || key === 'N') {
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
    async capture(state) {
        if (this.isProcessing)
            return;
        this.isProcessing = true;
        console.log('\n📸 Auto-capturing...');
        try {
            const screenshotData = await state.screenshotCapture.capture();
            const analysis = await state.aiClient.analyzeImage(screenshotData);
            (0, ui_1.printAnalysisResult)(analysis);
        }
        catch (error) {
            console.error('❌ Auto-capture failed:', error);
        }
        finally {
            this.isProcessing = false;
        }
    }
    stopMonitoring() {
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
exports.TimerMonitor = TimerMonitor;
/**
 * Mode 5: Watch Mode - Monitor clipboard
 * Works by watching for a specific text pattern in clipboard
 */
class ClipboardMonitor {
    constructor() {
        this.lastClipboard = '';
        this.checkInterval = null;
    }
    async startMonitoring(state) {
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
            }
            catch (error) {
                // Ignore clipboard read errors
            }
        }, 500);
    }
    async triggerCapture(state, question) {
        (0, ui_1.printStatus)('📸 Capturing screenshot...');
        try {
            const screenshotData = await state.screenshotCapture.capture();
            const analysis = await state.aiClient.analyzeImage(screenshotData, question);
            (0, ui_1.printAnalysisResult)(analysis);
        }
        catch (error) {
            console.error('❌ Capture failed:', error);
        }
    }
    stopMonitoring() {
        if (this.checkInterval) {
            clearInterval(this.checkInterval);
            this.checkInterval = null;
        }
    }
}
exports.ClipboardMonitor = ClipboardMonitor;
//# sourceMappingURL=terminal_monitor.js.map