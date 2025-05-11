use rust::{
    config::{AzureConfig, Config, GeminiConfig, OllamaConfig, OpenAIConfig},
    get_available_providers,
};

fn dummy_config_all() -> Config {
    Config {
        openai: Some(OpenAIConfig {
            api_key: "test_openai".into(),
        }),
        azure: Some(AzureConfig {
            api_key: "test_azure".into(),
            endpoint: "https://azure.example.com".into(),
            deployment_id: "deployment-x".into(),
        }),
        gemini: Some(GeminiConfig {
            api_key: "test_gemini".into(),
        }),
        ollama: Some(OllamaConfig {
            model: "llama3".into(),
        }),
    }
}

fn dummy_config_partial() -> Config {
    Config {
        openai: Some(OpenAIConfig {
            api_key: "test_openai".into(),
        }),
        azure: None,
        gemini: None,
        ollama: Some(OllamaConfig {
            model: "llama3".into(),
        }),
    }
}

#[test]
fn test_all_providers_present() {
    let config = dummy_config_all();
    let providers = get_available_providers(&config);
    assert_eq!(providers.len(), 4, "Expected 4 providers to be initialized");
}

#[test]
fn test_partial_providers_present() {
    let config = dummy_config_partial();
    let providers = get_available_providers(&config);
    assert_eq!(providers.len(), 2, "Expected 2 providers to be initialized");
}

#[test]
fn test_no_providers_present() {
    let config = Config {
        openai: None,
        azure: None,
        gemini: None,
        ollama: None,
    };

    let providers = get_available_providers(&config);
    assert!(
        providers.is_empty(),
        "Expected no providers to be initialized"
    );
}
