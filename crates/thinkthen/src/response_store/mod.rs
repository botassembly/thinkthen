//! Caller-owned response storage; no transaction spans a backend request.
mod access;
mod config;
mod credentials;
mod waiting;
pub(crate) use credentials::contains as contains_credential;
mod key;
mod storage;
pub use config::{ResponseMode, ResponseStore};
pub(crate) use key::identity;
pub(crate) use storage::Store;

use crate::{Code, Error};

pub(crate) fn error(code: Code) -> Error {
    let message = match code {
        Code::LocalStoreCorrupt => "the response store is corrupt or truncated",
        Code::LocalStoreFormat => "the response store format is incompatible",
        Code::LocalStoreAnswer => "the response store contains an invalid answer",
        Code::LocalStoreQuota => {
            "the response store quota is exhausted; completed responses may not have been recorded"
        }
        Code::LocalReplayMiss => {
            "the recording snapshot has no answer for a required ask; no request was sent"
        }
        _ => {
            "the response store could not be accessed; completed responses may not have been recorded"
        }
    };
    Error::from_contract(code, message)
}
fn sql(error: rusqlite::Error) -> Error {
    use rusqlite::ErrorCode;
    if matches!(
        error.sqlite_error_code(),
        Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked)
    ) && waiting::cancelled()
    {
        return crate::retry::cancelled();
    }
    let code = match error.sqlite_error_code() {
        Some(ErrorCode::DatabaseCorrupt | ErrorCode::NotADatabase) => Code::LocalStoreCorrupt,
        Some(ErrorCode::DiskFull | ErrorCode::TooBig) => Code::LocalStoreQuota,
        _ if matches!(
            error,
            rusqlite::Error::InvalidColumnType(..)
                | rusqlite::Error::IntegralValueOutOfRange(..)
                | rusqlite::Error::QueryReturnedNoRows
        ) =>
        {
            Code::LocalStoreCorrupt
        }
        _ => Code::LocalResponseStore,
    };
    self::error(code)
}

/// Validate the effective route before credentials or persistence are touched.
pub(crate) fn validate_route(backend: &crate::backend::Backend) -> Result<(), Error> {
    let Some(base) = backend.base_url() else {
        return Err(Error::from_contract(
            Code::UsageSettings,
            "response_store requires a direct remote backend",
        ));
    };
    if base.contains(['?', '#']) {
        return Err(Error::from_contract(
            Code::UsageSettings,
            "response_store requires an endpoint without user information, query or fragment",
        ));
    }
    crate::backend::validate_url(&backend.url())?;
    Ok(())
}

pub(crate) fn metadata(
    backend: &crate::backend::Backend,
    config: &ResponseStore,
) -> Result<Vec<u8>, Error> {
    #[derive(serde::Serialize)]
    struct Metadata<'a> {
        format: i64,
        fingerprint: &'a str,
        backend: &'a str,
        endpoint: String,
        model: &'a str,
        namespace: &'a str,
        model_generation: &'a str,
    }
    serde_json::to_vec(&Metadata {
        format: key::FORMAT,
        fingerprint: key::FINGERPRINT,
        backend: backend.name(),
        endpoint: backend.url(),
        model: backend.model(),
        namespace: &config.namespace,
        model_generation: &config.generation,
    })
    .map_err(|_| error(Code::LocalResponseStore))
}
