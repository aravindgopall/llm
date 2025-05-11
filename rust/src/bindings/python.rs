use pyo3::prelude::*;
use pyo3::types::PyModule;

use crate::config::Config;
use crate::provider::get_available_providers;

#[pyfunction]
fn generate_response(prompt: String) -> PyResult<String> {
    let config = Config::from_env();
    let providers = get_available_providers(&config);

    if providers.is_empty() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            "No providers available",
        ));
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(providers[0].generate(&prompt));

    result.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyfunction]
fn list_providers() -> Vec<String> {
    let config = Config::from_env();
    get_available_providers(&config)
        .iter()
        .map(|p| p.name()) // You should implement `fn name(&self) -> String` on each provider.
        .collect()
}
#[pymodule]
fn llm(_py: Python<'_>, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_response, m.clone())?)?;
    m.add_function(wrap_pyfunction!(list_providers, m.clone())?)?;
    Ok(())
}
