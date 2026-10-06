//! Response storage instructions through actual public calls and the command.

use conformance_backend::{Canned, Listener};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use thinkthen::{Answer, BatchSetting, CallOptions, Engine, ErrorKind, Question};

const YES: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const NO: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.1}}}"#;
const BODY: &[u8] = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#;

struct Folder(PathBuf);

impl Folder {
    fn new() -> std::io::Result<Self> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "native-cache-policy-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(Self(path))
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _removed = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn repeated_fields_and_exceeded_parser_bounds_forbid_actual_cache_writes() {
    let limits = [
        vec!["max-age=30".to_owned(), "no-store".to_owned()],
        vec!["x".repeat(8193)],
        vec!["x".to_owned(); 9],
        vec![vec!["x"; 65].join(",")],
    ];
    for fields in limits {
        let folder = Folder::new().unwrap();
        let mut reply = Canned::ok(YES);
        for field in fields {
            reply = reply.asking("Cache-Control", &field);
        }
        let listener = Listener::serving(vec![reply, Canned::ok(NO)]).unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("fixture-storage-policy")
            .unwrap()
            .cache_at(&folder.0)
            .unwrap()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        assert_eq!(
            engine.decide(&question, "Refund me.").unwrap().value(),
            &Answer::Yes
        );
        assert!(!folder.0.join("thinkthen.sqlite").exists());
        assert_eq!(
            engine.decide(&question, "Refund me.").unwrap().value(),
            &Answer::No
        );
        let requests = listener.requests();
        assert_eq!(requests.len(), 2);
        assert!(requests.iter().all(|request| request.body == BODY));
    }
}

#[test]
fn explicit_record_forbids_every_good_answer_of_a_nonstorable_partial_packed_reply() {
    let folder = Folder::new().unwrap();
    let listener =
        Listener::answering(|_| Canned::ok(YES).asking("Cache-Control", "no-store")).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-storage-policy")
        .unwrap()
        .record(&folder.0)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).unwrap(),
    ));
    let mut batch = engine.decide_many_with(&question, ["alpha", "beta"], options);
    let errors = batch.by_ref().collect::<Vec<_>>();
    assert_eq!(errors.len(), 1); // The released batch stops at its first terminal failure.
    for row in errors {
        let error = row.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Local);
        assert_eq!(
            error.to_string(),
            "the reply forbids storage, so the recording cannot be written"
        );
    }
    assert_eq!(batch.facts().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    assert!(!folder.0.join("thinkthen.sqlite").exists());
    let requests = listener.requests();
    assert_eq!(requests[0].body, br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#);
}

#[test]
fn native_refresh_marks_status_retries_and_split_children_without_changing_request_bodies() {
    let listener = Listener::serving(vec![
        Canned::status(503, "{}").asking("retry-after-ms", "1"),
        Canned::status(413, "{}"),
        Canned::ok(YES),
        Canned::ok(YES),
    ])
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-storage-policy")
        .unwrap()
        .refresh_cache(true)
        .max_retries(1)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).unwrap(),
    ));
    let mut batch = engine.decide_many_with(&question, ["alpha", "beta"], options);
    assert_eq!(
        batch.by_ref().collect::<Result<Vec<_>, _>>().unwrap().len(),
        2
    );
    assert_eq!(batch.facts().unwrap().requests_sent(), 4);
    let requests = listener.requests();
    assert_eq!(requests.len(), 4);
    let parent = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice();
    let expected = [parent, parent,
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"}}}"#.as_slice(),
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice(),
    ];
    for (request, body) in requests.iter().zip(expected) {
        assert_eq!(request.header("Cache-Control"), Some("no-cache"));
        assert_eq!(request.body, body);
    }
}

