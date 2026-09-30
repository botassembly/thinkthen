//! The Rust builder: every pair of tiers, the engine setting included, in
//! child processes that each start from a cleared environment.

use std::io::Write as _;
use std::process::Command;

use conformance_backend::Listener;
use thinkthen::{Engine, EngineBuilder, Question};

use crate::support::{
    EXPLICIT, EXPLICIT_PLACE, Home, KEYLESS, LIQUID_BASE, Proxy, by_marker, listener, only, said,
};

const CASE: &str = "THINKTHEN_TEST_NAMED_CASE";

/// Build as the setters say, then ask once. Each setter is `name=value`; a
/// leading `bare` starts from `Engine::builder()` instead of `from_env`.
fn build_and_ask(setters: &str) -> Vec<String> {
    let mut steps = setters
        .split(';')
        .filter(|step| !step.is_empty())
        .peekable();
    let mut builder = if steps.peek() == Some(&"bare") {
        steps.next();
        Engine::builder()
    } else {
        match EngineBuilder::from_env() {
            Ok(builder) => builder,
            Err(error) => return vec![format!("{:?}: {error}", error.kind())],
        }
    };
    for step in steps {
        let (name, value) = step.split_once('=').unwrap_or((step, ""));
        let value = if value == "EXPLICIT" { EXPLICIT } else { value };
        let next = match name {
            "backend" => builder.backend(value),
            "base_url" => builder.base_url(value),
            "api_key" => builder.api_key(value),
            "model" => builder.model(value),
            "skip-ask" => Ok(builder),
            other => panic!("no setter {other}"),
        };
        builder = match next {
            Ok(builder) => builder,
            Err(error) => return vec![format!("{:?}: {error}", error.kind())],
        };
    }
    let debug = format!("debug {builder:?}");
    let engine = match builder.no_cache().build() {
        Ok(engine) => engine,
        Err(error) => return vec![debug, format!("{:?}: {error}", error.kind())],
    };
    if setters.contains("skip-ask") {
        return vec![debug, "built".to_owned()];
    }
    let question = Question::decide("a refund?").expect("a question").cut();
    let asked = match engine.details(&question, "alpha") {
        Ok(call) => format!("model {}", call.value().model()),
        Err(error) => format!("{:?}: {error}", error.kind()),
    };
    vec![debug, asked]
}

#[test]
#[ignore = "the child half; its parent runs it with --ignored"]
fn builder_child() {
    let Ok(setters) = std::env::var(CASE) else {
        return;
    };
    // A direct write, because the test harness captures `println!`.
    let mut out = std::io::stdout().lock();
    for line in build_and_ask(&setters) {
        writeln!(out, "child: {}", line.replace('\n', "\\n")).expect("standard output");
    }
}

/// Run one child with the home's variables and these changes.
fn in_child(home: &Home, setters: &str, changes: &[(&str, &str)]) -> Vec<String> {
    let mut command = Command::new(std::env::current_exe().expect("this test binary"));
    command
        .args([
            "builder::builder_child",
            "--exact",
            "--ignored",
            "--test-threads=1",
        ])
        .env_clear()
        .env(CASE, setters);
    for (name, value) in home.environment() {
        command.env(name, value);
    }
    for (name, value) in changes {
        if value.is_empty() {
            command.env_remove(name);
        } else {
            command.env(name, value);
        }
    }
    let output = crate::run::output(&mut command).expect("the child runs");
    let (stdout, _) = said(&output);
    assert!(output.status.success(), "{setters}: {stdout}");
    stdout
        .lines()
        .filter_map(|line| line.split_once("child: ").map(|(_, said)| said.to_owned()))
        .collect()
}

/// (setters, THINKTHEN_BACKEND, THINKTHEN_BASE_URL, configuration's backend,
/// configuration's url, listener letter or `-`, marker place, last line).
type Case = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    usize,
    &'static str,
);

