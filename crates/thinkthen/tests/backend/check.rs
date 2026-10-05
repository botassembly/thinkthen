//! Both backend-check spellings against the loopback backend, with a fake key.
//!
//! Every run goes through `check`. It proves the key's bytes reach neither
//! output stream nor any file the run wrote under its cache folder.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::atomic::{AtomicUsize, Ordering};

use conformance_backend::Backend;

use crate::harness::spawn;
#[cfg(unix)]
use crate::harness::{Canned, Listener};

const KEY: &str = "sk-check-0121";

const PASS: &str = "ok connection\nok key\nok endpoint\nok noul\nok choice\nok score\nok mixed\nok usage\nanswered function decide\nanswered function choose\nanswered function tag\nanswered function score\nanswered function filter\nanswered function rank\nanswered function find\nanswered function annotate\nanswered function recognize\nanswered function relate\ncritical 0, warning 0\n";

/// The four decoded replies of `/arm/full/v1`, each answer by the arm's fixed rule.
const FULL_REPLIES: &str = concat!(
    r#"reply noul {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9}],"usage":{"input_tokens":1,"output_tokens":1}}"#,
    "\n",
    r#"reply choice {"model":"jev-1.13.0","answers":[{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.05,"Wednesday":0.05},"confidence":0.9}],"usage":{"input_tokens":1,"output_tokens":1}}"#,
    "\n",
    r#"reply score {"model":"jev-1.13.0","answers":[{"kind":"score","level":"fair","probabilities":{"fair":0.9,"good":0.05,"excellent":0.05},"confidence":0.9}],"usage":{"input_tokens":1,"output_tokens":1}}"#,
    "\n",
    r#"reply mixed {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9},{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.1},"confidence":0.9},{"kind":"score","level":"poor","probabilities":{"poor":0.9,"good":0.1},"confidence":0.9},{"kind":"tag","probabilities":{"on_time":0.9,"damaged":0.9}}],"usage":{"input_tokens":1,"output_tokens":1}}"#,
    "\n",
);

/// The generic arm's replies carry no confidence and no usage.
const GENERIC_REPLIES: &str = concat!(
    r#"reply noul {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9}],"usage":null}"#,
    "\n",
    r#"reply choice {"model":"jev-1.13.0","answers":[{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.05,"Wednesday":0.05}}],"usage":null}"#,
    "\n",
    r#"reply score {"model":"jev-1.13.0","answers":[{"kind":"score","level":"fair","probabilities":{"fair":0.9,"good":0.05,"excellent":0.05}}],"usage":null}"#,
    "\n",
    r#"reply mixed {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9},{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.1}},{"kind":"score","level":"poor","probabilities":{"poor":0.9,"good":0.1}},{"kind":"tag","probabilities":{"on_time":0.9,"damaged":0.9}}],"usage":null}"#,
    "\n",
);

const REFUSED: &str = "the backend answered with status 422: the backend refused the request as malformed or too large";

const FUNCTIONS: [&str; 10] = [
    "decide",
    "choose",
    "tag",
    "score",
    "filter",
    "rank",
    "find",
    "annotate",
    "recognize",
    "relate",
];

fn failures(mut sentence: impl FnMut(&str) -> String) -> String {
    FUNCTIONS
        .iter()
        .map(|name| format!("incompatible function {name}: {}\n", sentence(name)))
        .collect()
}

