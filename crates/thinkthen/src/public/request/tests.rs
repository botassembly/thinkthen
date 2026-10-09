//! Canonical decoding and native header behavior at the request boundary.
use super::*;

#[test]
fn committed_request_schema_is_derived_from_deserialization_types() {
    let mut generator = schemars::generate::SchemaSettings::draft2020_12()
        .for_deserialize()
        .into_generator();
    let _descriptor = generator.subschema_for::<super::session::DescriptorDocument>();
    let schema = generator.into_root_schema_for::<Request>();
    let text = serde_json::to_string_pretty(&schema).unwrap() + "\n";
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/request.schema.json"
    );
    if std::fs::read_to_string(path).is_ok_and(|committed| committed == text) {
        return;
    }
    if std::env::var_os("THINKTHEN_WRITE_SCHEMA").is_some_and(|v| v == "1") {
        std::fs::write(path, text).unwrap();
    }
    panic!(
        "generated request schema differs; rewrite with THINKTHEN_WRITE_SCHEMA=1 cargo test -p thinkthen --lib committed_request_schema, then rerun"
    );
}
#[test]
fn canonical_objects_refuse_duplicate_unknown_and_null_controls() {
    let valid = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Does it fit?"},"input":{"kind":"text","text":"yes"}}}"#;
    assert!(Request::from_json(valid).unwrap().admit().is_ok());
    for text in [
        valid.replace(
            "\"schema\":\"thinkthen.request/1\"",
            "\"schema\":\"thinkthen.request/2\"",
        ),
        valid.replace(
            "\"function\":\"decide\"",
            "\"function\":\"decide\",\"function\":\"decide\"",
        ),
        valid.replace(
            "\"kind\":\"text\",\"text\":\"yes\"",
            "\"kind\":\"text\",\"text\":\"yes\",\"secret\":true",
        ),
        valid.replace("\"input\":", "\"options\":{\"context\":null},\"input\":"),
        valid.replace(
            "\"input\":",
            "\"options\":{\"context\":\"\",\"context\":\"\"},\"input\":",
        ),
    ] {
        assert!(Request::from_json(&text).is_err(), "{text}");
    }
}

