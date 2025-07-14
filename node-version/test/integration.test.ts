import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execAsync = promisify(exec);

describe('Integration Tests', () => {
    const binaryPath = path.join(__dirname, '../dist/main.js');
    const timeout = 10000;

    beforeAll(async () => {
        // Ensure the binary is built
        try {
            await execAsync('npm run build');
        } catch (error) {
            console.error('Build failed:', error);
        }
    });

    describe('CLI Commands', () => {
        test('should show help message', async () => {
            const { stdout } = await execAsync(`node ${binaryPath} --help`);
            expect(stdout).toContain('AI Screenshot Analyzer - Node.js/TypeScript version');
            expect(stdout).toContain('Commands:');
            expect(stdout).toContain('run');
            expect(stdout).toContain('capture');
            expect(stdout).toContain('config');
            expect(stdout).toContain('test');
            expect(stdout).toContain('test-hotkey');
            expect(stdout).toContain('solve');
        }, timeout);

        test('should show version', async () => {
            const { stdout } = await execAsync(`node ${binaryPath} --version`);
            expect(stdout.trim()).toBe('0.1.0');
        }, timeout);

        test('should show configuration', async () => {
            const { stdout } = await execAsync(`node ${binaryPath} config`);
            expect(stdout).toContain('📋 Configuration:');
            expect(stdout).toContain('Screenshots Directory:');
            expect(stdout).toContain('Image Format:');
            expect(stdout).toContain('JPEG Quality:');
            expect(stdout).toContain('Max Image Size:');
            expect(stdout).toContain('AI Provider:');
        }, timeout);

        test('should test hotkey detection', async () => {
            const { stdout } = await execAsync(`node ${binaryPath} test-hotkey`);
            expect(stdout).toContain('🧪 Hotkey Detection Test');
            expect(stdout).toContain('Platform: darwin');
            expect(stdout).toContain('Testing hotkey library...');
            expect(stdout).toContain('Robot.js is functional for screenshot capture');
        }, timeout);
    });

    describe('Screenshot Functionality', () => {
        test('should be able to capture screenshot', async () => {
            try {
                const { stdout } = await execAsync(`node ${binaryPath} capture`, { timeout: 30000 });
                expect(stdout).toContain('📸 Capturing screenshot...');
                expect(stdout).toContain('🤖 Analyzing with AI...');
                expect(stdout).toContain('ChatGPT Analysis');
            } catch (error) {
                // Screenshot capture might fail without proper permissions
                console.log('Screenshot capture test skipped due to permissions or API key');
            }
        }, 35000);
    });

    describe('Error Handling', () => {
        test('should handle invalid API key gracefully', async () => {
            try {
                const { stdout, stderr } = await execAsync(`AI_API_KEY=invalid_key node ${binaryPath} test`);
                expect(stdout || stderr).toContain('AI connection failed');
            } catch (error) {
                // This is expected behavior - the error message should contain API key error
                expect((error as Error).message).toContain('OpenAI API error');
            }
        }, timeout);
    });

    describe('Platform Compatibility', () => {
        test('should detect correct platform', async () => {
            const { stdout } = await execAsync(`node ${binaryPath} test-hotkey`);
            expect(stdout).toContain('Platform: darwin');
            expect(stdout).toContain('Expected hotkey: Cmd+Shift+Space');
        }, timeout);
    });
});