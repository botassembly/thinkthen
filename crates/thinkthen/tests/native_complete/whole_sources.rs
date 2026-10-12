use super::*;
use thinkthen::{
    FindQuestionFile, InputFileReader, InputReaderOptions, QuestionInput, RawRecord, RecordInput,
    RecordReading,
};

#[test]
fn saved_find_reads_selected_located_units_once_and_retains_every_original_candidate_through_replay()
 {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"actual-model","answers":{"q1":{"type":"choice","probabilities":{"u001":0.4,"u002":0.4,"u003":0.2}}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let question_file = folder.join("find.json");
    std::fs::write(&question_file,r#"{"find":{"z":"Which?","a":["Read all."]},"on":"/body","model":"saved-model","profile":"saved-profile"}"#).unwrap();
    let saved = FindQuestionFile::load(&question_file).unwrap();
    let input = b"{\"z\":false,\"body\":\"Same.\",\"a\":[2,1]}\n{\"z\":null,\"body\":\"Same.\",\"a\":[9]}\n{\"z\":true,\"body\":\"Other.\"}\n";
    let records = || {
        InputFileReader::new(
            "private.jsonl",
            std::io::Cursor::new(input),
            InputReaderOptions::default(),
        )
        .unwrap()
        .map(|source| source.and_then(|source| saved.reading().compose_source(source)))
    };
    let build = || {
        crate::api::builder(listener.base(), "complete-private")
            .model("engine-model")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let held = std::sync::Mutex::new(Vec::new());
    let observer =
        |event: thinkthen::RecordObservation<'_>| held.lock().unwrap().push(event.to_owned());
    let call = crate::api::find(
        &engine,
        saved.question(),
        records(),
        CallOptions::new()
            .context("Guidance.")
            .attempts(true)
            .observe(&observer),
    )
    .unwrap();
    assert_eq!(call.facts().records(), 1);
    assert_eq!(call.facts().input_tokens(), Some(887));
    assert_eq!(call.facts().output_tokens(), None);
    let result = crate::api::found(&call).unwrap();
    assert_original_candidates(result);
    assert_eq!(result.question().profile(), Some("saved-profile"));
    assert_eq!(result.identity().answered_by(), Some("actual-model"));
    assert_eq!(
        result.identity().question_sources()[0].batch_size(),
        Some(1)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":{"context":"Guidance.","evidence":r#"[{"id":"u001","evidence":"Same."},{"id":"u002","evidence":"Same."},{"id":"u003","evidence":"Other."}]"#},"model":"saved-model","questions":{"q1":{"type":"choice","instructions":{"z":"Which?","a":["Read all."]},"criteria":{"u001":null,"u002":null,"u003":null}}}})
    );
    let doc: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert_serialized_candidates(&doc);
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let replayed = crate::api::find(
        &replay,
        saved.question(),
        records(),
        CallOptions::new().context("Guidance.").attempts(true),
    )
    .unwrap();
    let replayed_result = crate::api::found(&replayed).unwrap();
    assert_eq!(replayed_result.answer_id(), result.answer_id());
    assert_eq!(replayed_result.identity().origin(), Some(Origin::Replay));
    assert_eq!(
        replayed_result.identity().observations(),
        result.identity().observations()
    );
    assert_eq!(replayed_result.candidates().len(), 3);
    assert_eq!(replayed.facts().requests_sent(), 0);
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 1);
    drop(replay);
    drop(engine);
    let events = held.lock().unwrap();
    let detail = events
        .iter()
        .find_map(|event| match event {
            thinkthen::OwnedRecordObservation::Question { detail, .. } => Some(detail.detail()),
            _ => None,
        })
        .unwrap();
    assert_find_sources(detail);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn whole_set_find_and_rank_reader_failures_and_unsupported_controls_refuse_before_sending() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = engine(&listener);
    let find = Question::find("Where?").unwrap();
    let rank = Question::rank("Best?").unwrap();
    let records = || {
        [
            Ok(crate::api::record("Valid.")),
            Err(thinkthen::Error::new(
                ErrorKind::Local,
                "the selected source cannot be read",
            )),
        ]
    };
    let found = crate::api::find(&engine, &find, records(), CallOptions::new()).unwrap_err();
    assert_eq!(found.kind(), ErrorKind::Local);
    assert_eq!(found.stopped().at(), Some(2));
    assert!(found.facts().is_none());
    let ranked = crate::api::whole(
        &engine,
        thinkthen::RequestFunction::Rank,
        rank.clone().into(),
        records(),
        CallOptions::new(),
    )
    .unwrap_err();
    assert_eq!(ranked.kind(), ErrorKind::Local);
    assert_eq!(ranked.stopped().at(), Some(2));
    let originals = [
        crate::api::record("First."),
        RecordInput {
            context: Some("".into()),

            ..crate::api::record("Second.")
        },
    ];
    assert_eq!(
        crate::api::find(
            &engine,
            &find,
            (originals).into_iter().map(Ok),
            CallOptions::new()
        )
        .unwrap_err()
        .kind(),
        ErrorKind::Usage
    );
    let invalid = crate::api::find(
        &engine,
        &find,
        ([
            Ok("First."),
            Err(thinkthen::Error::new(
                ErrorKind::Local,
                "the source cannot be read",
            )),
        ])
        .into_iter()
        .map(|row| row.map(crate::api::record)),
        CallOptions::new(),
    )
    .unwrap_err();
    assert_eq!(invalid.kind(), ErrorKind::Local);
    assert_eq!(listener.count(), 0);
}

#[test]
fn composed_rank_keeps_original_location_and_stable_positions_without_observing_source_metadata() {
    let listener =
        crate::api::answering(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#);
    let engine = engine(&listener);
    let question = Question::rank("Best?").unwrap();
    let reading = RecordReading::new(&["/body"], None, None).unwrap();
    let records = [
        r#"{"z":false,"body":"Same."}"#,
        r#"{"z":null,"body":"Same."}"#,
    ]
    .into_iter()
    .enumerate()
    .map(|(at, original)| {
        let mut record = reading.compose(RawRecord::json(original).unwrap()).unwrap();
        record.original = record.original.with_location(
            thinkthen::SourceLocation::new("private.jsonl".into(), Some(at + 4), Some(at + 4))
                .unwrap(),
        );
        Ok(record)
    });
    let call = crate::api::whole(
        &engine,
        thinkthen::RequestFunction::Rank,
        question.into(),
        records,
        CallOptions::new(),
    )
    .unwrap();
    let ranked = crate::api::ranked(&call).unwrap();
    assert_eq!(ranked[0].result().value(), 1);
    assert_eq!(ranked[1].result().value(), 2);
    assert_eq!(ranked[0].ordinal(), 0);
    let QuestionInput::Record(original) = ranked[1].original() else {
        panic!("record original")
    };
    assert_eq!(original.location().unwrap().first_line(), Some(5));
    assert_eq!(
        ranked[0].result().identity().observations(),
        ranked[1].result().identity().observations()
    );
    assert_ne!(
        ranked[0].result().answer_id(),
        ranked[1].result().answer_id()
    );
    assert_eq!(listener.questions(), 1);
    assert_eq!(listener.count(), 1);
}

#[cfg(test)]
fn assert_original_candidates(result: &thinkthen::CompleteFound<QuestionInput>) {
    assert_eq!(result.candidates().len(), 3);
    let QuestionInput::Record(first) = result.selected().unwrap() else {
        panic!("selected original")
    };
    assert_eq!(
        first.original().content().unwrap().to_json().unwrap(),
        r#"{"z":false,"body":"Same.","a":[2,1]}"#
    );
    assert_eq!(first.location().unwrap().file(), "private.jsonl");
    assert_eq!(first.location().unwrap().first_line(), Some(1));
    let QuestionInput::Record(second) = result.candidates()[1].input().unwrap() else {
        panic!("duplicate original")
    };
    assert_eq!(
        second.original().content().unwrap().to_json().unwrap(),
        r#"{"z":null,"body":"Same.","a":[9]}"#
    );
    assert_eq!(second.location().unwrap().first_line(), Some(2));
}

#[cfg(test)]
fn assert_find_sources(detail: thinkthen::QuestionDetail<'_>) {
    let inputs = detail.inputs().collect::<Vec<_>>();
    assert_eq!(inputs.len(), 3);
    for (at, input) in inputs.iter().enumerate() {
        let QuestionInput::Record(record) = input else {
            panic!("original source")
        };
        assert_eq!(record.location().unwrap().first_line(), Some(at + 1));
        assert_eq!(record.location().unwrap().file(), "private.jsonl");
    }
}

#[cfg(test)]
fn assert_serialized_candidates(doc: &Value) {
    assert_eq!(doc["index"], 0);
    assert_eq!(doc["candidates"].as_array().unwrap().len(), 3);
    for (at, candidate) in doc["candidates"].as_array().unwrap().iter().enumerate() {
        assert_eq!(candidate["index"], at);
        assert_eq!(
            candidate["input"]["body"],
            if at == 2 { "Other." } else { "Same." }
        );
        assert_eq!(
            candidate["source"],
            json!({"file":"private.jsonl","first_line":at+1,"last_line":at+1})
        );
    }
    assert_eq!(doc["value"], json!({"z":false,"body":"Same.","a":[2,1]}));
}
