#![cfg(test)]
//! Fixture recording stays bounded and dropping it closes active peers.
use loopback::{Loopback, StressFixture};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[test]
fn dropping_fixture_joins_a_worker_blocked_on_an_incomplete_request() {
    let server = Loopback::noul(0.9).unwrap();
    server.keep_alive();
    let address = server
        .base()
        .trim_start_matches("http://")
        .trim_end_matches("/v1");
    let mut peer = TcpStream::connect(address).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    peer.write_all(b"POST /v1/systemone HTTP/1.1\r\nContent-Length: 16\r\n\r\n{\"questions\":{}}")
        .unwrap();
    let mut reader = BufReader::new(peer.try_clone().unwrap());
    let mut length = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.strip_prefix("content-length: ") {
            length = value.trim().parse().unwrap();
        }
    }
    reader.read_exact(&mut vec![0; length]).unwrap();
    assert_eq!(server.count(), 1);
    peer.write_all(b"POST /v1/systemone HTTP/1.1\r\nContent-Length: 100\r\n\r\n{")
        .unwrap();
    drop(server);
    let result = peer.read(&mut [0]);
    assert!(
        matches!(result, Ok(0))
            || result.is_err_and(|error| error.kind() == std::io::ErrorKind::ConnectionReset)
    );
}
#[test]
fn recording_is_bounded_while_total_request_count_keeps_increasing() {
    let fixture = StressFixture::open("healthy").unwrap();
    let address = fixture
        .base()
        .trim_start_matches("http://")
        .trim_end_matches("/v1");
    for _ in 0..32 {
        let mut peer = TcpStream::connect(address).unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        peer.write_all(
            b"POST /v1/systemone HTTP/1.1\r\nContent-Length: 16\r\n\r\n{\"questions\":{}}",
        )
        .unwrap();
        let mut response = Vec::new();
        peer.read_to_end(&mut response).unwrap();
        assert!(response.starts_with(b"HTTP/1.1 200"));
    }
    assert_eq!(fixture.stats()["requests"], 32);
    assert_eq!(fixture.stats()["recorded_requests"], 8);
    assert_eq!(fixture.stats()["connections"], 32);
}

#[test]
fn stress_process_exposes_faults_statistics_and_owned_shutdown() {
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_loopback"))
        .args(["--stress", "rate-limit"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut control = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap());
    let mut port = String::new();
    output.read_line(&mut port).unwrap();
    let mut peer = TcpStream::connect(format!("127.0.0.1:{}", port.trim())).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    peer.write_all(b"POST /v1/systemone HTTP/1.1\r\nContent-Length: 16\r\n\r\n{\"questions\":{}}")
        .unwrap();
    let mut response = String::new();
    peer.read_to_string(&mut response).unwrap();
    assert!(response.starts_with("HTTP/1.1 429"));
    assert!(response.contains("\r\nretry-after: 0\r\n"));
    writeln!(control, "count").unwrap();
    control.flush().unwrap();
    let mut count = String::new();
    output.read_line(&mut count).unwrap();
    assert_eq!(count.trim(), "1");
    writeln!(control, "stats").unwrap();
    control.flush().unwrap();
    let mut stats = String::new();
    output.read_line(&mut stats).unwrap();
    let stats: serde_json::Value = serde_json::from_str(&stats).unwrap();
    assert_eq!(stats["requests"], 1);
    assert_eq!(stats["recorded_requests"], 1);
    writeln!(control, "shutdown").unwrap();
    control.flush().unwrap();
    assert!(child.wait().unwrap().success());
}
