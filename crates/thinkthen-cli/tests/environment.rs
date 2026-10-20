//! Command configuration and usage share the native public library's persisted state.
#![cfg(unix)]
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "a failed isolated command or loopback fixture stops the boundary proof"
)]
#[path = "../../thinkthen/src/test_deadline/child.rs"]
mod child;
use child::{ChildEnvironment as _, Folder};
#[path = "../../thinkthen/tests/support/environment.rs"]
mod fixture;
use fixture::{ANSWERED, EVIDENCE};
#[path = "support/command.rs"]
mod command;
use conformance_backend::{Canned, Listener};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

fn home(label: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "command-environment-{}-{label}",
        std::process::id()
    ));
    let _old = fs::remove_dir_all(&path);
    fs::create_dir(&path).unwrap();
    path
}

#[test]
fn configuration_model_and_environment_route_reach_the_command_plan() {
    let listener = Listener::answering(|_| Canned::ok(ANSWERED)).unwrap();
    let home = home("plan");
    let configured = Folder::configure(&home, r#"{"model":"model-from-config"}"#).unwrap();
    let evidence = home.join("evidence");
    fs::write(&evidence, EVIDENCE).unwrap();
    let mut command = Command::new(crate::command::binary());
    command
        .args(["decide", "asks for a refund", "--plan"])
        .clear_environment()
        .isolated_home(&home)
        .env(configured.0, &configured.1)
        .env("THINKTHEN_BASE_URL", listener.base())
        .env("THINKTHEN_API_KEY", "sk-fake-loopback")
        .stdin(fs::File::open(&evidence).unwrap())
        .stdout(Stdio::piped());
    let output = child::run::output(&mut command).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let plan = String::from_utf8(output.stdout).unwrap();
    assert!(
        plan.contains(&format!(r#""url":"{}""#, listener.url())),
        "{plan}"
    );
    assert!(plan.contains(r#""model":"model-from-config""#), "{plan}");
    assert_eq!(listener.count(), 0);
    fs::remove_dir_all(home).unwrap();
}

#[test]
#[ignore = "isolated public library child for the command's persisted usage test"]
fn usage_child() {
    let report = std::env::var_os("THINKTHEN_TEST_USAGE_REPORT").expect("owned report");
    let engine = thinkthen::EngineBuilder::from_env()
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let question = thinkthen::Question::decide("asks for a refund")
        .unwrap()
        .cut();
    let call = engine.decide(&question, EVIDENCE).unwrap();
    assert_eq!(call.value(), &Some(true));
    assert_eq!(call.facts().requests_sent(), 2);
    assert_eq!(
        engine.finish_usage_status(),
        thinkthen::UsagePersistence::Written
    );
    fs::write(report, "Some(true) sent 2 cached 0").unwrap();
}

#[test]
fn the_command_and_a_seeded_engine_add_to_one_total() {
    let arrivals = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if arrivals.fetch_add(1, Ordering::SeqCst) == 1 {
            Canned::status(503, "busy").asking("retry-after-ms", "0")
        } else {
            Canned::ok(ANSWERED)
        }
    })
    .unwrap();
    let home = home("usage");
    let (cache, state) = (Folder::Cache.variable(&home), Folder::Usage.variable(&home));
    let evidence = home.join("evidence");
    fs::write(&evidence, format!("{EVIDENCE}\n")).unwrap();
    let configure = |command: &mut Command| {
        command
            .clear_environment()
            .isolated_home(&home)
            .env(cache.0, &cache.1)
            .env(state.0, &state.1)
            .env("THINKTHEN_API_KEY", "sk-fake-loopback")
            .env("THINKTHEN_BASE_URL", listener.base());
    };
    let mut command = Command::new(crate::command::binary());
    configure(&mut command);
    command
        .args(["decide", "asks for a refund"])
        .stdin(fs::File::open(evidence).unwrap());
    let output = child::run::output(&mut command).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = home.join("report");
    let mut seeded = Command::new(std::env::current_exe().unwrap());
    configure(&mut seeded);
    seeded
        .args(["--exact", "usage_child", "--ignored", "--nocapture"])
        .env("THINKTHEN_TEST_USAGE_REPORT", &report);
    let output = child::run::output(&mut seeded).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(
        fs::read_to_string(report).unwrap(),
        "Some(true) sent 2 cached 0"
    );
    assert_eq!(listener.count(), 3);
    let mut status = Command::new(crate::command::binary());
    configure(&mut status);
    status.args(["status", "--json"]);
    let output = child::run::output(&mut status).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        document["usage"]["total"],
        serde_json::json!({
            "requests_sent":3,"retries":1,"input_tokens":624,"output_tokens":96,"cache_answers":0
        })
    );
    fs::remove_dir_all(home).unwrap();
}
