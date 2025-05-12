use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::config::Config;
use crate::models::model::{Role,Message};
use crate::provider::get_available_providers;

#[napi(js_name = "listProviders")]
pub fn list_providers() -> Vec<String> {
    let config = Config::from_env();
    get_available_providers(&config)
        .iter()
        .map(|p| p.name())
        .collect()
}

#[napi(object)]
pub struct MessageInput {
    pub role: String,
    pub content: String,
}

impl From<MessageInput> for Message {
    fn from(input: MessageInput) -> Self {
        let role = match input.role.as_str() {
            "system" => Role::System,
            "assistant" => Role::Assistant,
            _ => Role::User,
        };

        Message {
            role,
            content: input.content,
        }
    }
}

#[napi(js_name = "chat")]
pub async fn chat(messages: Vec<MessageInput>) -> Result<String> {
    let config = Config::from_env();
    let providers = get_available_providers(&config);

    if providers.is_empty() {
        return Err(Error::from_reason("No providers available"));
    }

    let input_messages: Vec<Message> = messages.into_iter().map(Message::from).collect();
    let response = providers[0].chat(&input_messages).await;

    response.map_err(|e| Error::from_reason(e.to_string()))
}
