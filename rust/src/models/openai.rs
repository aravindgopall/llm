use crate::models::model::LLMProvider;
use serde::{Deserialize, Serialize};

pub struct OpenAIProvider {
    pub api_key: String,
}

#[derive(Serialize)]
pub struct OpenAIRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
pub struct Message<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: MessageResp,
}

#[derive(Deserialize)]
pub struct MessageResp {
    pub content: String,
}

#[async_trait::async_trait]
impl LLMProvider for OpenAIProvider {
    async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        let body = OpenAIRequest {
            model: "gpt-4",
            messages: vec![Message {
                role: "user",
                content: prompt,
            }],
        };

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let parsed: OpenAIResponse = res.json().await?;
        Ok(parsed
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }
    fn name(&self) -> String {
        "openai".to_string()
    }
}