#[test]
fn generated_deserialization_schema_agrees_with_canonical_control_shapes() {
    use crate::test_deadline::child::ChildEnvironment as _;
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let valid = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Fits?"},"input":{"kind":"text","text":"Alpha."}}}"#;
    let definition = |value: serde_json::Value| {
        valid.replace(
            r#"{"kind":"text","text":"Fits?"}"#,
            &serde_json::json!({"kind":"definition","value":value}).to_string(),
        )
    };
    let kinds: serde_json::Map<_, _> = (0..256)
        .map(|n| (format!("kind{n}"), serde_json::Value::Null))
        .collect();
    let texts = [
        valid.to_owned(),
        definition(serde_json::json!({"decide":"Fits?","true":null,"false":{"label":"no"}})),
        definition(serde_json::json!({"choose":"Which?"})),
        definition(serde_json::json!({"choose":"Which?","options":["a","b"]})),
        definition(serde_json::json!({"tag":"Which?","labels":["a"]})),
        definition(serde_json::json!({"score":"Grade?","levels":["low","high"]})),
        definition(serde_json::json!({"find":"Which?"})),
        definition(serde_json::json!({"version":1,"recognize":{"kinds":{"person":null}}})),
        definition(serde_json::json!({"version":1,"recognize":{"kinds":kinds}})),
        definition(serde_json::json!({"version":1,"recognize":{"snippet_pieces":0}})),
        definition(serde_json::json!({"version":1,"recognize":{"snippet_pieces":4294967295u32}})),
        definition(serde_json::json!({"version":1,"recognize":{"snippet_pieces":4294967296u64}})),
        definition(serde_json::json!({"version":1,"recognize":{"snippet_pieces":1.5}})),
        definition(serde_json::json!({"version":1,"recognize":{"snippet_pieces":-1}})),
        definition(serde_json::json!({"version":1,"recognize":{"mode":"boundary_only"}})),
        definition(serde_json::json!({"version":1,"recognize":{"mode":"whole"}})),
        definition(serde_json::json!({"version":1,"recognize":{"mode":null}})),
        definition(serde_json::json!({"version":1,"recognize":{"mode":"unknown"}})),
        definition(serde_json::json!({"version":1,"questions":{"fits":{"decide":"Fits?"}}})),
        definition(serde_json::json!({})),
        definition(serde_json::json!({"decide":"Fits?","unknown":true})),
        definition(serde_json::json!({"rank":"Fits?"})),
        definition(serde_json::json!({"version":1,"recognize":{"unknown":true}})),
        definition(serde_json::json!({"version":1,"recognize":{"kinds":null}})),
        definition(
            serde_json::json!({"version":1,"questions":{"fits":{"decide":"Fits?","model":"fixed"}}}),
        ),
        valid.replace("\"input\":", "\"options\":{\"context\":\"\"},\"input\":"),
        valid.replace("thinkthen.request/1", "thinkthen.request/2"),
        valid.replace("\"input\":", "\"options\":{\"context\":null},\"input\":"),
        valid.replace("\"input\":", "\"options\":{\"examples\":null},\"input\":"),
        valid.replace("\"input\":", "\"options\":{\"unexpected\":true},\"input\":"),
        valid.replace(
            "\"kind\":\"text\",\"text\":\"Alpha.\"",
            "\"kind\":\"text\",\"text\":\"Alpha.\",\"extra\":true",
        ),
    ];
    let cases = texts
        .iter()
        .map(|text| {
            (
                serde_json::from_str::<serde_json::Value>(text).unwrap(),
                Request::from_json(text).is_ok(),
            )
        })
        .collect::<Vec<_>>();
    let mut child = Command::new("python3").clear_environment()
        .arg("-c").arg("import json,sys; from jsonschema import Draft202012Validator; schema=json.load(open(sys.argv[1])); Draft202012Validator.check_schema(schema); validator=Draft202012Validator(schema); cases=json.load(sys.stdin); assert all(validator.is_valid(value)==expected for value,expected in cases)")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../../specification/request.schema.json"))
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&cases).unwrap())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn stage_context_is_closed_strict_and_recognition_only() {
    let request = r#"{"schema":"thinkthen.request/1","call":{"function":"recognize","question":{"kind":"definition","value":{"version":1,"recognize":{}}},"input":{"kind":"text","text":"Ada"},"options":{"stage_context":CONTROL}}}"#;
    for control in [
        r#"{"boundary":"","kind_edge":"two\nlines","relation":"r"}"#,
        "{}",
    ] {
        let text = request.replace("CONTROL", control);
        Request::from_json(&text).unwrap().admit().unwrap();
    }
    for control in [
        "null",
        r#"{"boundary":null}"#,
        r#"{"boundary":1}"#,
        r#"{"other":"x"}"#,
        r#"{"boundary":"x","boundary":"y"}"#,
    ] {
        assert!(Request::from_json(&request.replace("CONTROL", control)).is_err());
    }
    let saved = r#"{"version":1,"recognize":{"stage_context":CONTROL}}"#;
    for control in [
        "null",
        r#"{"relation":null}"#,
        r#"{"other":"x"}"#,
        r#"{"relation":"x","relation":"y"}"#,
    ] {
        assert!(
            crate::RecognizeQuestionFile::from_json(&saved.replace("CONTROL", control)).is_err()
        );
    }
    let question = crate::RecognizeQuestionFile::from_json(
        &saved.replace("CONTROL", r#"{"boundary":"saved","relation":"retained"}"#),
    )
    .unwrap();
    let prepared = question
        .question()
        .clone()
        .with_stage_context(crate::RecognitionStageContext {
            boundary: Some("".into()),
            ..crate::RecognitionStageContext::default()
        });
    let encoded = serde_json::to_string(&RequestDefinition::from(prepared)).unwrap();
    assert!(encoded.contains(r#""stage_context":{"boundary":"","relation":"retained"}"#));
    assert!(!format!("{:?}", question).contains("retained"));
}

#[test]
fn recognition_mode_resolves_before_source_reads_and_refuses_skipped_controls() {
    let request = r#"{"schema":"thinkthen.request/1","call":{"function":"recognize","question":{"kind":"definition","value":{"version":1,"recognize":DECL}},"input":{"kind":"source","source":{"paths":["missing-evidence"]}},"options":OPTIONS}}"#;
    for mode in ["whole", "boundary_only"] {
        let text = request
            .replace("DECL", "{}")
            .replace("OPTIONS", &format!(r#"{{"mode":"{mode}"}}"#));
        assert!(Request::from_json(&text).unwrap().admit().is_ok());
    }
    for control in [
        r#""relations":[]"#,
        r#""stage_context":{"kind_edge":""}"#,
        r#""stage_context":{"relation":""}"#,
    ] {
        let text = request
            .replace("DECL", &format!("{{{control}}}"))
            .replace("OPTIONS", r#"{"mode":"boundary_only"}"#);
        let error = Request::from_json(&text).unwrap().admit().unwrap_err();
        assert_eq!(
            error.detail().message(),
            "boundary_only recognition takes no relations, relation threshold, kind_edge context or relation context"
        );
    }
}

#[test]
fn recognition_authored_controls_survive_native_wire_round_trips_without_adding_defaults() {
    let request = |ask: crate::Recognize, mode| {
        Request::new(RequestCall::Recognize(RequestArguments {
            question: RequestQuestion::Definition { value: ask.into() },
            input: RequestInput::Text {
                text: "Ada".into(),
                images: Vec::new(),
            },
            options: RequestOptions {
                mode: Some(mode),
                ..RequestOptions::default()
            },
        }))
    };
    let implicit = request(
        crate::Recognize::builder().build().unwrap(),
        crate::RecognitionMode::BoundaryOnly,
    );
    let wire = serde_json::to_string(&implicit).unwrap();
    assert!(!wire.contains("relation_threshold"));
    assert!(Request::from_json(&wire).unwrap().admit().is_ok());
    for ask in [
        crate::Recognize::builder()
            .relation_threshold(0.5)
            .unwrap()
            .build()
            .unwrap(),
        crate::Recognize::from_json(r#"{"version":1,"recognize":{"relations":[]}}"#).unwrap(),
    ] {
        let authored = request(ask, crate::RecognitionMode::BoundaryOnly);
        let native = authored.clone().admit().unwrap_err();
        let decoded = Request::from_json(&serde_json::to_string(&authored).unwrap())
            .unwrap()
            .admit()
            .unwrap_err();
        assert_eq!(native.detail().message(), decoded.detail().message());
    }
    let saved = crate::Recognize::from_json(r#"{"version":1,"recognize":{"mode":"boundary_only","stage_context":{"relation":""}},"relation_threshold":0.5}"#).unwrap();
    let whole = request(saved, crate::RecognitionMode::Whole);
    assert!(whole.clone().admit().is_ok());
    assert!(
        Request::from_json(&serde_json::to_string(&whole).unwrap())
            .unwrap()
            .admit()
            .is_ok()
    );
}

#[test]
fn recognition_mode_and_cuts_have_closed_safe_admission() {
    let request = r#"{"schema":"thinkthen.request/1","call":{"function":"recognize","question":{"kind":"definition","value":{"version":1,"recognize":{}}},"input":{"kind":"text","text":"Ada"},"options":CONTROL}}"#;
    for controls in [
        r#"{"mode":null}"#,
        r#"{"mode":"secret-invalid"}"#,
        r#"{"mode":"whole","mode":"boundary_only"}"#,
        r#"{"threshold":"0.4:0.6"}"#,
        r#"{"relation_threshold":"0.4:0.6"}"#,
        r#"{"mode":"boundary_only","relation_threshold":0.5}"#,
    ] {
        let text = request.replace("CONTROL", controls);
        let error = Request::from_json(&text)
            .and_then(Request::admit)
            .unwrap_err();
        assert!(!error.detail().message().contains("secret-invalid"));
    }
    for controls in [
        r#"{"threshold":0.5}"#,
        r#"{"mode":"whole","relation_threshold":0.5}"#,
        r#"{"mode":"boundary_only","threshold":0.5,"stage_context":{"boundary":""}}"#,
    ] {
        assert!(
            Request::from_json(&request.replace("CONTROL", controls))
                .unwrap()
                .admit()
                .is_ok()
        );
    }
}

#[test]
fn snippet_width_is_unsigned_strict_and_recognition_only() {
    let request = r#"{"schema":"thinkthen.request/1","call":{"function":"recognize","question":{"kind":"definition","value":{"version":1,"recognize":{}}},"input":{"kind":"text","text":"Ada"},"options":{"snippet_pieces":WIDTH}}}"#;
    for width in ["0", "6", "4294967295"] {
        Request::from_json(&request.replace("WIDTH", width))
            .unwrap()
            .admit()
            .unwrap();
        let saved = format!(r#"{{"version":1,"recognize":{{"snippet_pieces":{width}}}}}"#);
        assert_eq!(
            crate::Recognize::from_json(&saved)
                .unwrap()
                .reading()
                .snippet_pieces(),
            width.parse::<u32>().unwrap()
        );
    }
    for width in ["null", "-1", "1.5", "4294967296", r#""6""#] {
        assert!(Request::from_json(&request.replace("WIDTH", width)).is_err());
        let saved = format!(r#"{{"version":1,"recognize":{{"snippet_pieces":{width}}}}}"#);
        assert!(crate::Recognize::from_json(&saved).is_err());
    }
    let default = crate::Recognize::builder().build().unwrap();
    let six = crate::Recognize::builder()
        .snippet_pieces(6)
        .build()
        .unwrap();
    assert_eq!(default, six);
    let decide = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Fits?"},"input":{"kind":"text","text":"Ada"},"options":{"snippet_pieces":0}}}"#;
    assert!(Request::from_json(decide).unwrap().admit().is_err());
}

#[test]
fn native_rank_admission_does_not_promote_the_decide_default_to_a_cutoff() {
    let request = Request::new(RequestCall::Rank(RequestArguments {
        question: RequestQuestion::Definition {
            value: crate::Question::decide("Best?").unwrap().cut().into(),
        },
        input: RequestInput::Records { items: Vec::new() },
        options: RequestOptions::default(),
    }));
    assert!(request.call.arguments().options.threshold.is_none());
    let admitted = request.admit().unwrap();
    let RequestDefinition::Rank(question) = admitted.resolve_question().unwrap() else {
        panic!("rank admission");
    };
    assert!(question.threshold.is_none());
}
