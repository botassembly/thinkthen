//! An offline loopback backend that every surface's tests can start.
//!
//! The listener serves scripted replies for the command's own tests. The
//! arms serve the shared cases, one generic rule, and the wire faults, chosen
//! by the URL path a caller gives as its base.

mod arms;
mod capture;
mod images;
mod lifetime;
mod listener;
mod observations;

pub use arms::{Backend, run};
pub use lifetime::Rendezvous;
pub use listener::{Canned, Listener, Observed, Recorded};
pub use observations::Paths;
