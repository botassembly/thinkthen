#![cfg(test)]
//! Canonical requests admit only each function's own typed settings.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use serde_json::json;
use thinkthen::{BackendName, Code, Key, Question, Request, Settings, Threshold, questions};

#[test]
fn questions_require_their_domains_and_round_trip_the_canonical_contract() {
    let cases = [
        json!({"function":"decide","question":"Ready?"}),
        json!({"function":"filter","question":"Keep?","threshold":{"low":0.2,"high":0.8}}),
        json!({"function":"choose","question":"Which?","choices":["a","b"]}),
        json!({"function":"tag","question":"Which?","labels":["a"],"threshold":0.7}),
        json!({"function":"score","question":"How?","levels":["low","high"]}),
        json!({"function":"rank","question":"How?","levels":["low","high"],"top":2}),
        json!({"function":"find","question":"Where?"}),
        json!({"function":"recognize","kinds":["person"]}),
        json!({"function":"relate","rules":[{"relation":"knows","source":"person","target":"person"}]}),
        json!({"function":"annotate","questions":{"ready":{"function":"decide","question":"Ready?"}}}),
    ];
    let schema = serde_json::to_value(schemars::schema_for!(Request)).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for question in cases {
        let envelope = json!({"question":question,"name":"named","records":["evidence"]});
        validator.validate(&envelope).unwrap();
        let request = Request::from_json(&envelope.to_string()).unwrap();
        assert_eq!(request.records().len(), 1);
        assert_eq!(
            serde_json::to_value(request.question().function()).unwrap(),
            question["function"]
        );
        assert_eq!(request.question().evidence_pointer().to_string(), "");
        assert_eq!(request.name(), Some("named"));
        let encoded = serde_json::to_string(&request).unwrap();
        assert_eq!(
            serde_json::to_value(Request::from_json(&encoded).unwrap()).unwrap(),
            serde_json::to_value(request).unwrap()
        );
    }
    for function in ["tag", "find", "recognize", "relate"] {
        let mut question = match function {
            "tag" => json!({"function":function,"question":"Which?","labels":["a"]}),
            "find" => json!({"function":function,"question":"Where?"}),
            "recognize" => json!({"function":function,"kinds":["person"]}),
            _ => {
                json!({"function":function,"rules":[{"relation":"knows","source":"person","target":"person"}]})
            }
        };
        question["threshold"] = json!({"low":0.2,"high":0.8});
        assert_eq!(
            Question::from_json(&question.to_string())
                .unwrap_err()
                .code(),
            Code::UsageThreshold
        );
    }
    for question in [
        json!({"function":"choose","question":"Which?"}),
        json!({"function":"recognize","kinds":[]}),
        json!({"function":"annotate","questions":{"rank":{"function":"rank","question":"How?","levels":["low","high"]}}}),
        json!({"function":"rank","question":"How?","levels":["low","high"],"top":0}),
        json!({"function":"decide","question":"Ready?","options":{}}),
        json!({"function":"decide","question":{"type":"reference","name":"ready"}}),
    ] {
        assert!(Question::from_json(&question.to_string()).is_err());
        assert!(!validator.is_valid(&json!({"question":question})));
    }
}

