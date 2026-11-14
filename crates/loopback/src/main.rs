//! Run a canned, held or unavailable backend until standard input closes.

use loopback::{HeldReply, Loopback, StressFixture, UnavailableBackend};
use std::io::{BufRead, Write};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

fn wait_arrivals(
    arrived: &Receiver<()>,
    count: usize,
    deadline: Instant,
    mut progress: impl FnMut(usize) -> std::io::Result<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    progress(0)?;
    for received in 0..count {
        arrived
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .map_err(|error| {
                format!("arrival wait failed: received={received} required={count} reason={error}")
            })?;
        progress(received + 1)?;
    }
    Ok(())
}

enum Fixture {
    Case(Loopback),
    Stress(StressFixture),
    Held {
        server: Loopback,
        arrived: Receiver<()>,
        closed: Receiver<std::io::Result<usize>>,
    },
    First {
        server: StressFixture,
        arrived: Receiver<()>,
        closed: Receiver<std::io::Result<usize>>,
    },
    Unavailable(UnavailableBackend),
}

impl Fixture {
    fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = std::env::args().skip(1);
        let first = args.next().ok_or(
            "usage: loopback CASE.json | --guide-scenario NAME | --stress NAME [--prime REQUESTS] | --held headers|body|refusal-body | --unavailable",
        )?;
        match first.as_str() {
            "--guide-scenario" => {
                let name = args.next().ok_or("expected guide scenario name")?;
                if args.next().is_some() {
                    return Err("expected one guide scenario name".into());
                }
                Ok(Self::Case(Loopback::guide_scenario(&name)?))
            }
            "--stress" => {
                let name = args.next().ok_or("expected stress scenario name")?;
                let prime = match (args.next().as_deref(), args.next()) {
                    (None, None) => 0,
                    (Some("--prime"), Some(count)) => count.parse()?,
                    _ => return Err("expected --stress NAME [--prime REQUESTS]".into()),
                };
                Ok(Self::Stress(StressFixture::primed(&name, prime)?))
            }
            "--first-response" => {
                let marker = args.next().ok_or("expected first response marker")?;
                let server = StressFixture::open("healthy")?;
                let StressFixture::Server(answering, _) = &server else {
                    return Err("expected an answering fixture".into());
                };
                let (arrived, closed) =
                    answering.hold_matching(HeldReply::Headers, move |request| {
                        !request
                            .json()
                            .is_ok_and(|body| body.to_string().contains(&marker))
                    });
                Ok(Self::First {
                    server,
                    arrived,
                    closed,
                })
            }
            "--held" => {
                let reply = match args.next().as_deref() {
                    Some("reset") => HeldReply::Reset,
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
            Self::Stress(server) | Self::First { server, .. } => server.base(),
        }
    }

    fn command(
        &self,
        command: &str,
        output: &mut impl Write,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match (self, command) {
            (Self::Stress(server) | Self::First { server, .. }, "stats") => {
                Ok(server.stats().to_string())
            }
            (Self::Stress(server), "primed") => server
                .stats()
                .get("prime")
                .and_then(|prime| prime.get("reached"))
                .map(ToString::to_string)
                .ok_or_else(|| "stress statistics omitted pool priming status".into()),
            (Self::Stress(server) | Self::First { server, .. }, "count") => server
                .stats()
                .get("requests")
                .map(ToString::to_string)
                .ok_or_else(|| "stress statistics omitted request count".into()),
            (Self::Case(server) | Self::Held { server, .. }, "count") => {
                Ok(server.count().to_string())
            }
            (Self::Unavailable(_), "count") => Ok("0".to_owned()),
            (Self::Held { arrived, .. } | Self::First { arrived, .. }, command)
                if command == "arrived" || command.starts_with("arrived ") =>
            {
                let mut args = command.split_whitespace().skip(1);
                let count = args
                    .next()
                    .map(str::parse::<usize>)
                    .transpose()?
                    .unwrap_or(1);
                let milliseconds = args.next().map(str::parse::<u64>).transpose()?;
                if count == 0 || milliseconds == Some(0) || args.next().is_some() {
                    return Err(
                        "expected arrived [positive count [positive budget milliseconds]]".into(),
                    );
                }
                let budget = Duration::from_millis(milliseconds.unwrap_or(30_000));
                let started = Instant::now();
                let deadline = started
                    .checked_add(budget)
                    .ok_or("arrival budget is too large")?;
                wait_arrivals(arrived, count, deadline, |received| {
                    if milliseconds.is_some() {
                        writeln!(output, "{}", serde_json::json!({
                            "event": "arrival", "received": received, "required": count,
                            "elapsed_ms": started.elapsed().as_millis(), "budget_ms": budget.as_millis(),
                        }))?;
                        output.flush()?;
                    }
                    Ok(())
                }).map_err(|error| format!("{error}; elapsed_ms={} budget_ms={}",
                                          started.elapsed().as_millis(), budget.as_millis()))?;
                Ok("arrived".to_owned())
            }
            (Self::Held { closed, .. } | Self::First { closed, .. }, "closed") => {
                match closed.recv()? {
                    Ok(0) => Ok("closed".to_owned()),
                    Err(error) if error.kind() == std::io::ErrorKind::ConnectionReset => {
                        Ok("closed".to_owned())
                    }
                    Ok(_) => Err("held connection received bytes instead of EOF".into()),
                    Err(error) => Err(error.into()),
                }
            }
            _ => Err("expected count, arrived, closed or shutdown for this fixture".into()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--limits") {
        writeln!(std::io::stdout().lock(), "{}", loopback::limits())?;
        return Ok(());
    }
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
        let response = fixture.command(&command, &mut output)?;
        writeln!(output, "{response}")?;
        output.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::wait_arrivals;
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    #[test]
    fn expired_total_deadline_keeps_partial_arrivals_without_starting_another_wait() {
        let (send, receive) = mpsc::channel();
        for _ in 0..3 {
            send.send(()).unwrap();
        }
        let mut seen = Vec::new();
        let started = Instant::now();
        let error = wait_arrivals(&receive, 4, started, |received| {
            seen.push(received);
            Ok(())
        })
        .unwrap_err();
        assert!(started.elapsed() < Duration::from_secs(15));
        assert_eq!(seen, [0, 1, 2, 3]);
        assert!(error.to_string().contains("received=3 required=4"));
    }
}
