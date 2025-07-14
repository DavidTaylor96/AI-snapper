import axios, { AxiosInstance } from 'axios';

interface OpenAIResponse {
    choices: Array<{
        message: {
            content: string;
        };
    }>;
}

export class AIClient {
    private client: AxiosInstance;
    private apiKey: string;

    constructor(provider: string, apiKey: string) {
        this.apiKey = apiKey;
        this.client = axios.create({
            timeout: 60000,
            headers: {
                'User-Agent': 'ai-screenshot-analyzer-node/1.0'
            }
        });
    }

    provider(): string {
        return 'openai'; // Always return openai since we only support ChatGPT now
    }

    async analyzeImage(imageData: Buffer, userQuestion?: string): Promise<string> {
        return this.analyzeWithOpenAI(imageData, userQuestion);
    }

    private async analyzeWithOpenAI(imageData: Buffer, userQuestion?: string): Promise<string> {
        try {
            // Encode image as base64 for OpenAI Vision API
            const base64Image = imageData.toString('base64');

            // Detect image format for proper MIME type
            const mimeType = this.detectImageFormat(imageData);

            // Create the enhanced prompt
            const prompt = this.createEnhancedPrompt(userQuestion);

            const payload = {
                model: 'gpt-4o-mini',
                messages: [
                    {
                        role: 'system',
                        content: 'You are an expert programming assistant that analyzes screenshots. When you see a coding challenge or problem, provide a working solution. Always format code in proper markdown blocks. Be concise and focus on practical solutions.'
                    },
                    {
                        role: 'user',
                        content: [
                            {
                                type: 'text',
                                text: prompt
                            },
                            {
                                type: 'image_url',
                                image_url: {
                                    url: `data:${mimeType};base64,${base64Image}`,
                                    detail: 'high'
                                }
                            }
                        ]
                    }
                ],
                max_tokens: 1000, // Increased for better code explanations
                temperature: 0.1   // Keep deterministic for coding
            };

            const response = await this.client.post(
                'https://api.openai.com/v1/chat/completions',
                payload,
                {
                    headers: {
                        'Authorization': `Bearer ${this.apiKey}`,
                        'Content-Type': 'application/json'
                    }
                }
            );

            const openaiResponse: OpenAIResponse = response.data;

            const content = openaiResponse.choices[0]?.message?.content;
            if (!content) {
                throw new Error('No response from OpenAI');
            }

            // Format the response for better readability
            return this.formatResponse(content);
        } catch (error) {
            if (axios.isAxiosError(error)) {
                const errorMessage = error.response?.data?.error?.message || error.message;
                throw new Error(`OpenAI API error: ${errorMessage}`);
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
        formatted += '🤖 ChatGPT Analysis\n';
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