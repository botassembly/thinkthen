//! An outside user of the `thinkthen` library.
//!
//! This crate builds `thinkthen` by path with default features off, as a
//! host does, and its tests drive only the public API. The command parser
//! and its modules do not compile in this graph.

pub use thinkthen::Engine;
