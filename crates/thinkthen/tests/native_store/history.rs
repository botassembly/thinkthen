use super::*;

#[test]
fn valid_v2_without_lookup_index_replays_unchanged_and_writer_indexes_retained_answers() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let question = Question::decide("Refund?").unwrap().cut();
    let engine = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let live = engine.decide(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 1);
    drop(engine);
    let database = place.join("thinkthen.sqlite");
    let db = Connection::open(&database).unwrap();
    db.execute_batch("DROP INDEX IF EXISTS answers_route_question_state")
        .unwrap();
    let retained = stored_metadata(&db);
    let saved_answer: String = db
        .query_row("SELECT answer FROM answers", [], |row| row.get(0))
        .unwrap();
    drop(db);
    let original = std::fs::read(&database).unwrap();
    let modified = std::fs::metadata(&database).unwrap().modified().unwrap();
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let historical = replay.decide(&question, "Refund me.").unwrap();
    assert_eq!(historical.value(), live.value());
    assert_eq!(
        historical.results()[0].identity().observations(),
        live.results()[0].identity().observations()
    );
    assert_eq!(
        historical.results()[0].meta().requests(),
        live.results()[0].meta().requests()
    );
    assert_eq!(
        historical.results()[0].identity().question_sources()[0].origin(),
        Origin::Replay
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(std::fs::read(&database).unwrap(), original);
    assert_eq!(
        std::fs::metadata(&database).unwrap().modified().unwrap(),
        modified
    );
    drop(replay);
    let writer = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let held = writer.decide(&question, "Refund me.").unwrap();
    assert_eq!(held.value(), live.value());
    assert_eq!(
        held.results()[0].identity().observations(),
        live.results()[0].identity().observations()
    );
    assert_eq!(
        held.results()[0].meta().requests(),
        live.results()[0].meta().requests()
    );
    assert_eq!(
        held.results()[0].identity().question_sources()[0].origin(),
        Origin::Cache
    );
    assert_eq!(listener.count(), 1);
    let db = Connection::open(&database).unwrap();
    let columns: Vec<String> = db
        .prepare("PRAGMA index_info(answers_route_question_state)")
        .unwrap()
        .query_map([], |row| row.get(2))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(columns, ["url", "model", "question", "state"]);
    assert_eq!(stored_metadata(&db), retained);
    assert_eq!(
        db.query_row("SELECT answer FROM answers", [], |row| row
            .get::<_, String>(0))
            .unwrap(),
        saved_answer
    );
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        2
    );
    drop(db);
    drop(writer);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn differing_reported_models_never_reuse_online_and_ambiguous_history_refuses_offline() {
    let responses = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        let model = if responses.fetch_add(1, Ordering::Relaxed) == 0 {
            "model-one"
        } else {
            "model-two"
        };
        Canned::ok(&REPLY.replace("fixed", model))
    })
    .unwrap();
    let place = folder();
    let engine = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let first = engine.decide(&question, "Refund me.").unwrap();
    let second = engine.decide(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 2);
    assert_ne!(
        first.results()[0].identity().observations(),
        second.results()[0].identity().observations()
    );
    assert_ne!(
        first.results()[0].meta().requests(),
        second.results()[0].meta().requests()
    );
    assert_eq!(
        first.results()[0].identity().question_sources()[0].answered_by(),
        "model-one"
    );
    assert_eq!(
        second.results()[0].identity().question_sources()[0].answered_by(),
        "model-two"
    );
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let error = replay.decide(&question, "Refund me.").unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(
        error.to_string(),
        "the cache or recording folder holds a malformed entry"
    );
    assert_eq!(listener.count(), 2);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn conflicting_canonical_url_identities_refuse_without_mutation_or_sends() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let url = format!("{}/systemone", listener.base());
    let seed = build(&listener).record(&place).unwrap().build().unwrap();
    seed.decide(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
        .unwrap();
    drop(seed);
    let uppercase = url.replacen("http://", "HTTP://", 1);
    let key = framed(
        "thinkthen.question-key/2",
        &[
            "systemone",
            &uppercase,
            "\"fixed\"",
            "\"fixed\"",
            STATE,
            QUESTION,
        ],
    );
    let bytes: Vec<u8> = (0..key.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&key[at..at + 2], 16).unwrap())
        .collect();
    let database = place.join("thinkthen.sqlite");
    let db = Connection::open(&database).unwrap();
    db.execute("INSERT INTO answers SELECT 2,?1,?2,model,state,question,answer,answered_by,input_tokens,output_tokens,taken_at,origin,key_version,adapter,observation_id,batch_size FROM answers WHERE id=1",(&bytes,uppercase)).unwrap();
    drop(db);
    let original = std::fs::read(&database).unwrap();
    let engine = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let error = engine
        .decide(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(listener.count(), 1);
    assert_eq!(std::fs::read(&database).unwrap(), original);
    let db = Connection::open(&database).unwrap();
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        2
    );
    assert_eq!(
        db.query_row("SELECT count(*) FROM answers", [], |row| row
            .get::<_, u32>(0))
            .unwrap(),
        2
    );
    drop(db);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn coalesced_occurrences_share_one_observation_and_equal_refresh_creates_a_new_one() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let question = Question::decide("Refund?").unwrap().cut();
    let engine = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let rows = crate::api::pull_decisions(
        &engine,
        &question,
        crate::api::plain_records(["Refund me.", "Refund me."]),
        CallOptions::new().batch(BatchSetting::Max),
    )
    .collect::<Result<Vec<_>, _>>()
    .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        rows[0].result().identity().observations(),
        rows[1].result().identity().observations()
    );
    assert_eq!(
        rows[0].result().identity().question_sources()[0].batch_size(),
        Some(1)
    );
    assert_eq!(
        rows[1].result().identity().question_sources()[0].batch_size(),
        Some(1)
    );
    let refresh = build(&listener)
        .cache_at(&place)
        .unwrap()
        .refresh_cache(true)
        .build()
        .unwrap();
    let renewed = refresh.decide(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 2);
    assert_ne!(
        renewed.results()[0].identity().observations(),
        rows[0].result().identity().observations()
    );
    assert_eq!(
        renewed.results()[0].meta().requests(),
        rows[0].result().meta().requests()
    );
    let retained = engine.decide(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 2);
    assert_eq!(
        retained.results()[0].identity().observations(),
        renewed.results()[0].identity().observations()
    );
    drop(refresh);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn an_unrepresentable_reported_count_refuses_storage_without_erasing_the_known_live_count() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":18446744073709551615}}"#)).unwrap();
    let place = folder();
    let engine = build(&listener).record(&place).unwrap().build().unwrap();
    let error = engine
        .decide(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(listener.count(), 1);
    assert_eq!(error.facts().unwrap().input_tokens(), Some(u64::MAX));
    assert_eq!(error.facts().unwrap().output_tokens(), None);
    let database = Connection::open(place.join("thinkthen.sqlite")).unwrap();
    assert_eq!(
        database
            .query_row("SELECT count(*) FROM answers", [], |row| row
                .get::<_, u32>(0))
            .unwrap(),
        0
    );
    assert_eq!(
        database
            .query_row("SELECT count(*) FROM states", [], |row| row
                .get::<_, u32>(0))
            .unwrap(),
        0
    );
    drop(database);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}