/// Run `check` with the arguments and environment, and hold it to the key rule.
fn check(command: &[&str], arguments: &[&str], environment: &[(&str, &str)]) -> Output {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let cache = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "check-cache-{}-{}",
        std::process::id(),
        RUNS.fetch_add(1, Ordering::Relaxed)
    ));
    // The case's own variables come last, so a case that names its own home
    // keeps it, and the scan below reads that home too.
    let moved = crate::child::Folder::Cache.variable(&cache);
    let environment = [&[(moved.0, moved.1.as_str())], environment].concat();
    let output = spawn(&[command, arguments].concat(), &environment, b"").expect("runs");
    let streams = [("stdout", &output.stdout), ("stderr", &output.stderr)];
    let streams = streams.map(|(name, bytes)| (name.to_owned(), bytes.clone()));
    let homes = environment
        .iter()
        .filter(|(name, _)| *name == "HOME")
        .flat_map(|(_, home)| files(Path::new(home)));
    for (place, bytes) in streams.into_iter().chain(files(&cache)).chain(homes) {
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
fn at(command: &[&str], backend: &Backend, path: &str) -> Output {
    let url = format!("{}{path}", backend.origin());
    check(
        command,
        &["--url", url.as_str()],
        &[("THINKTHEN_API_KEY", KEY)],
    )
}

/// The report's four opening lines for one arm when no model is named.
fn head(backend: &Backend, path: &str) -> String {
    format!(
        "url {}{path}/systemone\nprovider systemone\nmodel asked unspecified\nmodel sent jev-1.13.0\n",
        backend.origin()
    )
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_backend_that_carries_every_field_passes_the_rich_probes_and_ten_functions() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let backend = Backend::start().expect("backend");
        let output = at(command, &backend, "/arm/full/v1");
        assert_eq!(
            text(&output.stdout),
            head(&backend, "/arm/full/v1") + FULL_REPLIES + PASS
        );
        assert_eq!(text(&output.stderr), "");
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(backend.count(), 15);
    }
}

#[test]
fn missing_confidence_and_token_counts_warn_and_keep_exit_zero() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let backend = Backend::start().expect("backend");
        let output = at(command, &backend, "/generic/v1");
        let report = "ok connection\nok key\nok endpoint\nok noul\n\
        warning choice: the answer to question `q1` carries no confidence\n\
        warning score: the answer to question `q1` carries no confidence\n\
        warning mixed: the answer to question `q2` carries no confidence\n\
        warning mixed: the answer to question `q3` carries no confidence\n\
        warning usage: a reply carries no token counts, so results and usage totals leave them out\n\
        answered function decide\nanswered function choose\nanswered function tag\nanswered function score\nanswered function filter\nanswered function rank\nanswered function find\nanswered function annotate\nanswered function recognize\nanswered function relate\ncritical 0, warning 5\n";
        assert_eq!(
            text(&output.stdout),
            head(&backend, "/generic/v1") + GENERIC_REPLIES + report
        );
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(backend.count(), 15);
    }
}

#[test]
fn a_missing_answer_is_critical_in_every_probe_and_names_the_tag_range() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let backend = Backend::start().expect("backend");
        let path = "/arm/malformed/missing_answer/v1";
        let output = at(command, &backend, path);
        let refused = "the reply was refused: the response carries no answer for question `q1`";
        let functions = failures(|name| match name {
            "tag" => "a backend question failed in a batch".to_owned(),
            "relate" => "the backend failed 1 of 2 relation questions".to_owned(),
            _ => refused.to_owned(),
        });
        let report = format!(
            "{}\n\
        ok connection\nok key\nok endpoint\n\
        critical noul: {refused}\ncritical choice: {refused}\ncritical score: {refused}\n\
        warning mixed: the answer to question `q2` carries no confidence\n\
        warning mixed: the answer to question `q3` carries no confidence\n\
        critical mixed: the answer to questions `q4` to `q5` failed as `missing_answer`\n\
        warning usage: a reply carries no token counts, so results and usage totals leave them out\n\
        {functions}critical 14, warning 3\n",
            // Only the mixed reply decodes. Its tag answer carries the failure marker.
            r#"reply mixed {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9},{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.1}},{"kind":"score","level":"poor","probabilities":{"poor":0.9,"good":0.1}},{"failed":{"kind":"backend","cause":"missing_answer"}}],"usage":null}"#
        );
        assert_eq!(text(&output.stdout), head(&backend, path) + &report);
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(backend.count(), 14);
    }
}

