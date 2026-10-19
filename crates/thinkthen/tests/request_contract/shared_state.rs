//! Shared-state preparation uses the same public call, native preview and decoder.
use super::*;

fn arguments() -> RequestArguments {
    args(QuestionSet::from_json(r#"{"version":1,"questions":{"yes":{"decide":"Fits?","true":{"what":"yes detail","examples":["extra-detail"]}},"pick":{"choose":"Which?","options":{"z":{"what":"first detail","examples":["extra-option"]},"a":null}}}}"#).unwrap().into(),
        RequestInput::Text { text: "shared evidence".into(), images: vec![] })
}

#[test]
fn shared_state_preview_and_execution_keep_original_questions_descriptions_and_exact_body() {
    for backend in ["typesafe", "ollama"] {
        let listener = Listener::answering(response).unwrap();
        let engine = Engine::builder().backend(backend).unwrap().base_url(listener.base()).unwrap()
            .api_key("shared-state-fixture-key").unwrap().model("fixed").unwrap().no_cache().build().unwrap();
        let request = Request::named("annotate", arguments()).unwrap().admit().unwrap();
        let preview = engine.plan_request(&request, RequestEnvironment {
            controls: CallOptions::new().shared_state_groups(), feed: None,
        }).unwrap();
        assert_eq!(listener.count(), 0);
        assert_eq!(preview.records(), 1);
        assert_eq!(preview.requests(), 1);
        let body = preview.first_body().unwrap();
        let wire: Value = serde_json::from_slice(body).unwrap();
        assert_eq!(wire["state"], "shared evidence");
        assert_eq!(wire["questions"]["q1"]["instructions"], "Fits?");
        assert_eq!(wire["questions"]["q2"]["instructions"], "Which?");
        let descriptions = std::str::from_utf8(body).unwrap();
        assert_eq!(descriptions.contains("extra-detail"), backend == "typesafe");
        assert_eq!(descriptions.contains("extra-option"), backend == "typesafe");
        let call = engine.call("annotate", arguments(), RequestEnvironment {
            controls: CallOptions::new().shared_state_groups(), feed: None,
        }).unwrap();
        assert_eq!(call.facts().requests_sent(), 1);
        assert_eq!(listener.count(), 1);
        assert_eq!(listener.requests()[0].body, body);
        let ordinary = engine.plan_request(&request, RequestEnvironment::default()).unwrap();
        assert_ne!(ordinary.first_body().unwrap(), body);
        assert!(std::str::from_utf8(ordinary.first_body().unwrap()).unwrap().contains("The text is"));
    }
}

#[test]
fn incompatible_preparation_refuses_before_loading_a_question_or_advancing_a_feed() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let mut advanced = 0;
    let arguments = RequestArguments {
        question: RequestQuestion::File { path: "absent-shared-state-question".into() },
        input: RequestInput::Feed { name: "input".into(), framing: RequestFraming::Document,
            reading: ReaderOptions::default(), images: vec![] },
        options: RequestOptions::default(),
    };
    let request = Request::named("decide", arguments.clone()).unwrap().admit().unwrap();
    assert_eq!(engine.plan_request(&request, RequestEnvironment {
        controls: CallOptions::new().shared_state_groups(), feed: None,
    }).unwrap_err().detail().message(), "shared-state groups require an annotation request");
    let error = engine.call("decide", arguments, RequestEnvironment {
        controls: CallOptions::new().shared_state_groups(),
        feed: Some(RequestFeed::new("input", [Ok(item("evidence"))].into_iter().inspect(|_| advanced += 1))),
    }).unwrap_err();
    assert_eq!(error.detail().message(), "shared-state groups require an annotation request");
    assert_eq!(advanced, 0);
    assert_eq!(listener.count(), 0);
}

