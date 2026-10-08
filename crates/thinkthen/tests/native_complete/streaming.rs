use super::*;
use std::cell::Cell;
use std::num::NonZeroUsize;
use std::rc::Rc;
use thinkthen::{
    BatchSetting, CancelToken, Evidence, InputFileReader, InputReaderOptions, RawRecord,
    RecordInput, RecordReading, StopCause,
};

#[test]
fn a_located_reader_failure_returns_the_complete_prefix_then_joined_facts_without_reading_a_suffix()
{
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":887}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Refund?").unwrap().cut();
    let reading = RecordReading::new(&["/body"], Some("/context"), None).unwrap();
    let reader = InputFileReader::new("private.jsonl", std::io::Cursor::new(b"{\"z\":false,\"body\":\"First.\",\"context\":\"Guide.\"}\n{not-valid-private\n{\"body\":\"Unread.\",\"context\":\"Guide.\"}\n"), InputReaderOptions::default()).unwrap();
    let pulled = Cell::new(0);
    let records = reader
        .inspect(|_| pulled.set(pulled.get() + 1))
        .map(|source| source.and_then(|source| reading.compose_source(source)));
    let mut batch = engine.try_decide_records_complete_with(
        &question,
        records,
        CallOptions::new()
            .batch(BatchSetting::Records(NonZeroUsize::new(1).unwrap()))
            .attempts(true),
    );
    let first = batch.next().unwrap().unwrap();
    assert_eq!(pulled.get(), 1);
    assert_eq!(first.ordinal(), 0);
    assert_eq!(first.result().value(), Answer::Yes);
    let thinkthen::QuestionInput::Record(original) = first.original() else {
        panic!("selected record")
    };
    assert_eq!(original.location().unwrap().first_line(), Some(1));
    assert_eq!(
        original.original().content().unwrap().to_json().unwrap(),
        r#"{"z":false,"body":"First.","context":"Guide."}"#
    );
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(error.stopped().cause(), StopCause::Usage);
    assert_eq!(pulled.get(), 2);
    assert!(batch.next().is_none());
    assert_eq!(error.facts().unwrap().records(), 1);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(error.facts().unwrap().input_tokens(), Some(887));
    assert_eq!(error.facts().unwrap().output_tokens(), None);
    assert_eq!(
        batch.facts().unwrap().call_id(),
        error.facts().unwrap().call_id()
    );
    assert_eq!(batch.facts().unwrap().attempts().unwrap().len(), 1);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Guide.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"First.\". Refund?"}}})
    );
    assert!(!format!("{error:?} {error} {first:?}").contains("not-valid-private"));
}

struct Original {
    number: usize,
    local: Rc<()>,
}
impl Evidence for Original {
    fn evidence(&self) -> &str {
        "Same."
    }
}
#[test]
fn pulled_originals_need_no_clone_send_or_serialize_and_collection_keeps_complete_facts() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.2}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Refund?").unwrap().cut();
    let records = (0..2).map(|number| {
        Ok(RecordInput {
            examples: None,
            original: Original {
                number,
                local: Rc::new(()),
            },
            context: Some("Guide.".into()),
            options: None,
        })
    });
    let call = engine
        .try_decide_records_complete_with(&question, records, CallOptions::new().attempts(true))
        .into_call()
        .unwrap();
    assert_eq!(call.facts().records(), 2);
    assert_eq!(call.facts().requests_sent(), 1);
    assert!(call.complete().is_some());
    assert_eq!(call.value()[0].result().value(), Answer::No);
    assert_eq!(call.value()[1].original().number, 1);
    assert_eq!(Rc::strong_count(&call.value()[1].original().local), 1);
    assert_eq!(
        call.value()[0].result().identity().observations(),
        call.value()[1].result().identity().observations()
    );
    assert_ne!(
        call.value()[0].result().answer_id(),
        call.value()[1].result().answer_id()
    );
    assert_eq!(listener.count(), 1);
    let cancelled = CancelToken::new();
    cancelled.cancel();
    let pulled = Cell::new(0);
    let records = (0..2).map(|_| {
        pulled.set(pulled.get() + 1);
        Ok(RecordInput {
            examples: None,
            original: "Never.",
            context: None,
            options: None,
        })
    });
    let refused = engine
        .try_decide_records_complete_with(&question, records, CallOptions::new().cancel(&cancelled))
        .into_call()
        .unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Cancelled);
    assert_eq!(pulled.get(), 0);
    assert_eq!(listener.count(), 1);
}

