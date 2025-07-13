use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct AIClient {
    client: Client,
    pub provider: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChoice {
    pub message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeResponse {
    pub content: Vec<ClaudeContent>,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeContent {
    pub text: String,
}


impl AIClient {
    pub fn new(provider: &str, api_key: &str) -> Result<Self> {
        debug!("Initializing AIClient with provider: {}", provider);
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent("ai-screenshot-analyzer/1.0")
            .build()?;
        
        debug!("AIClient initialized successfully with timeout: 60s");
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
        debug!("Starting image analysis with provider: {}, image size: {} bytes", self.provider, image_data.len());
        debug!("Prompt length: {} characters", prompt.len());
        match self.provider.as_str() {
            "openai" => self.analyze_with_openai(image_data, prompt).await,
            "claude" => self.analyze_with_claude(image_data, prompt).await,
            "gemini" => self.analyze_with_gemini(image_data, prompt).await,
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", self.provider)),
        }
    }
    
    
    async fn analyze_with_openai(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        debug!("Starting OpenAI analysis with image size: {} bytes", image_data.len());
        println!("🔄 Sending request to OpenAI API...");
        // Encode image as base64 for OpenAI Vision API
        debug!("Encoding image to base64 for OpenAI...");
        let base64_image = STANDARD.encode(image_data);
        debug!("Base64 encoded image length: {} characters", base64_image.len());
        
        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;
        debug!("Detected MIME type for OpenAI: {}", mime_type);
        
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
        
        debug!("Sending POST request to OpenAI API...");
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        debug!("OpenAI API response status: {}", response.status());
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("OpenAI API error (status: {}): {}", status, error_text);
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        debug!("Parsing OpenAI response JSON...");
        let openai_response: OpenAIResponse = response.json().await?;
        debug!("OpenAI response parsed, choices count: {}", openai_response.choices.len());
        
        openai_response.choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))
    }
    
    async fn analyze_with_claude(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        debug!("Starting Claude analysis with image size: {} bytes", image_data.len());
        println!("🔄 Sending request to Claude API...");
        // Encode image as base64 for Claude
        debug!("Encoding image to base64 for Claude...");
        let base64_image = STANDARD.encode(image_data);
        debug!("Base64 encoded image length: {} characters", base64_image.len());
        
        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;
        debug!("Detected MIME type for Claude: {}", mime_type);
        
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
        
        debug!("Sending POST request to Claude API...");
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        debug!("Claude API response status: {}", response.status());
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("Claude API error (status: {}): {}", status, error_text);
            return Err(anyhow::anyhow!("Claude API error: {}", error_text));
        }
        
        debug!("Parsing Claude response JSON...");
        let claude_response: ClaudeResponse = response.json().await?;
        debug!("Claude response parsed, content count: {}", claude_response.content.len());
        
        claude_response.content
            .first()
            .map(|content| content.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Claude"))
    }
    
    async fn analyze_with_gemini(&self, image_data: &[u8], prompt: &str) -> Result<String> {
        debug!("Starting Gemini analysis with image size: {} bytes", image_data.len());
        println!("🔄 Sending request to Gemini API...");
        let base64_image = STANDARD.encode(image_data);
        
        // Detect image format for proper MIME type
        let mime_type = self.detect_image_format(image_data)?;
        debug!("Detected MIME type for Gemini: {}", mime_type);
        
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
        
        debug!("Sending POST request to Gemini API...");
        let response = self.client
            .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
        
        debug!("Gemini API response status: {}", response.status());
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("Gemini API error (status: {}): {}", status, error_text);
            return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
        }
        
        debug!("Parsing Gemini response JSON...");
        let response_json: serde_json::Value = response.json().await?;
        debug!("Gemini response parsed successfully");
        
        response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No response from Gemini"))
    }
    
    pub fn detect_image_format(&self, image_data: &[u8]) -> Result<&'static str> {
        debug!("Detecting image format for {} bytes of data", image_data.len());
        if image_data.len() < 8 {
            debug!("Image data too small ({} bytes), defaulting to PNG", image_data.len());
            return Ok("image/png");  // Default fallback
        }
        
        // Check PNG signature
        if image_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            debug!("Detected PNG format from signature");
            return Ok("image/png");
        }
        
        // Check JPEG signature
        if image_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            debug!("Detected JPEG format from signature");
            return Ok("image/jpeg");
        }
        
        // Check WebP signature
        if image_data.len() >= 12 
            && image_data.starts_with(b"RIFF") 
            && &image_data[8..12] == b"WEBP" {
            debug!("Detected WebP format from signature");
            return Ok("image/webp");
        }
        
        // Default to PNG
        debug!("No format signature detected, defaulting to PNG");
        Ok("image/png")
    }
}



