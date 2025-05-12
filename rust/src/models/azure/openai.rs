use crate::models::model::{LLMProvider, Message};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct AzureOpenAIProvider {
    pub api_key: String,
    pub endpoint: String,
    pub deployment_id: String,
}

#[derive(Serialize)]
struct AzureRequest<'a> {
    messages: Vec<AzureMessage<'a>>,
}

#[derive(Serialize)]
struct AzureMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct AzureResponse {
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
impl LLMProvider for AzureOpenAIProvider {
    async fn chat(
        &self,
        messages: &[Message],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let azure_messages = messages
            .iter()
            .map(|m| AzureMessage {
                role: m.role.as_str(),
                content: &m.content,
            })
            .collect();

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2023-03-15-preview",
            self.endpoint, self.deployment_id
        );

        let res = client
            .post(&url)
            .header("api-key", &self.api_key)
            .json(&AzureRequest {
                messages: azure_messages,
            })
            .send()
            .await?;

        let parsed: AzureResponse = res.json().await?;
        Ok(parsed
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    fn name(&self) -> String {
        "azure-openai".into()
    }
}
