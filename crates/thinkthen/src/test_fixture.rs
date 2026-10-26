//! Explicit test builds can route an owned process to owned loopback servers.
use crate::{Code, Error};
use std::sync::{Mutex, PoisonError};
static ROUTES: Mutex<Option<(String, String)>> = Mutex::new(None);
/// Own the test process's backend routes until this guard is dropped.
#[non_exhaustive]
#[derive(Debug)]
pub struct RouteGuard {
    _private: (),
}
impl RouteGuard {
    /// Install both loopback routes without changing backend, model or key selection.
    /// # Errors
    /// Returns a usage error for a non-loopback route or an existing owner.
    pub fn install(systemone: &str, openai: &str) -> Result<Self, Error> {
        for url in [systemone, openai] {
            let uri = crate::backend::validate_url(url)?;
            if !matches!(uri.host(), Some("localhost" | "127.0.0.1" | "[::1]")) {
                return Err(Error::from_contract(
                    Code::UsageUrl,
                    "fixture routes must be loopback addresses",
                ));
            }
        }
        let mut routes = ROUTES.lock().unwrap_or_else(PoisonError::into_inner);
        if routes.is_some() {
            return Err(Error::from_contract(
                Code::UsageSettings,
                "the process already has a fixture route owner",
            ));
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
        crate::BackendName::SystemOne => Some(systemone.clone()),
        crate::BackendName::OpenAi => Some(openai.clone()),
        crate::BackendName::Stub => None,
    }
}
