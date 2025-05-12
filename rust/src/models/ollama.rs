use crate::models::model::{LLMProvider, Message, flatten_messages};
use serde::{Deserialize, Serialize};

pub struct OllamaProvider {
    pub model: String,
}

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait::async_trait]
impl LLMProvider for OllamaProvider {
    async fn chat(
        &self,
        messages: &[Message],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = flatten_messages(messages);
        let client = reqwest::Client::new();
        let body = OllamaRequest {
            model: &self.model,
            prompt: &prompt,
        };

        let res = client
            .post("http://localhost:11434/api/generate")
            .json(&body)
            .send()
            .await?;

        let parsed: OllamaResponse = res.json().await?;
        Ok(parsed.response)
    }
    fn name(&self) -> String {
        "ollama".to_string()
    }
}
