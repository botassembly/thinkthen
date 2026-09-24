//! `thinkthen check` against the loopback backend, with a fake key.
//!
//! Every run goes through `check`. It proves the key's bytes reach neither
//! output stream nor any file the run wrote under its cache folder.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::atomic::{AtomicUsize, Ordering};

use conformance_backend::Backend;

use crate::harness::spawn;

const KEY: &str = "sk-check-0121";

const PASS: &str = "ok connection\nok key\nok endpoint\nok noul\nok choice\nok score\nok mixed\nok usage\ncritical 0, warning 0\n";

const REFUSED: &str = "the backend answered with status 422: the backend refused the request as malformed or too large";

/// Run `check` with the arguments and environment, and hold it to the key rule.
fn check(arguments: &[&str], environment: &[(&str, &str)]) -> Output {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let cache = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "check-cache-{}-{}",
        std::process::id(),
        RUNS.fetch_add(1, Ordering::Relaxed)
    ));
    let cache_text = cache.to_str().expect("a UTF-8 path");
    let environment = [environment, &[("XDG_CACHE_HOME", cache_text)]].concat();
    let output = spawn(&[&["check"], arguments].concat(), &environment, b"").expect("runs");
    let streams = [("stdout", &output.stdout), ("stderr", &output.stderr)];
    let streams = streams.map(|(name, bytes)| (name.to_owned(), bytes.clone()));
    for (place, bytes) in streams.into_iter().chain(files(&cache)) {
        let held = bytes
            .windows(KEY.len())
            .any(|window| window == KEY.as_bytes());
        assert!(!held, "the key reached {place}");
    }
    output
}

/// Every file under a folder, by path, with its bytes.
fn files(folder: &Path) -> Vec<(String, Vec<u8>)> {
    let paths = fs::read_dir(folder).into_iter().flatten().flatten();
    let file = |path: PathBuf| match path.is_dir() {
        true => files(&path),
        false => vec![(
            path.display().to_string(),
            fs::read(&path).expect("readable"),
        )],
    };
    paths.flat_map(|entry| file(entry.path())).collect()
}

/// Run a live check at one arm with the key set.
fn at(backend: &Backend, path: &str) -> Output {
    let url = format!("{}{path}", backend.origin());
    check(&["--url", url.as_str()], &[("THINKTHEN_API_KEY", KEY)])
}

