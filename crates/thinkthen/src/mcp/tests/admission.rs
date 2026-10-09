use super::super::admission::{CallParams, Invocation};
use conformance_backend::Backend;
use serde_json::{Value, json};

fn admit(name: &str, arguments: Value) -> Result<Invocation, crate::Error> {
    let params: CallParams = serde_json::from_value(json!({"name":name,"arguments":arguments}))
        .map_err(|_| crate::Error::usage("invalid tool arguments"))?;
    Invocation::admit(params)
}

#[test]
fn conflicting_sources_questions_credentials_and_unknown_arguments_send_nothing() {
    let backend = Backend::start().expect("loopback");
    for arguments in [
        json!({"question":"q","question_file":"file","evidence":"x"}),
        json!({"question":"q","evidence":"x","records":[]}),
        json!({"question":"q","evidence":"x","inputs":[]}),
        json!({"question":"q","inputs":[{"text":"x","json":null}]}),
        json!({"question":"q","inputs":[{"text":"x","context":"c"}],"options":{"context_field":"/context"}}),
        json!({"question":"q","inputs":[{"source":{"paths":["absent"],"unit":"line"}}]}),
        json!({"question":"q","source":{"paths":["absent"]},"records":[]}),
        json!({"question":"q","evidence":"x","options":{"api_key":"fake-secret"}}),
        json!({"question":"q","evidence":"x","unrecognized":"fake-secret"}),
        json!({"question":"q","source":{"paths":[],"unit":"line"}}),
        json!({"question":"q","source":{"paths":["absent"],"unit":"file","window":2}}),
    ] {
        let error = admit("decide", arguments).unwrap_err();
        assert_eq!(error.kind(), crate::ErrorKind::Usage);
        assert!(!format!("{error:?}").contains("fake-secret"));
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn seven_text_only_tools_refuse_images_before_open_or_send() {
    let backend = Backend::start().expect("loopback");
    for tool in [
        "tag",
        "filter",
        "rank",
        "find",
        "annotate",
        "recognize",
        "relate",
    ] {
        for arguments in [
            json!({"question":{},"images":["does-not-exist.png"]}),
            json!({"question":{},"inputs":[{"text":"x","images":["does-not-exist.png"]}]}),
            json!({"question":{},"source":{"paths":["does-not-exist"],"unit":"file","media":"image"}}),
        ] {
            let error = admit(tool, arguments).unwrap_err();

            assert_eq!(error.kind(), crate::ErrorKind::Usage);
        }
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn native_controls_validate_deadline_batch_and_admission_without_sends() {
    let backend = Backend::start().expect("loopback");
    for options in [
        json!({"deadline_ms":-2}),
        json!({"batch":0}),
        json!({"batch":"2"}),
        json!({"field":"invalid-pointer"}),
        json!({"top":1}),
    ] {
        assert_eq!(
            admit(
                "decide",
                json!({"question":"q","evidence":"x","options":options})
            )
            .unwrap_err()
            .kind(),
            crate::ErrorKind::Usage
        );
    }
    let call = admit(
        "decide",
        json!({"question":"q","evidence":"x","options":{"deadline_ms":0,"max_requests_total":0}}),
    )
    .unwrap();
    let token = crate::CancelToken::new();
    assert!(call.controls(&token).is_ok());
    assert_eq!(backend.count(), 0);
}

#[test]
fn explicit_null_or_empty_selectors_are_rejected_without_sends() {
    let backend = Backend::start().unwrap();
    for arguments in [
        json!({"question":"q","evidence":null,"images":["absent.png"]}),
        json!({"question":null,"question_file":"absent","evidence":"x"}),
        json!({"question":"q","source":null,"evidence":"x"}),
        json!({"question_file":"","evidence":"x"}),
        json!({"question":"q","source":{"paths":[""]}}),
        json!({"question":"q","images":[""]}),
        json!({"question":"q","evidence":"x","options":{"deadline_ms":null}}),
    ] {
        assert_eq!(
            admit("decide", arguments).unwrap_err().kind(),
            crate::ErrorKind::Usage
        );
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn all_named_tools_reject_credential_arguments_and_withhold_question_debug() {
    let backend = Backend::start().unwrap();
    for tool in [
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
    ] {
        let arguments = json!({"question":"private-question","evidence":"private-evidence","options":{"api_key":"fake-secret"}});
        let error = admit(tool, arguments).unwrap_err();
        for secret in ["fake-secret", "private-question", "private-evidence"] {
            assert!(!format!("{error:?}").contains(secret));
        }
        let arguments: super::super::admission::Arguments = serde_json::from_value(
            json!({"question":"private-question","evidence":"private-evidence"}),
        )
        .unwrap();
        assert!(!format!("{arguments:?}").contains("private-evidence"));
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn inline_records_keep_original_value_types_and_authored_candidate_order() {
    let params: CallParams=serde_json::from_str(r#"{"name":"decide","arguments":{"question":"q","records":[false,null,{"options":{"zebra":"first","alpha":"second"},"text":"x"}]}}"#).unwrap();
    let call = Invocation::admit(params).unwrap();
    let records = call.arguments.records.unwrap();
    assert_eq!(records[0].get(), "false");
    assert_eq!(records[1].get(), "null");
    assert_eq!(
        records[2].get(),
        r#"{"options":{"zebra":"first","alpha":"second"},"text":"x"}"#
    );
}
