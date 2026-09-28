//! Read-only comparison of a selected cache with its validated backend marker.

use std::path::Path;

use serde::Serialize;

use crate::core::Backend;
use crate::engine::error::Error;
use crate::engine::recorder;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Binding {
    Disabled,
    Unavailable,
    Missing,
    Unbound,
    Legacy,
    Matching,
    Mismatched,
}

impl Binding {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Unavailable => "unavailable",
            Self::Missing => "missing",
            Self::Unbound => "unbound",
            Self::Legacy => "legacy",
            Self::Matching => "matching",
            Self::Mismatched => "mismatched",
        }
    }
}

pub(crate) fn inspect(
    folder: &Path,
    selected: Option<&Backend>,
    has_final_entry: bool,
) -> Result<Binding, Error> {
    let Some(backend) = selected else {
        return Ok(Binding::Disabled);
    };
    Ok(match recorder::binding_matches(folder, backend)? {
        Some(true) => Binding::Matching,
        Some(false) => Binding::Mismatched,
        None if has_final_entry => Binding::Legacy,
        None => Binding::Unbound,
    })
}