#[test]
fn partial_nonstorable_refresh_evicts_accepted_answers_and_preserves_failed_question_history() {
    let folder = Folder::new().unwrap();
    let both = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::serving(vec![
        Canned::ok(both),
        Canned::ok(NO).asking("Cache-Control", "no-store"),
        Canned::ok(NO),
    ])
    .unwrap();
    let builder = || {
        Engine::builder()
            .base_url(listener.base())
            .and_then(|b| b.model("fixed"))
            .and_then(|b| b.api_key("fixture-storage-policy"))
            .and_then(|b| b.cache_at(&folder.0))
    };
    let engine = builder().unwrap().build().unwrap();
    let refresh = builder().unwrap().refresh_cache(true).build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = || {
        CallOptions::new().batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(2).unwrap(),
        ))
    };
    assert_eq!(
        engine
            .decide_many_with(&question, ["alpha", "beta"], options())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        2
    );
    let mut rows = refresh.decide_many_with(&question, ["alpha", "beta"], options());
    assert!(rows.next().unwrap().is_ok());
    assert_eq!(rows.next().unwrap().unwrap_err().kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    let failed = engine.decide(&question, "beta").unwrap();
    assert_eq!(failed.value(), &Answer::Yes);
    assert_eq!(failed.facts().requests_sent(), 0);
    let accepted = engine.decide(&question, "alpha").unwrap();
    assert_eq!(accepted.value(), &Answer::No);
    assert_eq!(accepted.facts().requests_sent(), 1);
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let parent = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice();
    assert_eq!(requests[0].body, parent);
    assert_eq!(requests[1].body, parent);
    assert_eq!(requests[2].body, br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"}}}"#);
}

#[test]
fn nonstorable_answers_succeed_without_reuse_and_valid_quoted_text_still_caches() {
    for header in ["No-StOrE", "private=\"unterminated", "max-age=30, no-store"] {
        let folder = Folder::new().unwrap();
        let listener = Listener::serving(vec![
            Canned::ok(YES).asking("Cache-Control", header),
            Canned::ok(NO),
        ])
        .unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("fixture-storage-policy")
            .unwrap()
            .cache_at(&folder.0)
            .unwrap()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        let first = engine.decide(&question, "Refund me.").unwrap();
        assert_eq!(first.value(), &Answer::Yes);
        let second = engine.decide(&question, "Refund me.").unwrap();
        assert_eq!(second.value(), &Answer::No);
        assert_eq!(second.facts().requests_sent(), 1);
        let held = engine.decide(&question, "Refund me.").unwrap();
        assert_eq!(held.value(), &Answer::No);
        assert_eq!(held.facts().requests_sent(), 0);
        let requests = listener.requests();
        assert_eq!(requests.len(), 2);
        assert!(requests.iter().all(|request| request.body == BODY));
    }
    let folder = Folder::new().unwrap();
    let listener = Listener::answering(|_| {
        Canned::ok(YES).asking("Cache-Control", "private=\"no-store\", max-age=30")
    })
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-storage-policy")
        .unwrap()
        .cache_at(&folder.0)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    assert_eq!(
        engine.decide(&question, "Refund me.").unwrap().value(),
        &Answer::Yes
    );
    assert_eq!(
        engine
            .decide(&question, "Refund me.")
            .unwrap()
            .facts()
            .requests_sent(),
        0
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn explicit_record_refuses_after_one_paid_send_with_started_facts_and_no_answer_write() {
    let folder = Folder::new().unwrap();
    let listener =
        Listener::answering(|_| Canned::ok(YES).asking("Cache-Control", "no-store")).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fixture-storage-policy")
        .unwrap()
        .record(&folder.0)
        .unwrap()
        .max_retries(3)
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine.decide(&question, "Refund me.").unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(
        error.to_string(),
        "the reply forbids storage, so the recording cannot be written"
    );
    let facts = error.facts().unwrap();
    assert_eq!(facts.requests_sent(), 1);
    assert!(facts.call_id().is_some());
    assert_eq!(listener.count(), 1);
    assert!(!folder.0.join("thinkthen.sqlite").exists());
    assert!(!folder.0.join("thinkthen.jsonl").exists());
}

fn command(
    listener: &Listener,
    folder: &Folder,
    options: &[&str],
) -> std::io::Result<std::process::Output> {
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("THINKTHEN_API_KEY", "fixture-storage-policy")
        .env("XDG_STATE_HOME", folder.0.join("state"))
        .env("LOCALAPPDATA", folder.0.join("state"))
        .args([
            "decide",
            "Refund?",
            "--model",
            "fixed",
            "--url",
            listener.base(),
            "--facts",
        ])
        .args(options)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::other("missing stdin"))?
        .write_all(b"Refund me.")?;
    child.wait_with_output()
}

#[test]
fn refresh_evicts_only_after_a_valid_nonstorable_reply_and_failed_refresh_keeps_the_old_answer() {
    for failed in [false, true] {
        let folder = Folder::new().unwrap();
        let listener = Listener::serving(vec![
            Canned::ok(YES),
            if failed {
                Canned::status(400, "{}")
            } else {
                Canned::ok(NO).asking("Cache-Control", "no-store")
            },
            Canned::ok(NO),
        ])
        .unwrap();
        let cache = folder.0.join("cache");
        let path = cache.to_str().unwrap();
        let first = command(&listener, &folder, &["--cache", path]).unwrap();
        assert_eq!(first.status.code(), Some(0));
        assert_eq!(first.stdout, b"true\n");
        let refreshed = command(&listener, &folder, &["--cache", path, "--refresh-cache"]).unwrap();
        assert_eq!(refreshed.status.code(), Some(if failed { 4 } else { 1 }));
        let again = command(&listener, &folder, &["--cache", path]).unwrap();
        assert_eq!(
            again.stdout,
            if failed {
                b"true\n".as_slice()
            } else {
                b"false\n".as_slice()
            }
        );
        let requests = listener.requests();
        assert_eq!(requests.len(), if failed { 2 } else { 3 });
        assert_eq!(requests[0].header("Cache-Control"), None);
        assert_eq!(requests[1].header("Cache-Control"), Some("no-cache"));
        assert!(requests.iter().all(|request| request.body == BODY));
    }
}

#[test]
fn command_record_refusal_reports_local_exit_and_final_started_facts_without_repeating_headers() {
    let folder = Folder::new().unwrap();
    let listener = Listener::answering(|_| {
        Canned::ok(YES)
            .asking("Cache-Control", "no-store")
            .asking("X-Private-Marker", "secret-sentinel")
    })
    .unwrap();
    let path = folder.0.join("record");
    let output = command(&listener, &folder, &["--record", path.to_str().unwrap()]).unwrap();
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    let lines: Vec<_> = stderr.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(
        lines[0],
        "thinkthen: the reply forbids storage, so the recording cannot be written"
    );
    let facts: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(facts["requests_sent"], 1);
    assert_eq!(facts["stopped"]["cause"], "local");
    assert!(facts["call_id"].as_str().is_some());
    assert!(!stderr.contains("secret-sentinel"));
    assert_eq!(listener.count(), 1);
    assert!(!path.join("thinkthen.sqlite").exists());
}
