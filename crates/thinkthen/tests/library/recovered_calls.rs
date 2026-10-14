//! Caller results retain all fields without another request, and recovery preserves packing.
use conformance_backend::{Canned, Listener};
use thinkthen::{
    CallOptions, ErrorKind, Question, QuestionInput, Request, RequestArguments, RequestDefinition,
    RequestEnvironment, RequestFeed, RequestFraming, RequestFunction, RequestInput, RequestOptions,
    RequestQuestion, RequestValue,
};

fn admitted(
    function: RequestFunction,
    definition: RequestDefinition,
) -> thinkthen::AdmittedRequest {
    Request::for_function(
        function,
        RequestArguments {
            question: RequestQuestion::Definition { value: definition },
            input: RequestInput::Feed {
                name: "records".into(),
                framing: RequestFraming::Document,
                reading: thinkthen::ReaderOptions::default(),
                images: vec![],
            },
            options: RequestOptions::default(),
        },
    )
    .admit()
    .unwrap()
}
fn records(
    values: &[&str],
) -> Vec<Result<thinkthen::RecordInput<QuestionInput>, thinkthen::Error>> {
    values
        .iter()
        .map(|text| Ok(crate::api::record(QuestionInput::Text((*text).into()))))
        .collect()
}

#[test]
fn decision_call_retains_nullable_value_probability_record_and_facts_without_resending() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.99}}}"#)
    })
    .unwrap();
    let engine = crate::api::loopback(listener.base(), "sk-one-result");
    let question = Question::decide("Refund?").unwrap().cut();
    let call = engine.decide(&question, "Please refund me.").unwrap();
    assert_eq!(call.value(), &Some(true));
    assert_eq!(call.probability(), &Some(0.99));
    assert_eq!(call.results().len(), 1);
    assert_eq!(call.results()[0].probability(), Some(0.99));
    assert_eq!(call.facts().requests_sent(), 1);
    let json = serde_json::to_value(&call).unwrap();
    assert_eq!(json["value"], true);
    assert_eq!(json["probability"], 0.99);
    assert_eq!(json["results"][0]["answer"]["probability"], 0.99);
    assert_eq!(json["facts"]["requests_sent"], 1);
    assert_eq!(listener.count(), 1);

    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.5}}}"#)
    })
    .unwrap();
    let engine = crate::api::loopback(listener.base(), "sk-one-result");
    let band = Question::decide("Refund?").unwrap().band(0.2, 0.8).unwrap();
    let call = engine.decide(&band, "Maybe.").unwrap();
    assert_eq!(call.value(), &None);
    assert_eq!(call.probability(), &Some(0.5));
    assert_eq!(listener.count(), 1);
}

#[test]
fn recovering_rows_keep_good_members_after_a_bad_answer_in_one_packed_request() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"wrong"},"q2":{"type":"noul","noul":0.99}}}"#)).unwrap();
    let engine = crate::api::loopback(listener.base(), "sk-recovery");
    let request = admitted(
        RequestFunction::Decide,
        Question::decide("Refund?").unwrap().cut().into(),
    );
    let call = engine
        .execute_request_recovering(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(
                    RequestFeed::from_records("records", records(&["bad", "good"]).into_iter())
                        .eager(),
                ),
            },
        )
        .unwrap();
    assert_eq!(call.value().len(), 2);
    assert_eq!(call.value()[0].ordinal(), 0);
    assert_eq!(
        call.value()[0].outcome().unwrap_err().kind(),
        ErrorKind::Backend
    );
    let RequestValue::Decisions(good) = call.value()[1].outcome().unwrap() else {
        panic!("decision");
    };
    assert_eq!(good[0].ordinal(), 1);
    assert_eq!(good[0].result().probability(), Some(0.99));
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(call.facts().records(), 2);
    assert_eq!(listener.count(), 1);
}

#[test]
fn eager_recovering_admission_refuses_a_late_reader_error_before_any_send() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = crate::api::loopback(listener.base(), "sk-recovery");
    let request = admitted(
        RequestFunction::Decide,
        Question::decide("Refund?").unwrap().cut().into(),
    );
    let mut input = records(&["good"]);
    input.push(Err(thinkthen::Error::new(
        ErrorKind::Usage,
        "malformed descriptor",
    )));
    let error = engine
        .execute_request_recovering(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::from_records("records", input.into_iter()).eager()),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
}
