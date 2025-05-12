use crate::models::model::{LLMProvider, Message};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAIProvider {
    pub api_key: String,
}

#[derive(Serialize)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<OpenAIMessage<'a>>,
}

#[derive(Serialize)]
struct OpenAIMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: String,
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn chat(
        &self,
        messages: &[Message],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let body = OpenAIRequest {
            model: "gpt-4",
            messages: messages
                .iter()
                .map(|m| OpenAIMessage {
                    role: m.role.as_str(),
                    content: &m.content,
                })
                .collect(),
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
        "openai".into()
    }
}
