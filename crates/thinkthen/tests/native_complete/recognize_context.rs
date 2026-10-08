use super::*;
use thinkthen::{RawRecord, Recognize, RecordInput, RecordReading};

#[test]
fn recognition_keeps_record_context_separate_and_replays_only_unchanged_context() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let cache = folder();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::builder().build().unwrap();
    let rows = || {
        [Some("first"), Some("second"), Some(""), None].map(|context| RecordInput {
            original: "Ada met Acme.",
            context: context.map(Into::into),
            options: None,
        })
    };
    let options = CallOptions::new().context("shared");
    let first = engine
        .recognize_records_complete_with(&ask, rows(), options)
        .unwrap();
    assert_eq!(first.facts().requests_sent(), 8);
    assert_eq!(first.facts().records(), 4);
    let requests = listener.requests();
    for (pair, context) in requests
        .chunks_exact(2)
        .zip(["first", "second", "", "shared"])
    {
        for request in pair {
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(
                body["state"],
                if context.is_empty() {
                    json!("Ada met Acme.")
                } else {
                    json!({"context":context,"evidence":"Ada met Acme."})
                }
            );
        }
    }
    let replay = engine
        .recognize_records_complete_with(&ask, rows(), options)
        .unwrap();
    assert_eq!(replay.facts().requests_sent(), 0);
    let changed = engine
        .recognize_records_complete_with(
            &ask,
            [RecordInput {
                original: "Ada met Acme.",
                context: Some("changed".into()),
                options: None,
            }],
            options,
        )
        .unwrap();
    assert_eq!(changed.facts().requests_sent(), 2);
    assert_eq!(listener.count(), 10);
    assert_ne!(
        first.value()[0].result().identity().observations(),
        changed.value()[0].result().identity().observations()
    );
    std::fs::remove_dir_all(cache).unwrap();
}

#[test]
fn selected_recognition_context_rejects_missing_null_and_nontext_before_sends() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().build().unwrap();
    let reading = RecordReading::new(&["/body"], Some("/context"), None).unwrap();
    for invalid in [
        r#"{"body":"Ada met Acme."}"#,
        r#"{"body":"Ada met Acme.","context":null}"#,
        r#"{"body":"Ada met Acme.","context":4}"#,
    ] {
        let valid = reading
            .compose(RawRecord::json(r#"{"body":"Ada met Acme.","context":"first"}"#).unwrap())
            .unwrap();
        let failure = engine
            .try_recognize_records_complete_with(
                &ask,
                [
                    Ok(valid),
                    reading.compose(RawRecord::json(invalid).unwrap()),
                ],
                CallOptions::new(),
            )
            .unwrap_err();
        assert_eq!(failure.stopped().at(), Some(2));
    }
    assert_eq!(listener.count(), 0);
}
