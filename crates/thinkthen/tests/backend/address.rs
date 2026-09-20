//! The two variables at the edge: where a request goes, and the key it carries.

use std::io;
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};
use crate::recordings::{folder, only_entry};
use thinkthen_core::adapters::built_in;
use thinkthen_core::recording::Exchange;
use thinkthen_core::{Evidence, ModelName, Plan, Question, QuestionText, Url};

/// The response the listener gives to the one question the command asks.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// The evidence every case on this page judges.
const EVIDENCE: &[u8] = b"Refund me please.";

/// The address the tool posts to when nothing names another one.
const BUILT_IN: &str = "https://api.typesafe.ai/v1/systemone";

/// Run `decide` over the evidence, with no environment but what the case names.
fn decide(arguments: &[&str], environment: &[(&str, &str)]) -> io::Result<Output> {
    let asked = ["decide", "asks for a refund"];
    spawn(&[&asked[..], arguments].concat(), environment, EVIDENCE)
}

#[test]
fn the_option_outranks_the_variable_and_the_variable_outranks_the_default() {
    let chosen = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let ignored = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(
        &["--url", chosen.base(), "--model", "local-1"],
        &[
            ("THINKTHEN_BASE_URL", ignored.base()),
            ("THINKTHEN_API_KEY", "sk-test-value"),
        ],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let chosen = chosen.requests();
    let request = chosen.first().expect("the option names the address");
    assert_eq!(request.line, "POST /v1/systemone HTTP/1.1");
    assert!(ignored.requests().is_empty(), "the variable was outranked");

    let named = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let output = decide(
        &[],
        &[
            ("THINKTHEN_BASE_URL", named.base()),
            ("THINKTHEN_API_KEY", "sk-test-value"),
        ],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let requests = named.requests();
    let request = requests.first().expect("the variable names the address");
    assert_eq!(request.line, "POST /v1/systemone HTTP/1.1");

    let output = decide(&["--dry-run"], &[]).expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(&format!(r#""url":"{BUILT_IN}""#)),
        "{printed}"
    );
}

#[test]
fn a_base_that_is_not_an_http_address_is_a_usage_error_that_shows_no_address() {
    for bad in [
        "ftp://127.0.0.1/v1",
        "127.0.0.1:8080/v1",
        "file:///tmp/v1",
        "http:/127.0.0.1/v1",
        "https//127.0.0.1/v1",
        "/v1",
        "https://someone:sk-in-the-address@127.0.0.1/v1",
    ] {
        let output = decide(&[], &[("THINKTHEN_BASE_URL", bad)]).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{bad}");
        assert!(output.stdout.is_empty(), "{bad}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(
            message.starts_with("thinkthen: a base address"),
            "{message}"
        );
        assert!(!message.contains("127.0.0.1"), "{message}");
        assert!(!message.contains("sk-in-the-address"), "{message}");
    }
}

#[test]
fn each_address_rule_refusal_names_only_the_rule_that_failed() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");
    let authority = listener
        .base()
        .strip_prefix("http://")
        .and_then(|base| base.strip_suffix("/v1"))
        .expect("the listener uses plain HTTP")
        .to_owned();
    let cases = [
        (
            format!("ftp://{authority}"),
            "thinkthen: a base address begins with `http://` or `https://`\n",
        ),
        (
            format!("https://someone:secret@{authority}"),
            "thinkthen: a base address carries no user information\n",
        ),
        (
            "http://127.0.0.1:/v1".to_owned(),
            "thinkthen: a port is digits naming a number from 0 to 65535\n",
        ),
        (
            "http://127.0.0.1:+80/v1".to_owned(),
            "thinkthen: a port is digits naming a number from 0 to 65535\n",
        ),
        (
            "http://127.0.0.1:65536/v1".to_owned(),
            "thinkthen: a port is digits naming a number from 0 to 65535\n",
        ),
        (
            format!("https://{authority}/v1?secret=value"),
            "thinkthen: a base address carries no query or fragment\n",
        ),
        (
            format!("https://{authority}/v1#secret"),
            "thinkthen: a base address carries no query or fragment\n",
        ),
    ];

    for (base, expected) in cases {
        for environment in [&[("THINKTHEN_API_KEY", "sk-test-value")][..], &[][..]] {
            let output = decide(&["--url", &base], environment).expect("the compiled binary runs");

            assert_eq!(output.status.code(), Some(2), "{base} {environment:?}");
            assert!(output.stdout.is_empty(), "{base} {environment:?}");
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                expected,
                "{base} {environment:?}"
            );
            assert!(
                !String::from_utf8_lossy(&output.stderr).contains(&base),
                "{base} {environment:?}"
            );
        }
    }

    assert!(
        listener.requests().is_empty(),
        "a refused address opens no connection"
    );
    assert_eq!(listener.connections(), 0);
}

/// A plain `http://` base to anywhere but this machine sends nothing at all.
///
/// The listener counts the requests, so the case proves no key crossed the
/// network rather than only that the exit code was 2. Each base is a spelling a
/// reader might expect to pass: a name that ends in `localhost`, a trailing
/// dot, the short form of the loopback address, the wildcard address, and an
/// IPv4-mapped IPv6 address.
#[test]
fn a_plain_http_base_that_is_not_loopback_is_refused_before_any_request() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");
    let refused = [
        "http://example.com/v1",
        "http://localhost.example.com/v1",
        "http://localhost./v1",
        "http://127.1/v1",
        "http://0.0.0.0/v1",
        "http://10.0.0.5:8080/v1",
        "http://[::ffff:127.0.0.1]/v1",
        "HTTP://example.com/v1",
    ];

    for base in refused {
        for arguments in [vec![], vec!["--dry-run"]] {
            let by_option = [arguments.clone(), vec!["--url", base]].concat();
            for (given, environment) in [
                (by_option, vec![("THINKTHEN_API_KEY", "sk-test-value")]),
                (
                    arguments.clone(),
                    vec![
                        ("THINKTHEN_BASE_URL", base),
                        ("THINKTHEN_API_KEY", "sk-test-value"),
                    ],
                ),
            ] {
                let output = decide(&given, &environment).expect("the compiled binary runs");

                assert_eq!(output.status.code(), Some(2), "{base} {given:?}");
                assert!(output.stdout.is_empty(), "{base} {given:?}");
                assert_eq!(
                    String::from_utf8_lossy(&output.stderr),
                    "thinkthen: `http://` sends the key across the network in clear text, \
                     so it reaches localhost, 127.0.0.1, and [::1] alone\n",
                    "{base} {given:?}"
                );
            }
        }
    }

    assert!(
        listener.requests().is_empty(),
        "a refused address opens no connection"
    );
    assert_eq!(listener.connections(), 0);
}

