//! Capture only the selected route's credential variables, once after host guards.
use std::fmt;
use std::sync::OnceLock;
use crate::ResolvedSettings;

#[derive(Default)]
pub(super) struct KeySnapshot(OnceLock<Option<String>>);
impl KeySnapshot {
    pub(super) fn capture(&self, route: &ResolvedSettings) {
        self.0.get_or_init(|| route.key_variables.iter().find_map(|name| super::read(name)));
    }
    pub(super) fn value(&self) -> Option<&str> {
        self.0.get().and_then(|value| value.as_deref())
    }
}
impl fmt::Debug for KeySnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("KeySnapshot(<withheld>)")
    }
}
