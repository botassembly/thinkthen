//! A stored image answer is usable only with its original validated constituents.
use super::{BLUE, LIQUID_DECIDE, RED, folder, pair, question, url};
use conformance_backend::{Canned, Listener};
use rusqlite::Connection;
use sha2::{Digest as _, Sha256};
use thinkthen::{Engine, ErrorKind};

#[expect(
    clippy::expect_used,
    reason = "invalid fixture data URLs stop the corruption regression"
)]
fn corrupted_states(state: &str) -> [(&'static str, String); 7] {
    let red = url(RED, "image/png");
    let red = red.split_once(',').expect("fixture data URL").1;
    let blue = url(BLUE, "image/png");
    let blue = blue.split_once(',').expect("fixture data URL").1;
    let ordered = format!(
        r#""images":[{{"media":"image/png","base64":"{red}"}},{{"media":"image/png","base64":"{blue}"}},{{"media":"image/png","base64":"{red}"}}]"#
    );
    let reversed = format!(
        r#""images":[{{"media":"image/png","base64":"{blue}"}},{{"media":"image/png","base64":"{red}"}},{{"media":"image/png","base64":"{red}"}}]"#
    );
    assert!(state.contains(&ordered));
    [
        ("envelope", "PRIVATE_CORRUPT_ENVELOPE".to_owned()),
        (
            "version",
            state.replace("thinkthen.image-state/1", "thinkthen.image-state/999"),
        ),
        ("media", state.replacen("image/png", "image/jpeg", 1)),
        ("base64", state.replacen(red, "PRIVATE_CORRUPT_BASE64%", 1)),
        ("bytes", state.replacen(red, blue, 1)),
        ("order", state.replace(&ordered, &reversed)),
        ("duplicates", state.replace(&ordered, r#""images":[]"#)),
    ]
}

#[test]
fn sqlite_image_replay_refuses_corrupt_constituents_and_identity_without_sending() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let folder = folder();
    let build = || {
        Engine::builder()
            .backend("liquid")
            .unwrap()
            .base_url(listener.base())
            .unwrap()
            .model("d1")
            .unwrap()
            .no_cache()
    };
    let recorder = build().record(&folder).unwrap().build().unwrap();
    recorder.decide_input(&question(), &pair()).unwrap();
    drop(recorder);
    let database = folder.join("thinkthen.sqlite");
    let original = std::fs::read(&database).unwrap();
    let connection = Connection::open(&database).unwrap();
    let state: String = connection
        .query_row("SELECT state FROM states", [], |row| row.get(0))
        .unwrap();
    drop(connection);
    for (name, changed) in corrupted_states(&state) {
        let connection = Connection::open(&database).unwrap();
        // Also correct the state digest: retaining the answer key must still
        // refuse changed constituents rather than trusting a self-consistent state.
        let mut hash = Sha256::new();
        hash.update(b"thinkthen.image-state/1\n");
        hash.update(changed.as_bytes());
        let digest: [u8; 32] = hash.finalize().into();
        connection
            .execute(
                "UPDATE states SET state=?1, sha256=?2",
                (&changed, digest.as_slice()),
            )
            .unwrap();
        drop(connection);
        let replay = build().replay(&folder).unwrap().build().unwrap();
        let error = replay.decide_input(&question(), &pair()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Local, "{name}");
        assert_eq!(
            error.to_string(),
            "the cache or recording folder holds a malformed entry"
        );
        assert!(!format!("{error:?}").contains("PRIVATE_CORRUPT"));
        assert_eq!(listener.count(), 1, "{name}: replay sends nothing");
        drop(replay);
        std::fs::write(&database, &original).unwrap();
    }
    for change in [
        "UPDATE states SET sha256=zeroblob(32)",
        "UPDATE answers SET state=99999",
        "UPDATE answers SET url='http://127.0.0.1:1/v1/systemone'",
        "UPDATE answers SET model='d1:free'",
        "UPDATE answers SET question='{}'",
    ] {
        let connection = Connection::open(&database).unwrap();
        connection.execute_batch("PRAGMA foreign_keys=OFF").unwrap();
        connection.execute(change, []).unwrap();
        drop(connection);
        let replay = build().replay(&folder).unwrap().build().unwrap();
        let error = replay.decide_input(&question(), &pair()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Local, "{change}");
        assert_eq!(
            error.to_string(),
            "the cache or recording folder holds a malformed entry"
        );
        assert_eq!(listener.count(), 1, "replay sends nothing");
        drop(replay);
        std::fs::write(&database, &original).unwrap();
    }
    let replay = build().replay(&folder).unwrap().build().unwrap();
    replay.decide_input(&question(), &pair()).unwrap();
    assert_eq!(listener.count(), 1, "restored replay sends nothing");
    drop(replay);
    std::fs::remove_dir_all(folder).unwrap();
}
