//! Real-fork proofs of ticket 0096 through the public API.
//!
//! Each proof forks this test process with [`fork_probe::in_child`]. The
//! conformance backend runs on the parent's threads, so a child's request
//! reaches it through the inherited address. One lock keeps the proofs from
//! sharing the process's permits at once.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[path = "../../../../crates/thinkthen/src/test_deadline/child.rs"]
mod child;
use child::{run, wait};

use conformance_backend::Backend;
use fork_probe::in_child;
use thinkthen::{Answer, Engine, Question};

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
const KEY: &str = "sk-fork-loopback";
const CHILD: &str = "FORK_PROBE_DEFAULT_BASE";

fn engine(base: &str, cache: Option<&PathBuf>) -> Result<Engine, thinkthen::Error> {
    let builder = Engine::builder().base_url(base)?.api_key(KEY)?;
    let builder = match cache {
        Some(folder) => builder.cache_at(folder)?,
        None => builder.no_cache(),
    };
    builder.build()
}

fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("fork-probe-{name}-{}", std::process::id()));
    let _absent = std::fs::remove_dir_all(&folder);
    folder
}

/// Wait until `path` exists, or 60 s pass.
fn appears(path: &Path) {
    let cap = Instant::now() + Duration::from_secs(60);
    while !path.exists() && Instant::now() < cap {
        thread::sleep(Duration::from_millis(10));
    }
}

fn decide() -> Question {
    Question::decide("Is this urgent?").map_or_else(
        |_| unreachable!("the text is not blank"),
        thinkthen::DecideBuilder::cut,
    )
}

fn answer(call: Result<thinkthen::Call<Answer>, thinkthen::Error>) -> Option<Answer> {
    call.ok().map(thinkthen::Call::into_value)
}

#[test]
fn a_warm_parent_engine_and_its_clone_answer_in_the_child() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let engine =
        engine(&format!("{}/generic/v1", backend.origin()), None).expect("a loopback engine");
    assert_eq!(answer(engine.decide(&decide(), "warm")), Some(Answer::Yes));
    let clone = engine.clone();
    in_child(|| {
        let before = engine.usage().requests_sent();
        let answered = answer(clone.decide(&decide(), "in the child")) == Some(Answer::Yes);
        answered && engine.usage().requests_sent() == before + 1
    })
    .expect("the child answered, and the clone counted on its source");
    assert_eq!(backend.count(), 2, "one send from each process");
    assert_eq!(
        engine.usage().requests_sent(),
        1,
        "the child's send stays the child's"
    );
}

#[test]
fn an_inherited_engine_and_a_child_engine_share_one_request_total() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let capped = || {
        Engine::builder()
            .base_url(&base)?
            .api_key(KEY)?
            .no_cache()
            .max_requests_total(Some(1))
            .build()
    };
    let inherited = capped().expect("the parent's capped engine");
    in_child(|| {
        let sent = answer(inherited.decide(&decide(), "inherited")) == Some(Answer::Yes);
        let own = capped().expect("the child's capped engine");
        let refused = own.decide(&decide(), "own").err();
        sent && refused.and_then(|error| error.send_budget_denial())
            == Some(thinkthen::SendBudgetDenial::BeforeFirstSend)
    })
    .expect("the child's two engines counted against one process total");
    assert_eq!(backend.count(), 1, "the child's second engine sent nothing");
}

/// An SQL host caps each call against the process total (ticket 0304 slice
/// 3e). The child starts from zero, so the parent's other proofs never count.
#[test]
fn a_call_total_counts_every_engine_of_the_process() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let capped = |total| {
        Engine::builder()
            .base_url(&base)?
            .api_key(KEY)?
            .no_cache()
            .max_requests_total(total)
            .build()
    };
    in_child(|| {
        let (first, second) = (capped(None).expect("one"), capped(Some(3)).expect("two"));
        let call = |engine: &Engine, evidence, total| {
            let options = thinkthen::CallOptions::new().max_requests_total(total);
            engine.decide_with(&decide(), evidence, options)
        };
        let denied = |result: Result<thinkthen::Call<Answer>, thinkthen::Error>| {
            result.err().and_then(|error| error.send_budget_denial())
                == Some(thinkthen::SendBudgetDenial::BeforeFirstSend)
        };
        answer(call(&first, "a", None)) == Some(Answer::Yes)
            && answer(call(&second, "b", Some(9))) == Some(Answer::Yes)
            && denied(call(&first, "c", Some(2)))
            && denied(call(&second, "d", Some(2)))
            && answer(call(&first, "e", Some(3))) == Some(Answer::Yes)
            && denied(call(&second, "f", Some(9)))
            && thinkthen::process_requests_sent() == 3
    })
    .expect("both engines counted against one process total, and the tighter limit held");
    assert_eq!(backend.count(), 3, "the child sent three times");
}

