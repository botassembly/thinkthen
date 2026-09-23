//! The loopback backend the table probes share: it answers every request
//! at once and keeps each connection alive, so every settings state the
//! probes build holds a real idle socket in its pool, and a leaked pool
//! shows up as open descriptors. Each test binary uses a part of it.

#![allow(
    dead_code,
    reason = "each test binary that includes this module uses a different part of it"
)]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};

/// A decide reply in the vendor's wire shape.
const REPLY: &str = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}},"usage":{"input_tokens":10,"output_tokens":2}}"#;

/// Start the backend and return its base URL. Its threads live for the
/// rest of the test process.
pub fn answering_backend() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("the loopback binds");
    let port = listener.local_addr().expect("an address").port();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            std::thread::spawn(move || serve(stream));
        }
    });
    format!("http://127.0.0.1:{port}/v1")
}

/// Answer every request on one keep-alive connection until the client
/// closes it.
fn serve(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("the stream clones"));
    let mut writer = stream;
    while read_request(&mut reader).is_some() {
        let head = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n",
            REPLY.len()
        );
        if writer.write_all(head.as_bytes()).is_err() || writer.write_all(REPLY.as_bytes()).is_err()
        {
            return;
        }
    }
}

/// Read one request whole and return its body; `None` when the client
/// closed or the request broke off.
pub fn read_request(reader: &mut BufReader<TcpStream>) -> Option<Vec<u8>> {
    let mut length = 0;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            return None;
        }
        if line == "\r\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse::<usize>().unwrap_or(0);
        }
    }
    let mut body = vec![0_u8; length];
    reader.read_exact(&mut body).ok()?;
    Some(body)
}

/// How many descriptors this process holds open.
pub fn open_descriptors() -> usize {
    std::fs::read_dir("/proc/self/fd")
        .expect("the descriptor table reads")
        .count()
}

/// Wait up to two seconds for the open descriptors to fall to `bound`,
/// and return the last count: the backend's side of a closed connection
/// closes when its thread reads the end of the stream, a moment later.
pub fn settled_descriptors(bound: usize) -> usize {
    let started = Instant::now();
    loop {
        let open = open_descriptors();
        if open <= bound || started.elapsed() > Duration::from_secs(2) {
            return open;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Keep the null backend out of a probe: the probes exist to drive the
/// wire path and its settings table.
pub fn wire_only() {
    // Sound: each probe binary calls this first, before any thread starts.
    unsafe {
        std::env::remove_var("THINKTHEN_NULL");
        std::env::remove_var("ENGINE_NULL");
    }
}
