//! An explicit zero send limit cannot bind an empty default cache.

use super::*;

pub(super) fn run_default_cache(argument: &str) -> Vec<String> {
    let [first, second] = argument.splitn(2, '|').collect::<Vec<_>>()[..] else {
        panic!("two endpoint addresses");
    };
    let cache = Path::new(&std::env::var("XDG_CACHE_HOME").expect("cache home")).join("thinkthen");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
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
        entries(&cache)
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

#[test]
fn zero_budget_sends_nothing_and_the_default_cache_serves_the_next_address() {
    const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    for (sent, (name, present)) in [("absent", false), ("present-empty", true)]
        .into_iter()
        .enumerate()
    {
        let home = folder(&format!("zero-budget-{name}"));
        let cache = home.join("thinkthen");
        if present {
            fs::create_dir_all(&cache).expect("empty cache");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                fs::set_permissions(&cache, fs::Permissions::from_mode(0o700))
                    .expect("private default cache");
            }
        }
        let addresses = format!("{}|{}", first.base(), second.base());
        let lines = in_child(
            "zero-budget-default-cache",
            &[
                ("XDG_CACHE_HOME", home.to_str().expect("cache home")),
                (ARGUMENT, &addresses),
            ],
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

/// ADR 0111 section 3 withdrew the folder marker: the address sits in every
/// question key. A folder another address filled, an old entry and a bad
/// marker all miss, so a zero budget refuses before any key read or send.
#[test]
fn zero_budget_refuses_beside_another_address_old_entries_and_a_bad_marker() {
    const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let budget = SendBudget::new();
    let engine = |base: &str, path: &Path, key: &str| {
        Engine::builder()
            .base_url(base)
            .and_then(|builder| builder.model("local-1"))
            .and_then(|builder| builder.api_key(key))
            .and_then(|builder| builder.cache_at(path))
            .and_then(EngineBuilder::build)
            .expect("engine")
    };
    let other = folder("zero-budget-other-address");
    engine(first.base(), &other, "sk-test")
        .decide(&question, EVIDENCE)
        .expect("first address fills cache");
    let old = folder("zero-budget-old-entry");
    fs::create_dir_all(&old).expect("old folder");
    fs::write(old.join(format!("{}.json", "a".repeat(64))), b"old entry").expect("old entry");
    let marked = folder("zero-budget-bad-marker");
    fs::create_dir_all(&marked).expect("marked folder");
    fs::write(
        marked.join(".thinkthen-backend.json"),
        b"secret invalid marker",
    )
    .expect("bad marker");
    for (name, path) in [("other", &other), ("old", &old), ("marked", &marked)] {
        let denied = engine(second.base(), path, "first\nsecond")
            .decide_with(
                &question,
                EVIDENCE,
                CallOptions::new().send_budget(&budget, Some(0)),
            )
            .expect_err("a zero budget refuses the miss");
        assert_eq!(denied.kind(), ErrorKind::Usage, "{name}");
        assert_eq!(
            denied.send_budget_denial(),
            Some(SendBudgetDenial::BeforeFirstSend),
            "{name}"
        );
        assert!(!denied.to_string().contains("secret"), "{name}");
    }
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
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let engine = Engine::builder()
        .base_url(listener.base())
        .and_then(|builder| builder.model("local-1"))
        .and_then(|builder| builder.api_key("first\nsecond"))
        .and_then(|builder| builder.cache_at(&cache))
        .and_then(EngineBuilder::build)
        .expect("engine");
    let budget = SendBudget::new();
    let denied = engine
        .decide_with(
            &question,
            EVIDENCE,
            CallOptions::new().send_budget(&budget, Some(0)),
        )
        .expect_err("zero budget wins before writable admission");
    assert_eq!(denied.kind(), ErrorKind::Usage);
    assert_eq!(
        denied.send_budget_denial(),
        Some(SendBudgetDenial::BeforeFirstSend)
    );
    assert!(!cache.exists());
    assert_eq!(listener.count(), 0);
}
