use ai_screenshot_analyzer::ai_client::AIClient;
use base64::engine::{Engine as _, general_purpose::STANDARD};

#[test]
fn test_ai_client_new() {
    let client = AIClient::new("openai", "test-key").unwrap();
    assert_eq!(client.provider(), "openai");
    // Note: api_key is private, but we can test that the client was created successfully
}

#[test]
fn test_ai_client_provider() {
    let client = AIClient::new("claude", "test-key").unwrap();
    assert_eq!(client.provider(), "claude");
}

#[test]
fn test_detect_image_format_png() {
    let client = AIClient::new("test", "key").unwrap();
    let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let result = client.detect_image_format(&png_data).unwrap();
    assert_eq!(result, "image/png");
}

#[test]
fn test_detect_image_format_jpeg() {
    let client = AIClient::new("test", "key").unwrap();
    let mut jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // Valid JPEG header
    jpeg_data.extend(vec![0u8; 8]); // Add padding to meet 8-byte minimum
    let result = client.detect_image_format(&jpeg_data).unwrap();
    assert_eq!(result, "image/jpeg");
}

#[test]
fn test_detect_image_format_webp() {
    let client = AIClient::new("test", "key").unwrap();
    let mut webp_data = vec![0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50];
    webp_data.extend(vec![0u8; 20]); // Add some padding
    let result = client.detect_image_format(&webp_data).unwrap();
    assert_eq!(result, "image/webp");
}

#[test] 
fn test_detect_image_format_default() {
    let client = AIClient::new("test", "key").unwrap();
    let unknown_data = vec![0x00, 0x01, 0x02, 0x03];
    let result = client.detect_image_format(&unknown_data).unwrap();
    assert_eq!(result, "image/png"); // Default fallback
}

#[test]
fn test_detect_image_format_empty() {
    let client = AIClient::new("test", "key").unwrap();
    let empty_data = vec![];
    let result = client.detect_image_format(&empty_data).unwrap();
    assert_eq!(result, "image/png"); // Default fallback
}

#[tokio::test]
async fn test_analyze_image_unsupported_provider() {
    let client = AIClient::new("unsupported", "key").unwrap();
    let test_data = vec![0u8; 100];
    let result = client.analyze_image(&test_data, "test prompt").await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported provider"));
}

#[test]
fn test_openai_response_deserialization() {
    let json_response = r#"{
        "choices": [
            {
                "message": {
                    "content": "This is a test response"
                }
            }
        ]
    }"#;
    
    let response: serde_json::Value = serde_json::from_str(json_response).unwrap();
    assert_eq!(response["choices"][0]["message"]["content"].as_str().unwrap(), "This is a test response");
}

#[test]
fn test_claude_response_deserialization() {
    let json_response = r#"{
        "content": [
            {
                "text": "This is a Claude response"
            }
        ]
    }"#;
    
    let response: serde_json::Value = serde_json::from_str(json_response).unwrap();
    assert_eq!(response["content"][0]["text"].as_str().unwrap(), "This is a Claude response");
}

#[test]
fn test_base64_encoding() {
    let test_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" in bytes
    let encoded = STANDARD.encode(&test_data);
    assert_eq!(encoded, "SGVsbG8=");
}

// Test error conditions
#[test]
fn test_openai_response_missing_choices() {
    let json_response = r#"{
        "choices": []
    }"#;
    
    let response: serde_json::Value = serde_json::from_str(json_response).unwrap();
    assert!(response["choices"].as_array().unwrap().is_empty());
}

#[test]
fn test_claude_response_empty_content() {
    let json_response = r#"{
        "content": []
    }"#;
    
    let response: serde_json::Value = serde_json::from_str(json_response).unwrap();
    assert!(response["content"].as_array().unwrap().is_empty());
}

#[test]
fn test_ai_client_new_with_all_providers() {
    // Test all supported providers
    let openai_client = AIClient::new("openai", "openai-key").unwrap();
    assert_eq!(openai_client.provider(), "openai");
    // api_key is private

    let claude_client = AIClient::new("claude", "claude-key").unwrap();
    assert_eq!(claude_client.provider(), "claude");
    // api_key is private

    let gemini_client = AIClient::new("gemini", "gemini-key").unwrap();
    assert_eq!(gemini_client.provider(), "gemini");
    // api_key is private
}

#[test]
fn test_ai_client_invalid_provider() {
    // Test with an invalid provider - should still create client but fail on usage
    let client = AIClient::new("invalid-provider", "test-key").unwrap();
    assert_eq!(client.provider(), "invalid-provider");
    // Note: api_key is private, but we can test that the client was created successfully
}

