import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';
import * as toml from 'toml';

export interface AppConfig {
    screenshotsDir: string;
    imageFormat: string;
    jpegQuality: number;
    maxImageSizeMb: number;
    apiKey?: string;
    defaultProvider: string;
}

export class AppConfig {
    public screenshotsDir: string;
    public imageFormat: string;
    public jpegQuality: number;
    public maxImageSizeMb: number;
    public apiKey?: string;
    public defaultProvider: string;

    constructor(config: Partial<AppConfig> = {}) {
        const screenshotsDir = config.screenshotsDir || path.join(os.homedir(), '.ai-screenshots');
        
        this.screenshotsDir = screenshotsDir;
        this.imageFormat = config.imageFormat || 'png';
        this.jpegQuality = config.jpegQuality || 95;
        this.maxImageSizeMb = config.maxImageSizeMb || 10;
        this.apiKey = config.apiKey;
        this.defaultProvider = config.defaultProvider || 'openai';
    }

    static async load(): Promise<AppConfig> {
        const configDir = path.join(os.homedir(), '.config', 'ai-screenshot-analyzer');
        const configFile = path.join(configDir, 'config.toml');

        try {
            // Check if config file exists
            await fs.access(configFile);
            
            // Read and parse config file
            const configStr = await fs.readFile(configFile, 'utf8');
            const configData = toml.parse(configStr);
            
            return new AppConfig(configData);
        } catch (error) {
            // Config file doesn't exist, create default config
            const config = new AppConfig();
            
            // Create config directory
            await fs.mkdir(configDir, { recursive: true });
            
            // Create screenshots directory
            await fs.mkdir(config.screenshotsDir, { recursive: true });
            
            // Save default config
            const configStr = this.toTomlString(config);
            await fs.writeFile(configFile, configStr);
            
            return config;
        }
    }

    private static toTomlString(config: AppConfig): string {
        return `# Screenshot storage (temporary)
screenshots_dir = "${config.screenshotsDir.replace(/\\/g, '\\\\')}"

# Image processing
image_format = "${config.imageFormat}"
jpeg_quality = ${config.jpegQuality}
max_image_size_mb = ${config.maxImageSizeMb}

# AI provider settings
default_provider = "${config.defaultProvider}"
`;
    }

    async save(): Promise<void> {
        const configDir = path.join(os.homedir(), '.config', 'ai-screenshot-analyzer');
        const configFile = path.join(configDir, 'config.toml');
        
        await fs.mkdir(configDir, { recursive: true });
        
        const configStr = AppConfig.toTomlString(this);
        await fs.writeFile(configFile, configStr);
    }
}