// A fixed test CA and a `localhost` leaf it signed, valid until 2126. The key
// guards nothing. The fixture serves TLS itself, so the proof never depends on
// which `openssl` is on `PATH` (ticket 0371). Made with OpenSSL 3:
//   openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
//     -keyout ca.key -out ca.pem -days 36500 -subj /CN=fork-probe-test-ca
//   openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
//     -keyout leaf.key -out leaf.csr -subj /CN=localhost
//   openssl x509 -req -in leaf.csr -CA ca.pem -CAkey ca.key -set_serial 1 \
//     -days 36500 -extfile leaf.cnf -out leaf.pem
// where leaf.cnf holds `subjectAltName=DNS:localhost`.
const TLS_CA: &[u8] = include_bytes!("tls/ca.pem");
const TLS_LEAF: &[u8] = include_bytes!("tls/leaf.pem");
const TLS_LEAF_KEY: &[u8] = include_bytes!("tls/leaf.key");

/// Serve one HTTPS connection on a parent thread: reply once the request's
/// head arrives, then return every byte the client sent until it closed.
#[allow(
    clippy::expect_used,
    reason = "a failed local TLS responder must stop this proof"
)]
fn tls_responder() -> (u16, thread::JoinHandle<String>) {
    use rustls::pki_types::pem::PemObject;
    use rustls::pki_types::{CertificateDer, PrivateKeyDer};

    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let config = rustls::ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .expect("TLS versions")
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from_pem_slice(TLS_LEAF).expect("leaf certificate")],
            PrivateKeyDer::from_pem_slice(TLS_LEAF_KEY).expect("leaf key"),
        )
        .expect("TLS server configuration");
    let listener = TcpListener::bind("127.0.0.1:0").expect("free TLS port");
    let port = listener.local_addr().expect("TLS port").port();
    // A broken child fails the proof after 60 s and does not hang it.
    listener.set_nonblocking(true).expect("polled accept");
    let served = thread::spawn(move || {
        let cap = Instant::now() + Duration::from_secs(60);
        let socket = loop {
            match listener.accept() {
                Ok((socket, _)) => break socket,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < cap, "no TLS connection within 60 s");
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => unreachable!("TLS accept failed: {error}"),
            }
        };
        socket.set_nonblocking(false).expect("blocking reads");
        socket
            .set_read_timeout(Some(Duration::from_secs(60)))
            .expect("read limit");
        let connection = rustls::ServerConnection::new(Arc::new(config)).expect("TLS session");
        let mut stream = rustls::StreamOwned::new(connection, socket);
        let answer = "{\"model\":\"local-1\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";
        let reply = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{answer}",
            answer.len()
        );
        let (mut sent, mut replied, mut chunk) = (Vec::new(), false, [0_u8; 4096]);
        // A client that leaves without a TLS goodbye ends the read with an error.
        while let Ok(read @ 1..) = stream.read(&mut chunk) {
            sent.extend_from_slice(chunk.get(..read).unwrap_or_default());
            if !replied && sent.windows(4).any(|end| end == b"\r\n\r\n") {
                stream.write_all(reply.as_bytes()).expect("fixed reply");
                stream.flush().expect("fixed reply sent");
                replied = true;
            }
        }
        String::from_utf8_lossy(&sent).into_owned()
    });

    (port, served)
}

#[test]
fn a_forked_child_keeps_parsed_tls_roots_after_the_file_changes() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let home = folder("tls-roots");
    std::fs::create_dir(&home).expect("certificate fixture folder");
    let cert = home.join("ca.pem");
    std::fs::write(&cert, TLS_CA).expect("test CA in its folder");
    let (port, served) = tls_responder();

    let engine = Engine::builder()
        .base_url(&format!("https://localhost:{port}"))
        .expect("local HTTPS base")
        .api_key(KEY)
        .expect("synthetic key")
        .ca_bundle(&cert)
        .expect("absolute certificate path")
        .no_cache()
        .build()
        .expect("parsed roots in the parent");
    std::fs::write(&cert, b"this is no longer a certificate").expect("change source file");
    in_child(|| answer(engine.decide(&decide(), "after fork")) == Some(Answer::Yes))
        .expect("forked child used the retained roots");
    let requests = served.join().expect("the TLS responder");
    assert_eq!(
        requests.matches("POST /systemone").count(),
        1,
        "{requests:?}"
    );
    std::fs::remove_dir_all(home).expect("remove certificate fixture");
}

