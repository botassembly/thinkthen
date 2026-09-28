//! `EngineBuilder::from_env` in child processes, each from a cleared environment.
//!
//! No test changes its own environment. Each case re-runs this test binary as
//! a child with only the variables the case names, a fake key only beside a
//! loopback base, and reads the lines the child writes. Listeners count every
//! send, so a case that expects none proves none.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a failed fixture or child stops the proof"
)]

#[path = "public_env/batch.rs"]
mod batch;
#[path = "public_env/cache_budget.rs"]
mod cache_budget;

#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use conformance_backend::{Canned, Listener};
use thinkthen::{
    BatchSetting, CallOptions, Engine, EngineBuilder, Error, ErrorKind, Question, SendBudget,
    SendBudgetDenial,
};

#[test]
fn process_send_budget_distinguishes_first_send_retry_and_backend_status() {
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let budget = SendBudget::new();
    let listener =
        Listener::answering(|_| Canned::status(503, "busy").asking("retry-after-ms", "0"))
            .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("key")
        .max_retries(1)
        .no_cache()
        .build()
        .expect("engine");
    let first = engine
        .decide_with(
            &question,
            "evidence",
            CallOptions::new().send_budget(&budget, Some(0)),
        )
        .expect_err("zero-cap first send");
    assert_eq!(first.kind(), ErrorKind::Usage);
    assert_eq!(
        first.send_budget_denial(),
        Some(SendBudgetDenial::BeforeFirstSend)
    );
    assert_eq!(listener.count(), 0);

    let retry = engine
        .decide_with(
            &question,
            "evidence",
            CallOptions::new().send_budget(&budget, Some(1)),
        )
        .expect_err("one send followed by a denied retry");
    assert_eq!(retry.kind(), ErrorKind::Backend);
    assert!(!retry.retryable());
    assert_eq!(
        retry.send_budget_denial(),
        Some(SendBudgetDenial::BeforeRetry { last_status: 503 })
    );
    assert_eq!(listener.count(), 1);

    let spent = engine
        .decide_with(
            &question,
            "evidence",
            CallOptions::new().send_budget(&budget, Some(1)),
        )
        .expect_err("shared budget spent");
    assert_eq!(
        spent.send_budget_denial(),
        Some(SendBudgetDenial::BeforeFirstSend)
    );
    assert_eq!(listener.count(), 1);

    let no_retry = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("key")
        .max_retries(0)
        .no_cache()
        .build()
        .expect("engine without retries");
    let ordinary = no_retry
        .decide_with(
            &question,
            "evidence",
            CallOptions::new().send_budget(&budget, Some(2)),
        )
        .expect_err("backend status after raised cap");
    assert_eq!(ordinary.kind(), ErrorKind::Backend);
    assert_eq!(ordinary.send_budget_denial(), None);
    assert!(ordinary.retryable());
    assert_eq!(listener.count(), 2);
}

#[test]
fn inline_profile_replaces_a_file_and_rejects_invalid_json_as_usage() {
    let profile =
        r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":8}"#;
    let missing = folder("missing-profile.json");
    let from_inline = Engine::builder()
        .no_cache()
        .profile(&missing)
        .expect("path")
        .profile_json(profile)
        .expect("inline profile")
        .build();
    assert!(
        from_inline.is_ok(),
        "the later inline profile replaces the path"
    );
    let invalid = Engine::builder().profile_json("{}");
    assert!(matches!(invalid, Err(Error::Usage(_))));
    let from_file = Engine::builder()
        .no_cache()
        .profile_json(profile)
        .expect("inline profile")
        .profile(&missing)
        .expect("path")
        .build();
    assert!(matches!(from_file, Err(Error::Local(_))));
}

/// The one answer every listener gives: a yes at 0.92.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);
const CASE: &str = "PUBLIC_ENV_CASE";
const ARGUMENT: &str = "PUBLIC_ENV_ARGUMENT";
const EVIDENCE: &str = "Refund me please.";

fn listener() -> Listener {
    Listener::answering(|_| Canned::ok(ANSWERED)).expect("a loopback listener")
}

