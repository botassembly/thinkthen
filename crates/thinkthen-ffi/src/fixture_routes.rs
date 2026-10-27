//! Explicit test builds can route an owned process to owned loopback servers.
use std::sync::{Mutex, PoisonError};
use thinkthen::Error;
static ROUTES: Mutex<Option<(String, String)>> = Mutex::new(None);
/// Own the test process's backend routes until this guard is dropped.
#[non_exhaustive]
#[derive(Debug)]
pub(crate) struct RouteGuard {
    _private: (),
}
impl RouteGuard {
    /// Install both loopback routes without changing backend, model or key selection.
    /// # Errors
    /// Returns a usage error for a non-loopback route or an existing owner.
    pub(crate) fn install(systemone: &str, openai: &str) -> Result<Self, Error> {
        for url in [systemone, openai] {
            thinkthen::native::validate_url(url)?;
            let authority = url
                .split("://")
                .nth(1)
                .unwrap_or_default()
                .split('/')
                .next()
                .unwrap_or_default();
            let host = authority
                .strip_prefix("[::1]")
                .map(|_| "[::1]")
                .unwrap_or_else(|| authority.split(':').next().unwrap_or_default());
            if !matches!(host, "localhost" | "127.0.0.1" | "[::1]") {
                return Err(serde_json::from_value(serde_json::json!({
                    "code": thinkthen::Code::UsageUrl,
                    "message": "fixture routes must be loopback addresses"
                }))
                .map_err(|_| thinkthen::native::internal_error())?);
            }
        }
        let mut routes = ROUTES.lock().unwrap_or_else(PoisonError::into_inner);
        if routes.is_some() {
            return Err(thinkthen::native::settings_conversion_error());
        }
        *routes = Some((systemone.to_owned(), openai.to_owned()));
        Ok(Self { _private: () })
    }
}
impl Drop for RouteGuard {
    fn drop(&mut self) {
        *ROUTES.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }
}
pub(crate) fn route(name: &str) -> Option<String> {
    let routes = ROUTES.lock().unwrap_or_else(PoisonError::into_inner);
    let (systemone, openai) = routes.as_ref()?;
    match name.parse().ok()? {
        thinkthen::BackendName::SystemOne => Some(systemone.clone()),
        thinkthen::BackendName::OpenAi => Some(openai.clone()),
        _ => None,
    }
}
