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

// These are the shared SQL result columns; question answers and details stay nested.
// Hosts validate additional input-column collisions when binding their physical schema.
pub(crate) fn validate_table(request: &thinkthen::Request) -> Result<(), Error> {
    if let thinkthen::Question::Annotate(question) = request.question() {
        question.validate_table_columns(
            &[
                "position",
                "id",
                "answer",
                "probability",
                "status",
                "error_code",
                "error_message",
                "error_http_status",
                "source",
                "detail",
            ],
            &[],
        )?;
    }
    Ok(())
}
