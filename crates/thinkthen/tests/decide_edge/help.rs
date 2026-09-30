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
