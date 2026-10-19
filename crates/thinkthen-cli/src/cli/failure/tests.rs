//! Command diagnostic and signal mappings withhold secrets and preserve counts.
use super::{Failure, report};
use thinkthen::TransportFailure as TransportKind;
use std::process::ExitCode;
const KEY: &str = "sk-marker-2f9d41c6";
const EVIDENCE: &str = "marker-evidence-7b3ac5";

#[test]
fn no_debug_line_shows_the_key_or_the_evidence() {
    let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
    let judged = crate::schedule::Judged {
        model: None,
        printed: Some(body.clone()),
        position: None,
        outcome: crate::schedule::Outcome::Yes,
        order_value: Some(0.91),
        replayed: false,
        partial_failure: false,
        profile_mismatch: None,
    };
    let mut written = Vec::new();
    let mut ordered = crate::schedule::Output::streaming(&mut written);
    ordered.take(crate::schedule::Judged {
        model: None,
        printed: Some(body.clone()),
        position: None,
        outcome: crate::schedule::Outcome::Yes,
        order_value: Some(0.91),
        replayed: false,
        partial_failure: false,
        profile_mismatch: None,
    }).expect("row held");
    let question_set = thinkthen::QuestionSet::from_json(&format!(
        r#"{{"version":1,"questions":{{"{KEY}.{EVIDENCE}":{{"decide":"Fits?"}},"{KEY}.{EVIDENCE}":{{"decide":"Fits?"}}}}}}"#
    )).expect_err("duplicate question name");
    let shown = format!(
        "{judged:?} {ordered:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        Failure::NoKey("THINKTHEN_API_KEY".to_owned()),
        Failure::Status(401, false),
        Failure::from(question_set),
        Failure::OpenProfile {
            path: "safe-profile.json".into(),
            error: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        Failure::Profile {
            path: "safe-profile.json".into(),
            error: thinkthen::Engine::builder().profile_json(r#"{"schema":"thinkthen.backend-profile/1","name":"INVALID","max_questions":1}"#).unwrap_err(),
        },
        Failure::ProfileLimit {
            profile: "safe-profile".to_owned(),
            dimension: "evidence bytes",
            limit: 1,
            actual: EVIDENCE.len(),
        },
        Failure::Recognize(super::recognize::Error::LogicalQuestion),
        Failure::RecordingStorage,
        Failure::Relate(super::relate::Error::Config {
            file: true,
            error: "a relate relation has a distinct name, source, target, and optional either".to_owned(),
        }),
    );
    assert!(!shown.contains(KEY), "{shown}");
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert!(shown.contains("withheld"), "{shown}");
}

/// The command plan document withholds a record's authored shortlist in both Debug forms.
#[test]
fn no_plan_document_debug_line_shows_the_evidence() {
    let builder = thinkthen::Engine::builder()
        .base_url("http://127.0.0.1:1/v1/systemone").expect("fixture address")
        .api_key(KEY).expect("fixture key")
        .no_cache();
    let settings = builder.resolved_settings().expect("settings");
    let engine = builder.build().expect("engine");
    let request = thinkthen::Request::from_json(&format!(r#"{{
        "schema":"thinkthen.request/1",
        "call":{{"function":"choose",
          "question":{{"kind":"definition","value":{{"choose":"Which one fits?","options":["fallback","other"]}}}},
          "input":{{"kind":"json","value":{{"labels":["{EVIDENCE}","other"]}}}},
          "options":{{"options_field":"/labels"}}
        }}
    }}"#)).expect("request").admit().expect("admitted request");
    let plan = engine.plan_request(&request, thinkthen::RequestEnvironment::default()).expect("plan");
    let body = plan.first_body().expect("prepared body").to_vec();
    let document = crate::plan_document::PlanDocument::of_body(&settings, body).expect("a plan document");
    let shown = format!("{document:?} {document:#?}");
    assert!(!shown.contains(EVIDENCE), "{shown}");
}

mod diagnostics;

#[test]
fn native_backend_causes_keep_command_status_and_sentence() {
    let listener = conformance_backend::Listener::answering(|_| {
        conformance_backend::Canned::status(302, "{}")
    }).expect("loopback redirect");
    let engine = crate::Engine::builder()
        .base_url(listener.base()).expect("loopback address")
        .api_key("command-error-fixture").expect("fixture key")
        .no_cache().max_retries(0).build().expect("fixture engine");
    let question = crate::Question::decide("Fits?").expect("question").cut();
    let error = engine.decide(&question, "evidence").expect_err("redirect refusal");
    assert_eq!(error.kind(), crate::ErrorKind::Backend);
    let mut output = Vec::new();
    assert_eq!(report(&Failure::from(error), &mut output), ExitCode::from(4));
    assert_eq!(String::from_utf8(output).expect("diagnostic UTF-8"),
        "thinkthen: the backend answered with status 302: the redirect was not followed; use the final --url directly\n");
    assert_eq!(listener.count(), 1);
}

#[test]
fn native_cancellation_keeps_the_commands_silent_signal_exit() {
    let error = crate::Error::new(crate::ErrorKind::Cancelled, "caller cancelled");
    let mut output = Vec::new();
    assert_eq!(report(&Failure::from(error), &mut output), ExitCode::from(130));
    assert!(output.is_empty());
}
