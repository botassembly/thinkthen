#![doc = include_str!("README.md")]

mod children;
mod entry;
mod pipeline;

pub(crate) use children::force_cleanup;
pub(crate) use entry::{RunEntry, load, names_entry};
pub(crate) use pipeline::{RunCompletion, execute};
