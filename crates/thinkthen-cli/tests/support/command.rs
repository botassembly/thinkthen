//! Select the supplied installed command or this package's compiled binary.
use std::path::PathBuf;

pub(crate) fn binary() -> PathBuf {
    std::env::var_os("THINKTHEN_COMMAND")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_thinkthen")))
}