#[test]
fn settings_type_backend_and_counts_and_never_serialize_or_debug_keys() {
    let secret = "synthetic-direct-key";
    let settings = Settings::default().api_key(Key::new(secret).unwrap());
    assert_eq!(settings.backend(), BackendName::SystemOne);
    assert!(!format!("{settings:?}").contains(secret));
    assert!(!serde_json::to_string(&settings).unwrap().contains(secret));
    for value in [
        json!({"mode":"proxy"}),
        json!({"adapter":"stub"}),
        json!({"key_source":"OTHER"}),
        json!({"backend":"unknown"}),
        json!({"batch":0}),
        json!({"batch":1.5}),
        json!({"retry":{"max_retries":1.5}}),
    ] {
        assert!(Settings::from_json(&value.to_string()).is_err());
    }
    assert_eq!(
        Settings::from_json(r#"{"backend":"unknown"}"#)
            .unwrap_err()
            .code(),
        Code::UsageBackend
    );
    let stub =
        Settings::from_json(r#"{"backend":"stub","batch":2,"retry":{"max_retries":0}}"#).unwrap();
    stub.engine(|_| panic!("stub must read no variables"))
        .unwrap();
}

#[test]
fn malformed_thresholds_keep_the_shared_threshold_code() {
    for threshold in [
        json!(0),
        json!(1.1),
        json!("0.2:0.8"),
        json!([0.2, 0.8]),
        json!({"low":0.8,"high":0.2}),
    ] {
        let question = json!({"function":"decide","question":"Ready?","threshold":threshold});
        assert_eq!(
            Question::from_json(&question.to_string())
                .unwrap_err()
                .code(),
            Code::UsageThreshold
        );
    }
}

#[test]
fn both_backends_use_direct_keys_and_only_their_own_environment_fallback() {
    use loopback::{KEY, Loopback, Reply};
    use std::time::Duration;
    use thinkthen::{Decide, Record, Retry};
    for (backend, variable, path) in [
        (BackendName::SystemOne, "THINKTHEN_API_KEY", "/v1/systemone"),
        (BackendName::OpenAi, "OPENAI_API_KEY", "/v1/decisions"),
    ] {
        let server = Loopback::start(move |request| {
            if backend == BackendName::OpenAi {
                Reply::json(200, &json!({"answers":[{"type":"predicate","name":null,"probability":0.8123456789012345}]}))
            } else {
                let body = request.json().unwrap();
                let answers: serde_json::Map<String, serde_json::Value> = body["questions"].as_object().unwrap().keys().map(|name| (name.clone(), json!({"type":"noul","noul":0.8123456789012345}))).collect();
                Reply::json(200, &json!({"answers":answers}))
            }
        }).unwrap();
        for direct in [false, true] {
            let mut settings = Settings::default()
                .with_backend(backend)
                .url(server.base())
                .batch(2_u64.try_into().unwrap())
                .model("custom-model")
                .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(5)).unwrap());
            if direct {
                settings = settings.api_key(Key::new(KEY).unwrap());
            }
            let reads = std::cell::Cell::new(0);
            let engine = settings
                .engine(|name| {
                    assert!(!direct, "a direct key must avoid credential lookup");
                    assert_eq!(name, variable);
                    reads.set(reads.get() + 1);
                    Some(KEY.to_owned())
                })
                .unwrap();
            let row = engine
                .run(&Decide::new("Ready?").unwrap(), Record::text("evidence"))
                .unwrap();
            assert_eq!(row.probability().unwrap().get(), 0.8123456789012345);
            assert_eq!(row.into_result().unwrap(), Some(true));
            assert_eq!(reads.get(), usize::from(!direct));
            assert!(!format!("{settings:?}").contains(KEY));
        }
        for request in server.requests() {
            assert_eq!(request.path, path);
            assert_eq!(request.json().unwrap()["model"], "custom-model");
            assert_eq!(
                request.header("authorization"),
                Some(format!("Bearer {KEY}").as_str())
            );
        }
    }
}

