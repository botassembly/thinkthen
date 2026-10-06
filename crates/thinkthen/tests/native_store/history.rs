use super::*;

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
    let first = engine.details(&question, "Refund me.").unwrap();
    let second = engine.details(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 2);
    assert_ne!(first.value().observations(), second.value().observations());
    assert_ne!(first.value().requests(), second.value().requests());
    assert_eq!(
        first.value().question_sources()[0].answered_by(),
        "model-one"
    );
    assert_eq!(
        second.value().question_sources()[0].answered_by(),
        "model-two"
    );
    let replay = build(&listener).replay(&place).unwrap().build().unwrap();
    let error = replay.details(&question, "Refund me.").unwrap_err();
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
fn conflicting_url_normalization_rolls_back_the_whole_original_store_without_sends() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let url = format!("{}/systemone", listener.base());
    legacy(&place, &url, "fixed");
    let uppercase = url.replacen("http://", "HTTP://", 1);
    let key = Sha256::digest(format!(
        "systemone\n{uppercase}\n\"fixed\"\n{STATE}\n{QUESTION}"
    ));
    let database = place.join("thinkthen.sqlite");
    let db = Connection::open(&database).unwrap();
    db.execute("INSERT INTO answers SELECT 2,?1,?2,model,state,question,answer,answered_by,input_tokens,output_tokens,taken_at,origin FROM answers WHERE id=1",(key.as_slice(),uppercase)).unwrap();
    drop(db);
    let original = std::fs::read(&database).unwrap();
    let engine = build(&listener).cache_at(&place).unwrap().build().unwrap();
    let error = engine
        .details(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(listener.count(), 0);
    assert_eq!(std::fs::read(&database).unwrap(), original);
    let db = Connection::open(&database).unwrap();
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        1
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
    let rows = engine
        .details_many_with(
            &question,
            ["Refund me.", "Refund me."],
            CallOptions::new().batch(BatchSetting::Max),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        rows[0].value().observations(),
        rows[1].value().observations()
    );
    assert_eq!(rows[0].value().question_sources()[0].batch_size(), Some(1));
    assert_eq!(rows[1].value().question_sources()[0].batch_size(), Some(1));
    let refresh = build(&listener)
        .cache_at(&place)
        .unwrap()
        .refresh_cache(true)
        .build()
        .unwrap();
    let renewed = refresh.details(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 2);
    assert_ne!(
        renewed.value().observations(),
        rows[0].value().observations()
    );
    assert_eq!(renewed.value().requests(), rows[0].value().requests());
    let retained = engine.details(&question, "Refund me.").unwrap();
    assert_eq!(listener.count(), 2);
    assert_eq!(
        retained.value().observations(),
        renewed.value().observations()
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
        .details(&Question::decide("Refund?").unwrap().cut(), "Refund me.")
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
