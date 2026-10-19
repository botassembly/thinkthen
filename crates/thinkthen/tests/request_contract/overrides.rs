//! Public request overlays retain native precedence, descriptions and list admission.
use super::*;

fn preview(engine: &Engine, function: &str, question: &str, options: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request = Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":"{function}","question":{{"kind":"definition","value":{question}}},"input":{{"kind":"text","text":"evidence"}},"options":{options}}}}}"#))
        ?.admit()?;
    let plan = engine
        .plan_request(&request, RequestEnvironment::default())
        ?;
    Ok(String::from_utf8(plan.first_body().ok_or("missing preview body")?.to_vec())?)
}

#[test]
fn explicit_meanings_replace_saved_content_and_absence_retains_it() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let question = r#"{"decide":"Fits?","true":"saved_yes_marker","false":"saved_no_marker"}"#;
    for (options, present, absent) in [
        (r#"{}"#, vec!["saved_yes_marker", "saved_no_marker"], vec![]),
        (
            r#"{"yes":{"definition":"replacement_yes_marker"}}"#,
            vec!["replacement_yes_marker", "saved_no_marker"],
            vec!["saved_yes_marker"],
        ),
        (
            r#"{"no":["replacement_no_marker"]}"#,
            vec!["saved_yes_marker", "replacement_no_marker"],
            vec!["saved_no_marker"],
        ),
        (
            r#"{"yes":null,"no":null}"#,
            vec![],
            vec!["saved_yes_marker", "saved_no_marker"],
        ),
    ] {
        let body = preview(&engine, "decide", question, options).unwrap();
        for marker in present {
            assert!(body.contains(marker), "{body}");
        }
        for marker in absent {
            assert!(!body.contains(marker), "{body}");
        }
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn whole_lists_replace_saved_lists_preserving_order_and_description_content() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for (function, question, key) in [
        (
            "choose",
            r#"{"choose":"Which?","options":["saved_a_marker","saved_b_marker"]}"#,
            "labels",
        ),
        (
            "tag",
            r#"{"tag":"Which?","labels":["saved_a_marker","saved_b_marker"]}"#,
            "labels",
        ),
        (
            "score",
            r#"{"score":"Which?","levels":["saved_a_marker","saved_b_marker"]}"#,
            "levels",
        ),
    ] {
        let options = format!(
            r#"{{"{key}":[{{"name":"replacement_z_marker","description":{{"detail":"description_z_marker"}}}},{{"name":"replacement_a_marker","description":["description_a_marker"]}}]}}"#
        );
        let body = preview(&engine, function, question, &options).unwrap();
        assert!(!body.contains("saved_a_marker"));
        assert!(!body.contains("saved_b_marker"));
        let authored_key = if function == "choose" { "options" } else { key };
        let authored = format!(
            r#"{{"{function}":"Which?","{authored_key}":{{"replacement_z_marker":{{"detail":"description_z_marker"}},"replacement_a_marker":["description_a_marker"]}}}}"#
        );
        let LoadedQuestion::Question(expected) = Question::from_json(&authored).unwrap() else {
            panic!("ordinary native question")
        };
        let expected = engine.plan(&expected, ["evidence"]).unwrap();
        assert_eq!(body.as_bytes(), expected.first_body().unwrap());
        if function != "score" {
            assert!(
                body.find("replacement_z_marker").unwrap()
                    < body.find("replacement_a_marker").unwrap()
            );
        }
        assert!(
            body.find("description_z_marker").unwrap() < body.find("description_a_marker").unwrap()
        );
        assert!(body.contains("description_z_marker"));
        assert!(body.contains("description_a_marker"));
        for list in [
            "[]",
            r#"[{"name":"duplicate"},{"name":"duplicate"}]"#,
            r#"[{"name":" "},{"name":"valid"}]"#,
        ] {
            let request = Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":"{function}","question":{{"kind":"definition","value":{question}}},"input":{{"kind":"text","text":"evidence"}},"options":{{"{key}":{list}}}}}}}"#)).unwrap();
            assert!(request.admit().is_err());
        }
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn wording_controls_keep_wire_presence_and_refuse_wrong_verbs_before_file_reads() {
    let options: RequestOptions = serde_json::from_str(
        r#"{"yes":null,"labels":[{"name":"z"},{"name":"a","description":null}]}"#,
    )
    .unwrap();
    let serialized = serde_json::to_value(options).unwrap();
    assert!(serialized.get("yes").unwrap().is_null());
    assert!(serialized.get("no").is_none());
    assert!(serialized["labels"][0].get("description").is_none());
    assert!(
        serialized["labels"][1]
            .get("description")
            .unwrap()
            .is_null()
    );
    assert!(serde_json::from_str::<RequestOptions>(r#"{"labels":null}"#).is_err());
    for (function, options) in [
        ("score", r#"{"yes":"meaning"}"#),
        ("decide", r#"{"labels":[]}"#),
        ("choose", r#"{"levels":[]}"#),
    ] {
        let request = Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":"{function}","question":{{"kind":"file","path":"absent-wording-file"}},"input":{{"kind":"text","text":"evidence"}},"options":{options}}}}}"#)).unwrap();
        assert_eq!(request.admit().unwrap_err().kind(), ErrorKind::Usage);
    }
}

#[test]
fn relation_threshold_overrides_share_native_preview_and_execution() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let saved = r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]},"threshold":0.5}"#;
    let input = || RequestInput::Records {
        items: ["Ada", "Bea"]
            .into_iter()
            .map(|name| RequestItem {
                original: Some(RequestOriginal::Json {
                    value: RawRecord::json(&json!({"name":name,"kind":"person"}).to_string())
                        .unwrap(),
                }),
                images: Vec::new(),
                context: None,
                options: None,
                seed_spans: None,
                examples: None,
            })
            .collect(),
    };
    for threshold in [None, Some(0.95)] {
        let request = Request::for_function(
            RequestFunction::Relate,
            RequestArguments {
                question: RequestQuestion::Definition {
                    value: Relate::from_records_json(saved).unwrap().into(),
                },
                input: input(),
                options: RequestOptions {
                    threshold: threshold.map(RequestThreshold::Cut),
                    ..Default::default()
                },
            },
        )
        .admit()
        .unwrap();
        let plan = engine
            .plan_request(&request, RequestEnvironment::default())
            .unwrap();
        let authored = saved.replace("0.5", &threshold.unwrap_or(0.5).to_string());
        let equivalent = Request::for_function(
            RequestFunction::Relate,
            RequestArguments {
                question: RequestQuestion::Definition {
                    value: Relate::from_records_json(&authored).unwrap().into(),
                },
                input: input(),
                options: RequestOptions::default(),
            },
        )
        .admit()
        .unwrap();
        let expected = engine
            .plan_request(&equivalent, RequestEnvironment::default())
            .unwrap();
        assert_eq!(plan.first_body(), expected.first_body());
        assert_eq!(listener.count(), if threshold.is_some() { 1 } else { 0 });
        let RequestOutcome::Complete(call) = engine
            .execute_request(&request, RequestEnvironment::default())
            .unwrap()
        else {
            panic!("complete relate")
        };
        let RequestValue::Related(row) = call.value() else {
            panic!("related")
        };
        assert_eq!(
            serde_json::to_value(row.result()).unwrap()["question"]["threshold"],
            threshold.unwrap_or(0.5)
        );
        assert_eq!(row.result().value().is_empty(), threshold.is_some());
        assert_eq!(call.facts().requests_sent(), 1);
    }
    assert_eq!(listener.count(), 2);
}
