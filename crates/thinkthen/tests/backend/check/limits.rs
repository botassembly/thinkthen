//! Backend check request, reply and retry limits.

use super::{KEY, at, check, failures, text};
use crate::harness::Listener;
use conformance_backend::Backend;
use std::fs;

/// Ticket 0132: a reply over its limit fails its probe, and the check goes on.
#[test]
fn a_reply_over_its_limit_fails_its_probe_and_the_check_goes_on() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let limits = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let seen = std::sync::Arc::clone(&limits);
        let listener = Listener::answering(move |body| {
            let limit = crate::resend::limit(body);
            seen.lock()
                .expect("limits")
                .push(crate::resend::past(limit));
            crate::resend::padded(r#"{"model":"jev-latest","answers":{}}"#, limit + 1)
        })
        .expect("listener");
        let output = check(
            command,
            &["--url", listener.base()],
            &[("THINKTHEN_API_KEY", KEY)],
        );
        let said = limits.lock().expect("limits").clone();
        assert_eq!(
            said.len(),
            14,
            "{}{}",
            text(&output.stdout),
            text(&output.stderr)
        );
        let mut place = 4;
        let functions = failures(|_| {
            let sentence = said[place].clone();
            place += 1;
            sentence
        });
        let report = format!(
            "url {}/systemone\nprovider systemone\nmodel asked unspecified\nmodel sent jev-1.13.0\n\
        ok connection\nok key\nok endpoint\n\
        critical noul: {}\ncritical choice: {}\ncritical score: {}\ncritical mixed: {}\n\
        unchecked usage\n{functions}critical 14, warning 0\n",
            listener.base(),
            said[0],
            said[1],
            said[2],
            said[3]
        );
        assert_eq!(text(&output.stdout), report);
        assert_eq!(output.status.code(), Some(4));
    }
}

/// The estimated input cap binds `check` as it binds every live request
/// (ticket 0364). A cap of 1 refuses the first probe and sends nothing; a cap
/// of the first probe's estimate refuses the second after one request.
#[test]
fn the_estimated_input_cap_stops_the_check_before_a_probe_it_cannot_admit() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let fixture = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../specification/fixtures/check/requests.jsonl"
        );
        let bodies = fs::read_to_string(fixture).expect("the fixture");
        let first = bodies.lines().next().expect("a first body").len();
        // `encoded-body-bytes-908-v1`: ceil(bytes × 908 / 1000).
        let first_estimate = (first * 908).div_ceil(1000).to_string();
        for (cap, sent, before) in [
            ("1", 0, "this call's first request"),
            (first_estimate.as_str(), 1, "another request in this call"),
        ] {
            let backend = Backend::start().expect("backend");
            let url = format!("{}/arm/full/v1", backend.origin());
            let output = check(
                command,
                &["--url", url.as_str()],
                &[
                    ("THINKTHEN_API_KEY", KEY),
                    ("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL", cap),
                ],
            );
            assert_eq!(
                text(&output.stderr),
                format!(
                    "thinkthen usage: max_estimated_input_tokens_total={cap} (encoded-body-bytes-908-v1) would be exceeded before {before}\n"
                ),
                "{cap}"
            );
            assert_eq!(text(&output.stdout), "", "{cap}");
            assert_eq!(output.status.code(), Some(2), "{cap}");
            assert_eq!(backend.count(), sent, "{cap}");
        }
    }
}

/// Each probe retries a retried status three times, the default of every
/// command, so four probes send sixteen requests (`specification/check.md`).
#[test]
fn a_retried_status_is_sent_four_times_for_each_probe() {
    for command in [&["backends", "check"][..], &["check"][..]] {
        let backend = Backend::start().expect("backend");
        let output = at(command, &backend, "/arm/503/v1");
        assert_eq!(output.status.code(), Some(4), "{}", text(&output.stderr));
        assert_eq!(backend.count(), 56);
    }
}

/// Production tag and relate calls halve a refused pair once; prepared plans
/// exclude those additional sends rather than claiming a total request bound.
#[test]
fn size_refusals_add_two_halves_to_tag_and_relate_requests() {
    let listener = Listener::answering(|body| {
        let request: serde_json::Value = serde_json::from_slice(body).expect("wire");
        let questions = request.get("questions").expect("questions");
        if questions.as_object().expect("questions map").len() == 2 {
            return crate::harness::Canned::status(413, "");
        }
        answered(&request)
    })
    .expect("listener");
    let output = check(&["backends", "check"], &["--url", listener.base()], &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}{}",
        text(&output.stdout),
        text(&output.stderr)
    );
    assert_eq!(listener.count(), 19);
    let requests = listener.requests();
    let pairs = requests
        .iter()
        .filter(|request| {
            let body: serde_json::Value = serde_json::from_slice(&request.body).expect("wire");
            body.get("questions")
                .expect("questions")
                .as_object()
                .expect("map")
                .len()
                == 2
        })
        .count();
    assert_eq!(pairs, 2);
    let plan = check(
        &["backends", "check"],
        &["--url", listener.base(), "--plan"],
        &[],
    );
    assert_eq!(plan.status.code(), Some(0), "{}", text(&plan.stderr));
    assert!(
        text(&plan.stdout)
            .ends_with("prepared-requests upper-bound 15 before retries and refusal splits\n")
    );
    assert_eq!(listener.count(), 19, "planning sends nothing");
}

#[test]
fn a_function_body_close_or_timeout_stops_later_functions() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    for (failed_at, delayed) in [(4, false), (11, false), (4, true)] {
        let ordinal = AtomicUsize::new(0);
        let listener = Listener::answering(move |body| {
            if ordinal.fetch_add(1, Ordering::SeqCst) == failed_at {
                return crate::harness::Canned::cut_short().after(1500 * u64::from(delayed));
            }
            let request: serde_json::Value = serde_json::from_slice(body).expect("wire");
            answered(&request)
        })
        .expect("listener");
        let output = check(
            &["backends", "check"],
            &["--url", listener.base(), "--timeout", "1"],
            &[],
        );
        let stdout = text(&output.stdout);
        assert_eq!(
            output.status.code(),
            Some(4),
            "{stdout}{}",
            text(&output.stderr)
        );
        assert_eq!(listener.count(), failed_at + 1, "{stdout}");
        let failed = failed_at - 4;
        for (place, name) in super::FUNCTIONS.iter().enumerate() {
            let state = if place < failed {
                "answered"
            } else if place == failed {
                "incompatible"
            } else {
                "unchecked"
            };
            assert!(
                stdout.contains(&format!("{state} function {name}")),
                "{stdout}"
            );
        }
        assert!(stdout.ends_with("critical 1, warning 0\n"), "{stdout}");
        assert_eq!(text(&output.stderr), "");
    }
}

fn answered(request: &serde_json::Value) -> crate::harness::Canned {
    let questions = request
        .get("questions")
        .expect("questions")
        .as_object()
        .expect("map");
    let answers = questions
        .iter()
        .map(|(name, question)| {
            format!(
                "\"{name}\":{}",
                crate::named_backends::ollama::answer(question)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    crate::harness::Canned::ok(&format!(
        r#"{{"model":"local","answers":{{{answers}}},"usage":{{"input_tokens":1,"output_tokens":1}}}}"#
    ))
}
