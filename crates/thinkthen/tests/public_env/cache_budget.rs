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
fn zero_budget_leaves_an_unbound_default_cache_for_another_address() {
    const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    for (name, present) in [("absent", false), ("present-empty", true)] {
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
        assert_eq!(second.count(), if present { 2 } else { 1 }, "{name}");
        assert!(cache.join(".thinkthen-backend.json").is_file(), "{name}");
    }
}

#[test]
fn zero_budget_preserves_bound_and_unusable_folder_refusals() {
    const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let budget = SendBudget::new();
    let options = || CallOptions::new().send_budget(&budget, Some(0));
    let engine = |base: &str, path: &Path| {
        Engine::builder()
            .base_url(base)
            .and_then(|builder| builder.model("local-1"))
            .and_then(|builder| builder.api_key("sk-test"))
            .and_then(|builder| builder.cache_at(path))
            .and_then(EngineBuilder::build)
            .expect("engine")
    };

    let bound = folder("zero-budget-bound-mismatch");
    engine(first.base(), &bound)
        .decide(&question, EVIDENCE)
        .expect("first address fills cache");
    let marker = fs::read(bound.join(".thinkthen-backend.json")).expect("bound marker");
    let mismatch = engine(second.base(), &bound)
        .decide_with(&question, EVIDENCE, options())
        .expect_err("mismatch precedes zero-budget denial");
    assert_eq!(mismatch.kind(), ErrorKind::Local);
    assert_eq!(mismatch.send_budget_denial(), None);
    assert_eq!(
        mismatch.to_string(),
        "the recording folder belongs to another backend address"
    );
    assert_eq!(
        fs::read(bound.join(".thinkthen-backend.json")).unwrap(),
        marker
    );
    assert_eq!(first.count(), 1);
    assert_eq!(second.count(), 0);

    let legacy = folder("zero-budget-legacy");
    fs::create_dir_all(&legacy).expect("legacy folder");
    fs::write(
        legacy.join(format!("{}.json", "a".repeat(64))),
        b"old entry",
    )
    .expect("legacy final entry");
    let legacy_error = engine(second.base(), &legacy)
        .decide_with(&question, EVIDENCE, options())
        .expect_err("legacy refusal precedes budget");
    assert_eq!(legacy_error.kind(), ErrorKind::Local);
    assert_eq!(legacy_error.send_budget_denial(), None);
    assert!(
        legacy_error
            .to_string()
            .contains("predates backend binding")
    );
    assert!(!legacy.join(".thinkthen-backend.json").exists());

    let malformed = folder("zero-budget-malformed");
    fs::create_dir_all(&malformed).expect("malformed folder");
    fs::write(
        malformed.join(".thinkthen-backend.json"),
        b"secret invalid marker",
    )
    .expect("malformed marker");
    let malformed_error = engine(second.base(), &malformed)
        .decide_with(&question, EVIDENCE, options())
        .expect_err("malformed marker precedes budget");
    assert_eq!(malformed_error.kind(), ErrorKind::Local);
    assert_eq!(malformed_error.send_budget_denial(), None);
    assert!(!malformed_error.to_string().contains("secret"));
    assert_eq!(second.count(), 0);
}
