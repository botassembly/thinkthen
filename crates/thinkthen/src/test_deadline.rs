//! Deadlines for test subprocesses, on both sides of the pipe.

pub(crate) mod child;
#[cfg(test)]
mod child_tests;
pub(crate) mod isolated;
use child::{run, wait};

pub(crate) use run::output;
pub(crate) use wait::finish;

/// Read the whole `{}` request before replying. A socket closed with unread
/// bytes resets the connection, and on Windows the reset can reach the client
/// before the reply does.
pub(crate) fn whole_request(stream: &mut std::net::TcpStream) {
    let (mut chunk, mut whole) = ([0_u8; 1024], Vec::new());
    while !whole.ends_with(b"\r\n\r\n{}") {
        let read = std::io::Read::read(stream, &mut chunk).expect("request bytes");
        assert!(read > 0, "the whole request arrives");
        whole.extend_from_slice(&chunk[..read]);
    }
}
