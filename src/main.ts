#!/usr/bin/env node

import 'dotenv/config';
import { Command } from 'commander';
import { AppConfig } from './config';
import { AIClient } from './ai_client';
import { ScreenshotCapture } from './screenshot';
import { HotkeyMonitor } from './hotkey_monitor';
import { TerminalMonitor, TimerMonitor } from './terminal_monitor';
import { printHeader, printStatus, printSuccess, printError, printAnalysisResult } from './ui';

export interface AppState {
    aiClient: AIClient;
    screenshotCapture: ScreenshotCapture;
    config: AppConfig;
    customQuestion?: string;
    customPrompt?: string;
}

async function main(): Promise<void> {
    const program = new Command();
    
    program
        .name('ai-screenshot-analyzer')
        .description('AI Screenshot Analyzer - Node.js/TypeScript version')
        .version('0.1.0');
    
    program
        .option('--api-key <key>', 'API key for AI service', process.env.AI_API_KEY)
        .option('--provider <provider>', 'AI provider (claude)', 'claude')
        .option('--prompt <prompt>', 'Custom prompt for AI analysis')
        .option('-q, --question <question>', 'Ask a specific question about the screenshot')
        .option('--mode <mode>', 'Input mode: terminal, hotkey, timer, command', 'terminal')
        .option('--interval <seconds>', 'Auto-capture interval for timer mode', '5')
        .option('--debug', 'Enable debug logging');
    
    program
        .command('run')
        .description('Run the screenshot analyzer daemon')
        .action(async (options) => {
            const state = await initializeAppState(program.opts());
            await runDaemon(state, program.opts());
        });
    
    program
        .command('capture')
        .description('Capture and analyze a single screenshot')
        .action(async (options) => {
            const state = await initializeAppState(program.opts());
            await captureOnce(state);
        });
    
    program
        .command('config')
        .description('Show configuration')
        .action(async (options) => {
            const state = await initializeAppState(program.opts());
            await showConfig(state);
        });
    
    program
        .command('test')
        .description('Test AI connection')
        .action(async (options) => {
            const state = await initializeAppState(program.opts());
            await testAiConnection(state);
        });
    
    program
        .command('test-hotkey')
        .description('Debug hotkey detection')
        .action(async () => {
            await testHotkeyDetection();
        });
    
    program
        .command('solve')
        .description('Solve coding problem on screen')
        .action(async (options) => {
            const state = await initializeAppState(program.opts());
            await solveCodingProblem(state);
        });
    
    // Default to run command if no command specified
    program.action(async (options) => {
        const state = await initializeAppState(program.opts());
        await runDaemon(state, program.opts());
    });
    
    try {
        await program.parseAsync(process.argv);
    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

async function initializeAppState(options: any): Promise<AppState> {
    // Initialize logging
    if (options.debug) {
        console.log('Debug logging enabled');
    }
    
    // Load configuration
    const config = await AppConfig.load();
    
    // Get API key from options, config, or environment
    const apiKey = options.apiKey || config.apiKey || process.env.AI_API_KEY;
    if (!apiKey) {
        throw new Error('API key required. Set AI_API_KEY environment variable or use --api-key');
    }
    
    // Initialize components
    const aiClient = new AIClient('claude', apiKey);
    const screenshotCapture = new ScreenshotCapture();
    
    return {
        aiClient,
        screenshotCapture,
        config,
        customQuestion: options.question,
        customPrompt: options.prompt
    };
}

async function runDaemon(state: AppState, options: any): Promise<void> {
    printHeader();
    
    const mode = options.mode || 'terminal';
    
    console.log('🚀 AI Screenshot Analyzer is running');
    if (state.customQuestion) {
        console.log(`📝 Active question: ${state.customQuestion}`);
    }
    console.log(`📺 Mode: ${mode}\n`);
    
    let monitor: any = null;
    
    switch (mode) {
        case 'terminal':
            // Default: Terminal input mode (no permissions required!)
            monitor = new TerminalMonitor();
            await monitor.startMonitoring(state, 'keypress');
            break;
            
        case 'command':
            // Command-line mode (type commands)
            monitor = new TerminalMonitor();
            await monitor.startMonitoring(state, 'command');
            break;
            
        case 'timer':
            // Auto-capture every N seconds
            const interval = parseInt(options.interval) || 5;
            monitor = new TimerMonitor();
            await monitor.startMonitoring(state, interval);
            break;
            
        case 'hotkey':
            // Try hotkey mode (may fail due to permissions)
            try {
                monitor = new HotkeyMonitor();
                await monitor.startMonitoring(state);
                console.log('✅ Hotkey monitoring started successfully');
            } catch (error) {
                console.log('⚠️  Hotkey mode failed (permissions issue)');
                console.log('📺 Falling back to terminal input mode...\n');
                
                // Fallback to terminal mode
                monitor = new TerminalMonitor();
                await monitor.startMonitoring(state, 'keypress');
            }
            break;
            
        default:
            console.error(`❌ Unknown mode: ${mode}`);
            console.log('Available modes: terminal, command, timer, hotkey');
            process.exit(1);
    }
    
    // Handle graceful shutdown
    process.on('SIGINT', () => {
        console.log('\n🛑 Shutting down...');
        if (monitor && monitor.stopMonitoring) {
            monitor.stopMonitoring();
        }
        process.exit(0);
    });
    
    // Keep the process alive
    process.stdin.resume();
}

async function captureOnce(state: AppState): Promise<void> {
    printHeader();
    
    printStatus('📸 Capturing screenshot...');
    
    // Capture screenshot
    const screenshotData = await state.screenshotCapture.capture();
    
    printStatus('🤖 Analyzing with AI...');
    
    // Use the question if provided, otherwise use custom prompt or default
    const questionToAsk = state.customQuestion || state.customPrompt;
    
    const analysis = await state.aiClient.analyzeImage(screenshotData, questionToAsk);
    
    // Display results
    printAnalysisResult(analysis);
}

async function showConfig(state: AppState): Promise<void> {
    console.log('📋 Configuration:');
    console.log(`├── Screenshots Directory: ${state.config.screenshotsDir}`);
    console.log(`├── Image Format: ${state.config.imageFormat}`);
    console.log(`├── JPEG Quality: ${state.config.jpegQuality}`);
    console.log(`├── Max Image Size: ${state.config.maxImageSizeMb} MB`);
    console.log(`└── AI Provider: ${state.aiClient.provider()}`);
}

async function testAiConnection(state: AppState): Promise<void> {
    printStatus('🧪 Testing AI connection...');
    
    try {
        // Create a simple test image
        const testImage = Buffer.from([
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
            0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x08, 0x06, 0x00, 0x00, 0x00, 0x73, 0x7A, 0x7A,
            0xF4, 0x00, 0x00, 0x00, 0x95, 0x49, 0x44, 0x41, 0x54, 0x58, 0x85, 0xED, 0xD7, 0x31, 0x0E, 0x80,
            0x20, 0x0C, 0x04, 0x50, 0xD7, 0xFF, 0xFF, 0x93, 0x3B, 0x05, 0x4A, 0x05, 0x52, 0x22, 0x05, 0x52,
            0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22,
            0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05,
            0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52,
            0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22,
            0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05,
            0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52,
            0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22,
            0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05,
            0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52,
            0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x05, 0x52, 0x22, 0x00, 0x00, 0x00,
            0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82
        ]);
        
        await state.aiClient.analyzeImage(testImage, 'This is a test. Please respond with "Connection successful".');
        printSuccess('✅ AI connection successful!');
    } catch (error) {
        printError(`❌ AI connection failed: ${error}`);
        throw error;
    }
}

async function testHotkeyDetection(): Promise<void> {
    printHeader();
    
    console.log('📋 Testing input methods...\n');
    
    // Test terminal input capability
    console.log('✅ Terminal input: Available');
    console.log('   No special permissions required!\n');
    
    // Test hotkey capability
    console.log('🔍 Testing hotkey capability...');
    console.log(`   Platform: ${process.platform}`);
    
    try {
        const monitor = new HotkeyMonitor();
        await monitor.testKeyDetection();
    } catch (error) {
        console.log('⚠️  Hotkey detection: Not available');
        console.log('   This is normal if accessibility permissions are not granted.\n');
        console.log('💡 Recommendation: Use terminal mode (default) instead:');
        console.log('   npm start --mode terminal');
    }
}

async function solveCodingProblem(state: AppState): Promise<void> {
    printHeader();
    
    printStatus('📸 Capturing screen for coding problem...');
    
    // Capture screenshot
    const screenshotData = await state.screenshotCapture.capture();
    
    printStatus('🤖 Analyzing and solving...');
    
    // Use a specific prompt for solving coding problems
    const solvePrompt = `This appears to be a coding challenge or problem. Please:
1. Briefly explain what the problem asks for
2. Provide a complete, working solution
3. Include any edge cases the solution handles
Keep it concise and focus on the solution.`;
    
    const analysis = await state.aiClient.analyzeImage(screenshotData, solvePrompt);
    
    // Display results
    printAnalysisResult(analysis);
}

// Export for testing
export { main, AppState };

// Run if this file is executed directly
if (require.main === module) {
    main().catch(console.error);
}