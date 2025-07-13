use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AIClient {
    client: Client,
    provider: String,
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

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    #[allow(dead_code)]
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

impl AIClient {
    pub fn new(provider: &str, api_key: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent("ai-screenshot-analyzer/1.0")
            .build()?;

        Ok(Self {
            client,
            provider: provider.to_string(),
            api_key: api_key.to_string(),
        })
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub async fn analyze_image(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        match self.provider.as_str() {
            "openai" => self.analyze_with_openai(image_data, prompt).await,
            "claude" => self.analyze_with_claude(image_data, prompt).await,
            "gemini" => self.analyze_with_gemini(image_data, prompt).await,
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", self.provider)),
        }
    }

    async fn analyze_with_openai(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        // Encode image as base64 for OpenAI Vision API
        let base64_image = base64::prelude::BASE64_STANDARD.encode(image_data);

        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;

        let payload = json!({
            "model": "gpt-4o-mini",  // Use cheaper, faster model
            "messages": [
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
            "max_tokens": 500,  // Reduced for cost optimization
            "temperature": 0.1  // More deterministic for code
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

        openai_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))
    }

    async fn analyze_with_claude(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        // Encode image as base64 for Claude
        let base64_image = base64::prelude::BASE64_STANDARD.encode(image_data);

        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;

        let payload = json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 500,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "image",
                            "source": {
                                "type": "base64",
                                "media_type": mime_type,
                                "data": base64_image
                            }
                        },
                        {
                            "type": "text",
                            "text": prompt
                        }
                    ]
                }
            ]
        });

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Claude API error: {}", error_text));
        }

        let claude_response: ClaudeResponse = response.json().await?;

        claude_response
            .content
            .first()
            .map(|content| content.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Claude"))
    }

    async fn analyze_with_gemini(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        let base64_image = base64::prelude::BASE64_STANDARD.encode(image_data);

        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;

        let payload = json!({
            "contents": [{
                "parts": [
                    {"text": prompt},
                    {
                        "inline_data": {
                            "mime_type": mime_type,
                            "data": base64_image
                        }
                    }
                ]
            }],
            "generationConfig": {
                "maxOutputTokens": 500,
                "temperature": 0.1
            }
        });

        let response = self.client
            .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
        }

        let response_json: serde_json::Value = response.json().await?;

        response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No response from Gemini"))
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
