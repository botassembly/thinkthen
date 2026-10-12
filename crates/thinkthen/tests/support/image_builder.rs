//! Shared image engine settings keep route values with their owning callers.
use thinkthen::{Engine, EngineBuilder};

/// Share image route setup while callers retain their own key and controls.
#[cfg(test)]
pub(crate) fn builder(base: &str, backend: &str, model: &str, key: Option<&str>) -> EngineBuilder {
    let builder = Engine::builder()
        .backend(backend)
        .and_then(|builder| builder.base_url(base))
        .and_then(|builder| builder.model(model))
        .expect("valid saved fixture or loopback configuration");
    match key {
        Some(key) => builder
            .api_key(key)
            .expect("valid saved fixture or loopback configuration"),
        None => builder,
    }
}
