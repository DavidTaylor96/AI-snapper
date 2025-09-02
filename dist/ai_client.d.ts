export declare class AIClient {
    private client;
    private apiKey;
    constructor(provider: string, apiKey: string);
    provider(): string;
    analyzeImage(imageData: Buffer, userQuestion?: string): Promise<string>;
    private analyzeWithClaude;
    private createConcisePrompt;
    private detectImageFormat;
}
//# sourceMappingURL=ai_client.d.ts.map