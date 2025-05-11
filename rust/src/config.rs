use dotenv;

pub struct Config {
    pub openai: Option<OpenAIConfig>,
    pub azure: Option<AzureConfig>,
    pub gemini: Option<GeminiConfig>,
    pub ollama: Option<OllamaConfig>,
}

pub struct OpenAIConfig {
    pub api_key: String,
}

pub struct AzureConfig {
    pub api_key: String,
    pub endpoint: String,
    pub deployment_id: String,
}

pub struct GeminiConfig {
    pub api_key: String,
}

pub struct OllamaConfig {
    pub model: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            openai: env::var("OPENAI_API_KEY")
                .ok()
                .map(|api_key| OpenAIConfig { api_key }),
            azure: match (
                env::var("AZURE_OPENAI_KEY"),
                env::var("AZURE_OPENAI_ENDPOINT"),
                env::var("AZURE_DEPLOYMENT_ID"),
            ) {
                (Ok(api_key), Ok(endpoint), Ok(deployment_id)) => Some(AzureConfig {
                    api_key,
                    endpoint,
                    deployment_id,
                }),
                _ => None,
            },
            gemini: env::var("GEMINI_API_KEY")
                .ok()
                .map(|api_key| GeminiConfig { api_key }),
            ollama: env::var("OLLAMA_MODEL")
                .ok()
                .map(|model| OllamaConfig { model }),
        }
    }
}
