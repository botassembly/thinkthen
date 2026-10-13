//! An explicit zero send limit cannot bind an empty default cache.

use super::*;

const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

pub(super) fn run_default_cache(argument: &str) -> Vec<String> {
    let [first, second, cache] = argument.splitn(3, '|').collect::<Vec<_>>()[..] else {
        panic!("two endpoint addresses and the default cache folder");
    };
    let cache = Path::new(cache);
    let question = refund_question();
    let build = |base: &str, key: bool| {
        let builder = Engine::builder()
            .base_url(base)
            .and_then(|builder| builder.model("local-1"))
            .expect("engine settings")
            .default_cache();
        if key {
            builder.api_key("sk-test").expect("key").build()
        } else {
            builder.build()
        }
        .expect("engine")
    };
    let budget = SendBudget::new();
    let denied = build(first, true)
        .decide_with(
            &question,
            EVIDENCE,
            CallOptions::new().send_budget(&budget, Some(0)),
        )
        .expect_err("zero limit refuses before first send");
    let before = format!(
        "{:?}|{:?}|{}|{}",
        denied.kind(),
        denied.send_budget_denial(),
        cache.exists(),
        entries(cache)
    );
    let bound = build(second, true)
        .decide(&question, EVIDENCE)
        .expect("second address binds and answers");
    let hit = build(second, false)
        .decide_with(
            &question,
            EVIDENCE,
            CallOptions::new().send_budget(&budget, Some(0)),
        )
        .expect("bound hit needs no key or send");
    vec![
        before,
        format!(
            "{}|{}|{}",
            bound.facts().requests_sent(),
            hit.facts().requests_sent(),
            hit.facts().cache_answers()
        ),
    ]
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn zero_budget_sends_nothing_and_the_default_cache_serves_the_next_address() {
    let first = crate::api::answering(ANSWER);
    let second = crate::api::answering(ANSWER);
    for (sent, (name, present)) in [("absent", false), ("present-empty", true)]
        .into_iter()
        .enumerate()
    {
        let home = folder(&format!("zero-budget-{name}"));
        let cache = crate::child::Folder::Cache.under(&home);
        if present {
            fs::create_dir_all(&cache).expect("empty cache");
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&cache, fs::Permissions::from_mode(0o700))
                .expect("private default cache");
        }
        let addresses = format!("{}|{}|{}", first.base(), second.base(), cache.display());
        let moved = crate::child::Folder::Cache.variable(&home);
        let lines = in_child(
            "zero-budget-default-cache",
            &[(moved.0, moved.1.as_str()), (ARGUMENT, &addresses)],
        );
        let expected = format!("Usage|Some(BeforeFirstSend)|{present}|0\n1|0|1");
        assert_eq!(lines, expected, "{name}");
        assert_eq!(first.count(), 0, "{name}: refused request sent nothing");
        assert_eq!(
            second.count(),
            sent + 1,
            "{name}: each fresh cache sends once"
        );
        assert!(cache.join("thinkthen.sqlite").is_file(), "{name}");
    }
}

/// Another address and a bad marker miss; an old store refuses before budget
/// admission. None sends or changes old files.
#[test]
fn zero_budget_refuses_beside_another_address_old_entries_and_a_bad_marker() {
    let first = crate::api::answering(ANSWER);
    let second = crate::api::answering(ANSWER);
    let question = refund_question();
    let other = folder("zero-budget-other-address");
    cache_engine(first.base(), &other, "sk-test")
        .decide(&question, EVIDENCE)
        .expect("first address fills cache");
    let old = folder("zero-budget-old-entry");
    fs::create_dir_all(&old).expect("old folder");
    let old_file = old.join(format!("{}.json", "a".repeat(64)));
    fs::write(&old_file, b"old entry").expect("old entry");
    let marked = folder("zero-budget-bad-marker");
    fs::create_dir_all(&marked).expect("marked folder");
    fs::write(
        marked.join(".thinkthen-backend.json"),
        b"secret invalid marker",
    )
    .expect("bad marker");
    for (name, path) in [("other", &other), ("old", &old), ("marked", &marked)] {
        let denied = denied_cache(second.base(), path);
        let (kind, denial) = if name == "old" {
            assert_eq!(
                denied.to_string(),
                format!(
                    "{}.json uses an old store format; delete it",
                    "a".repeat(64)
                )
            );
            (ErrorKind::Local, None)
        } else {
            (ErrorKind::Usage, Some(SendBudgetDenial::BeforeFirstSend))
        };
        assert_eq!(denied.kind(), kind, "{name}");
        assert_eq!(denied.send_budget_denial(), denial, "{name}");
        assert_eq!(
            denied.facts().expect("started call").requests_sent(),
            0,
            "{name}"
        );
        assert!(!denied.to_string().contains("secret"), "{name}");
    }
    assert_eq!(fs::read(&old_file).expect("old entry"), b"old entry");
    assert_eq!(entries(&old), 1);
    assert_eq!(first.count(), 1);
    assert_eq!(second.count(), 0);
    assert_eq!(
        fs::read(marked.join(".thinkthen-backend.json")).expect("marker"),
        b"secret invalid marker"
    );
}

#[test]
fn zero_budget_still_precedes_a_line_break_key_on_first_use() {
    let listener = Listener::answering(|_| Canned::ok("unused")).expect("listener");
    let cache = folder("zero-budget-line-break-key");
    let denied = denied_cache(listener.base(), &cache);
    assert_eq!(denied.kind(), ErrorKind::Usage);
    assert_eq!(
        denied.send_budget_denial(),
        Some(SendBudgetDenial::BeforeFirstSend)
    );
    assert!(!cache.exists());
    assert_eq!(listener.count(), 0);
}

fn denied_cache(base: &str, cache: &Path) -> Error {
    let budget = SendBudget::new();
    cache_engine(base, cache, "first\nsecond")
        .decide_with(
            &refund_question(),
            EVIDENCE,
            CallOptions::new().send_budget(&budget, Some(0)),
        )
        .expect_err("cache admission or zero budget refuses before the key read")
}

fn refund_question() -> Question {
    Question::decide("asks for a refund")
        .expect("question")
        .cut()
}

fn cache_engine(base: &str, cache: &Path, key: &str) -> Engine {
    crate::api::builder(base, key)
        .model("local-1")
        .and_then(|builder| builder.cache_at(cache))
        .and_then(EngineBuilder::build)
        .expect("engine")
}
