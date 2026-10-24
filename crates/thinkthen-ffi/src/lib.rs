//! A counted C boundary around the shared client and pulled operation.
//!
//! The shipped header defines pointer lifetimes and the single-driver rule.

mod exports;
mod memory;
mod row;
mod sql;

pub use exports::*;
pub use memory::{Buffer, CallHandle, ClientHandle, HostRow};
pub use row::*;
pub use sql::*;
