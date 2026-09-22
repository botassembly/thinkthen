//! The stub backend for experiment 205: a loopback server in the vendor's
//! wire shape, with a settable delay and counters a bench reads.
//!
//! `POST /v1/systemone` answers one request. The probability is fixed by the
//! evidence text: evidence holding `refund` gives 0.97, evidence holding
//! `maybe` gives 0.55, evidence holding `malformed` is refused with 422, and
//! anything else gives 0.03. `STUB_DELAY_MS` sleeps per request. `GET
//! /v1/stats` reports requests served, the highest number in flight at once,
//! and connections opened. `POST /v1/reset` clears the counts. `STUB_PORT`
//! names the port, 8091 by default.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// The most of one request body the stub reads before it refuses.
const MAX_REQUEST_BYTES: usize = 1024 * 1024;

/// The counters a bench reads back.
#[derive(Default)]
struct Stats {
    requests: AtomicU64,
    in_flight: AtomicU64,
    max_in_flight: AtomicU64,
    connections: AtomicU64,
}

/// The probability the evidence fixes, by the rule the engine's null backend
/// copies so one set of cases runs against both.
fn probability_of(evidence: &str) -> f64 {
    if evidence.contains("refund") {
        0.97
    } else if evidence.contains("maybe") {
        0.55
    } else {
        0.03
    }
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("STUB_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8091);
    let delay = Duration::from_millis(
        std::env::var("STUB_DELAY_MS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(0),
    );
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(address).await.expect("the stub binds");
    eprintln!("stub listening on http://{address} delay {delay:?}");

    let stats = Arc::new(Stats::default());
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(pair) => pair,
            Err(error) => {
                eprintln!("accept failed: {error}");
                continue;
            }
        };
        let opened = stats.connections.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("connection {opened} from {stream:?}");
        let stats = Arc::clone(&stats);
        let delay = delay;
        tokio::spawn(async move {
            let served = http1::Builder::new().serve_connection(
                TokioIo::new(stream),
                service_fn(move |request| handle(request, Arc::clone(&stats), delay)),
            );
            if let Err(error) = served.await {
                eprintln!("connection failed: {error}");
            }
        });
    }
}

/// One request, counted and answered.
async fn handle(
    request: Request<Incoming>,
    stats: Arc<Stats>,
    delay: Duration,
) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let answer = match (&method, path.as_str()) {
        (&Method::POST, "/v1/systemone") => Ok(answer_judgment(request, &stats, delay).await),
        (&Method::GET, "/v1/stats") => Ok(stats_page(&stats)),
        (&Method::POST, "/v1/reset") => {
            reset(&stats);
            Ok(text(StatusCode::OK, "{\"reset\":true}"))
        }
        _ => Ok(text(StatusCode::NOT_FOUND, "{\"error\":\"not found\"}")),
    };
    answer
}

/// Read the request, wait the delay, and answer in the vendor's shape.
async fn answer_judgment(
    request: Request<Incoming>,
    stats: &Stats,
    delay: Duration,
) -> Response<Full<Bytes>> {
    let body = match request.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return text(StatusCode::BAD_REQUEST, "{\"error\":\"unreadable\"}"),
    };
    if body.len() > MAX_REQUEST_BYTES {
        return text(StatusCode::PAYLOAD_TOO_LARGE, "{\"error\":\"too large\"}");
    }
    let evidence = serde_json::from_slice::<serde_json::Value>(&body)
        .ok()
        .and_then(|value| value.get("state").and_then(|state| state.as_str()).map(str::to_owned));
    let Some(evidence) = evidence else {
        return text(StatusCode::UNPROCESSABLE_ENTITY, "{\"error\":\"no state\"}");
    };

    let now = stats.in_flight.fetch_add(1, Ordering::Relaxed) + 1;
    record_max(stats, now);
    tokio::time::sleep(delay).await;
    stats.in_flight.fetch_sub(1, Ordering::Relaxed);
    stats.requests.fetch_add(1, Ordering::Relaxed);

    if evidence.contains("malformed") {
        return text(StatusCode::UNPROCESSABLE_ENTITY, "{\"error\":\"malformed\"}");
    }
    let probability = probability_of(&evidence);
    let reply = serde_json::json!({
        "model": "jev-latest",
        "answers": { "q1": { "type": "noul", "noul": probability } },
        "usage": { "input_tokens": 10, "output_tokens": 2 }
    });
    text(StatusCode::OK, &reply.to_string())
}

/// The counters, as JSON text.
fn stats_page(stats: &Stats) -> Response<Full<Bytes>> {
    let page = serde_json::json!({
        "requests": stats.requests.load(Ordering::Relaxed),
        "max_in_flight": stats.max_in_flight.load(Ordering::Relaxed),
        "connections": stats.connections.load(Ordering::Relaxed),
    });
    text(StatusCode::OK, &page.to_string())
}

/// Clear every counter.
fn reset(stats: &Stats) {
    stats.requests.store(0, Ordering::Relaxed);
    stats.in_flight.store(0, Ordering::Relaxed);
    stats.max_in_flight.store(0, Ordering::Relaxed);
    stats.connections.store(0, Ordering::Relaxed);
}

/// Hold the highest in-flight count seen so far.
fn record_max(stats: &Stats, now: u64) {
    let mut seen = stats.max_in_flight.load(Ordering::Relaxed);
    while now > seen {
        match stats.max_in_flight.compare_exchange_weak(
            seen,
            now,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(fresh) => seen = fresh,
        }
    }
}

/// One JSON response.
fn text(status: StatusCode, body: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body.to_owned())))
        .expect("a fixed response builds")
}
