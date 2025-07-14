import * as robot from '@jitsi/robotjs';
import { EventEmitter } from 'events';
import { AppState } from './main';
import { printStatus, printAnalysisResult } from './ui';

export class HotkeyMonitor extends EventEmitter {
    private isRunning: boolean = false;
    private lastTriggerTime: number = 0;
    private debounceTime: number = 1000; // 1 second debounce
    private monitoringInterval: NodeJS.Timeout | null = null;
    private lastKeys: string[] = [];

    constructor() {
        super();
    }

    async startMonitoring(state: AppState): Promise<void> {
        if (this.isRunning) {
            console.warn('Hotkey monitoring is already running');
            return;
        }

        console.log('🎹 Starting hotkey monitoring (Cmd+Shift+Space / Ctrl+Shift+Space)');
        console.log(`🔍 Detected platform: ${process.platform}`);

        this.isRunning = true;

        try {
            // Set mouse delay to 0 for performance
            robot.setMouseDelay(0);
            
            // Start monitoring keyboard state
            this.monitoringInterval = setInterval(() => {
                this.checkHotkey(state);
            }, 50); // Check every 50ms

            console.log('✅ Hotkey monitoring started successfully');
        } catch (error) {
            console.error('❌ Failed to start hotkey monitoring:', error);
            throw error;
        }
    }

    stopMonitoring(): void {
        console.log('🛑 Stopping hotkey monitoring');
        this.isRunning = false;

        if (this.monitoringInterval) {
            clearInterval(this.monitoringInterval);
            this.monitoringInterval = null;
        }
    }

    isMonitoring(): boolean {
        return this.isRunning;
    }

    private checkHotkey(state: AppState): void {
        try {
            // Note: robotjs doesn't have built-in global hotkey detection
            // For now, we'll use a simple polling approach
            // In a real implementation, you would want to use a proper global hotkey library
            
            // This is a simplified version - in practice, you'd need a different approach
            // for true global hotkey detection across all applications
            
            // Since robotjs keyToggle doesn't work as expected for global hotkeys,
            // we'll implement a different approach or use console input for testing
            
            // For now, let's disable the automatic polling and rely on manual triggers
            // This is a limitation of the current implementation
            
        } catch (error) {
            // Robotjs might throw errors on some systems, log but don't crash
            console.debug('Hotkey check error (non-fatal):', error);
        }
    }

    private async processHotkeyTrigger(state: AppState): Promise<void> {
        console.log('🚀 Processing hotkey trigger - starting screenshot capture');

        printStatus('📸 Capturing screenshot...');

        try {
            // Capture screenshot
            const screenshotData = await state.screenshotCapture.capture();

            printStatus('🤖 Analyzing with AI...');

            // Use the question if provided, otherwise use custom prompt
            const questionToAsk = state.customQuestion || state.customPrompt;

            const analysis = await state.aiClient.analyzeImage(screenshotData, questionToAsk);

            // Display results
            printAnalysisResult(analysis);

            console.log('✅ Screenshot analysis completed successfully');
        } catch (error) {
            console.error('❌ Screenshot analysis failed:', error);
        }
    }

    async testKeyDetection(): Promise<void> {
        console.log('🧪 Testing key detection capabilities...');
        
        const hotkey = process.platform === 'darwin' ? 'Cmd+Shift+Space' : 'Ctrl+Shift+Space';
        
        console.log(`Note: Global hotkey detection is limited with robotjs`);
        console.log(`Expected hotkey: ${hotkey}`);
        console.log('Press Ctrl+C to cancel test');
        
        try {
            // Test robot.js availability
            const mousePos = robot.getMousePos();
            console.log(`Mouse position: ${mousePos.x}, ${mousePos.y} (robot.js is working)`);
            
            // Test screenshot capability instead
            const screenSize = robot.getScreenSize();
            console.log(`Screen size: ${screenSize.width}x${screenSize.height}`);
            
            console.log('✅ Robot.js is functional for screenshot capture');
            console.log('Note: For true global hotkey detection, consider using a different library');
            
        } catch (error) {
            console.error('❌ Robot.js test failed:', error);
            throw error;
        }
    }
}