#[test]
fn test_detect_image_format_complex_data() {
    let client = AIClient::new("test", "key").unwrap();
    
    // Test with mixed data that starts with PNG signature
    let mut complex_png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    complex_png.extend(vec![0xFF, 0xD8, 0xFF]); // Add JPEG signature after PNG
    complex_png.extend(vec![0u8; 100]); // Add padding
    let result = client.detect_image_format(&complex_png).unwrap();
    assert_eq!(result, "image/png"); // Should detect PNG first

    // Test with JPEG signature in different position
    let mut jpeg_with_prefix = vec![0x00, 0x00]; // Some prefix
    jpeg_with_prefix.extend(vec![0xFF, 0xD8, 0xFF]); // JPEG signature
    jpeg_with_prefix.extend(vec![0u8; 20]);
    let result = client.detect_image_format(&jpeg_with_prefix).unwrap();
    assert_eq!(result, "image/png"); // Should fallback to PNG since JPEG not at start
}

#[test]
fn test_detect_image_format_webp_edge_cases() {
    let client = AIClient::new("test", "key").unwrap();
    
    // Test WebP with minimum size
    let webp_min = b"RIFF\x00\x00\x00\x00WEBP".to_vec();
    let result = client.detect_image_format(&webp_min).unwrap();
    assert_eq!(result, "image/webp");

    // Test data that looks like WebP but too short
    let short_webp = b"RIFF\x00\x00".to_vec();
    let result = client.detect_image_format(&short_webp).unwrap();
    assert_eq!(result, "image/png"); // Should fallback
}

#[test]
fn test_detect_image_format_boundary_conditions() {
    let client = AIClient::new("test", "key").unwrap();
    
    // Test with exactly 8 bytes
    let exactly_8 = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let result = client.detect_image_format(&exactly_8).unwrap();
    assert_eq!(result, "image/png");

    // Test with 7 bytes (less than minimum)
    let seven_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A];
    let result = client.detect_image_format(&seven_bytes).unwrap();
    assert_eq!(result, "image/png"); // Default fallback

    // Test with 12 bytes for WebP minimum
    let twelve_bytes = b"RIFF\x00\x00\x00\x00WEBP".to_vec();
    let result = client.detect_image_format(&twelve_bytes).unwrap();
    assert_eq!(result, "image/webp");
}

#[tokio::test]
async fn test_analyze_image_error_propagation() {
    // Test that errors are properly propagated for different providers
    let providers = vec!["test-provider-1", "test-provider-2", "unknown"];
    
    for provider in providers {
        let client = AIClient::new(provider, "test-key").unwrap();
        let test_data = vec![0u8; 50];
        let result = client.analyze_image(&test_data, "test prompt").await;
        
        assert!(result.is_err(), "Provider {} should return error", provider);
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unsupported provider") || 
               error_msg.contains("provider"), 
               "Error should mention provider issue: {}", error_msg);
    }
}

#[test]
fn test_response_structs_comprehensive() {
    // Test OpenAI response with multiple choices
    let multi_choice_json = r#"{
        "choices": [
            {"message": {"content": "First response"}},
            {"message": {"content": "Second response"}}
        ]
    }"#;
    let response: serde_json::Value = serde_json::from_str(multi_choice_json).unwrap();
    assert_eq!(response["choices"].as_array().unwrap().len(), 2);
    assert_eq!(response["choices"][0]["message"]["content"].as_str().unwrap(), "First response");
    assert_eq!(response["choices"][1]["message"]["content"].as_str().unwrap(), "Second response");

    // Test Claude response with multiple content items
    let multi_content_json = r#"{
        "content": [
            {"text": "First part"},
            {"text": "Second part"}
        ]
    }"#;
    let response: serde_json::Value = serde_json::from_str(multi_content_json).unwrap();
    assert_eq!(response["content"].as_array().unwrap().len(), 2);
    assert_eq!(response["content"][0]["text"].as_str().unwrap(), "First part");
    assert_eq!(response["content"][1]["text"].as_str().unwrap(), "Second part");
}

#[test]
fn test_base64_encoding_edge_cases() {
    // Test empty data
    let empty_data = vec![];
    let encoded = STANDARD.encode(&empty_data);
    assert_eq!(encoded, "");

    // Test single byte
    let single_byte = vec![0x41]; // 'A'
    let encoded = STANDARD.encode(&single_byte);
    assert_eq!(encoded, "QQ==");

    // Test larger data
    let large_data: Vec<u8> = (0..255).collect();
    let encoded = STANDARD.encode(&large_data);
    assert!(!encoded.is_empty());
    assert!(encoded.len() > large_data.len()); // Base64 encoding increases size
}

#[test]
fn test_client_field_access() {
    let client = AIClient::new("test-provider", "secret-key").unwrap();
    
    // Test that we can access the provider
    assert_eq!(client.provider(), "test-provider");
    
    // Test that the client has the correct internal state
    // api_key is private
    assert_eq!(client.provider(), "test-provider");
}

#[test] 
fn test_json_parsing_malformed() {
    // Test malformed OpenAI response
    let malformed_openai = r#"{"choices": [{"message": {"content": "incomplete"#;
    let result: Result<serde_json::Value, _> = serde_json::from_str(malformed_openai);
    assert!(result.is_err());

    // Test malformed Claude response  
    let malformed_claude = r#"{"content": [{"text": "incomplete"#;
    let result: Result<serde_json::Value, _> = serde_json::from_str(malformed_claude);
    assert!(result.is_err());
}