//! Reading one request off a loopback connection.

use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use super::Recorded;

/// How long a peek may see no new bytes before the request is read instead.
const STALL: Duration = Duration::from_secs(2);

/// The number of wire questions one request body holds, or 0 for a body
/// that is not a request.
pub(super) fn questions(body: &[u8]) -> usize {
    serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .and_then(|request| {
            request
                .get("questions")?
                .as_object()
                .map(serde_json::Map::len)
        })
        .unwrap_or(0)
}

/// Peek until one whole request sits in the socket buffer, then parse it.
///
/// The bytes stay unread, so a reset can drop them. The count says how many
/// bytes the request took, for [`consume`] to read past. A request too long to
/// peek whole, or one whose bytes stopped arriving for [`STALL`], is read at
/// once instead. That read ends at a closed client's end of file, and the
/// count of 0 turns a later reset into the drift status.
pub(super) fn peek_request(stream: &TcpStream) -> Option<(Recorded, usize)> {
    let mut buffer = vec![0; 32 * 1024];
    let mut last = (0, Instant::now());
    loop {
        let seen = stream.peek(&mut buffer).ok()?;
        if seen == 0 {
            return None;
        }
        let mut rest = buffer.get(..seen)?;
        if let Some(request) = read_request(&mut rest) {
            return Some((request, seen - rest.len()));
        }
        if seen != last.0 {
            last = (seen, Instant::now());
        }
        if seen == buffer.len() || last.1.elapsed() >= STALL {
            return read_request(&mut BufReader::new(stream)).map(|request| (request, 0));
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Read past the bytes one peeked request took.
pub(super) fn consume(mut stream: &TcpStream, used: usize) -> Option<()> {
    stream.read_exact(&mut vec![0; used]).ok()
}

/// Read one request line, its headers, and the body its content length names.
fn read_request(reader: &mut impl BufRead) -> Option<Recorded> {
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    let mut headers = Vec::new();
    let mut length = 0;
    loop {
        let mut header = String::new();
        reader.read_line(&mut header).ok()?;
        // A line cut off by the end of what arrived so far is no line yet.
        if !header.ends_with('\n') {
            return None;
        }
        let header = header.trim_end().to_owned();
        if header.is_empty() {
            break;
        }
        if let Some(value) = header.to_lowercase().strip_prefix("content-length: ") {
            length = value.trim().parse().ok()?;
        }
        headers.push(header);
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body).ok()?;
    Some(Recorded {
        line: line.trim_end().to_owned(),
        headers,
        body,
    })
}
