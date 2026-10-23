#![cfg(test)]
//! Annotation through the public Rust API.

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Annotate, Backend, Engine, Record, Toolbox};

#[test]
fn adapter_validation_fails_only_its_named_entry() {
    let toolbox = Toolbox::from_bytes(
        json!({"questions":{
            "a":{"decide":"Yes?"},
            "b":{"choose":"Which?","options":(0..256).map(|i| i.to_string()).collect::<Vec<_>>()}
        }})
        .to_string()
        .as_bytes(),
    )
    .unwrap();
    let server = Loopback::start(|_| {
        loopback::Reply::json(
            200,
            &json!({"answers":[{"type":"predicate","probability":0.9}]}),
        )
    })
    .unwrap();
    let backend = Backend::resolve_adapter(
        thinkthen::Adapter::OpenAi,
        Some(server.base()),
        None,
        |name| (name == "OPENAI_API_KEY").then(|| KEY.to_owned()),
    )
    .unwrap();
    Engine::new(backend)
        .annotate(&Annotate::new(&toolbox), [Record::text("record")], |row| {
            let row = serde_json::to_value(row).unwrap();
            assert_eq!(row["answer"]["a"]["answer"], true);
            assert_eq!(row["answer"]["b"]["status"], "failed");
            assert_eq!(row["answer"]["b"]["error"]["code"], "backend.limit");
        })
        .unwrap();
    assert_eq!(server.count(), 1);
}

