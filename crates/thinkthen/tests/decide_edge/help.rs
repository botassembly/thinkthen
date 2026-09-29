//! Compiled short and long help contracts.

use super::{DEFAULT_MODEL, run};

#[test]
fn the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest() {
    let short = run(&["decide", "-h"], &[], b"").expect("the compiled binary runs");
    let long = run(&["decide", "--help"], &[], b"").expect("the compiled binary runs");

    let short = String::from_utf8_lossy(&short.stdout);
    for option in ["--threshold", "--quiet", "--details", "--plan", "--profile"] {
        assert!(short.contains(option), "{option} is missing from {short}");
    }
    assert!(short.contains("--url"), "--url is missing from {short}");
    for option in ["--model", "--record", "--timeout", "--jobs"] {
        assert!(!short.contains(option), "{option} is in the short help");
    }

    let long = String::from_utf8_lossy(&long.stdout);
    let stop = long.find("The answer is a bare").expect("answer");
    let first = "printf 'Refund me please.' | thinkthen decide 'Does this ask for a refund?'";
    let second = "printf 'Refund me please.' | thinkthen decide 'Does this ask for a refund?' --threshold 0.1:0.9";
    let printed = long[..stop].lines().filter(|s| s.starts_with("printf "));
    assert!(printed.eq([first, second]), "{long}");
    for option in [
        "--threshold",
        "--quiet",
        "--details",
        "--plan",
        "--url",
        "--profile",
        "--model",
        "--record",
        "--replay",
        "--timeout",
        "--max-retries",
    ] {
        assert!(long.contains(option), "{option} is missing from {long}");
    }
    for gone in ["--adapter", "--key-env", "--config"] {
        assert!(!long.contains(gone), "{gone} is still in {long}");
    }
    assert!(
        long.contains(&format!("[default: {DEFAULT_MODEL}]")),
        "the long help does not name the default model {DEFAULT_MODEL}: {long}"
    );
    assert!(!long.contains("THINKTHEN_TEST_RETRY_WAIT_MS"), "{long}");
    assert!(!long.contains("THINKTHEN_TEST_SIGINT_ACK"), "{long}");
    assert!(long.contains("set -e"), "the help warns about set -e");
    assert!(
        long.contains("no or not sure"),
        "the help names both nonzero answers"
    );
    assert!(
        long.contains("It defaults to 0.5"),
        "the help does not name the threshold default: {long}"
    );
    assert!(
        long.contains("[default: 4]"),
        "the help does not name the jobs default: {long}"
    );
    for cache_rule in [
        "cached by default in the platform cache folder",
        "Entries contain the judged text",
        "overriding THINKTHEN_CACHE and the platform default",
        "An explicit recording folder suppresses the platform default cache",
    ] {
        assert!(long.contains(cache_rule), "{cache_rule}: {long}");
    }
}

#[test]
fn asking_help_names_plan_and_does_not_advertise_the_retired_flag() {
    for spelling in ["-h", "--help"] {
        let output = run(&["decide", spelling], &[], b"").expect("compiled help");
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(help.contains("--plan"), "{spelling}: {help}");
        assert!(!help.contains("--dry-run"), "{spelling}: {help}");
    }
}

#[test]
fn filter_help_names_default_batching() {
    let filter = run(&["filter", "--help"], &[], b"").expect("the compiled binary runs");
    let filter = String::from_utf8_lossy(&filter.stdout);
    assert!(filter.contains("Records share requests by default; --batch 1"));
    assert!(!filter.contains("one paid request for every record"));
}

#[test]
fn the_long_help_says_a_transport_failure_is_never_sent_again() {
    for command in ["decide", "find"] {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(
            help.contains("\n          How many times a retried status is sent again. A transport failure is never sent again\n"),
            "{command}: {help}"
        );
    }
}

#[test]
fn record_capable_help_pins_run_exit_behavior() {
    const RECORD_EXIT: &str = "A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers.";
    const SHORT: &str = "Answer one yes or no question about a text. A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers\n\nUsage:";
    const WHOLE_SET: &str = "A run that answers some relation questions and fails others prints what it has and exits 6. A run whose relation questions all fail prints nothing and exits 4.";
    const TEACHING: &str = "The exit code is 0 for yes, 1 for no, 3 for not sure, and any other code when the run is broken or interrupted.";
    let long = |command: &str| {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    let output = run(&["decide", "-h"], &[], b"").expect("the compiled binary runs");
    let short = String::from_utf8_lossy(&output.stdout);
    assert!(short.starts_with(SHORT), "decide short help: {short}");

    for command in [
        "decide",
        "filter",
        "rank",
        "choose",
        "score",
        "tag",
        "annotate",
        "recognize",
    ] {
        let help = long(command);
        assert_eq!(help.matches(RECORD_EXIT).count(), 1, "{command}: {help}");
    }
    let relate = long("relate");
    assert!(!relate.contains("A record run"), "relate: {relate}");
    assert_eq!(relate.matches(WHOLE_SET).count(), 1, "relate: {relate}");
    let decide = long("decide");
    assert_eq!(decide.matches(TEACHING).count(), 1, "decide: {decide}");
    assert!(!decide.contains("3 for unresolved"), "decide: {decide}");
}

#[test]
fn shared_help_defers_order_and_document_rules_to_each_command() {
    for command in ["decide", "filter", "rank"] {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(
            help.contains("Output follows the order the command defines."),
            "{command}: {help}"
        );
        assert!(
            help.contains("On a command that accepts a single text"),
            "{command}: {help}"
        );
    }
    let output = run(&["rank", "--help"], &[], b"").expect("the compiled binary runs");
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(
        help.contains("The printed order puts the highest value first."),
        "{help}"
    );
    assert!(help.contains("An exact tie keeps input order."), "{help}");
}

#[test]
fn record_framing_help_names_each_commands_output_rule() {
    const VALUE: &str = "Value verbs keep `input` beside `value`.";
    const RECORD: &str = "Record-returning verbs return records.";
    const ANNOTATE: &str = "`annotate` enriches object records.";

    for command in ["decide", "annotate", "filter"] {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        for rule in [VALUE, RECORD, ANNOTATE] {
            assert!(help.contains(rule), "{command}: {rule}\n{help}");
        }
    }
}
