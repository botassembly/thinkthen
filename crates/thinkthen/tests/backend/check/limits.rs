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
