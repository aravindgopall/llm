use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::exceptions::PyValueError;
use pyo3_asyncio;

use crate::config::Config;
use crate::provider::get_available_providers;
use crate::models::model::{Message, Role};

#[pyfunction]
pub fn list_providers() -> Vec<String> {
    let config = Config::from_env();
    get_available_providers(&config)
        .iter()
        .map(|p| p.name())
        .collect()
}

#[pyfunction]
pub fn chat(py: Python, messages: Vec<Py<PyDict>>) -> PyResult<String> {
    let mut message_vec = Vec::new();

    for obj in messages {
        let dict = obj.as_ref(py);
        let role: &str = dict
            .get_item("role")?
            .ok_or_else(|| PyValueError::new_err("Missing 'role' field"))?
            .extract()?;
        let content: &str = dict
            .get_item("content")?
            .ok_or_else(|| PyValueError::new_err("Missing 'content' field"))?
            .extract()?;

        message_vec.push(Message {
            role: match role {
                "system" => Role::System,
                "assistant" => Role::Assistant,
                _ => Role::User,
            },
            content: content.to_string(),
        });
    }

    let config = Config::from_env();
    let providers = get_available_providers(&config);

    if providers.is_empty() {
        return Err(PyValueError::new_err("No providers available"));
    }

    let result = pyo3_asyncio::tokio::get_runtime()
        .block_on(providers[0].chat(&message_vec))
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(result)
}

#[pymodule]
fn llm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_providers, m)?)?;
    m.add_function(wrap_pyfunction!(chat, m)?)?;
    Ok(())
}

