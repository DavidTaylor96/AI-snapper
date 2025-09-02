"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.ScreenshotCapture = void 0;
const screenshot_desktop_1 = __importDefault(require("screenshot-desktop"));
const sharp_1 = __importDefault(require("sharp"));
class ScreenshotCapture {
    constructor() { }
    async capture() {
        try {
            console.log('Capturing screenshot from primary display...');
            // Capture screenshot using screenshot-desktop
            const imageBuffer = await (0, screenshot_desktop_1.default)({ format: 'png' });
            console.log('Screenshot captured successfully');
            console.log(`Raw image buffer size: ${imageBuffer.length} bytes`);
            // Optimize the image
            const optimizedBuffer = await this.optimizeImage(imageBuffer);
            return optimizedBuffer;
        }
        catch (error) {
            console.error('Screenshot capture failed:', error);
            throw new Error(`Failed to capture screenshot: ${error}`);
        }
    }
    async optimizeImage(imageBuffer) {
        try {
            // Get image metadata
            const metadata = await (0, sharp_1.default)(imageBuffer).metadata();
            console.log(`Image metadata: ${metadata.width}x${metadata.height}, format: ${metadata.format}`);
            // Analyze image complexity to choose optimal format
            const complexity = await this.analyzeImageComplexity(imageBuffer);
            console.log(`Image complexity: ${complexity.toFixed(2)}`);
            let optimizedBuffer;
            if (complexity < 0.3) {
                // Low complexity - use PNG for better text preservation
                console.log('Using PNG format for low complexity image');
                optimizedBuffer = await (0, sharp_1.default)(imageBuffer)
                    .png({
                    compressionLevel: 9,
                    adaptiveFiltering: true
                })
                    .toBuffer();
            }
            else {
                // High complexity - use high-quality JPEG
                console.log('Using JPEG format for high complexity image');
                optimizedBuffer = await (0, sharp_1.default)(imageBuffer)
                    .jpeg({
                    quality: 95,
                    progressive: true
                })
                    .toBuffer();
            }
            console.log(`Optimized image size: ${optimizedBuffer.length} bytes`);
            // Check if optimized image is too large (> 10MB)
            const maxSizeBytes = 10 * 1024 * 1024; // 10MB
            if (optimizedBuffer.length > maxSizeBytes) {
                console.log('Image too large, applying additional compression...');
                // Apply more aggressive compression
                optimizedBuffer = await (0, sharp_1.default)(imageBuffer)
                    .jpeg({
                    quality: 85,
                    progressive: true
                })
                    .resize(2048, 2048, {
                    fit: 'inside',
                    withoutEnlargement: true
                })
                    .toBuffer();
                console.log(`Compressed image size: ${optimizedBuffer.length} bytes`);
            }
            return optimizedBuffer;
        }
        catch (error) {
            console.error('Image optimization failed:', error);
            // Return original buffer if optimization fails
            return imageBuffer;
        }
    }
    async analyzeImageComplexity(imageBuffer) {
        try {
            // Convert to raw RGB data for analysis
            const { data, info } = await (0, sharp_1.default)(imageBuffer)
                .raw()
                .toBuffer({ resolveWithObject: true });
            const { width, height, channels } = info;
            let totalVariance = 0;
            let pixelCount = 0;
            // Sample every 10th pixel for performance
            const sampleRate = 10;
            for (let y = 0; y < height; y += sampleRate) {
                for (let x = 0; x < width; x += sampleRate) {
                    const pixelIndex = (y * width + x) * channels;
                    if (pixelIndex + 2 < data.length) {
                        const r = data[pixelIndex];
                        const g = data[pixelIndex + 1];
                        const b = data[pixelIndex + 2];
                        // Calculate variance from grayscale
                        const gray = (r + g + b) / 3;
                        const variance = ((r - gray) ** 2 + (g - gray) ** 2 + (b - gray) ** 2) / 3;
                        totalVariance += variance;
                        pixelCount++;
                    }
                }
            }
            if (pixelCount > 0) {
                return (totalVariance / pixelCount) / 255;
            }
            else {
                return 0;
            }
        }
        catch (error) {
            console.warn('Image complexity analysis failed:', error);
            return 0.5; // Default to medium complexity
        }
    }
    detectImageFormat(imageBuffer) {
        if (imageBuffer.length < 8) {
            return 'image/png'; // Default fallback
        }
        // Check PNG signature
        if (imageBuffer.subarray(0, 8).equals(Buffer.from([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]))) {
            return 'image/png';
        }
        // Check JPEG signature
        if (imageBuffer.subarray(0, 3).equals(Buffer.from([0xFF, 0xD8, 0xFF]))) {
            return 'image/jpeg';
        }
        // Check WebP signature
        if (imageBuffer.length >= 12 &&
            imageBuffer.subarray(0, 4).equals(Buffer.from('RIFF')) &&
            imageBuffer.subarray(8, 12).equals(Buffer.from('WEBP'))) {
            return 'image/webp';
        }
        // Default to PNG
        return 'image/png';
    }
}
exports.ScreenshotCapture = ScreenshotCapture;
//# sourceMappingURL=screenshot.js.map