const CASES: [Case; 17] = [
    // An engine setting outranks a captured variable and the configuration.
    (
        "backend=local-d1",
        "",
        "B",
        "",
        "",
        "D",
        4,
        "model local-model",
    ),
    (
        "base_url=A",
        "local-d1",
        "",
        "",
        "",
        "A",
        3,
        "model config-model",
    ),
    (
        "backend=local-d1",
        "",
        "",
        "",
        "C",
        "D",
        4,
        "model local-model",
    ),
    (
        "base_url=A",
        "",
        "",
        "liquid",
        "",
        "A",
        3,
        "model config-model",
    ),
    (
        "backend=local-d1",
        "liquid",
        "",
        "",
        "",
        "D",
        4,
        "model local-model",
    ),
    ("base_url=A", "", "B", "", "C", "A", 3, "model config-model"),
    (
        "backend=liquid;base_url=A",
        "local-d1",
        "B",
        "",
        "",
        "A",
        1,
        "model d1:free",
    ),
    (
        "backend=local-d1;model=asked-model",
        "",
        "",
        "",
        "",
        "D",
        4,
        "model asked-model",
    ),
    // `from_env` keeps the variable apart from the configuration's `url`.
    ("", "", "B", "", "C", "B", 3, "model config-model"),
    ("", "", "B", "liquid", "", "B", 3, "model config-model"),
    ("", "local-d1", "", "", "C", "D", 4, "model local-model"),
    ("", "liquid", "B", "local-d1", "", "B", 1, "model d1:free"),
    // An explicit key outranks every variable.
    (
        "backend=local-d1;api_key=EXPLICIT",
        "",
        "",
        "",
        "",
        "D",
        EXPLICIT_PLACE,
        "model local-model",
    ),
    // A bare builder knows the built-ins and reads no key.
    (
        "bare;backend=liquid;base_url=A;api_key=EXPLICIT",
        "",
        "",
        "",
        "",
        "A",
        EXPLICIT_PLACE,
        "model d1:free",
    ),
    (
        "bare;backend=liquid;base_url=A",
        "",
        "",
        "",
        "",
        "A",
        KEYLESS,
        "model d1:free",
    ),
    (
        "bare;backend=local-d1",
        "",
        "",
        "",
        "",
        "-",
        0,
        "Usage: unknown backend `local-d1`; the built-in backends are `liquid`, `ollama` and `typesafe`, and the configuration file may name more",
    ),
    (
        "backend=Local",
        "",
        "",
        "",
        "",
        "-",
        0,
        "Usage: a backend name uses 1 to 32 lowercase letters, digits, and hyphens",
    ),
];

#[test]
fn every_pair_of_tiers_on_the_builder_sends_the_selected_key_to_the_selected_address() {
    for (index, (setters, backend, base, config_backend, config_url, letter, place, last)) in
        CASES.into_iter().enumerate()
    {
        let listeners: Vec<Listener> = (0..4).map(|_| listener()).collect();
        let at = |value: &str| match value {
            "A" => listeners[0].base().to_owned(),
            "B" => listeners[1].base().to_owned(),
            "C" => listeners[2].base().to_owned(),
            "D" => listeners[3].base().to_owned(),
            other => other.to_owned(),
        };
        let home = Home::new("builder");
        let mut config = format!(
            r#"{{"schema":"thinkthen.config/1","model":"config-model","backends":{{"local-d1":{{"url":"{}","key_env":"LOCAL_D1_KEY","model":"local-model"}}}}"#,
            at("D")
        );
        if !config_backend.is_empty() {
            config.push_str(&format!(r#","backend":"{config_backend}""#));
        }
        if !config_url.is_empty() {
            config.push_str(&format!(r#","url":"{}""#, at(config_url)));
        }
        home.config(&(config + "}"));
        let setters = setters.replace("base_url=A", &format!("base_url={}", at("A")));
        let (backend, base) = (at(backend), at(base));
        let lines = in_child(
            &home,
            &setters,
            &[
                ("THINKTHEN_BACKEND", &backend),
                ("THINKTHEN_BASE_URL", &base),
            ],
        );
        assert_eq!(
            lines.last().map(String::as_str),
            Some(last),
            "{index}: {lines:?}"
        );
        for (name, listener) in ["A", "B", "C", "D"].iter().zip(&listeners) {
            let wanted = if *name == letter {
                only(place, 1)
            } else {
                [0; 8]
            };
            assert_eq!(by_marker(listener), wanted, "{index} {name}");
        }
        home.assert_no_marker_in_files();
    }
}

#[test]
fn the_builder_refuses_a_built_in_key_at_the_other_host_unless_the_key_is_explicit() {
    let proxy = Proxy::start();
    let home = Home::new("builder-refusals");
    home.config(r#"{"schema":"thinkthen.config/1","backends":{"remote":{"url":"https://backend.example/v1","key_env":"REMOTE_KEY","model":"m"}}}"#);
    let proxied = [("HTTPS_PROXY", proxy.url.as_str())];
    let refused = in_child(
        &home,
        &format!("backend=typesafe;base_url={LIQUID_BASE}"),
        &proxied,
    );
    assert_eq!(
        refused.last().map(String::as_str),
        Some(
            "Usage: backend `typesafe` reads `TYPESAFE_API_KEY`, the key of backend `typesafe`, which never goes to the address of backend `liquid`"
        )
    );
    let explicit = in_child(
        &home,
        &format!("backend=typesafe;base_url={LIQUID_BASE};api_key=EXPLICIT;skip-ask"),
        &proxied,
    );
    assert_eq!(
        explicit.last().map(String::as_str),
        Some("built"),
        "{explicit:?}"
    );
    let missing = in_child(&home, "backend=remote", &proxied);
    assert_eq!(
        missing.last().map(String::as_str),
        Some("Usage: no key is set; configure an API key for the engine (REMOTE_KEY)")
    );
    let debug = in_child(&home, "backend=liquid;skip-ask", &proxied);
    assert!(debug[0].starts_with("debug EngineBuilder {"), "{debug:?}");
    assert!(
        debug[0].contains("keys: 5"),
        "the primary key and four built-in keys: {debug:?}"
    );
    assert_eq!(
        proxy.count(),
        0,
        "no refusal and no build opened a connection"
    );
    home.assert_no_marker_in_files();
}
