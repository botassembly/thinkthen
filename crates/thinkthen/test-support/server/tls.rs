//! A peer that sends HTTP bytes during a TLS handshake.
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Start a loopback peer that answers a TLS hello with invalid TLS records.
/// # Errors
/// Returns listener setup errors; the joined peer returns exchange errors.
pub fn invalid_tls_reply() -> std::io::Result<(String, JoinHandle<std::io::Result<()>>)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let url = format!("https://{}/v1", listener.local_addr()?);
    let peer = thread::spawn(move || {
        let (mut stream, _) = listener.accept()?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        if stream.read(&mut [0; 4096])? == 0 {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        stream.write_all(format!("HTTP/1.1 500 {}\r\n\r\n", super::KEY).as_bytes())
    });
    Ok((url, peer))
}