#[test]
fn a_refused_body_is_critical_for_its_probe_and_the_check_goes_on() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let backend = Backend::start().expect("backend");
        let output = at(command, &backend, "/arm/refuse/v1");
        let functions = failures(|_| "the backend answered with status 422".to_owned());
        let report = format!(
            "ok connection\nok key\nok endpoint\n\
        critical noul: {REFUSED}\ncritical choice: {REFUSED}\n\
        critical score: {REFUSED}\ncritical mixed: {REFUSED}\n\
        unchecked usage\n{functions}critical 14, warning 0\n"
        );
        assert_eq!(
            text(&output.stdout),
            head(&backend, "/arm/refuse/v1") + &report
        );
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(backend.count(), 14);
    }
}

#[test]
fn a_failure_no_body_causes_stops_the_check_at_the_first_probe() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let later = "unchecked noul\nunchecked choice\nunchecked score\nunchecked mixed\nunchecked usage\nunchecked function decide\nunchecked function choose\nunchecked function tag\nunchecked function score\nunchecked function filter\nunchecked function rank\nunchecked function find\nunchecked function annotate\nunchecked function recognize\nunchecked function relate\ncritical 1, warning 0\n";
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
            let output = at(command, &backend, path);
            assert_eq!(
                text(&output.stdout),
                head(&backend, path) + &report,
                "{path}"
            );
            assert_eq!(output.status.code(), Some(4), "{path}");
            assert_eq!(backend.count(), 1, "{path}");
        }
    }
}

#[test]
fn the_check_sends_only_to_the_address_the_user_named() {
    for command in [&["backends", "check"][..], &["check"][..]] {
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
        let output = check(command, &["--url", url.as_str()], &environment);
        assert_eq!(
            text(&output.stdout),
            head(&named, "/arm/full/v1") + FULL_REPLIES + PASS
        );
        assert_eq!((named.count(), beside.count()), (15, 0));

        // No key is set, so a regression that reached the built-in address still sends nothing.
        let output = check(command, &[], &[]);
        assert_eq!(
            text(&output.stderr),
            "thinkthen: check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file\n"
        );
        assert_eq!(text(&output.stdout), "");
        assert_eq!(output.status.code(), Some(2));
    }
}

#[test]
fn an_unset_key_stops_the_check_before_any_request() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let backend = Backend::start().expect("backend");
        // A loopback backend takes a run with no key, so the check names an
        // address the rules cannot prove is this machine.
        let url = format!("{}/arm/full/v1", backend.origin())
            .replace("http://127.0.0.1", "https://127.0.0.2");
        let output = check(command, &["--url", url.as_str()], &[]);
        assert_eq!(
            text(&output.stderr),
            "thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent\n"
        );
        assert_eq!(text(&output.stdout), "");
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(backend.count(), 0);
    }
}

#[test]
fn a_dry_run_prints_the_four_fixed_bodies_and_sends_nothing() {
    for command in [&["backends", "check"][..], &["check"][..]] {
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
            let output = check(command, &["--url", url.as_str(), "--plan"], environment);
            let printed = head(&backend, "/arm/full/v1")
                + &requests
                + "rich-probes {\"records\":4,\"requests\":4,\"estimated_bytes\":1568,\"estimated_input_tokens\":{\"lower\":809,\"upper\":1424},\"upper_bound\":false}\n";
            let stdout = text(&output.stdout);
            assert!(stdout.starts_with(&printed), "{stdout}");
            let plans = stdout
                .lines()
                .filter_map(|line| line.strip_prefix("function-plan "))
                .collect::<Vec<_>>();
            assert_eq!(plans.len(), FUNCTIONS.len());
            for (plan, name) in plans.iter().zip(FUNCTIONS) {
                let (function, body) = plan.split_once(' ').expect("function plan");
                assert_eq!(function, name);
                let counts: serde_json::Value = serde_json::from_str(body).expect("counts");
                assert_eq!(counts["requests"], 1 + usize::from(name == "recognize"));
                assert_eq!(counts["upper_bound"], name == "recognize");
            }
            assert!(
                stdout.ends_with(
                    "prepared-requests upper-bound 15 before retries and refusal splits\n"
                )
            );
            assert_eq!(text(&output.stderr), "");
            assert_eq!(output.status.code(), Some(0));
        }
        assert_eq!(backend.count(), 0);
    }
}

