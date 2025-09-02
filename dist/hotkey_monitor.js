"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HotkeyMonitor = void 0;
const node_global_key_listener_1 = require("node-global-key-listener");
const events_1 = require("events");
const ui_1 = require("./ui");
class HotkeyMonitor extends events_1.EventEmitter {
    constructor() {
        super();
        this.keyboardListener = null;
        this.isRunning = false;
        this.lastTriggerTime = 0;
        this.debounceTime = 1000; // 1 second debounce
        this.pressedKeys = new Set();
        this.isProcessing = false; // Prevent multiple simultaneous captures
        this.keyTimeouts = new Map(); // Track key release timeouts
        this.keyReleaseDelay = 500; // How long to wait before considering a key "released"
        // Define required keys based on platform
        this.requiredKeys = process.platform === 'darwin'
            ? ['LEFT META', 'LEFT SHIFT', 'SPACE'] // macOS: Cmd+Shift+Space
            : ['LEFT CTRL', 'LEFT SHIFT', 'SPACE']; // Windows/Linux: Ctrl+Shift+Space
    }
    async startMonitoring(state) {
        if (this.isRunning) {
            console.warn('Hotkey monitoring is already running');
            return;
        }
        const hotkeyStr = process.platform === 'darwin' ? 'Cmd+Shift+Space' : 'Ctrl+Shift+Space';
        console.log(`🎹 Starting hotkey monitoring (${hotkeyStr})`);
        console.log(`🔍 Detected platform: ${process.platform}`);
        console.log(`📋 Required keys: ${this.requiredKeys.join(', ')}`);
        try {
            this.keyboardListener = new node_global_key_listener_1.GlobalKeyboardListener();
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
                }
                else {
                    // Key released
                    this.handleKeyRelease(keyName);
                }
            });
            console.log('✅ Hotkey monitoring started successfully');
        }
        catch (error) {
            console.error('❌ Failed to start hotkey monitoring:', error);
            this.isRunning = false;
            throw error;
        }
    }
    handleKeyPress(keyName, state) {
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
    handleKeyRelease(keyName) {
        // Clear any existing timeout for this key
        const existingTimeout = this.keyTimeouts.get(keyName);
        if (existingTimeout) {
            clearTimeout(existingTimeout);
            this.keyTimeouts.delete(keyName);
        }
        // Remove key from pressed set
        this.pressedKeys.delete(keyName);
    }
    areAllKeysPressed() {
        // Check if ALL required keys are currently pressed
        const allPressed = this.requiredKeys.every(requiredKey => {
            // For modifier keys, accept either LEFT or RIGHT variants
            if (requiredKey === 'LEFT META') {
                return this.pressedKeys.has('LEFT META') || this.pressedKeys.has('RIGHT META');
            }
            else if (requiredKey === 'LEFT CTRL') {
                return this.pressedKeys.has('LEFT CTRL') || this.pressedKeys.has('RIGHT CTRL');
            }
            else if (requiredKey === 'LEFT SHIFT') {
                return this.pressedKeys.has('LEFT SHIFT') || this.pressedKeys.has('RIGHT SHIFT');
            }
            else {
                return this.pressedKeys.has(requiredKey);
            }
        });
        // Don't restrict extra keys - just ensure all required keys are pressed
        return allPressed;
    }
    stopMonitoring() {
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
    isMonitoring() {
        return this.isRunning;
    }
    shouldTrigger() {
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
    async processHotkeyTrigger(state) {
        this.isProcessing = true;
        // Clear all pressed keys immediately to prevent retriggering
        this.pressedKeys.clear();
        for (const timeout of this.keyTimeouts.values()) {
            clearTimeout(timeout);
        }
        this.keyTimeouts.clear();
        console.log('🚀 Processing hotkey trigger - starting screenshot capture');
        (0, ui_1.printStatus)('📸 Capturing screenshot...');
        try {
            // Capture screenshot
            const screenshotData = await state.screenshotCapture.capture();
            (0, ui_1.printStatus)('🤖 Analyzing with AI...');
            // Use the question if provided, otherwise use custom prompt
            const questionToAsk = state.customQuestion || state.customPrompt;
            const analysis = await state.aiClient.analyzeImage(screenshotData, questionToAsk);
            // Display results
            (0, ui_1.printAnalysisResult)(analysis);
            console.log('✅ Screenshot analysis completed successfully');
        }
        catch (error) {
            console.error('❌ Screenshot analysis failed:', error);
        }
        finally {
            this.isProcessing = false;
        }
    }
    async testKeyDetection() {
        console.log('🧪 Testing key detection capabilities...');
        const hotkey = process.platform === 'darwin' ? 'Cmd+Shift+Space' : 'Ctrl+Shift+Space';
        console.log(`Expected hotkey: ${hotkey}`);
        console.log(`Required keys: ${this.requiredKeys.join(' + ')}`);
        console.log('Press individual keys to see detection...');
        console.log('Press the full hotkey combination to test complete detection');
        console.log('Press Ctrl+C to cancel test');
        try {
            const listener = new node_global_key_listener_1.GlobalKeyboardListener();
            let testCompleted = false;
            const pressedTestKeys = new Set();
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
                        }
                        else if (requiredKey === 'LEFT CTRL') {
                            return pressedTestKeys.has('LEFT CTRL') || pressedTestKeys.has('RIGHT CTRL');
                        }
                        else if (requiredKey === 'LEFT SHIFT') {
                            return pressedTestKeys.has('LEFT SHIFT') || pressedTestKeys.has('RIGHT SHIFT');
                        }
                        else {
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
                }
                else {
                    pressedTestKeys.delete(keyName);
                }
            });
            // Wait for test to complete
            await new Promise(resolve => {
                const checkInterval = setInterval(() => {
                    if (testCompleted) {
                        clearInterval(checkInterval);
                        resolve();
                    }
                }, 100);
            });
        }
        catch (error) {
            console.error('❌ Key detection test failed:', error);
            throw error;
        }
    }
}
exports.HotkeyMonitor = HotkeyMonitor;
//# sourceMappingURL=hotkey_monitor.js.map