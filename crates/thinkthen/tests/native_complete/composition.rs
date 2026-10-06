use super::*;
use thinkthen::{BatchSetting, RawRecord, RecordReading, SourceLocation};

#[cfg(test)]
fn storage_builder(listener: &Listener, backend: &str, model: &str) -> thinkthen::EngineBuilder {
    Engine::builder()
        .backend(backend)
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model(model)
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .max_retries(0)
}

#[test]
fn native_projection_sends_selected_content_and_controls_retaining_entire_originals_and_locations()
{
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}}}"#)).unwrap();
    let folder = folder();
    let engine = storage_builder(&listener, "liquid", "fixed")
        .cache_at(&folder)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::choose_labels("Which?")
        .unwrap()
        .label("fixed-a", None)
        .unwrap()
        .label("fixed-b", None)
        .unwrap()
        .build()
        .unwrap();
    let reading = RecordReading::new(&["/body"], Some("/context"), Some("/candidates")).unwrap();
    let originals = [
        r#"{"z":false,"body":"Same.","context":"Guide.","candidates":{"b":{"z":null,"a":"B"},"a":null},"private":[2,1]}"#,
        r#"{"z":null,"body":"Same.","context":"Guide.","candidates":{"b":{"z":null,"a":"B"},"a":null},"private":[9,8]}"#,
    ];
    let inputs = || {
        originals.iter().enumerate().map(|(at, text)| {
            let mut record = reading.compose(RawRecord::json(text).unwrap()).unwrap();
            record.original = record.original.with_location(
                SourceLocation::new(format!("private-{at}.jsonl"), Some(at + 4), Some(at + 4))
                    .unwrap(),
            );
            record
        })
    };
    let rows = engine
        .choose_records_complete_with(
            &question,
            inputs(),
            CallOptions::new()
                .context("Ignored fallback.")
                .batch(BatchSetting::Max),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 1);
    assert_eq!(
        std::str::from_utf8(&listener.requests()[0].body).unwrap(),
        r#"{"state":"Guide.","model":"fixed","questions":{"q1":{"type":"choice","instructions":"The text is \"Same.\". Which?","criteria":{"b":{"z":null,"a":"B"},"a":null}}}}"#
    );
    assert_eq!(
        rows.value()[0].result().identity().observations(),
        rows.value()[1].result().identity().observations()
    );
    assert_ne!(
        rows.value()[0].result().answer_id(),
        rows.value()[1].result().answer_id()
    );
    for (at, row) in rows.value().iter().enumerate() {
        assert_eq!(row.result().value(), Some("b"));
        assert_eq!(
            row.original()
                .original()
                .content()
                .unwrap()
                .to_json()
                .unwrap(),
            originals[at]
        );
        assert_eq!(
            row.original().location().unwrap().first_line(),
            Some(at + 4)
        );
        let serialized = serde_json::to_string(row).unwrap();
        let original_json: Value = serde_json::from_str(originals[at]).unwrap();
        let doc: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(doc["input"], original_json);
        assert!(!format!("{:?}", row.original()).contains("private-"));
    }
    let replay = storage_builder(&listener, "liquid", "fixed")
        .replay(&folder)
        .unwrap()
        .build()
        .unwrap();
    let replayed = replay
        .choose_records_complete_with(&question, inputs(), CallOptions::new())
        .unwrap();
    for (first, held) in rows.value().iter().zip(replayed.value()) {
        assert_eq!(first.result().answer_id(), held.result().answer_id());
        assert_eq!(held.result().identity().origin(), Some(Origin::Replay));
    }
    assert_eq!(listener.count(), 1);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn shared_composition_refuses_missing_or_nonstring_controls_without_sending_or_echoing_content() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    for original in [
        r#"{"body":"private-evidence","context":null,"candidates":["a","b"]}"#,
        r#"{"body":"private-evidence","context":false,"candidates":["a","b"]}"#,
        r#"{"body":"private-evidence","context":"Guide.","candidates":["a","a"]}"#,
        r#"{"context":"Guide.","candidates":["a","b"]}"#,
    ] {
        let error = RecordReading::new(&["/body"], Some("/context"), Some("/candidates"))
            .unwrap()
            .compose(RawRecord::json(original).unwrap())
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(!format!("{error:?} {error}").contains("private-evidence"));
    }
    assert!(RecordReading::new(&["/one/body", "/two/body"], None, None).is_err());
    assert!(RawRecord::json(r#"{"body":false,"body":null}"#).is_err());
    assert!(RawRecord::json("NaN").is_err());
    let record = RecordReading::new(&["/body"], Some("/context"), None)
        .unwrap()
        .compose(RawRecord::json(r#"{"body":false,"context":""}"#).unwrap())
        .unwrap();
    assert_eq!(record.context.as_deref(), Some(""));
    assert_eq!(record.original.selected().to_json().unwrap(), "false");
    assert_eq!(listener.count(), 0);
}

#[test]
fn annotation_member_pointers_read_the_selected_record_and_keep_the_whole_original() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let set = thinkthen::QuestionSet::from_json(
        r#"{"version":1,"questions":{"refund":{"decide":"Refund?","on":"/message"}}}"#,
    )
    .unwrap();
    let original = r#"{"private":false,"selected":{"message":"Refund please.","other":null},"context":"Guide."}"#;
    let record = RecordReading::new(&["/selected"], Some("/context"), None)
        .unwrap()
        .compose(RawRecord::json(original).unwrap())
        .unwrap();
    let call = engine
        .annotate_records_complete_with(&set, [record], CallOptions::new())
        .unwrap();
    let doc: Value =
        serde_json::from_str(&serde_json::to_string(&call.value()[0]).unwrap()).unwrap();
    assert_eq!(
        doc["input"],
        serde_json::from_str::<Value>(original).unwrap()
    );
    assert_eq!(doc["value"]["refund"], true);
    assert_eq!(listener.count(), 1);
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        body,
        json!({"state":{"context":"Guide.","evidence":"Each question quotes the text it asks about."},"model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund please.\". Refund?"}}})
    );
}