#[test]
fn named_questions_pack_across_records_and_keep_independent_outcomes() {
    let toolbox = Toolbox::from_bytes(br#"{"questions":{"a":{"decide":"Yes?"},"b":{"tag":"Labels?","labels":["x"]},"c":{"score":"How?","levels":["low","high"]}}}"#).unwrap();
    let server = Loopback::answering(|q| {
        let instructions = q["instructions"].as_str().unwrap();
        let record: usize = instructions.split("record ").nth(1).unwrap()
            .split('"').next().unwrap().parse().unwrap();
        let high = record.is_multiple_of(2);
        match q["type"].as_str() {
            Some("score") => json!({"type":"score","probabilities":{"0":if high {0.2} else {0.8},"1":if high {0.8} else {0.2}}}),
            _ => json!({"type":"noul","noul":if high {0.9} else {0.1}}),
        }
    })
    .unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let mut rows = Vec::new();
    Engine::new(backend)
        .annotate(
            &Annotate::new(&toolbox),
            (0..100).map(|i| Record::text(format!("record {i}"))),
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(server.count(), 2);
    assert_eq!(rows.len(), 100);
    for (i, row) in rows.iter().enumerate() {
        let high = i.is_multiple_of(2);
        assert_eq!(row["id"], i + 1);
        assert_eq!(row["answer"]["a"]["answer"], high);
        assert_eq!(row["answer"]["a"]["p"], if high { 0.9 } else { 0.1 });
        assert_eq!(
            row["answer"]["b"]["answer"],
            if high { json!(["x"]) } else { json!([]) }
        );
        assert!(row["answer"]["b"].get("p").is_none());
        assert_eq!(row["answer"]["c"]["answer"], if high { 0.8 } else { 0.2 });
        assert!(row["answer"]["c"].get("p").is_none());
    }
}

#[test]
fn malformed_question_answer_fails_only_its_named_entry() {
    let toolbox =
        Toolbox::from_bytes(br#"{"questions":{"a":{"decide":"Good?"},"b":{"decide":"Bad?"}}}"#)
            .unwrap();
    let server = Loopback::answering(|q| {
        if q["instructions"].as_str().unwrap().contains("Bad?") {
            json!({"type":"noul","noul":2})
        } else {
            json!({"type":"noul","noul":0.9})
        }
    })
    .unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    Engine::new(backend)
        .annotate(&Annotate::new(&toolbox), [Record::text("record")], |row| {
            let row = serde_json::to_value(row).unwrap();
            assert_eq!(row["answer"]["a"]["answer"], true);
            assert_eq!(row["answer"]["b"]["status"], "failed");
        })
        .unwrap();
}

#[test]
fn evidence_pointers_and_missing_fields_belong_to_each_entry() {
    let toolbox = Toolbox::from_bytes(br#"{"questions":{"a":{"decide":"Yes?","on":"/body"},"b":{"decide":"No?","on":"/other"},"missing":{"decide":"Missing?","on":"/absent"}}}"#).unwrap();
    let server = Loopback::answering(|q| json!({"type":"noul","noul":if q["instructions"].as_str().unwrap().contains("yes evidence") {0.9} else {0.1}})).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    Engine::new(backend)
        .annotate(
            &Annotate::new(&toolbox),
            [Record::json(
                r#"{"id":"r","body":"yes evidence","other":"no evidence"}"#,
                &Default::default(),
            )],
            |row| {
                let row = serde_json::to_value(row).unwrap();
                assert_eq!(row["id"], "r");
                assert_eq!(row["answer"]["a"]["answer"], true);
                assert_eq!(row["answer"]["b"]["answer"], false);
                assert_eq!(row["answer"]["missing"]["error"]["code"], "usage.missing");
            },
        )
        .unwrap();
    assert_eq!(server.count(), 1);
}

#[test]
fn failed_request_keeps_other_requests_answers_intact() {
    use loopback::Reply;
    use thinkthen::RetryPolicy;
    let toolbox = Toolbox::from_bytes(
        br#"{"questions":{"a":{"decide":"A?"},"b":{"decide":"B?"},"c":{"decide":"C?"}}}"#,
    )
    .unwrap();
    let server = Loopback::start(|request| {
        let body = request.json().unwrap();
        let questions = body["questions"].as_object().unwrap();
        if questions
            .values()
            .any(|q| q["instructions"].as_str().unwrap().contains("record 0\""))
        {
            return Reply::json(400, &json!({}));
        }
        let answers: serde_json::Map<String, serde_json::Value> = questions
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Reply::json(200, &json!({"answers":answers}))
    })
    .unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let mut rows = Vec::new();
    Engine::new(backend)
        .retry_policy(RetryPolicy {
            max_retries: 0,
            ..Default::default()
        })
        .annotate(
            &Annotate::new(&toolbox),
            (0..100).map(|i| Record::text(format!("record {i}"))),
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(server.count(), 2);
    for (i, row) in rows.iter().enumerate() {
        for name in ["a", "b", "c"] {
            assert_eq!(
                row["answer"][name]["status"],
                if i < 50 { "failed" } else { "answered" }
            );
        }
    }
}

#[test]
fn local_entry_failures_skip_requests_and_root_pointer_keeps_text() {
    for (pointer, expected, requests) in [("/missing", "failed", 0), ("", "answered", 1)] {
        let toolbox = Toolbox::from_bytes(
            json!({"questions":{"q":{"decide":"Yes?","on":pointer}}})
                .to_string()
                .as_bytes(),
        )
        .unwrap();
        let server = Loopback::noul(0.9).unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        Engine::new(backend)
            .annotate(
                &Annotate::new(&toolbox),
                [Record::json(r#""plain text""#, &Default::default())],
                |row| {
                    let row = serde_json::to_value(row).unwrap();
                    assert_eq!(row["answer"]["q"]["status"], expected);
                },
            )
            .unwrap();
        assert_eq!(server.count(), requests);
    }
}

#[test]
fn dependent_questions_keep_their_evidence_histories_and_failures_separate() {
    for (bad_boundary, bad_kind) in [(false, false), (false, true), (true, false)] {
        let toolbox = Toolbox::from_bytes(
            br#"{"questions":{
            "a_find":{"find":"Which line?","on":"/lines","threshold":"0.8"},
            "b_name":{"recognize":["person"],"on":"/left"},
            "c_name":{"recognize":["place"],"on":"/right"},
            "d_decide":{"decide":"Ready?","on":"/decision"},
            "e_empty":{"recognize":[],"on":"/empty"},
            "f_missing":{"recognize":["person"],"on":"/absent"}
        }}"#,
        )
        .unwrap();
        let server = Loopback::answering(move |q| {
            let instructions = q["instructions"].as_str().unwrap();
            let criteria = &q["criteria"];
            if criteria.get("BEGIN").is_some() {
                if bad_boundary && instructions.contains("[[New]]") {
                    return json!({"type":"choice","probabilities":{}});
                }
                let tag = if instructions.contains("[[Ada]]") {"SINGLE"}
                    else if instructions.contains("[[New]]") {"BEGIN"}
                    else if instructions.contains("[[York]]") {"END"}
                    else {panic!("wrong recognition evidence: {instructions}")};
                let probabilities: serde_json::Map<_, _> = ["BEGIN","INSIDE","END","SINGLE","OUT"].into_iter().map(|label|(label.to_owned(),json!(if label==tag {1.0} else {0.0}))).collect();
                json!({"type":"choice","probabilities":probabilities})
            } else if criteria.get("person").is_some() {
                assert!(instructions.contains("[[Ada]]"));
                json!({"type":"choice","probabilities":{"person":0.9,"none of these":0.1}})
            } else if criteria.get("place").is_some() {
                assert!(instructions.contains("[[New York]]"));
                json!({"type":"choice","probabilities":{"place":if bad_kind {2.0}else{0.9},"none of these":0.1}})
            } else if criteria.get("u1").is_some() {
                assert_eq!(criteria["u1"],"first"); assert_eq!(criteria["u2"],"second");
                json!({"type":"choice","probabilities":{"u1":0.6,"u2":0.3,"none":0.1}})
            } else {
                assert!(instructions.contains("ready"));
                json!({"type":"noul","noul":0.9})
            }
        }).unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        let mut rows = Vec::new();
        Engine::new(backend).annotate(&Annotate::new(&toolbox),(0..2).map(|id|Record::json(&json!({"id":id,"lines":"first\nsecond","left":"Ada","right":"New York","decision":"ready","empty":""}).to_string(),&Default::default())),|row|rows.push(serde_json::to_value(row).unwrap())).unwrap();
        assert_eq!(rows.len(), 2);
        for (id, row) in rows.iter().enumerate() {
            assert_eq!(row["id"], id);
            assert_eq!(row["status"], "answered");
            assert!(row.get("p").is_none());
            let entries = &row["answer"];
            assert_eq!(entries["a_find"]["status"], "unsure");
            assert_eq!(
                entries["a_find"]["detail"],
                json!({"u1":0.6,"u2":0.3,"none":0.1})
            );
            assert_eq!(entries["b_name"]["answer"][0]["text"], "Ada");
            assert_eq!(entries["b_name"]["answer"][0]["kind"], "person");
            assert!(entries["b_name"].get("p").is_none());
            if bad_boundary || bad_kind {
                assert_eq!(entries["c_name"]["error"]["code"], "backend.reply");
            } else {
                assert_eq!(entries["c_name"]["answer"][0]["text"], "New York");
                assert_eq!(entries["c_name"]["answer"][0]["start"], 0);
                assert_eq!(entries["c_name"]["answer"][0]["end"], 8);
                assert_eq!(entries["c_name"]["answer"][0]["kind"], "place");
            }
            assert_eq!(entries["d_decide"]["answer"], true);
            assert_eq!(entries["e_empty"]["answer"], json!([]));
            assert_eq!(entries["f_missing"]["error"]["code"], "usage.missing");
        }
        // Name evidence separates the requests and their dependent rounds.
        assert_eq!(server.count(), if bad_boundary { 5 } else { 8 });
    }
}

#[test]
fn pointers_preserve_selected_json_types_and_never_parse_text() {
    for (record, pointer, expected) in [
        (Record::text(r#"{"body":"x"}"#), "/body", None),
        (
            Record::json(r#""{\"body\":\"x\"}""#, &Default::default()),
            "/body",
            None,
        ),
        (
            Record::json(
                r#"{"selected":"{\"body\":\"x\"}"}"#,
                &"/selected".parse().unwrap(),
            ),
            "/body",
            None,
        ),
        (
            Record::json(
                r#"{"selected":{"body":"x"}}"#,
                &"/selected".parse().unwrap(),
            ),
            "/body",
            Some("x"),
        ),
        (
            Record::json(r#"[{"body":"x"}]"#, &Default::default()),
            "/0/body",
            Some("x"),
        ),
        (
            Record::json(r#"{"a/b":{"~key":"x"}}"#, &Default::default()),
            "/a~1b/~0key",
            Some("x"),
        ),
        (Record::json("null", &Default::default()), "/body", None),
    ] {
        let toolbox = Toolbox::from_bytes(
            json!({"questions":{"q":{"decide":"Yes?","on":pointer}}})
                .to_string()
                .as_bytes(),
        )
        .unwrap();
        let server = Loopback::answering(|q| {
            assert_eq!(q["instructions"], "The text is \"x\". Yes?");
            json!({"type":"noul","noul":0.9})
        })
        .unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        let mut rows = Vec::new();
        Engine::new(backend)
            .annotate(&Annotate::new(&toolbox), [record], |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows.len(), 1);
        let entry = &rows[0]["answer"]["q"];
        if expected.is_some() {
            assert_eq!(entry["answer"], true);
        } else {
            assert_eq!(entry["error"]["code"], "usage.missing");
        }
        assert_eq!(server.count(), usize::from(expected.is_some()));
    }
}

#[test]
fn missing_or_null_packed_answer_fails_only_its_entry() {
    for null in [false, true] {
        let toolbox = Toolbox::from_bytes(
            br#"{"questions":{"a":{"decide":"A?"},"b":{"decide":"B?"},"c":{"decide":"C?"}}}"#,
        )
        .unwrap();
        let server = Loopback::start(move |_| {
            let mut answers =
                json!({"q1":{"type":"noul","noul":0.9},"q3":{"type":"noul","noul":0.1}});
            if null {
                answers["q2"] = serde_json::Value::Null;
            }
            loopback::Reply::json(200, &json!({"answers":answers}))
        })
        .unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        let mut rows = Vec::new();
        Engine::new(backend)
            .annotate(&Annotate::new(&toolbox), [Record::text("record")], |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows.len(), 1);
        let entries = &rows[0]["answer"];
        assert_eq!(entries["a"]["answer"], true);
        assert_eq!(entries["b"]["error"]["code"], "backend.reply");
        assert_eq!(entries["c"]["answer"], false);
        assert_eq!(server.count(), 1);
    }
}

#[test]
fn invalid_records_fail_admission_before_any_function_prepares_them() {
    use thinkthen::{AdmittedRecord, Decide, Function, Recognize};

    fn check(engine: &Engine, question: &impl Function) {
        let mut rows = Vec::new();
        engine
            .call(
                question,
                [Record::json("not JSON", &Default::default())],
                |row| rows.push(serde_json::to_value(row).unwrap()),
            )
            .unwrap_err();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["status"], "failed");
        assert_eq!(rows[0]["error"]["code"], "usage.record");
    }
    assert_eq!(
        AdmittedRecord::new(Record::json("not JSON", &Default::default()))
            .unwrap_err()
            .kind()
            .code(),
        "usage.record"
    );
    let server = Loopback::start(|_| panic!("an invalid record must not send")).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let engine = Engine::new(backend);
    check(&engine, &Decide::new("Yes?").unwrap());
    check(&engine, &Recognize::new(["person"]).unwrap());
    let toolbox = Toolbox::from_bytes(br#"{"questions":{"q":{"decide":"Yes?"}}}"#).unwrap();
    check(&engine, &Annotate::new(&toolbox));
    assert_eq!(server.count(), 0);
}
