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
            const prompt = this.createEnhancedPrompt(userQuestion);

            const response = await this.client.messages.create({
                model: 'claude-3-5-sonnet-20241022',
                max_tokens: 1000,
                temperature: 0.1,
                system: 'You are an expert programming assistant that analyzes screenshots. When you see a coding challenge or problem, provide a working solution. Always format code in proper markdown blocks. Be concise and focus on practical solutions.',
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

            // Format the response for better readability
            return this.formatResponse(content.text);
        } catch (error) {
            if (error instanceof Anthropic.APIError) {
                throw new Error(`Claude API error: ${error.message}`);
            }
            throw error;
        }
    }

    private createEnhancedPrompt(userQuestion?: string): string {
        const baseInstruction = 'Please view the screen and analyze what you see.';
        
        if (userQuestion && userQuestion.trim()) {
            return `${baseInstruction} Please answer the following question in the simplest way possible: ${userQuestion.trim()}

IMPORTANT: If your answer involves code, please format it in proper markdown code blocks with the appropriate language identifier. Provide clear, working code examples when applicable.`;
        } else {
            // Default prompt optimized for coding challenges and problems
            return `${baseInstruction} If this is a coding challenge or problem:
1. Briefly explain what the code/problem does
2. Provide a working solution in the same programming language
3. Format all code in proper markdown code blocks
4. Keep explanations concise and focused on the solution

If this is not a coding problem, describe what you see including any text, UI elements, or important information.`;
        }
    }

    private formatResponse(content: string): string {
        // Simplified formatting that's cleaner and easier to read
        let formatted = '';
        
        // Add a simple header
        formatted += '🤖 Claude Analysis\n';
        formatted += '─'.repeat(50) + '\n';
        formatted += '\n';
        
        // Process the content to highlight code blocks
        const lines = content.split('\n');
        let inCodeBlock = false;
        
        for (const line of lines) {
            if (line.trim().startsWith('```')) {
                if (!inCodeBlock) {
                    // Starting a code block - add visual separator
                    formatted += '\n┌─ CODE SOLUTION ';
                    if (line.length > 3) {
                        const lang = line.slice(3).trim().toUpperCase();
                        if (lang) {
                            formatted += `(${lang}) `;
                        }
                    }
                    formatted += '─'.repeat(20) + '\n';
                    formatted += line + '\n';
                    inCodeBlock = true;
                } else {
                    // Ending a code block
                    formatted += line + '\n';
                    formatted += '└' + '─'.repeat(45) + '\n';
                    inCodeBlock = false;
                }
            } else {
                formatted += line + '\n';
            }
        }
        
        formatted += '\n';
        formatted += '─'.repeat(50);
        
        return formatted;
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