import { EventEmitter } from 'events';
import { AppState } from './main';
export declare class HotkeyMonitor extends EventEmitter {
    private keyboardListener;
    private isRunning;
    private lastTriggerTime;
    private debounceTime;
    private pressedKeys;
    private requiredKeys;
    private isProcessing;
    private keyTimeouts;
    private keyReleaseDelay;
    constructor();
    startMonitoring(state: AppState): Promise<void>;
    private handleKeyPress;
    private handleKeyRelease;
    private areAllKeysPressed;
    stopMonitoring(): void;
    isMonitoring(): boolean;
    private shouldTrigger;
    private processHotkeyTrigger;
    testKeyDetection(): Promise<void>;
}
//# sourceMappingURL=hotkey_monitor.d.ts.map