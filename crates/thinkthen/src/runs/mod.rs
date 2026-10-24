#![doc = include_str!("README.md")]

mod children;
mod entry;
mod pipeline;

pub use children::force_cleanup;
pub use entry::{RunEntry, load};
pub use pipeline::{RunCompletion, execute};
