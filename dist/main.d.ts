#!/usr/bin/env node
import 'dotenv/config';
import { AppConfig } from './config';
import { AIClient } from './ai_client';
import { ScreenshotCapture } from './screenshot';
export interface AppState {
    aiClient: AIClient;
    screenshotCapture: ScreenshotCapture;
    config: AppConfig;
    customQuestion?: string;
    customPrompt?: string;
}
declare function main(): Promise<void>;
export { main };
//# sourceMappingURL=main.d.ts.map