/// Every loopback spelling is taken, and `https://` to anywhere is untouched.
#[test]
fn loopback_over_plain_http_is_taken_and_https_reaches_any_host() {
    for base in [
        "http://localhost:1/v1",
        "http://LOCALHOST:1/v1",
        "http://127.0.0.1:1/v1",
        "http://[::1]:1/v1",
        "https://example.com/v1",
        "https://10.0.0.5/v1",
    ] {
        let output = decide(&["--dry-run", "--url", base], &[]).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{base}");
        assert!(output.stderr.is_empty(), "{base}");
    }
}

/// One base in six spellings, each of which posts to the same path.
///
/// A trailing slash, surrounding space, and the case of the scheme all fall
/// away before the wire shape's own name is joined on. `--url` and the variable
/// compose through one function, and the ranking case above proves the wire
/// path the option reaches.
#[test]
fn every_spelling_of_one_base_reaches_the_same_path() {
    for shape in [
        "{base}",
        "{base}/",
        "{base}//",
        "  {base}  ",
        "\t{base}\n",
        "{upper}",
    ] {
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
        let upper = listener.base().replacen("http://", "HTTP://", 1);
        let given = shape
            .replace("{base}", listener.base())
            .replace("{upper}", &upper);

        let output = decide(
            &[],
            &[("THINKTHEN_BASE_URL", &given), ("THINKTHEN_API_KEY", "sk")],
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{given:?}");
        let requests = listener.requests();
        let request = requests.first().expect("one request reached the listener");
        assert_eq!(request.line, "POST /v1/systemone HTTP/1.1", "{given:?}");
    }
}

#[test]
fn dns_host_case_spellings_share_one_recording_identity() {
    let folder = folder("dns-host-case");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let uppercase = listener.base().replace("127.0.0.1", "LOCALHOST");
    let lowercase = listener.base().replace("127.0.0.1", "localhost");
    let canonical = format!("{lowercase}/systemone");
    let named = folder.to_string_lossy();

    let recorded = decide(
        &[
            "--url",
            &uppercase,
            "--model",
            "local-1",
            "--details",
            "--record",
            &named,
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
    )
    .expect("the compiled binary records");
    assert_eq!(recorded.status.code(), Some(0));
    let recorded = String::from_utf8(recorded.stdout).expect("a result is text");
    assert!(recorded.contains(r#""value":true"#), "{recorded}");
    assert!(
        recorded.contains(&format!(r#""url":"{canonical}""#)),
        "{recorded}"
    );
    assert!(recorded.contains(r#""replayed":false"#), "{recorded}");
    assert_eq!(listener.requests().len(), 1, "record sends one request");

    let (name, written) = only_entry(&folder).expect("one canonical entry");
    assert!(
        written.contains(&format!(r#""url": "{canonical}""#)),
        "{written}"
    );
    let plan = Plan::new(
        Evidence::new("Refund me please.").expect("evidence is not blank"),
        ModelName::new("local-1").expect("model is not blank"),
        vec![Question::Decide {
            text: QuestionText::new("asks for a refund").expect("question is not blank"),
            yes: None,
            no: None,
        }],
    )
    .expect("one question is a plan");
    let request = built_in::encode(&plan).expect("a plan is writable");
    let url = Url::new(&canonical).expect("the canonical URL is not blank");
    assert_eq!(name, Exchange::new(&url, &request).digest().file_name());

    let replayed = decide(
        &[
            "--url",
            &lowercase,
            "--model",
            "local-1",
            "--details",
            "--replay",
            &named,
        ],
        &[],
    )
    .expect("the compiled binary replays");
    assert_eq!(replayed.status.code(), Some(0));
    let replayed = String::from_utf8(replayed.stdout).expect("a result is text");
    assert!(replayed.contains(r#""value":true"#), "{replayed}");
    assert!(
        replayed.contains(&format!(r#""url":"{canonical}""#)),
        "{replayed}"
    );
    assert!(replayed.contains(r#""replayed":true"#), "{replayed}");
    assert!(listener.requests().is_empty(), "replay asks nothing");
    assert_eq!(only_entry(&folder).expect("one canonical entry").0, name);
}

/// A base given as white space is blank, and the message says so.
///
/// The variable holding nothing counts as absent, and the case below proves
/// that. `--url` was written on purpose, so it is refused instead.
#[test]
fn a_base_the_option_names_as_white_space_is_refused_as_a_blank_address() {
    for blank in ["", " ", "\t"] {
        let output = decide(&["--url", blank], &[]).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{blank:?}");
        assert!(output.stdout.is_empty(), "{blank:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: a URL is text, not white space\n",
            "{blank:?}"
        );
    }
}

#[test]
fn a_variable_that_holds_nothing_counts_as_absent() {
    for empty in ["", " "] {
        let output = decide(&["--dry-run"], &[("THINKTHEN_BASE_URL", empty)])
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{empty:?}");
        let printed = String::from_utf8_lossy(&output.stdout);
        assert!(
            printed.contains(&format!(r#""url":"{BUILT_IN}""#)),
            "{printed}"
        );
    }
}

#[test]
fn a_key_that_is_unset_or_empty_is_exit_four_and_names_the_variable_it_read() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");
    let unset: &[(&str, &str)] = &[("THINKTHEN_BASE_URL", listener.base())];
    let empty: &[(&str, &str)] = &[
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", ""),
    ];
    let blank: &[(&str, &str)] = &[
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", "   "),
    ];

    for environment in [unset, empty, blank] {
        let output = decide(&[], environment).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4), "{environment:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("THINKTHEN_API_KEY"), "{message}");
        assert!(output.stdout.is_empty(), "{environment:?}");
    }

    assert!(listener.requests().is_empty(), "no key, no request");
}

/// A proxy variable never carries a plain `http://` request off this machine.
///
/// The address rule refuses `http://` to anywhere but loopback, and a proxy
/// would undo it: the request would go to the proxy's host in clear text with
/// the key and the evidence in it. The proxy listener counts the connections,
/// so the case proves nothing reached it rather than only that the run passed.
#[test]
fn a_proxy_variable_carries_no_plain_http_request() {
    let proxy = Listener::answering(|_| Canned::ok(ANSWERED)).expect("a loopback listener");
    let backend = Listener::answering(|_| Canned::ok(ANSWERED)).expect("a loopback listener");
    let address = proxy
        .base()
        .strip_suffix("/v1")
        .expect("the listener base ends in the path it was built with")
        .to_owned();

    for variable in ["HTTP_PROXY", "http_proxy", "ALL_PROXY", "all_proxy"] {
        let output = decide(
            &["--url", backend.base()],
            &[
                (variable, address.as_str()),
                ("THINKTHEN_API_KEY", "sk-test-value"),
            ],
        )
        .expect("the compiled binary runs");

        assert_eq!(proxy.connections(), 0, "{variable} reached the proxy");
        assert_eq!(output.status.code(), Some(0), "{variable}");
    }

    let requests = backend.requests();
    assert_eq!(requests.len(), 4, "every run reached the backend itself");
}
