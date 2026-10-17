use super::*;
use thinkthen::{
    LoadedQuestion, QuestionInput, QuestionKind, RequestEnvironment, RequestFeed, RequestInput,
    ResolvedThreshold,
};

#[test]
fn complete_getters_keep_authored_content_order_null_and_raw_choice_before_cut() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"second":0.7,"first":0.3},"confidence":0.8}}}"#)).unwrap();
    let engine = engine(&listener);
    let LoadedQuestion::Question(question) = Question::from_json(
        r#"{"choose":{"z":"Authored.","a":["original",null]},"options":{"second":{"z":null,"a":"kept"},"first":null},"threshold":0.9,"model":"fixed","profile":"authored","batch":"max","on":[""]}"#).unwrap() else { panic!("choose") };
    let call = crate::api::request(
        &engine,
        thinkthen::RequestFunction::Choose,
        question.clone().into(),
        RequestInput::Feed {
            name: "originals".to_owned(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        RequestEnvironment {
            controls: CallOptions::new(),
            feed: Some(
                RequestFeed::from_records(
                    "originals",
                    std::iter::once(Ok(crate::api::record(QuestionInput::Text(
                        "Original evidence.".to_owned(),
                    )))),
                )
                .eager(),
            ),
        },
    )
    .unwrap();
    let result = crate::api::choices(&call).unwrap()[0].result();
    assert_eq!(result.value(), None);
    assert_eq!(result.raw_pick(), Some("second"));
    assert_eq!(result.threshold(), Some(ResolvedThreshold::Cut(0.9)));
    assert_eq!(result.question().kind(), QuestionKind::Choose);
    assert_eq!(result.question().text().text(), None);
    assert_eq!(
        result.question().text().to_json().unwrap(),
        r#"{"z":"Authored.","a":["original",null]}"#
    );
    let reading = result.question();
    let options = reading.options().collect::<Vec<_>>();
    assert_eq!(
        options.iter().map(|o| o.name()).collect::<Vec<_>>(),
        ["second", "first"]
    );
    assert_eq!(
        options[0].description().unwrap().to_json().unwrap(),
        r#"{"z":null,"a":"kept"}"#
    );
    let document = serde_json::to_value(&call).unwrap();
    assert_eq!(
        document["value"][0]["question"]["options"],
        json!(["second", "first"])
    );
    assert_eq!(
        document["value"][0]["question"]["label_details"],
        json!([
            {"name":"second","description":{"z":null,"a":"kept"}}, {"name":"first"}
        ])
    );
    assert_eq!(document["value"][0]["question"]["model"], "fixed");
    assert_eq!(document["value"][0]["question"]["profile"], "authored");
    assert_eq!(document["value"][0]["question"]["batch"], "max");
    assert_eq!(document["value"][0]["question"]["on"], json!([""]));
    schema::check(&document, "completeChoose");
    // In the ordinary choose grammar null descriptions mean absent.
    assert!(options[1].description().is_none());
    assert!(!format!("{:?}", result.question()).contains("Authored"));
    assert_eq!(listener.count(), 1);
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        body,
        json!({"model":"fixed","state":"Original evidence.","questions":{"q1":{"type":"choice","instructions":{"z":"Authored.","a":["original",null]},"criteria":{"second":{"z":null,"a":"kept"},"first":null}}}})
    );
}

#[test]
fn named_member_getters_retain_admitted_readings_failure_ids_and_actual_sources() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.5},"q2":{"type":"choice","probabilities":{"wrong":1}}},"usage":{"input_tokens":887}}"#)).unwrap();
    let set = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"uncertain":{"decide":"Sure?","true":null,"false":{"z":"No.","a":null},"threshold":"0.2:0.8"},"failed":{"decide":"Failed?","threshold":0.9}}}"#).unwrap();
    let engine = engine(&listener);
    let call = crate::api::annotate(
        &engine,
        &set,
        crate::api::plain_inputs(["Original."]),
        CallOptions::new(),
    )
    .unwrap();
    let members = call.value()[0].result().members().collect::<Vec<_>>();
    assert_eq!(
        members[0].threshold(),
        Some(ResolvedThreshold::Band {
            low: 0.2,
            high: 0.8
        })
    );
    assert!(members[0].question().yes().unwrap().is_null());
    assert_eq!(
        members[0].question().no().unwrap().to_json().unwrap(),
        r#"{"z":"No.","a":null}"#
    );
    assert_eq!(members[1].question().text().text(), Some("Failed?"));
    assert_eq!(members[1].threshold(), Some(ResolvedThreshold::Cut(0.9)));
    assert!(members[1].answer_id().is_none());
    let [Observation::Failed { failure_id }] = members[1].observations() else {
        panic!("actual failure")
    };
    assert_eq!(Some(failure_id), members[1].failure_id());
    assert_eq!(members[0].question_sources()[0].origin(), Origin::Live);
    assert_eq!(members[0].question_sources()[0].batch_size(), Some(2));
    assert_eq!(
        members[0].reported_usage().unwrap().input_tokens(),
        Some(444)
    );
    assert_eq!(
        members[1].reported_usage().unwrap().input_tokens(),
        Some(443)
    );
    assert_eq!(members[0].reported_usage().unwrap().output_tokens(), None);
    assert_eq!(listener.count(), 1);
}

