use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum Role {
    System,
    User,
    Assistant,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(
        &self,
        messages: &[Message],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    fn name(&self) -> String;
}

pub fn flatten_messages(messages: &[Message]) -> String {
    messages
        .iter()
        .map(|m| format!("[{}] {}", m.role.as_str(), m.content))
        .collect::<Vec<_>>()
        .join("\n")
}
