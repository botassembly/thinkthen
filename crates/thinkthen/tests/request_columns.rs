//! Nullable host columns project actual public Request results without sending.
use serde_json::{Value, json};
use thinkthen::*;
#[path = "support/api.rs"]
mod api;

#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "invalid isolated fixture setup must stop the regression"
)]
fn item(text: &str) -> RequestItem {
    RequestItem::from_json(&json!({"original":{"kind":"text","text":text}}).to_string()).unwrap()
}
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "invalid isolated fixture setup must stop the regression"
)]
fn response(body: &[u8]) -> conformance_backend::Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let answers = request["questions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, q)| {
            let instructions = q["instructions"].as_str().unwrap();
            let second_record = instructions.contains("Second.");
            let answer = if q["type"] == "noul" {
                let second_label = instructions.contains("label \"second\"");
                json!({"type":"noul","noul":probability("0",second_record != second_label)})
            } else {
                let labels: Vec<String> = match &q["criteria"] {
                    Value::Object(v) => v.keys().cloned().collect(),
                    Value::Array(v) => (0..v.len()).map(|at| at.to_string()).collect(),
                    _ => panic!("criteria: {q}"),
                };
                let probabilities = labels
                    .iter()
                    .map(|label| (label.clone(), json!(probability(label, second_record))))
                    .collect::<serde_json::Map<_, _>>();
                json!({"type":q["type"],"probabilities":probabilities})
            };
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    conformance_backend::Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}
fn probability(label: &str, second: bool) -> f64 {
    if ["0", "first", "low"].contains(&label) != second {
        0.9
    } else {
        0.1
    }
}
fn values(view: RequestColumnView) -> Value {
    match view.values {
        RequestColumnValues::Decisions(v) => json!(v),
        RequestColumnValues::Choices(v) => json!(v),
        RequestColumnValues::Tags(v) => json!(v),
        RequestColumnValues::Scores(v) => json!(v),
    }
}

#[test]
fn nullable_positions_and_result_prefixes_keep_typed_values_and_probabilities() {
    let cases = [
        (
            RequestFunction::Decide,
            r#"{"decide":"Fits?"}"#,
            json!([null, true, null, false, null]),
            Some(vec![None, Some(0.9), None, Some(0.1), None]),
        ),
        (
            RequestFunction::Choose,
            r#"{"choose":"Which?","options":["first","second"]}"#,
            json!([null, "first", null, "second", null]),
            Some(vec![None, Some(0.9), None, Some(0.9), None]),
        ),
        (
            RequestFunction::Tag,
            r#"{"tag":"Which?","labels":["first","second"]}"#,
            json!([[], ["first"], [], ["second"], []]),
            None,
        ),
        (
            RequestFunction::Score,
            r#"{"score":"Grade?","levels":{"low":{"what":"Little."},"high":null}}"#,
            json!([null, 0.1, null, 0.9, null]),
            None,
        ),
    ];
    for (function, definition, expected, probability) in cases {
        let listener = conformance_backend::Listener::answering(response).unwrap();
        let engine = api::loopback(listener.base(), "column-fixture");
        let first = Some(item("First."));
        let second = Some(item("Second."));
        let (column, items) =
            RequestColumn::new(function, vec![None, first, None, second, None]).unwrap();
        assert_eq!(column.positions(), &[1, 3]);
        assert_eq!(column.len(), 5);
        assert!(!column.is_empty());
        let call = api::request_question(
            &engine,
            function,
            RequestQuestion::from_json(&format!(r#"{{"kind":"definition","value":{definition}}}"#))
                .unwrap(),
            RequestInput::Records { items },
            RequestEnvironment::default(),
        )
        .unwrap();
        if let RequestValue::Scores(rows) = call.value() {
            let question = rows[0].result().question();
            let levels = question.options().collect::<Vec<_>>();
            assert_eq!(
                levels.iter().map(|level| level.name()).collect::<Vec<_>>(),
                ["low", "high"]
            );
            assert_eq!(
                levels[0].description().unwrap().to_json().unwrap(),
                r#"{"what":"Little."}"#
            );
            assert!(levels[1].description().unwrap().is_null());
        }
        let before = listener.count();
        assert!(before > 0);
        let view = column.view(call.value()).unwrap().unwrap();
        assert_eq!(view.probability, probability);
        assert_eq!(values(view), expected);
        let mut prefix = call.into_value();
        match &mut prefix {
            RequestValue::Decisions(v) => v.truncate(1),
            RequestValue::Choices(v) => v.truncate(1),
            RequestValue::Tags(v) => v.truncate(1),
            RequestValue::Scores(v) => v.truncate(1),
            _ => panic!("atomic result"),
        }
        let prefix_view = column.view(&prefix).unwrap().unwrap();
        let expected_probability = probability.map(|mut values| {
            values[3] = None;
            values
        });
        assert_eq!(prefix_view.probability, expected_probability);
        let mut expected_prefix = expected;
        expected_prefix[3] = expected_prefix[0].clone();
        assert_eq!(values(prefix_view), expected_prefix);
        let (empty, _) = RequestColumn::new(function, vec![]).unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.view(&prefix).unwrap_err().kind(), ErrorKind::Defect);
        assert_eq!(listener.count(), before, "projection sends nothing");
    }
}

#[test]
fn complete_collections_refuse_missing_cells_and_aggregate_results_have_no_column_view() {
    let listener =
        api::answering(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#);
    let engine = api::loopback(listener.base(), "column-fixture");
    for function in [
        RequestFunction::Filter,
        RequestFunction::Rank,
        RequestFunction::Find,
        RequestFunction::Annotate,
        RequestFunction::Relate,
    ] {
        let error = RequestColumn::new(function, vec![Some(item("First.")), None]).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(error.to_string().contains("takes no NA records"));
        let (empty, items) = RequestColumn::new(function, vec![]).unwrap();
        assert!(empty.is_empty());
        assert!(items.is_empty());
    }
    assert_eq!(listener.count(), 0);
    let call = api::request_question(
        &engine,
        RequestFunction::Filter,
        RequestQuestion::Text {
            text: "Fits?".into(),
        },
        RequestInput::Records {
            items: vec![item("First.")],
        },
        RequestEnvironment::default(),
    )
    .unwrap();
    let (column, _) =
        RequestColumn::new(RequestFunction::Filter, vec![Some(item("First."))]).unwrap();
    let before = listener.count();
    assert!(column.view(call.value()).unwrap().is_none());
    assert_eq!(listener.count(), before);
}

#[test]
fn inline_labels_keep_caller_order_descriptions_and_authored_fields_at_execution() {
    let labels = RequestQuestion::labels(
        vec!["zeta".into(), "alpha".into()],
        vec![(
            "alpha".into(),
            Description::text("Second description.").unwrap(),
        )],
        false,
    )
    .unwrap();
    assert_eq!(
        labels.as_json(),
        r#"{"zeta":null,"alpha":"Second description."}"#
    );
    let question = RequestQuestion::inline(
        RequestFunction::Choose,
        Some("Which?".into()),
        vec![
            ("options".into(), labels),
            ("name".into(), Description::text("authored").unwrap()),
        ],
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&question).unwrap()["value"]["name"],
        "authored"
    );
    let listener = api::answering(
        r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"zeta":0.9,"alpha":0.1}}}}"#,
    );
    let engine = api::loopback(listener.base(), "inline-fixture");
    let call = api::request_question(
        &engine,
        RequestFunction::Choose,
        question,
        RequestInput::Records {
            items: vec![item("Evidence.")],
        },
        RequestEnvironment::default(),
    )
    .unwrap();
    let RequestValue::Choices(rows) = call.value() else {
        panic!("choice result")
    };
    assert_eq!(rows[0].result().value(), Some("zeta"));
    assert_eq!(listener.count(), 1);
    let requests = listener.requests();
    let wire: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        wire["questions"]["q1"]["criteria"],
        json!({"zeta":null,"alpha":"Second description."})
    );
    let Probabilities::Named(probabilities) = rows[0].result().probabilities() else {
        panic!("named probabilities")
    };
    assert_eq!(
        probabilities.iter().map(|p| p.name()).collect::<Vec<_>>(),
        vec!["zeta", "alpha"]
    );
}

