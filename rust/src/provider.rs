use crate::config::*;
use crate::models::model::LLMProvider;

use crate::models::azure::openai::AzureOpenAIProvider;
use crate::models::google::gemini::GeminiProvider;
use crate::models::ollama::OllamaProvider;
use crate::models::openai::OpenAIProvider;

pub fn get_available_providers(config: &Config) -> Vec<Box<dyn LLMProvider>> {
    let mut providers: Vec<Box<dyn LLMProvider>> = vec![];

    if let Some(cfg) = &config.openai {
        providers.push(Box::new(OpenAIProvider {
            api_key: cfg.api_key.clone(),
        }));
    }

    if let Some(cfg) = &config.azure {
        providers.push(Box::new(AzureOpenAIProvider {
            api_key: cfg.api_key.clone(),
            endpoint: cfg.endpoint.clone(),
            deployment_id: cfg.deployment_id.clone(),
        }));
    }

    if let Some(cfg) = &config.gemini {
        providers.push(Box::new(GeminiProvider {
            api_key: cfg.api_key.clone(),
        }));
    }

    if let Some(cfg) = &config.ollama {
        providers.push(Box::new(OllamaProvider {
            model: cfg.model.clone(),
        }));
    }

    providers
}