#[test]
fn a_child_of_a_busy_parent_gets_its_own_permits() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let cache = folder("busy");
    let held = engine(&format!("{}/arm/held/v1", backend.origin()), Some(&cache))
        .expect("a loopback engine");
    // A recording folder belongs to one backend address, so the child's has its own.
    let open = engine(
        &format!("{}/generic/v1", backend.origin()),
        Some(&folder("busy-child")),
    )
    .expect("a loopback engine");
    let held = &held;
    thread::scope(|scope| {
        // Four calls hold the process's four permits, each with its reply held.
        let calls: Vec<_> = (0..4)
            .map(|at| scope.spawn(move || answer(held.decide(&decide(), &format!("held {at}")))))
            .collect();
        assert_eq!(backend.wait(4), 4, "every permit is in flight");
        let child = in_child(|| answer(open.decide(&decide(), "the child")) == Some(Answer::Yes));
        backend.release();
        child.expect("the child sent past the parent's full gate");
        for call in calls {
            assert_eq!(call.join().expect("a parent call"), Some(Answer::Yes));
        }
    });
    assert_eq!(backend.count(), 5);
}

#[test]
fn a_child_replays_the_parents_warm_cache_without_sending() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let cache = folder("warm-cache");
    let engine = engine(&format!("{}/generic/v1", backend.origin()), Some(&cache))
        .expect("a loopback engine");
    assert!(
        !engine
            .details(&decide(), "cached note")
            .expect("a first answer")
            .value()
            .cached()
    );
    in_child(|| {
        engine
            .details(&decide(), "cached note")
            .is_ok_and(|details| details.value().cached() && details.value().requests_sent() == 0)
    })
    .expect("the child read the parent's recording");
    assert_eq!(backend.count(), 1, "only the parent sent");
}

/// Ticket 0096 F6: a parent's concurrent question never waits for a forked
/// child. ADR 0111 dropped the digest lock and coalesces only within one call,
/// so the second call sends its own request.
#[test]
fn a_parents_concurrent_question_never_waits_for_a_forked_child() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let cache = folder("digest-lock");
    let held = engine(&format!("{}/arm/held/v1", backend.origin()), Some(&cache))
        .expect("a loopback engine");
    thread::scope(|scope| {
        let owner = scope.spawn(|| answer(held.decide(&decide(), "one note")));
        assert_eq!(backend.wait(1), 1, "the owner's request is held");
        let waiter = scope.spawn(|| {
            let result = answer(held.decide(&decide(), "one note"));
            (result, Instant::now())
        });
        thread::sleep(Duration::from_millis(200));
        // The child stays until the waiter has its answer, so the waiter
        // cannot have waited for it. The 60 s cap only stops a hang
        // (ticket 0352).
        let gate = folder("child-gate");
        let seen = gate.clone();
        let child = scope.spawn(|| {
            in_child(move || {
                appears(&seen);
                true
            })
        });
        thread::sleep(Duration::from_millis(200));
        let released = Instant::now();
        backend.release();
        assert_eq!(owner.join().expect("the owner"), Some(Answer::Yes));
        let (answer, done) = waiter.join().expect("the waiter");
        std::fs::write(&gate, "done").expect("the child's gate");
        assert_eq!(answer, Some(Answer::Yes));
        assert!(
            done - released < Duration::from_secs(30),
            "the waiter waited for the child: {:?}",
            done - released
        );
        child
            .join()
            .expect("the child thread")
            .expect("the child slept and left");
        let _removed = std::fs::remove_file(&gate);
    });
    assert_eq!(backend.count(), 2, "separate calls do not coalesce");
}

#[test]
fn a_default_engine_from_the_parent_answers_in_the_child() {
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let output = run::output(
        Command::new(std::env::current_exe().expect("this test binary"))
            .args(["--exact", "--ignored", "default_engine_child"])
            .env_clear()
            .env(CHILD, "1")
            .env("THINKTHEN_BASE_URL", &base)
            .env("THINKTHEN_API_KEY", KEY)
            .env("THINKTHEN_CACHE", folder("default-engine")),
    )
    .expect("the process ran");
    let said = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success() && said.contains("1 passed"),
        "{said}"
    );
    assert_eq!(
        backend.count(),
        2,
        "one send from the parent and one from its child"
    );
}

/// The process half of the proof above. It does nothing unless named.
#[test]
#[ignore = "the process half; its parent runs it with --ignored"]
fn default_engine_child() {
    if std::env::var_os(CHILD).is_none() {
        return;
    }
    assert_eq!(
        answer(thinkthen::decide(&decide(), "parent")),
        Some(Answer::Yes)
    );
    in_child(|| answer(thinkthen::decide(&decide(), "child")) == Some(Answer::Yes))
        .expect("the process engine answered in the child");
}
