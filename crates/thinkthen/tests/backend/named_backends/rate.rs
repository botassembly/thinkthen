//! Requests a minute from the configuration file (ticket 0343): a rate set
//! only in the file paces the backend it names, and
//! `THINKTHEN_REQUESTS_PER_MINUTE` outranks it. Each leg asserts only a lower
//! bound on the spread of request starts, so a loaded machine cannot fail it.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use conformance_backend::{Canned, Listener};

use super::support::{Home, said};

const ANSWER: &str = r#"{"model":"nimble","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":1,"output_tokens":1}}"#;

/// Six one-line requests to a loopback `ollama` under this configuration and
/// these variables; returns the spread from the first start to the last.
fn spread(config: &str, changes: &[(&str, &str)]) -> Duration {
    let starts = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&starts);
    let target = Listener::answering(move |_| {
        seen.lock().expect("the start list").push(Instant::now());
        Canned::ok(ANSWER)
    })
    .expect("a loopback listener");
    let home = Home::new("rate");
    home.config(config);
    let input = home.evidence(&["a", "b", "c", "d", "e", "f"]);
    let arguments = [
        "decide",
        "Is it?",
        "--lines",
        "--input",
        &input,
        "--jobs",
        "4",
        "--no-cache",
    ];
    let flags = ["--backend", "ollama", "--url", target.base()];
    let output = home.run(
        &[&arguments[..], &flags[..]].concat(),
        &[&[("THINKTHEN_BATCH", "1")], changes].concat(),
    );
    let (stdout, stderr) = said(&output);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    assert_eq!(stdout.lines().count(), 6);
    let starts = starts.lock().expect("the start list");
    assert_eq!(starts.len(), 6);
    *starts.iter().max().expect("a start") - *starts.iter().min().expect("a start")
}

fn paced(name: &str, rate: u32) -> String {
    format!(
        r#"{{"schema":"thinkthen.config/1","backends":{{"{name}":{{"requests_per_minute":{rate}}}}}}}"#
    )
}

#[test]
fn with_no_rate_for_the_backend_in_use_nothing_is_spaced() {
    // One a minute on another backend would stretch six starts to five minutes
    // if it leaked, so a generous bound still proves no spacing.
    let bound = Duration::from_secs(20);
    for config in ["", &paced("liquid", 1)] {
        let unpaced = spread(config, &[]);
        assert!(unpaced < bound, "{config}: {unpaced:?}");
    }
}

#[test]
fn a_rate_set_only_in_the_file_paces_its_backend_and_the_variable_outranks_it() {
    // 600 a minute is one start each 100 ms, so six starts span 500 ms. The
    // bound leaves one interval for a late first delivery.
    let bound = Duration::from_millis(400);
    let from_file = spread(&paced("ollama", 600), &[]);
    assert!(from_file >= bound, "the file's rate: {from_file:?}");
    let from_variable = spread(
        &paced("ollama", 60_000),
        &[("THINKTHEN_REQUESTS_PER_MINUTE", "600")],
    );
    assert!(
        from_variable >= bound,
        "the variable over the file: {from_variable:?}"
    );
}
