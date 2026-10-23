#![cfg(test)]
//! Toolboxes through the public Rust API.
use loopback::{KEY, Loopback};
use serde_json::json;
use std::io::{self, Read};
use thinkthen::{Backend, Engine, Record, TOOLBOX_LIMIT, Toolbox, read_toolbox};

#[test]
fn every_question_kind_runs_through_the_engine() {
    let cases = [
        (
            json!({"decide":"Is it urgent?", "yes":"today", "no":"later"}),
            json!({"type":"noul", "noul":0.9}),
            json!(true),
        ),
        (
            json!({"filter":"Is it urgent?"}),
            json!({"type":"noul", "noul":0.9}),
            json!(true),
        ),
        (
            json!({"choose":"What topic?", "options":["billing","shipping"]}),
            json!({"type":"choice", "probabilities":{"billing":0.9,"shipping":0.1}}),
            json!("billing"),
        ),
        (
            json!({"tag":"What applies?", "labels":["urgent"]}),
            json!({"type":"noul", "noul":0.9}),
            json!(["urgent"]),
        ),
        (
            json!({"score":"How urgent?", "levels":["low","high"]}),
            json!({"type":"score", "probabilities":{"0":0.2,"1":0.8}}),
            json!(0.8),
        ),
    ];
    for (spec, reply, expected) in cases {
        let server = Loopback::answering(move |_| reply.clone()).unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        let toolbox = Toolbox::from_bytes(spec.to_string().as_bytes()).unwrap();
        let question = toolbox.question(None).unwrap();
        let mut rows = Vec::new();
        Engine::new(backend)
            .call(question, [Record::text("now")], |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows[0]["status"], "answered");
        assert_eq!(rows[0]["answer"], expected);
        assert_eq!(server.count(), 1);
    }
}

#[test]
fn question_evidence_overrides_shared_evidence_and_names_are_explicit() {
    let toolbox = Toolbox::from_bytes(br#"{"on":"/body","questions":{"inherited":{"decide":"Yes?"},"own":{"decide":"Yes?","on":"/text"}}}"#).unwrap();
    for (name, pointer) in [("inherited", "/body"), ("own", "/text")] {
        assert_eq!(
            toolbox
                .question(Some(name))
                .unwrap()
                .on()
                .unwrap()
                .to_string(),
            pointer
        );
    }
    assert_eq!(
        toolbox.question(None).unwrap_err().kind().code(),
        "local.toolbox"
    );
    assert_eq!(
        toolbox.question(Some("missing")).unwrap_err().kind().code(),
        "local.toolbox"
    );
    assert_eq!(
        toolbox
            .questions()
            .map(|(name, _)| name)
            .collect::<Vec<_>>(),
        ["inherited", "own"]
    );
    for definition in [
        r#"{"decide":"Yes?","model":"route"}"#,
        r#"{"decide":"Yes?","batch":2}"#,
    ] {
        assert_eq!(
            Toolbox::from_bytes(definition.as_bytes())
                .unwrap_err()
                .kind()
                .code(),
            "local.toolbox"
        );
    }
}

#[test]
fn malformed_toolboxes_refuse_the_named_field() {
    let cases = [
        (r#"{"unknown":true,"decide":"Yes?"}"#, "unknown"),
        (
            r#"{"questions":{"q":{"decide":"Yes?"}},"unknown":1}"#,
            "unknown",
        ),
        (r#"{"model":"a","model":"b","decide":"Yes?"}"#, "model"),
        (
            r#"{"questions":{"q":{"decide":"Yes?"},"q":{"decide":"No?"}}}"#,
            "q",
        ),
        (r#"{"decide":"Yes?","decide":"No?"}"#, "decide"),
        (
            r#"{"questions":{"q":{"decide":"Yes?","threshold":".5","threshold":".9"}}}"#,
            "threshold",
        ),
        (r#"{"decide":"Yes?","options":["a","b"]}"#, "options"),
        (
            r#"{"choose":"Which?","options":["a","b"],"labels":["a"]}"#,
            "labels",
        ),
        (
            r#"{"tag":"Which?","labels":["a"],"levels":["a","b"]}"#,
            "levels",
        ),
        (
            r#"{"score":"How?","levels":["a","b"],"threshold":".8"}"#,
            "threshold",
        ),
        (r#"{"filter":"Yes?","levels":["a","b"]}"#, "levels"),
        (
            r#"{"tag":"Which?","labels":["a"],"threshold":".2:.8"}"#,
            "threshold",
        ),
        (r#"{"decide":"Yes?","threshold":"bad"}"#, "threshold"),
        (r#"{"decide":"Yes?","batch":0}"#, "batch"),
        (r#"{"decide":"Yes?","batch":1.5}"#, "batch"),
        (r#"{"decide":"Yes?","on":"bad"}"#, "on"),
        (r#"{"decide":"Yes?","model":null}"#, "model"),
        (r#"{"questions":{"q":{"decide":"Yes?","on":"bad"}}}"#, "on"),
    ];
    for (text, field) in cases {
        let error = Toolbox::from_bytes(text.as_bytes()).unwrap_err();
        assert_eq!(error.kind().code(), "local.toolbox", "{text}");
        assert!(error.to_string().contains(field), "{text}: {error}");
    }
    for text in [
        "",
        "[]",
        "{}",
        "{",
        r#"{"questions":{}}"#,
        r#"{"questions":{" ":{"decide":"Yes?"}}}"#,
        r#"{"decide":"Yes?","choose":"Which?"}"#,
        r#"{"decide":" "}"#,
        r#"{"score":"How?"}"#,
        r#"{"choose":"Which?","options":["a","a"]}"#,
        r#"{"tag":"Which?","labels":[]}"#,
    ] {
        assert_eq!(
            Toolbox::from_bytes(text.as_bytes())
                .unwrap_err()
                .kind()
                .code(),
            "local.toolbox",
            "{text}"
        );
    }
}

#[test]
fn toolbox_reads_stop_one_byte_past_the_limit() {
    struct Counted {
        read: usize,
    }
    impl Read for Counted {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            bytes.fill(b' ');
            self.read += bytes.len();
            Ok(bytes.len())
        }
    }
    let mut reader = Counted { read: 0 };
    assert_eq!(
        read_toolbox(&mut reader).unwrap_err().kind().code(),
        "local.toolbox"
    );
    assert_eq!(reader.read, TOOLBOX_LIMIT + 1);
    let mut bytes = br#"{"decide":"Yes?"}"#.to_vec();
    bytes.resize(TOOLBOX_LIMIT, b' ');
    assert!(read_toolbox(bytes.as_slice()).is_ok());
    bytes.push(b' ');
    assert_eq!(
        Toolbox::from_bytes(&bytes).unwrap_err().kind().code(),
        "local.toolbox"
    );
    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("broken"))
        }
    }
    assert_eq!(read_toolbox(Broken).unwrap_err().kind().code(), "local.io");
}