#[test]
fn reported_model_remains_truthful_but_debug_cannot_echo_a_private_value() {
    let listener = crate::api::answering(
        r#"{"model":"complete-private","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
    );
    let engine = engine(&listener);
    let question = Question::decide("Question?").unwrap().cut();
    let result = crate::api::decide(&engine, &question, "Evidence.", CallOptions::new()).unwrap();
    let value = crate::api::decision(&result).unwrap();
    assert_eq!(
        crate::api::decision(&result)
            .unwrap()
            .identity()
            .answered_by(),
        Some("complete-private")
    );
    let shown = format!(
        "{:?} {:?} {:?}",
        value.identity(),
        value.meta(),
        crate::api::decision(&result)
            .unwrap()
            .identity()
            .question_sources()
    );
    assert!(!shown.contains("complete-private"));
    assert_eq!(listener.count(), 1);
}

#[test]
fn aggregate_request_debug_withholds_authored_names_models_and_private_rules() {
    let secret = "private-credential-shaped-input";
    let kind = thinkthen::Kind::new(secret, None).unwrap();
    let rule = thinkthen::RelationRule::one_way(secret, "*", "*").unwrap();
    let recognize = thinkthen::Recognize::builder()
        .kind(kind.clone())
        .unwrap()
        .relation(rule.clone())
        .unwrap()
        .model(secret)
        .unwrap();
    let relate = thinkthen::Relate::builder()
        .relation(rule.clone())
        .unwrap()
        .model(secret)
        .unwrap();
    let reading = thinkthen::RecordReading::new(&[&format!("/{secret}")], None, None).unwrap();
    let saved = thinkthen::RecognizeQuestionFile::from_json(&format!(
        r#"{{"version":1,"recognize":{{}},"on":"/{secret}"}}"#
    ))
    .unwrap();
    assert!(!format!("{reading:?}").contains(secret));
    assert!(!format!("{saved:?}").contains(secret));
    for rendered in [
        format!("{kind:?}"),
        format!("{rule:?}"),
        format!("{recognize:?}"),
        format!("{relate:?}"),
        format!("{:?}", recognize.build().unwrap()),
        format!("{:?}", relate.build().unwrap()),
    ] {
        assert!(!rendered.contains(secret));
    }
}

