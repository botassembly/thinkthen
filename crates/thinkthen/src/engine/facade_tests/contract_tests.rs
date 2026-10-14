//! The facade's contract checks against the shared cases and key secrecy.

use conformance_backend::{Canned, Listener};

use super::{Scratch, TEST_KEY, ask, decide, evidence, settings};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::facade::{Engine, Settings, Storage};

#[test]
fn no_key_reaches_a_result_an_error_a_recording_or_a_count() {
    let listener = Listener::serving(vec![
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#),
        Canned::close_without_reply(),
    ])
    .expect("listener");
    let folder = Scratch::new("secrecy");
    let engine = Engine::new(Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(listener.base())
    })
    .expect("engine");
    let cancel = Cancel::default();

    let judged = engine
        .judge(&decide("Refund?"), None, evidence("Refund me."), &cancel)
        .expect("judged");
    let failed = ask(&engine, "broken", &cancel).expect_err("closed");
    let refused = Engine::new(Settings {
        key: std::sync::Arc::new(|| Err(Error::NoKey("THINKTHEN_API_KEY".to_owned()))),
        // Loopback takes a request with no key, so this engine names an
        // address the rules cannot prove is this machine.
        ..settings("https://127.0.0.2:9/v1")
    })
    .and_then(|engine| ask(&engine, "no key here", &cancel))
    .expect_err("no key");
    let seen = [
        format!("{:?} {:?}", judged.answer, judged.answered.reply),
        format!("{failed:?}"),
        format!("{refused:?}"),
        format!("{:?}", engine.usage()),
        folder.files().concat(),
    ];

    assert!(listener.requests().iter().all(|request| {
        request
            .header("authorization")
            .is_some_and(|value| value.contains(TEST_KEY))
    }));
    for text in seen {
        assert!(!text.contains(TEST_KEY), "{text}");
        assert!(!text.contains("sk-facade"), "{text}");
    }
}

/// Answers from two addresses never mix in one folder, because the address
/// is part of every question key (ADR 0111 section 3). A folder asked at
/// another address misses and sends, and the first address still answers
/// from the folder.
#[test]
fn a_folder_asked_at_another_address_misses_and_keeps_both_answers() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let folder = Scratch::new("other-backend");
    let caching = |base: &str| Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            replay: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(base)
    };
    let cancel = Cancel::default();
    let first = Engine::new(caching(listener.base())).expect("engine");
    let other = Engine::new(caching(&format!("{}/other", listener.base()))).expect("engine");

    ask(&first, "Refund me.", &cancel).expect("the first address");
    ask(&other, "Refund me.", &cancel).expect("another address misses and sends");
    ask(&other, "Refund me.", &cancel).expect("its own answer is stored");
    ask(&first, "Refund me.", &cancel).expect("the first answer stays");

    assert_eq!(listener.count(), 2, "each address sent once");
}
