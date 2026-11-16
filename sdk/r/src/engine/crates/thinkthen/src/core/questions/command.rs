//! Command metadata belongs to the shared function registry.

pub(crate) enum ThresholdOption {
    None,
    Cut,
    Band,
}
pub(crate) struct CommandSpec {
    pub(crate) description: &'static str,
    pub(crate) threshold: ThresholdOption,
    pub(crate) check: bool,
    pub(crate) list: Option<(&'static str, &'static str, &'static str, &'static str)>,
}
