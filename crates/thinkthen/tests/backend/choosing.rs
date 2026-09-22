//! The compiled binary asking a pick and a placement of a loopback backend.

use std::io;
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};

/// The teams a routing question picks between, in the order they are sent.
const TEAMS: [&str; 4] = ["billing", "shipping", "account", "other"];

/// The levels a placement question uses, lowest first.
const LEVELS: [&str; 3] = [
    "None.",
    "Work continues with a workaround.",
    "Work is blocked.",
];

/// One ticket, which every case on this page sends as the evidence.
const TICKET: &str = "The renewal charge bounced last night.";

/// A pick the backend answered with a clear winner.
const PICKED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
    r#""confidence":0.91,"probabilities":{"other":0.02,"billing":0.9,"#,
    r#""account":0.04,"shipping":0.04}}},"#,
    r#""usage":{"input_tokens":320,"output_tokens":46}}"#,
);

/// A pick whose top two options tie exactly.
const TIED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
    r#""confidence":0.4,"probabilities":{"billing":0.5,"shipping":0.5,"#,
    r#""account":0.0,"other":0.0}}}}"#,
);

/// A placement the backend answered on three levels.
const PLACED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"score","score":1.86,"#,
    r#""confidence":0.79,"legend":{"0":"None.","1":"Work continues with a workaround.","#,
    r#""2":"Work is blocked."},"#,
    r#""probabilities":{"0":0.0,"1":0.1,"2":0.9}}},"#,
    r#""usage":{"input_tokens":338,"output_tokens":18}}"#,
);

