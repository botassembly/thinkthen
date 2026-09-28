//! The optional API key a command checks once and later sends unchanged.

use std::fmt;
use std::sync::{Arc, OnceLock};

use crate::core::{Backend, KEY_IN_ADDRESS, KEY_VAR};
use crate::engine::error::Error as EngineError;
use crate::engine::facade::Key;
use crate::failure::Failure;

pub(super) type Reader = Arc<dyn Fn() -> Result<Key, EngineError> + Send + Sync>;

#[derive(Clone, Default)]
pub(super) struct KeySnapshot(Arc<OnceLock<Option<String>>>);

impl KeySnapshot {
    pub(super) fn check(&self, backend: &Backend) -> Result<(), Failure> {
        let key = self.0.get_or_init(|| super::read(KEY_VAR));
        if backend.address_contains_key(key.as_deref()) {
            return Err(Failure::Usage(KEY_IN_ADDRESS));
        }
        Ok(())
    }

    pub(super) fn is_set(&self) -> bool {
        self.0.get().is_some_and(Option::is_some)
    }

    pub(super) fn reader(&self) -> Reader {
        let snapshot = self.clone();
        Arc::new(move || {
            snapshot
                .0
                .get()
                .and_then(Option::as_deref)
                .map(|value| Key::new(value.to_owned()))
                .ok_or(EngineError::NoKey(KEY_VAR))
        })
    }
}

impl fmt::Debug for KeySnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("KeySnapshot(<withheld>)")
    }
}
