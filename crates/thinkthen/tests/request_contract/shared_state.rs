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
