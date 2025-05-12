use crate::models::model::{LLMProvider, Message, flatten_messages};
use serde::{Deserialize, Serialize};

pub struct GeminiProvider {
    pub api_key: String,
}

#[derive(Serialize)]
struct GeminiRequest<'a> {
    contents: Vec<GeminiMessage<'a>>,
}

#[derive(Serialize)]
struct GeminiMessage<'a> {
    parts: Vec<GeminiPart<'a>>,
}

#[derive(Serialize)]
struct GeminiPart<'a> {
    text: &'a str,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPartResp>,
}

#[derive(Deserialize)]
struct GeminiPartResp {
    text: String,
}

#[async_trait::async_trait]
impl LLMProvider for GeminiProvider {
    async fn chat(
        &self,
        messages: &[Message],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = flatten_messages(messages);
        let client = reqwest::Client::new();
        let body = GeminiRequest {
            contents: vec![GeminiMessage {
                parts: vec![GeminiPart { text: &prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
            self.api_key
        );

        let res = client.post(&url).json(&body).send().await?;
        let parsed: GeminiResponse = res.json().await?;

        Ok(parsed
            .candidates
            .get(0)
            .and_then(|c| c.content.parts.get(0))
            .map(|p| p.text.clone())
            .unwrap_or_default())
    }
    fn name(&self) -> String {
        "gemini".to_string()
    }
}
