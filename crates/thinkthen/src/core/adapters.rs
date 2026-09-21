//! The adapters compiled into this version, and the one every run uses.
//!
//! An adapter owns its name, its default address, its default model, and its
//! endpoint path. Nothing outside an adapter's own module names a vendor, so a
//! backend is added by writing one module here and changing no constant
//! elsewhere. ADR 0010's clarification of 2026-09-19 rules that other backends
//! will come, and `sdlc/scripts/policy.py` holds the seam.

pub(crate) mod systemone;

/// The adapter every run uses, under the name the rest of the crate calls it by.
///
/// Every caller asks this alias for the adapter's values rather than naming a
/// vendor, so the choice of adapter lives on this line alone. ADR 0017 holds
/// the trait and the option that would choose between adapters until a second
/// one exists, because a trait with one implementation is a guess.
pub(crate) use crate::core::adapters::systemone as built_in;