#[test]
fn toolbox_selection_retains_required_domains_and_rejects_duplicate_names() {
    use std::collections::BTreeMap;
    let toolbox = questions::Toolbox::new(BTreeMap::from([(
        "route".to_owned(),
        questions::Choose::new("Which?", ["billing", "shipping"])
            .unwrap()
            .into(),
    )]))
    .unwrap();
    let selected = toolbox.get("route").unwrap().clone();
    assert_eq!(toolbox.iter().count(), 1);
    let question = Question::from(selected);
    assert_eq!(
        serde_json::to_value(question).unwrap()["choices"],
        json!(["billing", "shipping"])
    );
    let entries = toolbox
        .iter()
        .map(|(name, question)| (name.to_owned(), question.clone().try_into().unwrap()))
        .collect();
    let annotate = questions::Annotate::new(entries)
        .unwrap()
        .evidence("/body".parse().unwrap());
    let rank: thinkthen::SimpleQuestion = questions::Rank::new("How?", ["low", "high"])
        .unwrap()
        .into();
    let toolbox =
        questions::Toolbox::new(BTreeMap::from([("rank".to_owned(), rank.clone())])).unwrap();
    assert!(toolbox.get("rank").is_some());
    assert!(questions::AnnotatableQuestion::try_from(rank).is_err());
    let error = Request::from_json(r#"{"question":{"function":"annotate","questions":{"rank":{"function":"rank","question":"How?","levels":["low","high"]}}}}"#).unwrap_err();
    assert_eq!(error.code(), thinkthen::Code::UsageQuestion);
    assert_eq!(annotate.questions().iter().count(), 1);
    assert_eq!(annotate.evidence_pointer().to_string(), "/body");
    let request = Request::new(annotate)
        .named("summary")
        .unwrap()
        .with_records([json!({"body":"refund"})]);
    assert_eq!(request.name(), Some("summary"));
    assert!(Request::from_json(r#"{"question":{"function":"annotate","questions":{"q":{"function":"decide","question":"A?"},"q":{"function":"decide","question":"B?"}}}}"#).is_err());
}

#[test]
fn typed_builders_map_thresholds_evidence_rules_and_ranking_limits() {
    use std::num::NonZeroUsize;
    use std::time::Duration;
    use thinkthen::{Cut, Retry};
    let cut = Cut::new(0.7123456789012345).unwrap();
    let band = Threshold::band(0.2, 0.8).unwrap();
    let rows: Vec<(Question, serde_json::Value)> = vec![
        (
            questions::Decide::new("Ready?")
                .unwrap()
                .threshold(band)
                .evidence("/body".parse().unwrap())
                .into(),
            json!({"low":0.2,"high":0.8}),
        ),
        (
            questions::Filter::new("Keep?")
                .unwrap()
                .threshold(cut.into())
                .into(),
            json!(cut.get()),
        ),
        (
            questions::Choose::new("Which?", ["a", "b"])
                .unwrap()
                .threshold(band)
                .into(),
            json!({"low":0.2,"high":0.8}),
        ),
        (
            questions::Tag::new("Which?", ["a"])
                .unwrap()
                .threshold(cut)
                .into(),
            json!(cut.get()),
        ),
        (
            questions::Find::new("Where?")
                .unwrap()
                .threshold(cut)
                .into(),
            json!(cut.get()),
        ),
        (
            questions::Recognize::new(["person"])
                .unwrap()
                .threshold(cut)
                .into(),
            json!(cut.get()),
        ),
        (
            questions::Relate::new([questions::RelationRule::new("knows", "person", "person")
                .unwrap()
                .reads("knows well")
                .unwrap()
                .either(true)])
            .unwrap()
            .threshold(cut)
            .into(),
            json!(cut.get()),
        ),
    ];
    for (question, threshold) in rows {
        let encoded = serde_json::to_value(&question).unwrap();
        assert_eq!(encoded["threshold"], threshold);
        if encoded["function"] == "decide" {
            assert_eq!(encoded["evidence"], "/body");
        }
        if encoded["function"] == "relate" {
            assert_eq!(
                encoded["rules"][0],
                json!({"relation":"knows","source":"person","target":"person","reads":"knows well","either":true})
            );
        }
        assert_eq!(
            serde_json::to_value(Question::from_json(&encoded.to_string()).unwrap()).unwrap(),
            encoded
        );
    }
    let rank = questions::Rank::new("How?", ["low", "high"])
        .unwrap()
        .top(NonZeroUsize::new(2).unwrap())
        .unwrap();
    assert_eq!(
        serde_json::to_value(Question::from(rank)).unwrap()["top"],
        2
    );
    let score = questions::Score::new("How?", ["low", "high"]).unwrap();
    assert!(
        serde_json::to_value(Question::from(score))
            .unwrap()
            .get("threshold")
            .is_none()
    );
    assert!(Retry::new(0, Duration::ZERO, Duration::ZERO).is_err());
    assert!(Retry::new(0, Duration::from_nanos(1), Duration::from_secs(1)).is_err());
}

#[test]
fn annotation_preserves_inherited_evidence_and_explicit_root_selection() {
    let schema = serde_json::to_value(schemars::schema_for!(Request)).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for child in [
        json!({"function":"decide","question":"Ready?"}),
        json!({"function":"filter","question":"Keep?"}),
        json!({"function":"choose","question":"Which?","choices":["a","b"]}),
        json!({"function":"tag","question":"Which?","labels":["a"]}),
        json!({"function":"score","question":"How?","levels":["low","high"]}),
        json!({"function":"find","question":"Where?"}),
        json!({"function":"recognize","kinds":["person"]}),
        json!({"function":"relate","rules":[{"relation":"knows","source":"person","target":"person"}]}),
    ] {
        for evidence in [
            None,
            Some(json!("")),
            Some(json!("/title")),
            Some(json!(null)),
        ] {
            let mut child = child.clone();
            if let Some(value) = &evidence {
                child["evidence"] = value.clone();
            }
            let input = json!({"question":{"function":"annotate","evidence":"/body","questions":{"q":child}}});
            if evidence == Some(json!(null)) {
                assert!(!validator.is_valid(&input));
                assert!(Request::from_json(&input.to_string()).is_err());
                continue;
            }
            validator.validate(&input).unwrap();
            let admitted = Request::from_json(&input.to_string()).unwrap();
            let Question::Annotate(annotation) = admitted.question() else {
                panic!("expected annotation")
            };
            let selected = annotation.questions()["q"].evidence_override();
            assert_eq!(
                selected.map(ToString::to_string),
                evidence
                    .as_ref()
                    .map(|value| value.as_str().unwrap().to_owned())
            );
            assert_eq!(
                selected
                    .unwrap_or(annotation.evidence_pointer())
                    .to_string(),
                evidence
                    .as_ref()
                    .map_or("/body", |value| value.as_str().unwrap())
            );
            let encoded = serde_json::to_value(&admitted).unwrap();
            assert_eq!(
                encoded["question"]["questions"]["q"].get("evidence"),
                evidence.as_ref()
            );
            validator.validate(&encoded).unwrap();
            assert_eq!(
                serde_json::to_value(Request::from_json(&encoded.to_string()).unwrap()).unwrap(),
                encoded
            );
        }
    }
}

#[test]
fn admitted_labels_keep_original_bytes_and_execution_identity() {
    use thinkthen::{OffsetUnit, Record};
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    for (function, field) in [
        ("choose", "choices"),
        ("tag", "labels"),
        ("score", "levels"),
        ("rank", "levels"),
        ("recognize", "kinds"),
    ] {
        let mut identities = Vec::new();
        for label in ["é", "e\u{301}"] {
            // NFC does not fold case, trim text or merge compatibility characters.
            let labels = json!([label, "e", "É", " é", "①", "1"]);
            let mut question = json!({"function":function, field:labels});
            if function != "recognize" {
                question["question"] = json!("Which fits?");
            }
            let request = Request::from_json(&json!({"question":question}).to_string()).unwrap();
            assert_eq!(
                serde_json::to_value(request.question()).unwrap()[field],
                labels
            );
            let call = engine
                .open_request(&request, OffsetUnit::CodePoints)
                .unwrap();
            let sender = call.sender();
            sender.send(Record::text("Alice")).unwrap();
            sender.close();
            let rows = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
            assert_eq!(rows.len(), 1);
            identities.push(rows[0].source().question_identity().to_owned());
        }
        assert_ne!(identities[0], identities[1], "{function}");
    }
    let rules = json!([{"relation":"e\u{301}","source":"é","target":"é"},{"relation":"É","source":"é","target":"person"}]);
    let question =
        Question::from_json(&json!({"function":"relate","rules":rules}).to_string()).unwrap();
    let encoded = serde_json::to_value(question).unwrap();
    for (actual, expected) in encoded["rules"]
        .as_array()
        .unwrap()
        .iter()
        .zip(rules.as_array().unwrap())
    {
        for field in ["relation", "source", "target"] {
            assert_eq!(actual[field], expected[field]);
        }
    }
}

#[test]
fn retry_counts_above_the_engine_budget_are_usage_errors() {
    use std::time::Duration;
    use thinkthen::Retry;
    for count in [11_u32, u32::MAX] {
        let settings = format!(r#"{{"backend":"stub","retry":{{"max_retries":{count}}}}}"#);
        let error = Settings::from_json(&settings).unwrap_err();
        assert_eq!(error.code().as_str(), "usage.settings");
        assert!(Retry::new(count, Duration::ZERO, Duration::from_secs(1)).is_err());
    }
    for count in 0..=10 {
        let settings = format!(r#"{{"backend":"stub","retry":{{"max_retries":{count}}}}}"#);
        Settings::from_json(&settings).unwrap();
        Retry::new(count, Duration::ZERO, Duration::from_secs(1)).unwrap();
    }
}

#[cfg(feature = "native")]
#[test]
fn sql_retry_settings_cannot_raise_the_default_budget() {
    for count in 0..=10 {
        let text = format!(r#"{{"backend":"stub","retry":{{"max_retries":{count}}}}}"#);
        let result = thinkthen::native::sql_settings(&text);
        if count > 3 {
            assert_eq!(result.unwrap_err().code().as_str(), "usage.settings");
        } else {
            result.unwrap();
        }
    }
}

#[cfg(feature = "native")]
#[test]
fn postgres_input_nul_is_not_an_output_representation_failure() {
    use thinkthen::{AnyRow, Decide, Record, RecordId};
    let server = loopback::Loopback::noul(0.9).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(loopback::KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Decide::new("Ready?").unwrap();
    for text in [
        r#""before\u0000after""#,
        r#"{"id":7,"body":"before\u0000after","key\u0000name":"value"}"#,
    ] {
        let record = thinkthen::native::sql_postgres_record(text);
        let row = engine.run(&question, record).unwrap();
        let row = thinkthen::native::sql_postgres_row(AnyRow::Decide(row)).unwrap();
        let value = serde_json::to_value(row).unwrap();
        assert_eq!(value["status"], "answered", "{text}");
        assert_eq!(value["answer"], true);
    }
    assert_eq!(server.count(), 2);
    let record =
        thinkthen::native::sql_postgres_record(r#"{"id":"bad\u0000id","body":"evidence"}"#);
    let row = engine.run(&question, record).unwrap();
    let row = thinkthen::native::sql_postgres_row(AnyRow::Decide(row)).unwrap();
    let value = serde_json::to_value(row).unwrap();
    assert_eq!(value["status"], "failed");
    assert_eq!(value["error"]["code"], "record.json");
    assert!(value.get("id").is_none());
    assert_eq!(server.count(), 2, "unrepresentable ID sent a paid request");
    // Output identifiers still obey PostgreSQL's text/jsonb representation limit.
    let record = Record::text("evidence").with_id(RecordId::String("bad\0id".into()));
    let row = engine.run(&question, record).unwrap();
    let row = thinkthen::native::sql_postgres_row(AnyRow::Decide(row)).unwrap();
    let value = serde_json::to_value(row).unwrap();
    assert_eq!(value["status"], "failed");
    assert_eq!(value["error"]["code"], "record.json");
    assert!(value.get("id").is_none());
    assert_eq!(server.count(), 3);
}
