//! SQL arguments admit the canonical per-function question exactly once.
use thinkthen::{Engine, Error};

pub use thinkthen::native::sql_request as request;

/// Admit database settings without allowing caller addresses or credentials.
/// # Errors
/// Returns shared settings or credential errors.
pub fn client(settings: &str, variable: impl Fn(&str) -> Option<String>) -> Result<Engine, Error> {
    thinkthen::native::sql_engine_unrestricted(settings, variable)
}

/// Admit the administrator's backend/model policy before resolving credentials.
/// # Errors
/// Returns shared settings, policy, selection or credential errors.
pub fn client_with_policy(
    settings: &str,
    allowed: Option<&str>,
    variable: impl Fn(&str) -> Option<String>,
) -> Result<Engine, Error> {
    thinkthen::native::sql_engine(settings, allowed, variable)
}
