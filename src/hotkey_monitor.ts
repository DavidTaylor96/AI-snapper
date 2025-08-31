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
    private requiredKeys: string[];
    private isProcessing: boolean = false; // Prevent multiple simultaneous captures
    private keyTimeouts: Map<string, NodeJS.Timeout> = new Map(); // Track key release timeouts
    private keyReleaseDelay: number = 500; // How long to wait before considering a key "released"

    constructor() {
        super();
        // Define required keys based on platform
        this.requiredKeys = process.platform === 'darwin' 
            ? ['LEFT META', 'LEFT SHIFT', 'SPACE']  // macOS: Cmd+Shift+Space
            : ['LEFT CTRL', 'LEFT SHIFT', 'SPACE']; // Windows/Linux: Ctrl+Shift+Space
    }

    async startMonitoring(state: AppState): Promise<void> {
        if (this.isRunning) {
            console.warn('Hotkey monitoring is already running');
            return;
        }

        const hotkeyStr = process.platform === 'darwin' ? 'Cmd+Shift+Space' : 'Ctrl+Shift+Space';
        console.log(`🎹 Starting hotkey monitoring (${hotkeyStr})`);
        console.log(`🔍 Detected platform: ${process.platform}`);
        console.log(`📋 Required keys: ${this.requiredKeys.join(', ')}`);

        try {
            this.keyboardListener = new GlobalKeyboardListener();
            this.isRunning = true;

            this.keyboardListener.addListener((e, down) => {
                const keyName = e.name;
                
                // Skip if key name is undefined
                if (!keyName) {
                    return;
                }
                
                if (down) {
                    // Key pressed down
                    this.handleKeyPress(keyName, state);
                } else {
                    // Key released
                    this.handleKeyRelease(keyName);
                }
            });

            console.log('✅ Hotkey monitoring started successfully');
        } catch (error) {
            console.error('❌ Failed to start hotkey monitoring:', error);
            this.isRunning = false;
            throw error;
        }
    }

    private handleKeyPress(keyName: string, state: AppState): void {
        // Clear any existing timeout for this key
        const existingTimeout = this.keyTimeouts.get(keyName);
        if (existingTimeout) {
            clearTimeout(existingTimeout);
            this.keyTimeouts.delete(keyName);
        }

        // Add key to pressed set
        this.pressedKeys.add(keyName);
        
        // Check if all required keys are now pressed
        if (this.areAllKeysPressed() && this.shouldTrigger()) {
            console.log('🔥 All hotkeys detected! Triggering screenshot...');
            this.processHotkeyTrigger(state);
        }

        // Set a timeout to automatically remove this key if no release event comes
        const timeout = setTimeout(() => {
            this.pressedKeys.delete(keyName);
            this.keyTimeouts.delete(keyName);
        }, this.keyReleaseDelay);
        
        this.keyTimeouts.set(keyName, timeout);
    }

    private handleKeyRelease(keyName: string): void {
        // Clear any existing timeout for this key
        const existingTimeout = this.keyTimeouts.get(keyName);
        if (existingTimeout) {
            clearTimeout(existingTimeout);
            this.keyTimeouts.delete(keyName);
        }

        // Remove key from pressed set
        this.pressedKeys.delete(keyName);
    }

    private areAllKeysPressed(): boolean {
        // Check if ALL required keys are currently pressed
        const allPressed = this.requiredKeys.every(requiredKey => {
            // For modifier keys, accept either LEFT or RIGHT variants
            if (requiredKey === 'LEFT META') {
                return this.pressedKeys.has('LEFT META') || this.pressedKeys.has('RIGHT META');
            } else if (requiredKey === 'LEFT CTRL') {
                return this.pressedKeys.has('LEFT CTRL') || this.pressedKeys.has('RIGHT CTRL');
            } else if (requiredKey === 'LEFT SHIFT') {
                return this.pressedKeys.has('LEFT SHIFT') || this.pressedKeys.has('RIGHT SHIFT');
            } else {
                return this.pressedKeys.has(requiredKey);
            }
        });

        // Don't restrict extra keys - just ensure all required keys are pressed
        return allPressed;
    }

    stopMonitoring(): void {
        console.log('🛑 Stopping hotkey monitoring');
        this.isRunning = false;
        this.isProcessing = false;

        // Clear all key timeouts
        for (const timeout of this.keyTimeouts.values()) {
            clearTimeout(timeout);
        }
        this.keyTimeouts.clear();

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
        // Prevent multiple simultaneous captures
        if (this.isProcessing) {
            console.log('⚠️ Already processing a capture, ignoring trigger');
            return false;
        }

        // Debounce to prevent rapid successive triggers
        const now = Date.now();
        if (now - this.lastTriggerTime < this.debounceTime) {
            console.log('⚠️ Debounce period active, ignoring trigger');
            return false;
        }
        
        this.lastTriggerTime = now;
        return true;
    }

    private async processHotkeyTrigger(state: AppState): Promise<void> {
        this.isProcessing = true;
        
        // Clear all pressed keys immediately to prevent retriggering
        this.pressedKeys.clear();
        for (const timeout of this.keyTimeouts.values()) {
            clearTimeout(timeout);
        }
        this.keyTimeouts.clear();
        
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
        } finally {
            this.isProcessing = false;
        }
    }

    async testKeyDetection(): Promise<void> {
        console.log('🧪 Testing key detection capabilities...');
        
        const hotkey = process.platform === 'darwin' ? 'Cmd+Shift+Space' : 'Ctrl+Shift+Space';
        
        console.log(`Expected hotkey: ${hotkey}`);
        console.log(`Required keys: ${this.requiredKeys.join(' + ')}`);
        console.log('Press individual keys to see detection...');
        console.log('Press the full hotkey combination to test complete detection');
        console.log('Press Ctrl+C to cancel test');
        
        try {
            const listener = new GlobalKeyboardListener();
            let testCompleted = false;
            const pressedTestKeys = new Set<string>();
            
            const timeout = setTimeout(() => {
                if (!testCompleted) {
                    console.log('❌ Test timed out - no complete hotkey detected within 30 seconds');
                    listener.kill();
                    testCompleted = true;
                }
            }, 30000);

            listener.addListener((e, down) => {
                const keyName = e.name;
                
                // Skip if key name is undefined
                if (!keyName) {
                    return;
                }

                if (down) {
                    pressedTestKeys.add(keyName);
                    
                    // Check if all required keys are pressed
                    const allRequired = this.requiredKeys.every(requiredKey => {
                        if (requiredKey === 'LEFT META') {
                            return pressedTestKeys.has('LEFT META') || pressedTestKeys.has('RIGHT META');
                        } else if (requiredKey === 'LEFT CTRL') {
                            return pressedTestKeys.has('LEFT CTRL') || pressedTestKeys.has('RIGHT CTRL');
                        } else if (requiredKey === 'LEFT SHIFT') {
                            return pressedTestKeys.has('LEFT SHIFT') || pressedTestKeys.has('RIGHT SHIFT');
                        } else {
                            return pressedTestKeys.has(requiredKey);
                        }
                    });

                    if (allRequired) {
                        console.log('🎉 SUCCESS: Complete hotkey combination detected!');
                        console.log('✅ Hotkey detection is working correctly');
                        testCompleted = true;
                        clearTimeout(timeout);
                        listener.kill();
                    }
                } else {
                    pressedTestKeys.delete(keyName);
                }
            });
            
            // Wait for test to complete
            await new Promise<void>(resolve => {
                const checkInterval = setInterval(() => {
                    if (testCompleted) {
                        clearInterval(checkInterval);
                        resolve();
                    }
                }, 100);
            });
            
        } catch (error) {
            console.error('❌ Key detection test failed:', error);
            throw error;
        }
    }
}