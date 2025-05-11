use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::config::Config;
use crate::provider::get_available_providers;

#[napi(js_name = "list_providers")]
pub fn list_providers() -> Vec<String> {
    let config = Config::from_env();
    get_available_providers(&config)
        .iter()
        .map(|p| p.name())
        .collect()
}

#[napi(js_name = "generate_response")]
pub async fn generate_response(prompt: String) -> Result<String> {
    let config = Config::from_env();
    let providers = get_available_providers(&config);

    if providers.is_empty() {
        return Err(Error::from_reason("No providers available"));
    }

    let result = providers[0].generate(&prompt).await;
    result.map_err(|e| Error::from_reason(e.to_string()))
}
