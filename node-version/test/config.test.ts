import { AppConfig } from '../src/config';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';

describe('AppConfig', () => {
  const testConfigDir = path.join(os.tmpdir(), 'ai-screenshot-analyzer-test');
  const testConfigFile = path.join(testConfigDir, 'config.toml');

  beforeEach(async () => {
    // Clean up test directory
    try {
      await fs.rm(testConfigDir, { recursive: true, force: true });
    } catch (error) {
      // Directory might not exist
    }
  });

  afterEach(async () => {
    // Clean up test directory
    try {
      await fs.rm(testConfigDir, { recursive: true, force: true });
    } catch (error) {
      // Directory might not exist
    }
  });

  test('should create default config', () => {
    const config = new AppConfig();
    
    expect(config.imageFormat).toBe('png');
    expect(config.jpegQuality).toBe(95);
    expect(config.maxImageSizeMb).toBe(10);
    expect(config.defaultProvider).toBe('openai');
    expect(config.screenshotsDir).toContain('.ai-screenshots');
  });

  test('should create config with custom values', () => {
    const customConfig = {
      imageFormat: 'jpeg',
      jpegQuality: 85,
      maxImageSizeMb: 5,
      defaultProvider: 'claude',
      screenshotsDir: '/custom/path'
    };

    const config = new AppConfig(customConfig);
    
    expect(config.imageFormat).toBe('jpeg');
    expect(config.jpegQuality).toBe(85);
    expect(config.maxImageSizeMb).toBe(5);
    expect(config.defaultProvider).toBe('claude');
    expect(config.screenshotsDir).toBe('/custom/path');
  });

  test('should handle partial config objects', () => {
    const partialConfig = {
      imageFormat: 'jpeg',
      jpegQuality: 85
    };

    const config = new AppConfig(partialConfig);
    
    expect(config.imageFormat).toBe('jpeg');
    expect(config.jpegQuality).toBe(85);
    expect(config.maxImageSizeMb).toBe(10); // default
    expect(config.defaultProvider).toBe('openai'); // default
  });
});