#[test]
fn shared_state_retains_decoded_failed_members_and_observes_them_before_completion() {
    use std::sync::Mutex;
    for all_failed in [false, true] {
        let listener = Listener::answering(move |_| Canned::ok(if all_failed {
            r#"{"model":"fixed","answers":{},"usage":{"input_tokens":6,"output_tokens":2}}"#
        } else {
            r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":6,"output_tokens":2}}"#
        })).unwrap();
        let engine = engine(&listener);
        let events = Mutex::new(Vec::new());
        let observe = |event: RecordObservation<'_>| match event {
            RecordObservation::Question { member, .. } => events.lock().unwrap().push(member.unwrap().to_owned()),
            RecordObservation::Row { .. } => events.lock().unwrap().push("row".into()),
        };
        let call = engine.call("annotate", arguments(), RequestEnvironment {
            controls: CallOptions::new().shared_state_groups().observe(&observe), feed: None,
        }).unwrap();
        assert_eq!(*events.lock().unwrap(), ["yes", "pick", "row"]);
        let RequestValue::Annotations(rows) = call.value() else { panic!("annotations") };
        let result = rows[0].result();
        let members = result.members().collect::<Vec<_>>();
        assert_eq!(members.iter().map(|m| m.name()).collect::<Vec<_>>(), ["yes", "pick"]);
        assert_eq!(members.iter().filter(|m| m.failure().is_some()).count(), if all_failed { 2 } else { 1 });
        for member in members.iter().filter(|m| m.failure().is_some()) {
            assert_eq!(member.failure(), Some(FailureCause::MissingAnswer));
            assert!(member.value().is_none());
            assert!(member.probabilities().is_none());
            assert!(member.leader().is_none());
            assert!(member.answer_id().is_none());
            assert!(member.failure_id().is_some());
            assert_eq!(member.observations().len(), 1);
            assert!(member.reported_usage().is_some());
        }
        assert_eq!(call.facts().records(), 1);
        assert_eq!(call.facts().requests_sent(), 1);
        assert_eq!(call.facts().input_tokens(), Some(6));
        assert_eq!(call.facts().output_tokens(), Some(2));
        let request = Request::named("annotate", arguments()).unwrap().admit().unwrap();
        let recovered = engine.execute_request_recovering(&request, RequestEnvironment {
            controls: CallOptions::new().shared_state_groups(), feed: None,
        }).unwrap();
        assert!(recovered.value()[0].outcome().is_ok());
        let ordinary = engine.call("annotate", arguments(), RequestEnvironment::default());
        assert_eq!(ordinary.is_err(), all_failed);
        assert_eq!(listener.count(), 3);
    }
}

#[test]
fn annotation_leaders_retain_native_answers_before_thresholds_and_weighted_readings() {
    for (distribution, confidence) in [(r#"{"z":0.6,"a":0.4}"#, "0.8"), (r#"{"z":0.5,"a":0.5}"#, "0.7")] {
        let body = format!(r#"{{"model":"fixed","answers":{{"q1":{{"type":"choice","probabilities":{distribution},"confidence":{confidence}}},"q2":{{"type":"score","probabilities":{{"0":0.4,"1":0.6}}}},"q3":{{"type":"noul","noul":0.9}},"q4":{{"type":"noul","noul":0.8}}}}}}"#);
        let listener = Listener::answering(move |_| Canned::ok(&body)).unwrap();
        let set = QuestionSet::from_json(r#"{"version":1,"questions":{"pick":{"choose":"Which?","options":["z","a"],"threshold":0.9},"score":{"score":"How?","levels":["low","high"]},"yes":{"decide":"Fits?"},"tag":{"tag":"Tags?","labels":["x"]}}}"#).unwrap();
        let arguments = args(set.into(), RequestInput::Text { text: "evidence".into(), images: vec![] });
        let call = engine(&listener).call("annotate", arguments, RequestEnvironment {
            controls: CallOptions::new().shared_state_groups(), feed: None,
        }).unwrap();
        let RequestValue::Annotations(rows) = call.value() else { panic!("annotations") };
        let members = rows[0].result().members().collect::<Vec<_>>();
        assert_eq!(members[0].leader(), Some("z"));
        assert_eq!(members[0].value(), Some(Judgment::Choice(None)));
        assert_eq!(members[1].leader(), Some("high"));
        assert_eq!(members[1].value(), Some(Judgment::Score(0.6)));
        assert_eq!(members[2].leader(), None);
        assert_eq!(members[3].leader(), None);
        assert_eq!(listener.count(), 1);
    }
}
