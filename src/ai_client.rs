use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AIClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

impl AIClient {
    pub fn new(_provider: &str, api_key: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent("ai-screenshot-analyzer/1.0")
            .build()?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
        })
    }

    pub fn provider(&self) -> &str {
        "openai" // Always return openai since we only support ChatGPT now
    }

    pub async fn analyze_image(&self, image_data: &[u8], user_question: Option<&str>) -> Result<String> {
        self.analyze_with_openai(image_data, user_question).await
    }

    async fn analyze_with_openai(&self, image_data: &[u8], user_question: Option<&str>) -> Result<String> {
        // Encode image as base64 for OpenAI Vision API
        let base64_image = base64::prelude::BASE64_STANDARD.encode(image_data);

        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;

        // Create the enhanced prompt
        let prompt = self.create_enhanced_prompt(user_question);

        let payload = json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": "You are an expert programming assistant that analyzes screenshots. When you see a coding challenge or problem, provide a working solution. Always format code in proper markdown blocks. Be concise and focus on practical solutions."
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": prompt
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": format!("data:{};base64,{}", mime_type, base64_image),
                                "detail": "high"
                            }
                        }
                    ]
                }
            ],
            "max_tokens": 1000, // Increased for better code explanations
            "temperature": 0.1   // Keep deterministic for coding
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let content = openai_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?;

        // Format the response for better readability
        Ok(self.format_response(&content))
    }

    fn create_enhanced_prompt(&self, user_question: Option<&str>) -> String {
        let base_instruction = "Please view the screen and analyze what you see.";
        
        match user_question {
            Some(question) if !question.trim().is_empty() => {
                format!(
                    "{} Please answer the following question in the simplest way possible: {}\n\n\
                    IMPORTANT: If your answer involves code, please format it in proper markdown code blocks \
                    with the appropriate language identifier. Provide clear, working code examples when applicable.",
                    base_instruction, question.trim()
                )
            }
            _ => {
                // Default prompt optimized for coding challenges and problems
                format!(
                    "{} If this is a coding challenge or problem:\n\
                    1. Briefly explain what the code/problem does\n\
                    2. Provide a working solution in the same programming language\n\
                    3. Format all code in proper markdown code blocks\n\
                    4. Keep explanations concise and focused on the solution\n\n\
                    If this is not a coding problem, describe what you see including any text, UI elements, or important information.",
                    base_instruction
                )
            }
        }
    }

    fn format_response(&self, content: &str) -> String {
        // Simplified formatting that's cleaner and easier to read
        let mut formatted = String::new();
        
        // Add a simple header
        formatted.push_str("🤖 ChatGPT Analysis\n");
        formatted.push_str("─".repeat(50).as_str());
        formatted.push('\n');
        formatted.push('\n');
        
        // Process the content to highlight code blocks
        let lines: Vec<&str> = content.lines().collect();
        let mut in_code_block = false;
        
        for line in lines {
            if line.trim().starts_with("```") {
                if !in_code_block {
                    // Starting a code block - add visual separator
                    formatted.push_str("\n┌─ CODE SOLUTION ");
                    if line.len() > 3 {
                        let lang = &line[3..].trim().to_uppercase();
                        if !lang.is_empty() {
                            formatted.push_str(&format!("({}) ", lang));
                        }
                    }
                    formatted.push_str("─".repeat(20).as_str());
                    formatted.push('\n');
                    formatted.push_str(line);
                    formatted.push('\n');
                    in_code_block = true;
                } else {
                    // Ending a code block
                    formatted.push_str(line);
                    formatted.push('\n');
                    formatted.push_str("└");
                    formatted.push_str("─".repeat(45).as_str());
                    formatted.push('\n');
                    in_code_block = false;
                }
            } else {
                formatted.push_str(line);
                formatted.push('\n');
            }
        }
        
        formatted.push('\n');
        formatted.push_str("─".repeat(50).as_str());
        
        formatted
    }

    pub fn detect_image_format(&self, image_data: &[u8]) -> Result<&'static str> {
        if image_data.len() < 8 {
            return Ok("image/png"); // Default fallback
        }

        // Check PNG signature
        if image_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Ok("image/png");
        }

        // Check JPEG signature
        if image_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Ok("image/jpeg");
        }

        // Check WebP signature
        if image_data.len() >= 12
            && image_data.starts_with(b"RIFF")
            && &image_data[8..12] == b"WEBP"
        {
            return Ok("image/webp");
        }

        // Default to PNG
        Ok("image/png")
    }
}