/// One canned reply per probe, each naming `other-1` whatever model was sent.
#[cfg(unix)]
fn other_model() -> Listener {
    let answers = [
        r#""q1":{"type":"noul","noul":0.9}"#,
        r#""q1":{"type":"choice","probabilities":{"Monday":0.1,"Tuesday":0.8,"Wednesday":0.1}}"#,
        r#""q1":{"type":"score","probabilities":{"0":0.1,"1":0.1,"2":0.8}}"#,
        r#""q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"Monday":0.2,"Tuesday":0.8}},"q3":{"type":"score","probabilities":{"0":0.3,"1":0.7}},"q4":{"type":"noul","noul":0.8},"q5":{"type":"noul","noul":0.1}"#,
    ];
    let canned =
        answers.map(|said| Canned::ok(&format!(r#"{{"model":"other-1","answers":{{{said}}}}}"#)));
    let canned = std::sync::Mutex::new(std::collections::VecDeque::from(canned));
    Listener::answering(move |body| {
        if let Some(reply) = canned.lock().expect("replies").pop_front() {
            return reply;
        }
        let request: serde_json::Value = serde_json::from_slice(body).expect("wire");
        let answers = request
            .get("questions")
            .expect("questions")
            .as_object()
            .expect("questions")
            .iter()
            .map(|(name, question)| {
                format!(
                    "\"{name}\":{}",
                    crate::named_backends::ollama::answer(question)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        Canned::ok(&format!(r#"{{"model":"other-1","answers":{{{answers}}}}}"#))
    })
    .expect("listener")
}

/// The header lines and the model each reply line names.
#[cfg(unix)]
fn models(output: &Output) -> (Vec<String>, Vec<String>) {
    let printed = text(&output.stdout);
    let header = printed.lines().skip(1).take(3).map(str::to_owned).collect();
    let replies = printed
        .lines()
        .filter_map(|line| line.strip_prefix("reply "));
    let named = replies.map(|line| {
        let json = line.split_once(' ').expect("a probe name").1;
        let said: serde_json::Value = serde_json::from_str(json).expect("reply JSON");
        let model = said.get("model").and_then(serde_json::Value::as_str);
        model.expect("a model").to_owned()
    });
    (header, named.collect())
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn the_report_names_the_model_asked_the_model_sent_and_the_model_each_reply_names() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let other = ["other-1"; 4].map(str::to_owned).to_vec();
        let listener = other_model();
        let output = check(
            command,
            &["--url", listener.base(), "--model", "local-1"],
            &[("THINKTHEN_API_KEY", KEY)],
        );
        let header = [
            "provider systemone",
            "model asked local-1",
            "model sent local-1",
        ];
        assert_eq!(
            models(&output),
            (header.map(str::to_owned).to_vec(), other.clone())
        );
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stdout));

        let sent = listener.requests();
        assert_eq!(
            sent.len(),
            15,
            "rich probes and minimal functions with an empty recognition result"
        );
        assert_eq!(
        sent[0].body,
        br#"{"state":"The parcel arrived on Tuesday and the box was intact.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"Did the parcel arrive undamaged?","criteria":{"true":"The text says the box or its contents were intact."}}}}"#
    );

        let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("check-config-{}", std::process::id()));
        let config = crate::child::Folder::Config;
        fs::create_dir_all(config.under(&root)).expect("configuration directory");
        fs::write(
            config.under(&root).join("config.json"),
            r#"{"schema":"thinkthen.config/1","model":"configured-1"}"#,
        )
        .expect("configuration");
        let listener = other_model();
        let moved = config.variable(&root);
        let environment = [("THINKTHEN_API_KEY", KEY), (moved.0, moved.1.as_str())];
        let output = check(command, &["--url", listener.base()], &environment);
        let header = [
            "provider systemone",
            "model asked configured-1",
            "model sent configured-1",
        ];
        assert_eq!(models(&output), (header.map(str::to_owned).to_vec(), other));
    }
}

#[path = "check/limits.rs"]
mod limits;
