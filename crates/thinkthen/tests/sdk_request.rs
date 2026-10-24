#![cfg(test)]
//! The SDK admission boundary retains typed toolboxes and isolates client operations.

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Cancel, SdkRequest, SdkSettings, Toolbox};

#[test]
fn sdk_clients_reuse_admitted_questions_with_their_own_settings_and_cancel() {
    let server = Loopback::noul(0.9).unwrap();
    let settings = SdkSettings::from_json(
        &json!({"url":server.base(),"batch":1,"model":"client-model"}).to_string(),
    )
    .unwrap();
    let engine = settings
        .engine(|name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()))
        .unwrap();
    let toolbox =
        Toolbox::from_bytes(br#"{"on":"/wrong","questions":{"ready":{"decide":"Ready?"}}}"#)
            .unwrap();
    assert_eq!(
        SdkRequest::from_question(
            "choose",
            toolbox.question(Some("ready")).unwrap().clone(),
            "{}"
        )
        .unwrap_err()
        .kind()
        .code(),
        "usage.question"
    );
    let request = SdkRequest::from_question(
        "decide",
        toolbox.question(Some("ready")).unwrap().clone(),
        r#"{"on":"/body","threshold":0.95}"#,
    )
    .unwrap();
    let cancelled = Cancel::default();
    cancelled.cancel();
    let stopped = request.prepare(&engine, cancelled).unwrap();
    let error = stopped
        .run(
            [(stopped.record(&json!({"body":"evidence"})), ())],
            |(), _| panic!("cancelled call emitted a row"),
        )
        .unwrap_err();
    assert_eq!(error.kind().code(), "local.cancelled");
    assert_eq!(server.count(), 0);
    let live = request.prepare(&engine, Cancel::default()).unwrap();
    let mut rows = Vec::new();
    live.run(
        [(live.record(&json!({"id":"original","body":"evidence"})), 42)],
        |original, row| {
            assert_eq!(original, 42);
            rows.push(serde_json::to_value(row).unwrap());
        },
    )
    .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["id"], "original");
    assert_eq!(rows[0]["answer"], false);
    assert_eq!(
        server.requests()[0].json().unwrap()["model"],
        "client-model"
    );
    assert_eq!(server.count(), 1);
    let annotation = SdkRequest::from_toolbox(toolbox, r#"{"on":"/body"}"#)
        .unwrap()
        .prepare(&engine, Cancel::default())
        .unwrap();
    rows.clear();
    annotation
        .run(
            [(annotation.record(&json!({"body":"evidence"})), ())],
            |(), row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["answer"]["ready"]["answer"], true);
    assert_eq!(server.count(), 2);
}

#[test]
fn named_questions_apply_vector_overrides_before_running() {
    for (function, field, overrides, reply, expected) in [
        (
            "choose",
            "options",
            json!(["first", "middle", "last"]),
            json!({"type":"choice","probabilities":{"first":0,"middle":0,"last":1}}),
            json!("last"),
        ),
        (
            "tag",
            "labels",
            json!(["first", "middle", "last"]),
            json!({"type":"noul","noul":0.9}),
            json!(["first", "middle", "last"]),
        ),
        (
            "score",
            "levels",
            json!(["first", "middle", "last"]),
            json!({"type":"score","probabilities":{"0":0,"1":0,"2":1}}),
            json!(1.0),
        ),
        (
            "choose",
            "options",
            json!(null),
            json!({"type":"choice","probabilities":{"old":1,"older":0}}),
            json!("old"),
        ),
        (
            "tag",
            "labels",
            json!(null),
            json!({"type":"noul","noul":0.9}),
            json!(["old", "older"]),
        ),
        (
            "score",
            "levels",
            json!(null),
            json!({"type":"score","probabilities":{"0":1,"1":0}}),
            json!(0.0),
        ),
    ] {
        let server = Loopback::answering(move |_| reply.clone()).unwrap();
        let settings = SdkSettings::from_json(&json!({"url":server.base()}).to_string()).unwrap();
        let engine = settings
            .engine(|name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()))
            .unwrap();
        let toolbox = Toolbox::from_bytes(
            json!({(function):"Which?",(field):["old", "older"]})
                .to_string()
                .as_bytes(),
        )
        .unwrap();
        let options = if overrides.is_null() {
            json!({})
        } else {
            json!({(field):overrides})
        };
        let request = SdkRequest::from_question(
            function,
            toolbox.question(None).unwrap().clone(),
            &options.to_string(),
        )
        .unwrap();
        let call = request.prepare(&engine, Cancel::default()).unwrap();
        let mut rows = Vec::new();
        call.run([(call.record(&json!("evidence")), ())], |(), row| {
            rows.push(serde_json::to_value(row).unwrap());
        })
        .unwrap();
        assert_eq!(rows.len(), 1, "{function}");
        assert_eq!(rows[0]["status"], "answered", "{function}");
        assert_eq!(rows[0]["answer"], expected, "{function}");
        assert_eq!(server.count(), 1, "{function}");
    }
}