#[test]
fn invalid_inline_fields_and_label_sets_refuse_with_usage_errors() {
    for (names, descriptions) in [
        (vec!["same".into(), "same".into()], vec![]),
        (
            vec!["known".into()],
            vec![("unknown".into(), Description::text("Unknown.").unwrap())],
        ),
    ] {
        assert_eq!(
            RequestQuestion::labels(names, descriptions, false)
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
    for (function, text, fields) in [
        (
            RequestFunction::Choose,
            Some("Which?".into()),
            vec![("options".into(), Description::text("invalid").unwrap())],
        ),
        (
            RequestFunction::Choose,
            Some("Which?".into()),
            vec![(
                "options".into(),
                RequestQuestion::labels(vec!["".into()], vec![], false).unwrap(),
            )],
        ),
        (
            RequestFunction::Choose,
            Some("Which?".into()),
            vec![
                (
                    "options".into(),
                    Description::from_json(r#"["a","b"]"#).unwrap(),
                ),
                (
                    "options".into(),
                    Description::from_json(r#"["x","y"]"#).unwrap(),
                ),
            ],
        ),
        (
            RequestFunction::Decide,
            Some("Fits?".into()),
            vec![("unknown".into(), Description::from_json("null").unwrap())],
        ),
        (
            RequestFunction::Recognize,
            None,
            vec![("kinds".into(), Description::from_json("[42]").unwrap())],
        ),
        (
            RequestFunction::Annotate,
            None,
            vec![("name".into(), Description::text("Authored.").unwrap())],
        ),
    ] {
        assert_eq!(
            RequestQuestion::inline(function, text, fields)
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
}

#[test]
fn inline_literal_and_authored_decision_readings_use_the_same_public_execution() {
    let listener =
        api::answering(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#);
    let engine = api::loopback(listener.base(), "inline-fixture");
    for fields in [
        vec![],
        vec![
            (
                "true".into(),
                Description::text("Matches the reading.").unwrap(),
            ),
            (
                "false".into(),
                Description::from_json(r#"{"reading":"Does not match."}"#).unwrap(),
            ),
            ("threshold".into(), Description::text("0.95:1").unwrap()),
        ],
    ] {
        let authored = !fields.is_empty();
        let question =
            RequestQuestion::inline(RequestFunction::Decide, Some("Fits?".into()), fields).unwrap();
        let call = api::request_question(
            &engine,
            RequestFunction::Decide,
            question,
            RequestInput::Records {
                items: vec![item("Evidence.")],
            },
            RequestEnvironment::default(),
        )
        .unwrap();
        let RequestValue::Decisions(rows) = call.value() else {
            panic!("decision result")
        };
        assert_eq!(
            rows[0].result().value(),
            if authored { Answer::No } else { Answer::Yes }
        );
        assert_eq!(call.facts().requests_sent(), 1);
    }
    assert_eq!(listener.count(), 2);
    let requests = listener.requests();
    let wire = String::from_utf8_lossy(&requests[1].body);
    assert!(wire.contains("Matches the reading."));
    assert!(wire.contains("Does not match."));
}
