import { AppState } from './main';
export declare class TerminalMonitor {
    private rl;
    private isProcessing;
    private lastCommand;
    constructor();
    startMonitoring(state: AppState, mode?: 'keypress' | 'command'): Promise<void>;
    /**
     * Mode 1: Single keypress trigger (Space/Enter)
     * No special permissions required!
     */
    private startKeypressMode;
    /**
     * Mode 2: Command-based input (type commands + Enter)
     */
    private startCommandMode;
    /**
     * Mode 3: Interactive question mode
     */
    private askQuestion;
    private triggerCapture;
    private showHelp;
    stopMonitoring(): void;
}
/**
 * Mode 4: Countdown Timer Mode
 * Automatically captures every N seconds
 */
export declare class TimerMonitor {
    private interval;
    private countdown;
    private isProcessing;
    startMonitoring(state: AppState, intervalSeconds?: number): Promise<void>;
    private capture;
    stopMonitoring(): void;
}
/**
 * Mode 5: Watch Mode - Monitor clipboard
 * Works by watching for a specific text pattern in clipboard
 */
export declare class ClipboardMonitor {
    private lastClipboard;
    private checkInterval;
    startMonitoring(state: AppState): Promise<void>;
    private triggerCapture;
    stopMonitoring(): void;
}
//# sourceMappingURL=terminal_monitor.d.ts.map