#[test]
fn explicit_composed_images_keep_bytes_duplicates_and_filename_out_of_live_and_replay_keys() {
    use base64::Engine as _;
    use thinkthen::{ImageInput, ImageMedia};
    const RED: &[u8] = include_bytes!("../../../../specification/fixtures/images/red.png");
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"d1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let folder = folder();
    let engine = storage_builder(&listener, "liquid", "d1")
        .cache_at(&folder)
        .unwrap()
        .build()
        .unwrap();
    let reading = RecordReading::new(&["/body"], None, None).unwrap();
    let record = || {
        let mut record = reading
            .compose(RawRecord::json(r#"{"body":"Compare.","private":false}"#).unwrap())
            .unwrap();
        let image = ImageInput::new(ImageMedia::Png, RED).unwrap();
        record.original = record
            .original
            .with_images(vec![image.clone(), image])
            .unwrap();
        record
    };
    let question = Question::decide("Same?").unwrap().cut();
    let rows = engine
        .decide_records_complete_with(&question, [record()], CallOptions::new())
        .unwrap();
    assert_eq!(rows.value()[0].original().images().len(), 2);
    assert_eq!(rows.value()[0].original().images()[0].bytes(), RED);
    let encoded = format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(RED)
    );
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        body,
        json!({"state":"Compare.","model":"d1","images":[encoded,encoded],"questions":{"q1":{"type":"noul","instructions":"Same?"}}})
    );
    let replay = storage_builder(&listener, "liquid", "d1")
        .replay(&folder)
        .unwrap()
        .build()
        .unwrap();
    let mut located = record();
    located.original = located
        .original
        .with_location(SourceLocation::new("renamed.jsonl".into(), Some(9), Some(9)).unwrap());
    let held = replay
        .decide_records_complete_with(&question, [located], CallOptions::new())
        .unwrap();
    assert_eq!(
        held.value()[0].result().answer_id(),
        rows.value()[0].result().answer_id()
    );
    assert_eq!(held.value()[0].original().images()[1].bytes(), RED);
    assert_eq!(listener.count(), 1);
    let source = thinkthen::InputFileReader::new(
        "original.png",
        std::io::Cursor::new(RED),
        thinkthen::InputReaderOptions {
            reading: thinkthen::ReaderOptions {
                unit: thinkthen::SourceUnit::File,
                window: None,
            },
            media: thinkthen::ReaderMedia::Image,
        },
    )
    .unwrap()
    .next()
    .unwrap()
    .unwrap();
    let composed = RecordReading::new(&[], None, None)
        .unwrap()
        .compose_source(source)
        .unwrap();
    let thinkthen::QuestionInput::Images(images) = composed.original else {
        panic!("explicit image")
    };
    assert_eq!(images.location().unwrap().file(), "original.png");
    assert_eq!(images.location().unwrap().first_line(), None);
    assert_eq!(images.location().unwrap().last_line(), None);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}