#[test]
fn json_admission_keeps_library_errors_before_sending() {
    let server = Loopback::noul(0.9).unwrap();
    let settings = SdkSettings::from_json(&json!({"url":server.base()}).to_string()).unwrap();
    let engine = settings
        .engine(|name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()))
        .unwrap();
    assert_eq!(
        SdkSettings::from_json("{").unwrap_err().kind().code(),
        "usage.settings"
    );
    assert_eq!(
        SdkRequest::from_json("{").unwrap_err().kind().code(),
        "usage.question"
    );
    for (function, question, options, code) in [
        ("annotate", json!({}), json!({}), "usage.question"),
        (
            "find",
            json!("Who?"),
            json!({"threshold":"0.2:0.8"}),
            "usage.threshold",
        ),
        (
            "recognize",
            json!(["person"]),
            json!({"threshold":"0.2:0.8"}),
            "usage.threshold",
        ),
        (
            "relate",
            json!([{"name":"works_for","source":"person","target":"company"}]),
            json!({"threshold":"0.2:0.8"}),
            "usage.threshold",
        ),
    ] {
        let request = SdkRequest::from_json(
            &json!({"function":function,"question":{"type":"inline","value":question},"options":options}).to_string(),
        )
        .unwrap();
        assert_eq!(
            request
                .prepare(&engine, Cancel::default())
                .unwrap_err()
                .kind()
                .code(),
            code,
            "{function}"
        );
    }
    let choice = Toolbox::from_bytes(br#"{"choose":"Which?","options":["old","older"]}"#).unwrap();
    let invalid = SdkRequest::from_question(
        "choose",
        choice.question(None).unwrap().clone(),
        r#"{"options":["only"]}"#,
    )
    .unwrap();
    assert_eq!(
        invalid
            .prepare(&engine, Cancel::default())
            .unwrap_err()
            .kind()
            .code(),
        "usage.question"
    );
    assert_eq!(server.count(), 0);
}

#[test]
fn json_rank_admission_uses_score_levels_and_top() {
    let server = Loopback::answering(|question| {
        assert_eq!(question["type"], "score");
        assert_eq!(question["criteria"], json!(["low", "high"]));
        let high = question["instructions"]
            .as_str()
            .unwrap()
            .contains("\"higher\"");
        let p = if high { 1 } else { 0 };
        json!({"type":"score","probabilities":{"0":1-p,"1":p}})
    })
    .unwrap();
    let settings = SdkSettings::from_json(&json!({"url":server.base()}).to_string()).unwrap();
    let engine = settings
        .engine(|name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()))
        .unwrap();
    let call = SdkRequest::from_json(r#"{"function":"rank","question":{"type":"inline","value":"How urgent?"},"options":{"levels":["low","high"],"top":1}}"#)
        .unwrap()
        .prepare(&engine, Cancel::default())
        .unwrap();
    let mut rows = Vec::new();
    call.run(
        [
            (call.record(&json!("lower")), 10),
            (call.record(&json!("higher")), 20),
        ],
        |original, row| rows.push((original, serde_json::to_value(row).unwrap())),
    )
    .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, 20);
    assert_eq!(rows[0].1["id"], 2);
    assert_eq!(rows[0].1["answer"], 1);
    assert_eq!(server.count(), 1);
}
