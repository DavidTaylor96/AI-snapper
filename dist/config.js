"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.AppConfig = void 0;
const fs = __importStar(require("fs/promises"));
const path = __importStar(require("path"));
const os = __importStar(require("os"));
const toml = __importStar(require("toml"));
class AppConfig {
    constructor(config = {}) {
        const screenshotsDir = config.screenshotsDir || path.join(os.homedir(), '.ai-screenshots');
        this.screenshotsDir = screenshotsDir;
        this.imageFormat = config.imageFormat || 'png';
        this.jpegQuality = config.jpegQuality || 95;
        this.maxImageSizeMb = config.maxImageSizeMb || 10;
        this.apiKey = config.apiKey;
        this.defaultProvider = config.defaultProvider || 'claude';
    }
    static async load() {
        const configDir = path.join(os.homedir(), '.config', 'ai-screenshot-analyzer');
        const configFile = path.join(configDir, 'config.toml');
        try {
            // Check if config file exists
            await fs.access(configFile);
            // Read and parse config file
            const configStr = await fs.readFile(configFile, 'utf8');
            const configData = toml.parse(configStr);
            return new AppConfig(configData);
        }
        catch (error) {
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
    static toTomlString(config) {
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
    async save() {
        const configDir = path.join(os.homedir(), '.config', 'ai-screenshot-analyzer');
        const configFile = path.join(configDir, 'config.toml');
        await fs.mkdir(configDir, { recursive: true });
        const configStr = AppConfig.toTomlString(this);
        await fs.writeFile(configFile, configStr);
    }
}
exports.AppConfig = AppConfig;
//# sourceMappingURL=config.js.map