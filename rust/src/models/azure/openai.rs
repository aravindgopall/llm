use crate::models::model::LLMProvider;
use crate::models::openai::{Message, OpenAIRequest, OpenAIResponse};

pub struct AzureOpenAIProvider {
    pub api_key: String,
    pub endpoint: String,
    pub deployment_id: String,
}

#[async_trait::async_trait]
impl LLMProvider for AzureOpenAIProvider {
    async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2023-03-15-preview",
            self.endpoint, self.deployment_id
        );

        let client = reqwest::Client::new();
        let body = OpenAIRequest {
            model: "gpt-4", // model field may be ignored by Azure
            messages: vec![Message {
                role: "user",
                content: prompt,
            }],
        };

        let res = client
            .post(&url)
            .header("api-key", &self.api_key)
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
        "azure_openai".to_string()
    }
}
