//! The optional API key a command checks once and later sends unchanged.

use std::fmt;
use std::sync::{Arc, OnceLock};

use crate::core::{Backend, KEY_IN_ADDRESS, KEY_VAR};
use crate::engine::error::Error as EngineError;
use crate::engine::facade::Key;
use crate::failure::Failure;

pub(super) type Reader = Arc<dyn Fn() -> Result<Key, EngineError> + Send + Sync>;

/// The key read from the selected backend's variables, the first variable,
/// which the missing-key sentence names, and the backend's name.
struct Snapshot {
    value: Option<String>,
    variable: String,
    name: Option<String>,
}

#[derive(Clone, Default)]
pub(super) struct KeySnapshot(Arc<OnceLock<Snapshot>>);

impl KeySnapshot {
    /// Read the first nonblank variable of `keys` once, then refuse a key the
    /// posting URL holds.
    pub(super) fn check(
        &self,
        backend: &Backend,
        keys: &[&str],
        name: Option<&str>,
    ) -> Result<(), Failure> {
        let snapshot = self.0.get_or_init(|| Snapshot {
            value: keys.iter().find_map(|key| super::read(key)),
            variable: (*keys.first().unwrap_or(&KEY_VAR)).to_owned(),
            name: name.map(str::to_owned),
        });
        if backend.address_contains_key(snapshot.value.as_deref()) {
            return Err(Failure::Usage(KEY_IN_ADDRESS));
        }
        Ok(())
    }

    pub(super) fn is_set(&self) -> bool {
        self.0
            .get()
            .is_some_and(|snapshot| snapshot.value.is_some())
    }

    /// The first key variable of the selected backend.
    pub(super) fn variable(&self) -> &str {
        self.0.get().map_or(KEY_VAR, |snapshot| &snapshot.variable)
    }

    /// The selected backend's name, or `None` on the unnamed path.
    pub(super) fn name(&self) -> Option<&str> {
        self.0.get().and_then(|snapshot| snapshot.name.as_deref())
    }

    pub(super) fn reader(&self) -> Reader {
        let snapshot = self.clone();
        Arc::new(move || {
            let taken = snapshot.0.get();
            taken
                .and_then(|taken| taken.value.as_deref())
                .map(|value| Key::new(value.to_owned()))
                .ok_or_else(|| {
                    EngineError::NoKey(taken.map_or(KEY_VAR, |taken| &taken.variable).to_owned())
                })
        })
    }
}

impl fmt::Debug for KeySnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("KeySnapshot(<withheld>)")
    }
}