#[test]
fn complete_score_retains_explicit_null_and_array_level_descriptions() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"score","probabilities":{"0":0.25,"1":0.75}}}}"#)).unwrap();
    let engine = engine(&listener);
    let thinkthen::LoadedQuestion::Question(question) =
        Question::from_json(r#"{"score":"Grade?","levels":{"low":null,"high":["Higher.",null]}}"#)
            .unwrap()
    else {
        panic!("score")
    };
    let call = crate::api::request(
        &engine,
        thinkthen::RequestFunction::Score,
        question.clone().into(),
        crate::api::text("Evidence."),
        RequestEnvironment::default(),
    )
    .unwrap();
    let document = serde_json::to_value(&call).unwrap();
    assert_eq!(
        document["value"][0]["question"]["label_details"],
        json!([
            {"name":"low","description":null}, {"name":"high","description":["Higher.",null]}
        ])
    );
    schema::check(&document, "completeScore");
    assert_eq!(listener.count(), 1);
}

#[test]
fn saved_profiled_questions_emit_one_profile_and_preserve_native_readings() {
    // RawRecord uses the native duplicate-rejecting JSON reader before Value can collapse keys.
    assert!(thinkthen::RawRecord::json(r#"{"profile":"first","profile":"second"}"#).is_err());
    let listener = Listener::answering(|_| {
        Canned::ok(
            r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":1,"u002":0}}}}"#,
        )
    })
    .unwrap();
    let engine = engine(&listener);
    for profile in [None, Some("saved-profile")] {
        let extra = profile.map_or(String::new(), |value| format!(r#", "profile":"{value}""#));
        let find = Question::find_from_json(&format!(
            r#"{{"find":{{"z":"Which?","a":[null,false]}},"model":"fixed","on":""{extra}}}"#
        ))
        .unwrap();
        let recognition = thinkthen::RecognizeQuestionFile::from_json(&format!(
            r#"{{"version":1,"recognize":{{"kinds":{{"person":null}}}},"model":"fixed","on":"/body"{extra}}}"#
        )).unwrap();
        let relation = thinkthen::Relate::from_json(&format!(
            r#"{{"version":1,"relate":{{"relations":[{{"name":"knows","source":"person","target":"person","reads":"knows"}}]}},"model":"fixed"{extra}}}"#
        ))
        .unwrap();
        let found = crate::api::find(
            &engine,
            &find,
            crate::api::plain_records(["first", "second"]),
            CallOptions::new(),
        )
        .unwrap();
        let recognized =
            crate::api::recognize(&engine, recognition.question(), "", CallOptions::new()).unwrap();
        let related = crate::api::relation(
            &engine,
            &relation,
            crate::api::plain_records(std::iter::empty::<thinkthen::RecordEvidence>()),
            CallOptions::new(),
        )
        .unwrap();
        let found = crate::api::found(&found).unwrap();
        assert_eq!(found.question().profile(), profile);
        assert_eq!(
            crate::api::recognition(&recognized)
                .unwrap()
                .question()
                .profile(),
            profile
        );
        let related = crate::api::related(&related).unwrap().result();
        assert_eq!(related.question().profile(), profile);
        for (function, bytes) in [
            ("find", found.to_json().unwrap()),
            (
                "recognize",
                crate::api::recognition(&recognized)
                    .unwrap()
                    .to_json()
                    .unwrap(),
            ),
            ("relate", related.to_json().unwrap()),
        ] {
            let parsed = thinkthen::RawRecord::json(&bytes)
                .unwrap_or_else(|error| panic!("{function}: {error}"));
            let document = serde_json::to_value(parsed).unwrap();
            assert_eq!(document["question"]["verb"], function);
            assert_eq!(document["question"]["model"], "fixed");
            assert_eq!(
                document["question"].get("profile"),
                profile.map(|value| json!(value)).as_ref()
            );
            match function {
                "find" => {
                    assert_eq!(
                        document["question"]["text"],
                        json!({"z":"Which?","a":[null,false]})
                    );
                    assert_eq!(document["question"]["on"], json!([""]));
                }
                "recognize" => {
                    assert_eq!(document["question"]["kinds"], json!({"person":null}));
                    assert_eq!(document["question"]["on"], json!(["/body"]));
                }
                "relate" => assert_eq!(document["question"]["relations"][0]["reads"], "knows"),
                _ => unreachable!(),
            }
        }
    }
    assert_eq!(listener.count(), 2);
}
