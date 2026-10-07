use super::{IMAJEV, build, folder, pair, question};
use conformance_backend::{Canned, Listener};
use serde_json::Value;

#[test]
fn native_partial_usage_recording_keeps_input_unknown_mass_and_absent_output_verbatim() {
    let exchange: Value = serde_json::from_str(include_str!(
        "../../../../../specification/fixtures/images/local/0036-imajev-partial.json"
    ))
    .unwrap();
    let original = exchange["response"].clone();
    let response = original.to_string();
    let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
    let place = folder();
    let engine = build(&listener, "llamacpp", "imajev-2b", IMAJEV)
        .record(&place)
        .unwrap()
        .build()
        .unwrap();
    let call = engine
        .details_input(&question(), &pair())
        .expect("native integration accepts original partial usage");
    assert_eq!(call.facts().input_tokens(), Some(887));
    assert_eq!(call.facts().output_tokens(), None);
    assert_eq!(
        call.value().reported_usage().unwrap().input_tokens(),
        Some(887)
    );
    assert_eq!(call.value().reported_usage().unwrap().output_tokens(), None);
    assert_eq!(listener.count(), 1);
    let database = rusqlite::Connection::open(place.join("thinkthen.sqlite")).unwrap();
    let response: Vec<u8> = database
        .query_row("SELECT response FROM exchanges", [], |row| row.get(0))
        .unwrap();
    let recorded: Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(recorded, original);
    let counts: (Option<i64>, Option<i64>) = database
        .query_row(
            "SELECT input_tokens, output_tokens FROM answers",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(counts, (Some(887), None));
    assert_eq!(
        call.value().probabilities(),
        &thinkthen::Probabilities::YesNo {
            yes: 0.6316676506859308
        }
    );
    assert_eq!(recorded["answers"]["q1"]["abstained"], false);
    let replay = build(&listener, "llamacpp", "imajev-2b", IMAJEV)
        .replay(&place)
        .unwrap()
        .build()
        .unwrap();
    let retained = replay.details_input(&question(), &pair()).unwrap();
    assert_eq!(
        retained.value().reported_usage(),
        call.value().reported_usage()
    );
    assert_eq!(listener.count(), 1);
    drop(replay);
    drop(database);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}