/// The report's two opening lines for one arm at the default model.
fn head(backend: &Backend, path: &str) -> String {
    format!(
        "url {}{path}/systemone\nmodel jev-latest\n",
        backend.origin()
    )
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_backend_that_carries_every_field_passes_on_four_requests() {
    let backend = Backend::start().expect("backend");
    let output = at(&backend, "/arm/full/v1");
    assert_eq!(text(&output.stdout), head(&backend, "/arm/full/v1") + PASS);
    assert_eq!(text(&output.stderr), "");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(backend.count(), 4);
}

#[test]
fn missing_confidence_and_token_counts_warn_and_keep_exit_zero() {
    let backend = Backend::start().expect("backend");
    let output = at(&backend, "/generic/v1");
    let report = "ok connection\nok key\nok endpoint\nok noul\n\
        warning choice: the answer to question `q1` carries no confidence\n\
        warning score: the answer to question `q1` carries no confidence\n\
        warning mixed: the answer to question `q2` carries no confidence\n\
        warning mixed: the answer to question `q3` carries no confidence\n\
        warning usage: a reply carries no token counts, so results and usage totals leave them out\n\
        critical 0, warning 5\n";
    assert_eq!(text(&output.stdout), head(&backend, "/generic/v1") + report);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(backend.count(), 4);
}

#[test]
fn a_missing_answer_is_critical_in_every_probe_and_names_the_tag_range() {
    let backend = Backend::start().expect("backend");
    let path = "/arm/malformed/missing_answer/v1";
    let output = at(&backend, path);
    let refused = "the reply was refused: the response carries no answer for question `q1`";
    let report = format!(
        "ok connection\nok key\nok endpoint\n\
        critical noul: {refused}\ncritical choice: {refused}\ncritical score: {refused}\n\
        warning mixed: the answer to question `q2` carries no confidence\n\
        warning mixed: the answer to question `q3` carries no confidence\n\
        critical mixed: the answer to questions `q4` to `q5` failed as `missing_answer`\n\
        warning usage: a reply carries no token counts, so results and usage totals leave them out\n\
        critical 4, warning 3\n"
    );
    assert_eq!(text(&output.stdout), head(&backend, path) + &report);
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(backend.count(), 4);
}

#[test]
fn a_refused_body_is_critical_for_its_probe_and_the_check_goes_on() {
    let backend = Backend::start().expect("backend");
    let output = at(&backend, "/arm/refuse/v1");
    let report = format!(
        "ok connection\nok key\nok endpoint\n\
        critical noul: {REFUSED}\ncritical choice: {REFUSED}\n\
        critical score: {REFUSED}\ncritical mixed: {REFUSED}\n\
        unchecked usage\ncritical 4, warning 0\n"
    );
    assert_eq!(
        text(&output.stdout),
        head(&backend, "/arm/refuse/v1") + &report
    );
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(backend.count(), 4);
}

#[test]
fn a_failure_no_body_causes_stops_the_check_at_the_first_probe() {
    let later = "unchecked noul\nunchecked choice\nunchecked score\nunchecked mixed\nunchecked usage\ncritical 1, warning 0\n";
    let key = |said: &str| {
        format!(
            "ok connection\ncritical key: the backend answered with status {said}\nunchecked endpoint\n{later}"
        )
    };
    let stops = [
        ("/arm/status/401/v1", key("401: the key was refused")),
        ("/arm/status/402/v1", key("402: the account has no credit")),
        (
            "/arm/status/403/v1",
            key("403: the key may not use this model or address"),
        ),
        (
            "/arm/status/404/v1",
            format!(
                "ok connection\nunchecked key\ncritical endpoint: the backend answered with status 404: nothing answers at this address\n{later}"
            ),
        ),
        (
            "/arm/reset/v1",
            format!(
                "critical connection: the backend closed the connection before a reply and may have received the request; it was not sent again\nunchecked key\nunchecked endpoint\n{later}"
            ),
        ),
    ];
    for (path, report) in stops {
        let backend = Backend::start().expect("backend");
        let output = at(&backend, path);
        assert_eq!(
            text(&output.stdout),
            head(&backend, path) + &report,
            "{path}"
        );
        assert_eq!(output.status.code(), Some(4), "{path}");
        assert_eq!(backend.count(), 1, "{path}");
    }
}

#[test]
fn the_check_sends_only_to_the_address_the_user_named() {
    let (named, beside) = (
        Backend::start().expect("backend"),
        Backend::start().expect("backend"),
    );
    let url = format!("{}/arm/full/v1", named.origin());
    let base = format!("{}/arm/full/v1", beside.origin());
    let environment = [
        ("THINKTHEN_API_KEY", KEY),
        ("THINKTHEN_BASE_URL", base.as_str()),
    ];
    let output = check(&["--url", url.as_str()], &environment);
    assert_eq!(text(&output.stdout), head(&named, "/arm/full/v1") + PASS);
    assert_eq!((named.count(), beside.count()), (4, 0));

    // No key is set, so a regression that reached the built-in address still sends nothing.
    let output = check(&[], &[]);
    assert_eq!(
        text(&output.stderr),
        "thinkthen: check needs an address you name: give --url, set THINKTHEN_BASE_URL, or set url in the configuration file\n"
    );
    assert_eq!(text(&output.stdout), "");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn an_unset_key_stops_the_check_before_any_request() {
    let backend = Backend::start().expect("backend");
    let url = format!("{}/arm/full/v1", backend.origin());
    let output = check(&["--url", url.as_str()], &[]);
    assert_eq!(
        text(&output.stderr),
        "thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent\n"
    );
    assert_eq!(text(&output.stdout), "");
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(backend.count(), 0);
}

#[test]
fn a_dry_run_prints_the_four_fixed_bodies_and_sends_nothing() {
    let backend = Backend::start().expect("backend");
    let url = format!("{}/arm/full/v1", backend.origin());
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/fixtures/check/requests.jsonl"
    );
    let bodies = fs::read_to_string(fixture).expect("the fixture");
    let requests: String = ["noul", "choice", "score", "mixed"]
        .iter()
        .zip(bodies.lines())
        .map(|(probe, body)| format!("request {probe} {body}\n"))
        .collect();
    // Keyless, a dry run that read the key would fail. Keyed, the helper
    // proves the dry run's output carries no key.
    for environment in [&[][..], &[("THINKTHEN_API_KEY", KEY)][..]] {
        let output = check(&["--url", url.as_str(), "--dry-run"], environment);
        let printed = head(&backend, "/arm/full/v1") + &requests;
        assert_eq!(text(&output.stdout), printed);
        assert_eq!(text(&output.stderr), "");
        assert_eq!(output.status.code(), Some(0));
    }
    assert_eq!(backend.count(), 0);
}
