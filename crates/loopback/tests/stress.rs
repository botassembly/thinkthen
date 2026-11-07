#![cfg(test)]
//! Fixture recording stays bounded and dropping it closes active peers.
use loopback::{Loopback, StressFixture, stress_table};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

const REQUEST: &[u8] =
    b"POST /v1/systemone HTTP/1.1\r\nContent-Length: 16\r\n\r\n{\"questions\":{}}";

/// Read one reply's headers and body, leaving a kept connection open.
fn read_reply(reader: &mut impl BufRead) -> String {
    let mut reply = String::new();
    let mut length = 0;
    loop {
        let start = reply.len();
        assert_ne!(
            reader.read_line(&mut reply).unwrap(),
            0,
            "connection ended mid-reply"
        );
        let line = &reply[start..];
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.strip_prefix("content-length: ") {
            length = value.trim().parse().unwrap();
        }
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body).unwrap();
    reply + &String::from_utf8(body).unwrap()
}

fn connect(fixture: &StressFixture) -> TcpStream {
    let address = fixture
        .base()
        .trim_start_matches("http://")
        .trim_end_matches("/v1");
    let peer = TcpStream::connect(address).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(15)))
        .unwrap();
    peer
}

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
    peer.write_all(REQUEST).unwrap();
    read_reply(&mut BufReader::new(peer.try_clone().unwrap()));
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
    for _ in 0..32 {
        let mut peer = connect(&fixture);
        peer.write_all(REQUEST).unwrap();
        let response = read_reply(&mut BufReader::new(peer));
        assert!(response.starts_with("HTTP/1.1 200"));
    }
    assert_eq!(fixture.stats()["requests"], 32);
    assert_eq!(fixture.stats()["recorded_requests"], 8);
    assert_eq!(fixture.stats()["connections"], 32);
}

#[test]
fn answering_scenarios_serve_each_request_on_one_kept_connection() {
    for (scenario, first_status) in [
        ("healthy", "200"),
        ("rate-limit", "429"),
        ("server-burst", "503"),
        ("slow", "200"),
    ] {
        let fixture = StressFixture::open(scenario).unwrap();
        let mut peer = connect(&fixture);
        let mut reader = BufReader::new(peer.try_clone().unwrap());
        peer.write_all(REQUEST).unwrap();
        let first = read_reply(&mut reader);
        assert!(
            first.starts_with(&format!("HTTP/1.1 {first_status}")),
            "{scenario}: {first}"
        );
        peer.write_all(REQUEST).unwrap();
        read_reply(&mut reader);
        assert_eq!(fixture.stats()["requests"], 2, "{scenario}");
        assert_eq!(fixture.stats()["connections"], 1, "{scenario}");
    }
}

#[test]
fn slow_reply_soak_delays_every_reply_of_all_its_rounds() {
    let table = stress_table().unwrap();
    let case = &table["slow_replies"];
    let fault = &case["faults"][0];
    let requests = case["records"]
        .as_u64()
        .unwrap()
        .div_ceil(table["client"]["batch"].as_u64().unwrap())
        * table["records"]["soak_rounds"].as_u64().unwrap();
    assert!(fault["repeat"].as_u64().unwrap() >= requests);
    let delay = Duration::from_millis(fault["delay_ms"].as_u64().unwrap());
    let fixture = StressFixture::open(case["name"].as_str().unwrap()).unwrap();
    let mut peer = connect(&fixture);
    let mut reader = BufReader::new(peer.try_clone().unwrap());
    for _ in 0..2 {
        let sent = Instant::now();
        peer.write_all(REQUEST).unwrap();
        let reply = read_reply(&mut reader);
        assert!(reply.starts_with("HTTP/1.1 200"), "{reply}");
        assert!(sent.elapsed() >= delay);
    }
    assert_eq!(fixture.stats()["connections"], 1);
}

#[test]
fn reply_ending_faults_close_their_connection() {
    for (scenario, reply) in [("closed", ""), ("truncated", "{")] {
        let fixture = StressFixture::open(scenario).unwrap();
        let mut peer = connect(&fixture);
        peer.write_all(REQUEST).unwrap();
        let mut response = String::new();
        peer.read_to_string(&mut response).unwrap();
        assert!(response.ends_with(reply), "{scenario}: {response}");
        assert_eq!(response.is_empty(), reply.is_empty(), "{scenario}");
    }
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
    peer.set_read_timeout(Some(Duration::from_secs(15)))
        .unwrap();
    peer.write_all(REQUEST).unwrap();
    let response = read_reply(&mut BufReader::new(peer));
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

#[test]
fn held_process_waits_for_the_whole_paid_frontier() {
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_loopback"))
        .args(["--held", "headers"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut control = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap());
    let mut line = String::new();
    output.read_line(&mut line).unwrap();
    let address = format!("127.0.0.1:{}", line.trim());
    // The counted command consumes arrivals, just as the previous command does.
    writeln!(control, "arrived 2").unwrap();
    control.flush().unwrap();
    let mut peers = Vec::new();
    for _ in 0..2 {
        let mut peer = TcpStream::connect(&address).unwrap();
        peer.write_all(REQUEST).unwrap();
        peers.push(peer);
    }
    line.clear();
    output.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "arrived");
    writeln!(control, "count").unwrap();
    control.flush().unwrap();
    line.clear();
    output.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "2");
    let mut third = TcpStream::connect(&address).unwrap();
    third.write_all(REQUEST).unwrap();
    writeln!(control, "arrived").unwrap();
    control.flush().unwrap();
    line.clear();
    output.read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "arrived");
    drop(peers);
    drop(third);
    writeln!(control, "shutdown").unwrap();
    control.flush().unwrap();
    assert!(child.wait().unwrap().success());
}