/// Run one verb against one URL, with no environment but the key.
fn ask(base: &str, verb: &str, labels: &[&str], arguments: &[&str]) -> io::Result<Output> {
    let asked = [verb, "Which team owns this request?"];
    let named = ["--url", base, "--model", "local-1"];
    spawn(
        &[&asked[..], labels, &named[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        TICKET.as_bytes(),
    )
}

/// Ask a pick of a listener serving this one response.
fn choose(response: &str, arguments: &[&str]) -> io::Result<(Listener, Output)> {
    let listener = Listener::serving(vec![Canned::ok(response)])?;
    let output = ask(listener.base(), "choose", &TEAMS, arguments)?;
    Ok((listener, output))
}

/// Ask a placement of a listener serving this one response.
fn score(response: &str, arguments: &[&str]) -> io::Result<(Listener, Output)> {
    let listener = Listener::serving(vec![Canned::ok(response)])?;
    let output = ask(listener.base(), "score", &LEVELS, arguments)?;
    Ok((listener, output))
}

/// What one command printed on standard output.
fn printed(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn a_pick_sends_the_options_as_criteria_in_the_order_they_were_typed() {
    let (listener, output) = choose(PICKED, &[]).expect("a loopback exchange");

    let requests = listener.requests();
    let request = requests.first().expect("one request reached the listener");
    let body = String::from_utf8_lossy(&request.body);
    assert!(
        body.contains(concat!(
            r#""questions":{"q1":{"type":"choice","#,
            r#""instructions":"Which team owns this request?","#,
            r#""criteria":{"billing":null,"shipping":null,"account":null,"other":null}}}"#,
        )),
        "{body}"
    );
    assert_eq!(printed(&output), "\"billing\"\n");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_placement_sends_the_levels_as_an_ordered_list_and_prints_a_number() {
    let (listener, output) = score(PLACED, &[]).expect("a loopback exchange");

    let requests = listener.requests();
    let request = requests.first().expect("one request reached the listener");
    let body = String::from_utf8_lossy(&request.body);
    assert!(
        body.contains(concat!(
            r#""questions":{"q1":{"type":"score","#,
            r#""instructions":"Which team owns this request?","#,
            r#""criteria":["None.","Work continues with a workaround.","Work is blocked."]}}"#,
        )),
        "{body}"
    );
    assert_eq!(printed(&output), "1.9\n");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_raw_view_drops_the_quotation_marks_and_prints_nothing_when_unresolved() {
    let (_listener, output) = choose(PICKED, &["--raw"]).expect("a loopback exchange");
    assert_eq!(printed(&output), "billing\n");
    assert_eq!(output.status.code(), Some(0));

    let (_listener, output) = choose(TIED, &["--raw"]).expect("a loopback exchange");
    assert_eq!(printed(&output), "");
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn a_winner_under_the_cut_and_an_exact_tie_are_both_unresolved() {
    let cases: [(&str, &[&str], &str); 4] = [
        (PICKED, &["--threshold", "0.9"], "\"billing\"\n"),
        (PICKED, &["--threshold", "0.95"], "null\n"),
        (TIED, &[], "null\n"),
        (TIED, &["--threshold", "0.1"], "null\n"),
    ];
    for (response, arguments, expected) in cases {
        let (_listener, output) = choose(response, arguments).expect("a loopback exchange");
        assert_eq!(printed(&output), expected, "{arguments:?}");
        let code = if expected == "null\n" { 3 } else { 0 };
        assert_eq!(output.status.code(), Some(code), "{arguments:?}");
    }
}

#[test]
fn quiet_prints_nothing_and_leaves_the_exit_code_carrying_the_answer() {
    let (_listener, output) = choose(PICKED, &["--quiet"]).expect("a loopback exchange");
    assert_eq!(printed(&output), "");
    assert_eq!(output.status.code(), Some(0));

    let (_listener, output) = choose(TIED, &["--quiet"]).expect("a loopback exchange");
    assert_eq!(printed(&output), "");
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn a_detailed_pick_keeps_every_option_in_order_and_the_backends_confidence() {
    let (_listener, output) =
        choose(PICKED, &["--details", "--threshold", "0.8"]).expect("a loopback exchange");

    let printed = printed(&output);
    assert!(
        printed.starts_with(concat!(
            r#"{"schema":"thinkthen.result/1","value":"billing","#,
            r#""question":{"verb":"choose","text":"Which team owns this request?","#,
            r#""options":["billing","shipping","account","other"]},"#,
            r#""answer":{"kind":"choice","pick":"billing","#,
            r#""probabilities":{"billing":0.9,"shipping":0.04,"account":0.04,"other":0.02},"#,
            r#""confidence":0.91},"threshold":0.8,"#,
            r#""meta":{"tool":"thinkthen 0.0.1","#,
            r#""question_sha256":"84ee9333d3eef7fb2cd628892cd284a033919333149a5c9475393341dd91f172","#,
            r#""url":""#,
        )),
        "{printed}"
    );
    assert!(printed.contains(r#""model":"jev-1.13.0""#), "{printed}");
    assert!(
        printed.contains(r#""cached":false,"requests":[""#),
        "{printed}"
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_detailed_unresolved_pick_names_the_option_that_led_and_prints_a_null_value() {
    let (_listener, output) = choose(TIED, &["--details"]).expect("a loopback exchange");
    let printed = printed(&output);

    assert!(printed.contains(r#""value":null,"#), "{printed}");
    assert!(printed.contains(r#""pick":"billing","#), "{printed}");
    assert!(printed.contains(r#""threshold":null,"#), "{printed}");
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn a_detailed_placement_keeps_every_level_and_takes_no_rule() {
    let (_listener, output) = score(PLACED, &["--details"]).expect("a loopback exchange");
    let printed = printed(&output);

    assert!(
        printed.contains(concat!(
            r#""answer":{"kind":"score","level":"Work is blocked.","#,
            r#""probabilities":{"None.":0.0,"Work continues with a workaround.":0.1,"#,
            r#""Work is blocked.":0.9},"confidence":0.79},"threshold":null,"#,
        )),
        "{printed}"
    );
    assert!(
        printed.contains(
            r#""levels":["None.","Work continues with a workaround.","Work is blocked."]"#
        ),
        "{printed}"
    );
    assert!(
        printed.contains(r#""usage":{"input_tokens":338,"output_tokens":18}"#),
        "{printed}"
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_reply_that_answers_with_the_wrong_shape_or_leaves_an_option_out_is_refused() {
    let cases: [&str; 3] = [
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
        concat!(
            r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
            r#""probabilities":{"billing":0.9,"shipping":0.1}}}}"#,
        ),
        concat!(
            r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
            r#""probabilities":{"billing":1.4,"shipping":0.1,"account":0.0,"other":0.0}}}}"#,
        ),
    ];
    for response in cases {
        let (_listener, output) = choose(response, &[]).expect("a loopback exchange");

        assert_eq!(output.status.code(), Some(4), "{response}");
        assert!(output.stdout.is_empty(), "{response}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("`q1`"), "{message}");
    }
}

/// A list of the given length, so a case can sit on each edge of the range.
fn many(count: usize) -> Vec<String> {
    (0..count).map(|place| format!("option{place}")).collect()
}

/// The same list as borrowed arguments.
fn listed(values: &[String]) -> Vec<&str> {
    values.iter().map(String::as_str).collect()
}

/// One refusal case: the verb, its labels, the options it carries, and its name.
type Refusal<'a> = (&'a str, Vec<&'a str>, &'a [&'a str], &'a str);

#[test]
fn a_list_the_verb_does_not_take_is_refused_and_no_request_leaves_the_machine() {
    let long_options = many(256);
    let long_levels = many(11);
    let band: &[&str] = &["--threshold", "0.1:0.9"];
    let cut: &[&str] = &["--threshold", "0.8"];
    let none: &[&str] = &[];
    let cases: [Refusal<'_>; 11] = [
        ("choose", vec!["billing"], none, "one option"),
        ("choose", Vec::new(), none, "no option at all"),
        ("choose", listed(&long_options), none, "256 options"),
        ("choose", vec!["billing", "  "], none, "a blank label"),
        ("choose", vec!["billing", ""], none, "an empty label"),
        (
            "choose",
            vec!["billing", "other", "billing"],
            none,
            "a repeat",
        ),
        ("score", vec!["none"], none, "one level"),
        ("score", listed(&long_levels), none, "eleven levels"),
        ("score", vec!["none", "none"], none, "a repeated level"),
        ("choose", TEAMS.to_vec(), band, "a band on a pick"),
        ("score", LEVELS.to_vec(), cut, "a rule on a placement"),
    ];

    for (verb, labels, arguments, what) in cases {
        let listener = Listener::serving(vec![Canned::ok(PICKED)]).expect("a loopback listener");
        let output =
            ask(listener.base(), verb, &labels, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{what}");
        assert!(output.stdout.is_empty(), "{what}");
        assert!(!output.stderr.is_empty(), "{what}");
        assert_eq!(listener.requests().len(), 0, "{what}");
    }
}
