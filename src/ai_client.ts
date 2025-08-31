import Anthropic from '@anthropic-ai/sdk';

export class AIClient {
    private client: Anthropic;
    private apiKey: string;

    constructor(provider: string, apiKey: string) {
        this.apiKey = apiKey;
        this.client = new Anthropic({
            apiKey: apiKey,
        });
    }

    provider(): string {
        return 'claude'; // Always return claude since we only support Claude now
    }

    async analyzeImage(imageData: Buffer, userQuestion?: string): Promise<string> {
        return this.analyzeWithClaude(imageData, userQuestion);
    }

    private async analyzeWithClaude(imageData: Buffer, userQuestion?: string): Promise<string> {
        try {
            // Encode image as base64 for Claude Vision API
            const base64Image = imageData.toString('base64');

            // Detect image format for proper MIME type
            const mimeType = this.detectImageFormat(imageData);

            // Create the enhanced prompt
            const prompt = this.createConcisePrompt(userQuestion);

            const response = await this.client.messages.create({
                model: 'claude-3-5-sonnet-20241022',
                max_tokens: 500, // Reduced from 1000 for more concise responses
                temperature: 0.1,
                system: 'You are a concise programming assistant. Provide direct, minimal responses. For coding problems, give working code in markdown blocks without extra explanation. For questions, give brief, direct answers.',
                messages: [
                    {
                        role: 'user',
                        content: [
                            {
                                type: 'text',
                                text: prompt
                            },
                            {
                                type: 'image',
                                source: {
                                    type: 'base64',
                                    media_type: mimeType as 'image/jpeg' | 'image/png' | 'image/gif' | 'image/webp',
                                    data: base64Image
                                }
                            }
                        ]
                    }
                ]
            });

            const content = response.content[0];
            if (!content || content.type !== 'text') {
                throw new Error('No text response from Claude');
            }

            // Return the raw response without additional formatting
            return content.text.trim();
        } catch (error) {
            if (error instanceof Anthropic.APIError) {
                throw new Error(`Claude API error: ${error.message}`);
            }
            throw error;
        }
    }

    private createConcisePrompt(userQuestion?: string): string {
        if (userQuestion && userQuestion.trim()) {
            return `Answer this question directly and concisely: ${userQuestion.trim()}

If code is needed, provide it in markdown code blocks without extra explanation.`;
        } else {
            // Default prompt optimized for direct responses
            return `Analyze what you see in this image. If it's a coding problem:
- Provide the working solution in a code block
- No explanations unless essential

If it's not code:
- Give a brief, direct answer
- Be concise and to the point`;
        }
    }

    private detectImageFormat(imageData: Buffer): string {
        if (imageData.length < 8) {
            return 'image/png'; // Default fallback
        }

        // Check PNG signature
        if (imageData.subarray(0, 8).equals(Buffer.from([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]))) {
            return 'image/png';
        }

        // Check JPEG signature
        if (imageData.subarray(0, 3).equals(Buffer.from([0xFF, 0xD8, 0xFF]))) {
            return 'image/jpeg';
        }

        // Check WebP signature
        if (imageData.length >= 12 &&
            imageData.subarray(0, 4).equals(Buffer.from('RIFF')) &&
            imageData.subarray(8, 12).equals(Buffer.from('WEBP'))) {
            return 'image/webp';
        }

        // Default to PNG
        return 'image/png';
    }
}