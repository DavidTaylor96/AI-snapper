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
    }
    async startMonitoring(state) {
        if (this.isRunning) {
            console.warn('Hotkey monitoring is already running');
            return;
        }
        console.log('🎹 Starting hotkey monitoring (Cmd+Shift+Space / Ctrl+Shift+Space)');
        console.log(`🔍 Detected platform: ${process.platform}`);
        try {
            this.keyboardListener = new node_global_key_listener_1.GlobalKeyboardListener();
            this.isRunning = true;
            // Define the hotkey combination based on platform
            const expectedKeys = process.platform === 'darwin'
                ? ['LEFT META', 'LEFT SHIFT', 'SPACE'] // macOS: Cmd+Shift+Space
                : ['LEFT CTRL', 'LEFT SHIFT', 'SPACE']; // Windows/Linux: Ctrl+Shift+Space
            this.keyboardListener.addListener((e, down) => {
                const keyName = e.name;
                if (down && keyName) {
                    this.pressedKeys.add(keyName);
                    console.debug(`Key pressed: ${keyName}, Current keys: ${Array.from(this.pressedKeys).join(', ')}`);
                    // Check if all required keys are pressed
                    const allKeysPressed = expectedKeys.every(key => this.pressedKeys.has(key) ||
                        this.pressedKeys.has(key.replace('LEFT ', '')) ||
                        (key === 'LEFT META' && this.pressedKeys.has('RIGHT META')) ||
                        (key === 'LEFT CTRL' && this.pressedKeys.has('RIGHT CTRL')) ||
                        (key === 'LEFT SHIFT' && this.pressedKeys.has('RIGHT SHIFT')));
                    if (allKeysPressed && this.shouldTrigger()) {
                        console.log('🔥 Global hotkey triggered!');
                        this.processHotkeyTrigger(state);
                    }
                }
                else if (keyName) {
                    this.pressedKeys.delete(keyName);
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
    stopMonitoring() {
        console.log('🛑 Stopping hotkey monitoring');
        this.isRunning = false;
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
        const now = Date.now();
        if (now - this.lastTriggerTime < this.debounceTime) {
            console.debug('⚡ Hotkey trigger ignored due to debounce');
            return false;
        }
        this.lastTriggerTime = now;
        return true;
    }
    async processHotkeyTrigger(state) {
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
    }
    async testKeyDetection() {
        console.log('🧪 Testing key detection capabilities...');
        const hotkey = process.platform === 'darwin' ? 'Cmd+Shift+Space' : 'Ctrl+Shift+Space';
        console.log(`Expected hotkey: ${hotkey}`);
        console.log('Press the hotkey combination to test...');
        console.log('Press Ctrl+C to cancel test');
        try {
            const listener = new node_global_key_listener_1.GlobalKeyboardListener();
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
        }
        catch (error) {
            console.error('❌ Key detection test failed:', error);
            throw error;
        }
    }
}
exports.HotkeyMonitor = HotkeyMonitor;
//# sourceMappingURL=hotkey_monitor.js.map