#[test]
fn pulled_choose_score_tag_and_filter_execute_concrete_complete_results() {
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        let answers = match body["questions"]["q1"]["type"].as_str().unwrap() {
            "choice" => json!({"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}}),
            "score" => json!({"q1":{"type":"score","probabilities":{"0":0.25,"1":0.75}}}),
            "noul" if body["questions"].as_object().unwrap().len() == 2 => {
                json!({"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}})
            }
            "noul" => json!({"q1":{"type":"noul","noul":0.2}}),
            other => panic!("unexpected {other}"),
        };
        Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
    })
    .unwrap();
    let engine = engine(&listener);
    let choice =
        Question::from_json(r#"{"choose":"Which?","options":["fixed-a","fixed-b"]}"#).unwrap();
    let reading = RecordReading::new(&["/body"], None, Some("/options")).unwrap();
    let original = reading
        .compose(
            RawRecord::json(r#"{"z":false,"body":"Text.","options":{"b":null,"a":null}}"#).unwrap(),
        )
        .unwrap();
    let choices = engine
        .try_choose_records_complete_with(&choice, [Ok(original)], CallOptions::new())
        .into_call()
        .unwrap();
    assert_eq!(choices.value()[0].result().value(), Some("b"));
    assert_eq!(
        choices.value()[0]
            .original()
            .original()
            .content()
            .unwrap()
            .to_json()
            .unwrap(),
        r#"{"z":false,"body":"Text.","options":{"b":null,"a":null}}"#
    );
    let inputs = || {
        [Ok(RecordInput {
            examples: None,
            original: "Text.",
            context: None,
            options: None,
        })]
    };
    let thinkthen::LoadedQuestion::Question(score) =
        Question::from_json(r#"{"score":"Grade?","levels":["Low","High"]}"#).unwrap()
    else {
        panic!("score")
    };
    let scored = engine
        .try_score_records_complete_with(&score, inputs(), CallOptions::new())
        .into_call()
        .unwrap();
    assert_eq!(scored.value()[0].result().value(), 0.75);
    let tag = Question::from_json(r#"{"tag":"Topics?","labels":["a","b"]}"#).unwrap();
    let tagged = engine
        .try_tag_records_complete_with(&tag, inputs(), CallOptions::new())
        .into_call()
        .unwrap();
    assert_eq!(tagged.value()[0].result().value(), &["a"]);
    let filter = Question::decide("Keep?").unwrap().cut();
    let filtered = engine
        .try_filter_records_complete_with(&filter, inputs(), CallOptions::new())
        .into_call()
        .unwrap();
    schema::call(&filtered, "completeFilter");
    assert!(!filtered.value()[0].result().value());
    assert_eq!(listener.count(), 4);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"choice","instructions":"The text is \"Text.\". Which?","criteria":{"b":null,"a":null}}}})
    );
}

#[test]
fn pulled_annotations_preserve_partial_member_failure_original_location_and_owned_observation() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"wrong":1.0}}}}"#)).unwrap();
    let engine = engine(&listener);
    let set = thinkthen::QuestionSet::from_json(
        r#"{"version":1,"questions":{"good":{"decide":"Good?"},"bad":{"decide":"Bad?"}}}"#,
    )
    .unwrap();
    let reading = RecordReading::new(&[], None, None).unwrap();
    let mut original = reading
        .compose(RawRecord::json(r#"{"z":false,"body":"Original."}"#).unwrap())
        .unwrap();
    original.original = original.original.with_location(
        thinkthen::SourceLocation::new("private.txt".into(), Some(4), Some(4)).unwrap(),
    );
    let observations = std::sync::Mutex::new(Vec::new());
    let observer = |event: thinkthen::RecordObservation<'_>| {
        observations.lock().unwrap().push(event.to_owned())
    };
    let call = engine
        .try_annotate_records_complete_with(
            &set,
            [Ok(original)],
            CallOptions::new().observe(&observer).attempts(true),
        )
        .into_call()
        .unwrap();
    assert_eq!(call.facts().records(), 1);
    assert_eq!(call.value()[0].result().members().count(), 2);
    assert_eq!(
        call.value()[0]
            .original()
            .original()
            .content()
            .unwrap()
            .to_json()
            .unwrap(),
        r#"{"z":false,"body":"Original."}"#
    );
    let doc = serde_json::to_value(call.complete().unwrap()).unwrap();
    assert!(doc["value"][0]["answers"]["bad"]["failure_id"].is_string());
    assert_eq!(observations.lock().unwrap().len(), 3);
    assert_eq!(listener.count(), 1);
}

#[test]
fn an_explicit_per_record_context_obeys_the_caller_byte_cap_before_its_row_sends() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .no_cache()
        .max_request_bytes(120)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let records = || {
        [
            RecordInput {
                examples: None,
                original: "First.",
                context: None,
                options: None,
            },
            RecordInput {
                examples: None,
                original: "Second.",
                context: Some("x".repeat(200).into()),
                options: None,
            },
        ]
    };
    let eager = engine
        .decide_records_complete_with(&question, records(), CallOptions::new())
        .unwrap_err();
    assert_eq!(eager.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
    let mut streamed = engine.try_decide_records_complete_with(
        &question,
        records().into_iter().map(Ok),
        CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::new(1).unwrap())),
    );
    assert_eq!(streamed.next().unwrap().unwrap().original(), &"First.");
    let error = streamed.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(error.facts().unwrap().records(), 1);
    assert!(streamed.next().is_none());
    assert_eq!(listener.count(), 1);
}