fn folder(name: &str) -> PathBuf {
    static MADE: AtomicUsize = AtomicUsize::new(0);
    let made = MADE.fetch_add(1, Ordering::Relaxed);
    let path = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("public-env-{}-{made}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    path
}

fn entries(path: &Path) -> usize {
    fs::read_dir(path).map_or(0, Iterator::count)
}

/// Run one child case with only these variables, and return what it wrote.
fn in_child(case: &str, environment: &[(&str, &str)]) -> String {
    let output = run::output(
        Command::new(std::env::current_exe().expect("this test binary"))
            .args(["child_case", "--exact", "--ignored", "--test-threads=1"])
            .env_clear()
            .env(CASE, case)
            .envs(environment.iter().copied()),
    )
    .expect("the child runs");
    let written = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{case}: {written}");
    written
        .lines()
        .filter_map(|line| line.split_once("child: ").map(|(_, said)| said))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The child: one named case, reported line by line.
#[test]
#[ignore = "the child half; its parent runs it with --ignored"]
fn child_case() {
    let Ok(case) = std::env::var(CASE) else {
        return;
    };
    let argument = std::env::var(ARGUMENT).unwrap_or_default();
    let mut out = std::io::stdout().lock();
    for line in run(&case, &argument) {
        // One output line per result, so a pretty debug line reaches the parent whole.
        let line = line.replace('\n', "\\n");
        writeln!(out, "child: {line}").expect("standard output");
    }
}

fn shown<T>(result: Result<T, Error>) -> String {
    result.map_or_else(
        |error| format!("{:?}: {error}", error.kind()),
        |_| "ok".to_owned(),
    )
}

fn ask(engine: &Engine) -> String {
    let question = Question::decide("asks for a refund")
        .expect("a question")
        .cut();
    match engine.details(&question, EVIDENCE) {
        Ok(call) => {
            let details = call.value();
            format!(
                "sent {} cached {} model {} digests {:?} sha {}",
                details.requests_sent(),
                details.cached(),
                details.model(),
                details.requests(),
                details.question_sha256()
            )
        }
        Err(error) => format!("{:?}: {error}", error.kind()),
    }
}

fn run(case: &str, argument: &str) -> Vec<String> {
    let seed = || EngineBuilder::from_env().expect("a seed");
    match case {
        "second-base" => vec![ask(&seed()
            .base_url(argument)
            .unwrap()
            .no_cache()
            .build()
            .unwrap())],
        "oracle" => {
            let seeded = ask(&seed().build().expect("the seeded engine"));
            let [base, key, model, cache] = argument.splitn(4, '|').collect::<Vec<_>>()[..] else {
                panic!("four settings");
            };
            let explicit = Engine::builder()
                .base_url(base)
                .and_then(|b| b.api_key(key))
                .and_then(|b| b.model(model))
                .and_then(|b| b.cache_at(cache))
                .and_then(EngineBuilder::build)
                .expect("the explicit engine");
            vec![seeded, ask(&explicit)]
        }
        "throttle" => vec![
            shown(seed().no_cache().build()),
            shown(Engine::from_env()),
            shown(seed().throttle(8).and_then(|b| b.no_cache().build())),
            shown(seed().throttle(8).and_then(|b| b.no_cache().build())),
            shown(seed().throttle(4).and_then(|b| b.no_cache().build())),
        ],
        "overrides" => overrides(argument),
        "refused" => vec![shown(EngineBuilder::from_env())],
        "batch-env" => {
            let engine = seed().no_cache().build().expect("environment batch");
            let question = Question::decide("Refund?").unwrap().cut();
            let rows = engine
                .decide_many(&question, ["alpha", "beta"])
                .collect::<Result<Vec<_>, _>>()
                .expect("two rows");
            vec![format!("rows {}", rows.len())]
        }
        "batch-override" => {
            let refused = shown(seed().no_cache().build());
            let engine = seed()
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
                .no_cache()
                .build()
                .expect("explicit batch overrides an invalid environment");
            let question = Question::decide("Refund?").unwrap().cut();
            let rows = engine
                .decide_many(&question, ["gamma"])
                .collect::<Result<Vec<_>, _>>()
                .expect("explicit batch row");
            vec![refused, format!("rows {}", rows.len())]
        }
        "batch-conflict" => {
            let engine = seed()
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
                .no_cache()
                .build()
                .expect("explicit batch outranks environment max");
            let question = Question::decide("Refund?").unwrap().cut();
            let rows = engine
                .decide_many(&question, ["delta", "epsilon"])
                .collect::<Result<Vec<_>, _>>()
                .expect("two explicit batch rows");
            vec![format!("rows {}", rows.len())]
        }
        "no-home" => vec![
            shown(seed().no_cache().build()),
            shown(seed().build()),
            shown(Engine::from_env()),
        ],
        "secrecy" => secrecy(argument),
        "effects" => {
            let builder = seed();
            let seeded = entries(Path::new(argument));
            let engine = builder.build().unwrap();
            vec![format!(
                "seeded {seeded} built {}",
                engine.usage().requests_sent()
            )]
        }
        "zero-budget-default-cache" => cache_budget::run_default_cache(argument),
        _ => panic!("no child case {case}"),
    }
}

fn secrecy(argument: &str) -> Vec<String> {
    let seeded = EngineBuilder::from_env().expect("a seed");
    let mut lines = vec![format!("{seeded:?}"), format!("{seeded:#?}")];
    let builder = seeded.api_key(argument).unwrap();
    lines.extend([format!("{builder:?}"), format!("{builder:#?}")]);
    let engine = builder.no_cache().build().unwrap();
    lines.extend([format!("{engine:?}"), format!("{engine:#?}")]);
    let question = Question::decide("asks for a refund").unwrap().cut();
    let error = engine
        .decide(&question, EVIDENCE)
        .expect_err("the backend refuses");
    lines.extend([
        format!("{error}"),
        format!("{error:?}"),
        format!("{error:#?}"),
    ]);
    lines
}

/// Each setting after the seed, observed through a call.
fn overrides(argument: &str) -> Vec<String> {
    let seed = || EngineBuilder::from_env().expect("a seed");
    let [second, at] = argument.splitn(2, '|').collect::<Vec<_>>()[..] else {
        panic!("two settings");
    };
    let moved = seed()
        .api_key("sk-key-b")
        .unwrap()
        .base_url(second)
        .unwrap();
    let moved = moved.model("model-override").unwrap().cache_at(at).unwrap();
    let limited = seed()
        .no_cache()
        .max_requests(Some(1))
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("asks for a refund").unwrap().cut();
    let kinds: Vec<String> = limited
        .decide_many(&question, ["one", "two"])
        .map(|row| row.map_or_else(|e| format!("{:?}", e.kind()), |_| "row".to_owned()))
        .collect();
    let uncached = seed().default_cache().no_cache().build().unwrap();
    vec![
        ask(&moved.build().unwrap()),
        kinds.join(" "),
        ask(&uncached),
        ask(&uncached),
        ask(&seed().no_cache().default_cache().build().unwrap()),
    ]
}

#[test]
fn r4_24_the_engine_base_url_outranks_the_environment_base() {
    let (first, second) = (listener(), listener());
    let said = in_child(
        "second-base",
        &[
            ("THINKTHEN_BASE_URL", first.base()),
            ("THINKTHEN_API_KEY", "sk-fake-loopback"),
            (ARGUMENT, second.base()),
        ],
    );
    assert!(said.starts_with("sent 1 cached false"), "{said}");
    assert_eq!((first.count(), second.count()), (0, 1));
}

#[test]
fn a_seeded_engine_equals_one_given_each_value_and_the_command_plan() {
    let served = listener();
    let (config, cache) = (folder("config"), folder("cache"));
    fs::create_dir_all(config.join("thinkthen")).unwrap();
    let file = r#"{"schema":"thinkthen.config/1","model":"model-from-config"}"#;
    fs::write(config.join("thinkthen/config.json"), file).unwrap();
    let cache_text = cache.to_str().unwrap();
    let environment = [
        ("THINKTHEN_BASE_URL", served.base()),
        ("THINKTHEN_API_KEY", "sk-fake-loopback"),
        ("THINKTHEN_CACHE", cache_text),
        ("XDG_CONFIG_HOME", config.to_str().unwrap()),
    ];
    let explicit = format!(
        "{}|sk-fake-loopback|model-from-config|{cache_text}",
        served.base()
    );
    let said = in_child(
        "oracle",
        &[&environment[..], &[(ARGUMENT, &explicit)]].concat(),
    );
    let [seeded, given] = said.lines().collect::<Vec<_>>()[..] else {
        panic!("two lines: {said}");
    };
    // One send, then the explicit engine answers from the same folder with the same digest.
    assert_eq!(
        seeded.replace("sent 1 cached false", "same"),
        given.replace("sent 0 cached true", "same")
    );
    assert!(seeded.starts_with("sent 1 cached false"), "{seeded}");
    assert_eq!(served.count(), 1);
    let request = served.requests().pop().expect("one request");
    assert_eq!(
        request.header("authorization"),
        Some("Bearer sk-fake-loopback")
    );
    assert!(String::from_utf8_lossy(&request.body).contains(r#""model":"model-from-config""#));
    assert!(
        entries(&cache) > 0,
        "the answer is cached under THINKTHEN_CACHE"
    );

    // The command plan needs the binary, which the library-only build lacks.
    #[cfg(feature = "cli")]
    {
        fs::write(config.join("evidence"), EVIDENCE).unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .args(["decide", "asks for a refund", "--dry-run"])
            .env_clear()
            .envs(environment)
            .stdin(fs::File::open(config.join("evidence")).unwrap())
            .stdout(std::process::Stdio::piped());
        let plan = wait::finish(command.spawn().expect("the command runs"), "the plan")
            .expect("the command ends");
        let plan = String::from_utf8_lossy(&plan.stdout);
        assert!(
            plan.contains(&format!(r#""url":"{}""#, served.url())),
            "{plan}"
        );
        assert!(plan.contains(r#""model":"model-from-config""#), "{plan}");
    }
}

#[test]
fn a_seed_leaves_the_throttle_omitted_and_an_explicit_one_registers_at_build() {
    let cache = folder("throttle");
    let said = in_child("throttle", &[("THINKTHEN_CACHE", cache.to_str().unwrap())]);
    let conflict = "Usage: throttle 8 is already active for this process; \
                    use throttle 8 or drop the throttle argument";
    assert_eq!(said, ["ok", "ok", "ok", "ok", conflict].join("\n"));
}

#[test]
fn settings_after_the_seed_take_effect() {
    let (first, second) = (listener(), listener());
    let (environment_cache, at) = (folder("env-cache"), folder("at"));
    let argument = format!("{}|{}", second.base(), at.display());
    let said = in_child(
        "overrides",
        &[
            ("THINKTHEN_BASE_URL", first.base()),
            ("THINKTHEN_API_KEY", "sk-key-a"),
            ("THINKTHEN_CACHE", environment_cache.to_str().unwrap()),
            (ARGUMENT, &argument),
        ],
    );
    let lines: Vec<&str> = said.lines().collect();
    assert!(lines[0].starts_with("sent 1 cached false"), "{said}");
    assert_eq!(
        lines[1], "row Usage",
        "max_requests refuses the second record"
    );
    assert!(
        lines[2].starts_with("sent 1 cached false") && lines[3].starts_with("sent 1 cached false")
    );
    assert!(lines[4].starts_with("sent 1 cached false"), "{said}");
    let moved = second.requests();
    assert_eq!(moved.len(), 1);
    assert_eq!(moved[0].header("authorization"), Some("Bearer sk-key-b"));
    assert!(String::from_utf8_lossy(&moved[0].body).contains(r#""model":"model-override""#));
    assert!(entries(&at) > 0 && entries(&environment_cache) > 0);
    assert_eq!(
        first.count(),
        4,
        "one limited row, two uncached asks, one default-cache ask"
    );
}
