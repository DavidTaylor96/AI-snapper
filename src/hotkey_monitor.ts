import { GlobalKeyboardListener } from 'node-global-key-listener';
import { EventEmitter } from 'events';
import { AppState } from './main';
import { printStatus, printAnalysisResult } from './ui';

export class HotkeyMonitor extends EventEmitter {
    private keyboardListener: GlobalKeyboardListener | null = null;
    private isRunning: boolean = false;
    private lastTriggerTime: number = 0;
    private debounceTime: number = 1000; // 1 second debounce
    private pressedKeys: Set<string> = new Set();

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

        try {
            this.keyboardListener = new GlobalKeyboardListener();
            this.isRunning = true;

            // Define the hotkey combination based on platform
            const expectedKeys = process.platform === 'darwin' 
                ? ['LEFT META', 'LEFT SHIFT', 'SPACE']  // macOS: Cmd+Shift+Space
                : ['LEFT CTRL', 'LEFT SHIFT', 'SPACE']; // Windows/Linux: Ctrl+Shift+Space

            this.keyboardListener.addListener((e, down) => {
                const keyName = e.name;
                
                if (down && keyName) {
                    this.pressedKeys.add(keyName);
                    // Check if all required keys are pressed
                    const allKeysPressed = expectedKeys.every(key => 
                        this.pressedKeys.has(key) || 
                        this.pressedKeys.has(key.replace('LEFT ', '')) ||
                        (key === 'LEFT META' && this.pressedKeys.has('RIGHT META')) ||
                        (key === 'LEFT CTRL' && this.pressedKeys.has('RIGHT CTRL')) ||
                        (key === 'LEFT SHIFT' && this.pressedKeys.has('RIGHT SHIFT'))
                    );

                    if (allKeysPressed && this.shouldTrigger()) {
                        console.log('🔥 Global hotkey triggered!');
                        this.processHotkeyTrigger(state);
                    }
                } else if (keyName) {
                    this.pressedKeys.delete(keyName);
                }
            });

            console.log('✅ Hotkey monitoring started successfully');
        } catch (error) {
            console.error('❌ Failed to start hotkey monitoring:', error);
            this.isRunning = false;
            throw error;
        }
    }

    stopMonitoring(): void {
        console.log('🛑 Stopping hotkey monitoring');
        this.isRunning = false;

        if (this.keyboardListener) {
            this.keyboardListener.kill();
            this.keyboardListener = null;
        }
        
        this.pressedKeys.clear();
    }

    isMonitoring(): boolean {
        return this.isRunning;
    }

    private shouldTrigger(): boolean {
        const now = Date.now();
        if (now - this.lastTriggerTime < this.debounceTime) {
            return false;
        }
        this.lastTriggerTime = now;
        return true;
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
        
        console.log(`Expected hotkey: ${hotkey}`);
        console.log('Press the hotkey combination to test...');
        console.log('Press Ctrl+C to cancel test');
        
        try {
            const listener = new GlobalKeyboardListener();
            let testCompleted = false;
            
            const timeout = setTimeout(() => {
                if (!testCompleted) {
                    console.log('❌ Test timed out - hotkey not detected within 10 seconds');
                    listener.kill();
                }
            }, 10000);

            listener.addListener((e, down) => {
                if (down) {
                    console.log(`Key detected: ${e.name}`);
                    
                    // Simple test - just check if we can detect any keys
                    if (e.name === 'SPACE') {
                        console.log('✅ SUCCESS: Key detection is working!');
                        console.log('Note: Full hotkey combination detection requires the complete implementation');
                        testCompleted = true;
                        clearTimeout(timeout);
                        listener.kill();
                    }
                }
            });
            
            // Wait for test to complete
            await new Promise(resolve => {
                const checkInterval = setInterval(() => {
                    if (testCompleted || !listener) {
                        clearInterval(checkInterval);
                        resolve(void 0);
                    }
                }, 100);
            });
            
        } catch (error) {
            console.error('❌ Key detection test failed:', error);
            throw error;
        }
    }
}