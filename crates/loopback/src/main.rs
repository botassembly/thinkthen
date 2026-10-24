//! Run a canned, held or unavailable backend until standard input closes.

use loopback::{HeldReply, Loopback, UnavailableBackend};
use std::io::{BufRead, Write};
use std::sync::mpsc::Receiver;

enum Fixture {
    Case(Loopback),
    Held {
        server: Loopback,
        arrived: Receiver<()>,
        closed: Receiver<std::io::Result<usize>>,
    },
    Unavailable(UnavailableBackend),
}

impl Fixture {
    fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = std::env::args().skip(1);
        let first = args.next().ok_or(
            "usage: loopback CASE.json | --held headers|body|refusal-body | --unavailable",
        )?;
        match first.as_str() {
            "--held" => {
                let reply = match args.next().as_deref() {
                    Some("headers") => HeldReply::Headers,
                    Some("body") => HeldReply::Body,
                    Some("refusal-body") => HeldReply::RefusalBody,
                    _ => return Err("expected headers, body or refusal-body".into()),
                };
                let (server, arrived, closed) = Loopback::holding(reply)?;
                Ok(Self::Held {
                    server,
                    arrived,
                    closed,
                })
            }
            "--unavailable" => Ok(Self::Unavailable(UnavailableBackend::new()?)),
            path => {
                let case = serde_json::from_slice(&std::fs::read(path)?)?;
                Ok(Self::Case(Loopback::case(&case)?))
            }
        }
    }

    fn base(&self) -> &str {
        match self {
            Self::Case(server) | Self::Held { server, .. } => server.base(),
            Self::Unavailable(server) => server.base(),
        }
    }

    fn command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        match (self, command) {
            (Self::Case(server) | Self::Held { server, .. }, "count") => {
                Ok(server.count().to_string())
            }
            (Self::Unavailable(_), "count") => Ok("0".to_owned()),
            (Self::Held { arrived, .. }, "arrived") => {
                arrived.recv()?;
                Ok("arrived".to_owned())
            }
            (Self::Held { closed, .. }, "closed") => match closed.recv()? {
                Ok(0) => Ok("closed".to_owned()),
                Err(error) if error.kind() == std::io::ErrorKind::ConnectionReset => {
                    Ok("closed".to_owned())
                }
                Ok(_) => Err("held connection received bytes instead of EOF".into()),
                Err(error) => Err(error.into()),
            },
            _ => Err("expected count, arrived, closed or shutdown for this fixture".into()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = Fixture::open()?;
    let port = fixture
        .base()
        .strip_prefix("http://127.0.0.1:")
        .and_then(|base| base.strip_suffix("/v1"))
        .ok_or("missing port")?;
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    writeln!(output, "{port}")?;
    output.flush()?;
    for line in std::io::stdin().lock().lines() {
        let command = line?;
        if command == "shutdown" {
            break;
        }
        writeln!(output, "{}", fixture.command(&command)?)?;
        output.flush()?;
    }
    Ok(())
}
