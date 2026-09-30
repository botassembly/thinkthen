//! Real-fork proofs of ticket 0096 through the public API.
//!
//! Each proof forks this test process with [`fork_probe::in_child`]. The
//! conformance backend runs on the parent's threads, so a child's request
//! reaches it through the inherited address. One lock keeps the proofs from
//! sharing the process's permits at once.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

#[path = "../../../../crates/thinkthen/src/test_deadline/child.rs"]
mod child;
#[path = "../../../../crates/thinkthen/src/test_deadline/run.rs"]
mod run;
#[path = "../../../../crates/thinkthen/src/test_deadline/wait.rs"]
mod wait;

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

#[allow(
    clippy::expect_used,
    reason = "a failed local certificate fixture must stop this proof"
)]
fn tls_certificate_pair(home: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let (cert, key) = (home.join("cert.pem"), home.join("cert.key"));
    let created = child::command("openssl", &[])
        .args(["req", "-x509", "-newkey", "rsa:2048", "-nodes"])
        .arg("-keyout")
        .arg(&key)
        .arg("-out")
        .arg(&cert)
        .args([
            "-days",
            "1",
            "-subj",
            "/CN=localhost",
            "-addext",
            "subjectAltName=DNS:localhost",
        ])
        .output()
        .expect("OpenSSL fixture generator");
    assert!(
        created.status.success(),
        "local certificate generator failed"
    );
    let (leaf, leaf_key, csr, san) = (
        home.join("leaf.pem"),
        home.join("leaf.key"),
        home.join("leaf.csr"),
        home.join("leaf.cnf"),
    );
    std::fs::write(&san, "subjectAltName=DNS:localhost\n").expect("localhost SAN");
    let requested = child::command("openssl", &[])
        .args(["req", "-newkey", "rsa:2048", "-nodes"])
        .arg("-keyout")
        .arg(&leaf_key)
        .arg("-out")
        .arg(&csr)
        .args(["-subj", "/CN=localhost"])
        .output()
        .expect("OpenSSL leaf generator");
    assert!(requested.status.success(), "local leaf generator failed");
    let signed = child::command("openssl", &[])
        .args(["x509", "-req", "-in"])
        .arg(&csr)
        .arg("-CA")
        .arg(&cert)
        .arg("-CAkey")
        .arg(&key)
        .args(["-set_serial", "1", "-out"])
        .arg(&leaf)
        .args(["-days", "1", "-extfile"])
        .arg(&san)
        .output()
        .expect("OpenSSL leaf signer");
    assert!(signed.status.success(), "local leaf signer failed");

    (cert, leaf, leaf_key)
}

#[allow(
    clippy::expect_used,
    reason = "a failed local TLS responder must stop this proof"
)]
fn tls_responder(leaf: &Path, leaf_key: &Path) -> (TlsResponder, u16) {
    let reserved = TcpListener::bind("127.0.0.1:0").expect("free TLS port");
    let port = reserved.local_addr().expect("TLS port").port();
    drop(reserved);
    let address = format!("127.0.0.1:{port}");
    let mut server = child::command("openssl", &[])
        .args([
            "s_server", "-quiet", "-ign_eof", "-naccept", "1", "-accept", &address,
        ])
        .arg("-cert")
        .arg(leaf)
        .arg("-key")
        .arg(leaf_key)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("local TLS responder");
    let answer = "{\"model\":\"local-1\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";
    let reply = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{answer}",
        answer.len()
    );
    server
        .stdin
        .as_mut()
        .expect("responder input")
        .write_all(reply.as_bytes())
        .expect("fixed reply");
    // Wait up to five seconds: a loaded host starts the responder slowly.
    for _ in 0..500 {
        if let Ok(probe) = TcpListener::bind(&address) {
            drop(probe);
            thread::sleep(Duration::from_millis(10));
        } else {
            break;
        }
    }

    (TlsResponder(server), port)
}

struct TlsResponder(Child);

impl Drop for TlsResponder {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn a_forked_child_keeps_parsed_tls_roots_after_the_file_changes() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let home = folder("tls-roots");
    std::fs::create_dir(&home).expect("certificate fixture folder");
    let (cert, leaf, leaf_key) = tls_certificate_pair(&home);
    let (mut server, port) = tls_responder(&leaf, &leaf_key);

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
    for _ in 0..500 {
        if server.0.try_wait().expect("responder state").is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(server.0.try_wait().expect("responder state").is_some());
    let mut requests = String::new();
    server
        .0
        .stdout
        .take()
        .expect("request capture")
        .read_to_string(&mut requests)
        .expect("request bytes");
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
