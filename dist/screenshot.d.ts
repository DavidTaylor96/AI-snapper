export declare class ScreenshotCapture {
    constructor();
    capture(): Promise<Buffer>;
    private optimizeImage;
    private analyzeImageComplexity;
    detectImageFormat(imageBuffer: Buffer): string;
}
//# sourceMappingURL=screenshot.d.ts.map