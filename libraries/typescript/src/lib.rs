//! The Node-API addon behind the `thinkthen` package for Node. `index.js`
//! shapes each call into one op, a question, and a payload. `door` answers it
//! through the public engine on a worker thread and writes one JSON
//! envelope. `node` holds every Node-API item and is compiled out of the
//! unit tests, so they run with no Node present.

#![cfg_attr(
    test,
    allow(
        dead_code,
        reason = "the unit tests compile out the Node-API module that calls the door"
    )
)]

use thinkthen_host::complete;
#[cfg(not(test))]
pub mod complete_node;
mod door;

#[cfg(not(test))]
pub mod node;

#[cfg(not(test))]
pub mod request_node;
