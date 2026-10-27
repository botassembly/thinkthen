#![cfg(test)]
//! A stale pooled connection can be resent only before any HTTP reply byte.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use std::time::Duration;

use loopback::{CloseAt, ClosingPeer, KEY};
use thinkthen::{Decide, Key, Record, Retry, Settings};

#[test]
fn closed_pooled_connections_resend_once_only_before_reply_bytes() {
    // The peer waits for the next batch on a kept-alive socket before closing.
    // This forces the close past the pool's is_open probe without a timing race.
    for (at, expected, requests) in [
        (CloseAt::Reused, "answered", 3),
        (CloseAt::Headers, "failed", 2),
        (CloseAt::Body, "failed", 2),
        (CloseAt::Fresh, "failed", 1),
    ] {
        let peer = ClosingPeer::start(at).unwrap();
        let engine = Settings::default()
            .url(peer.base())
            .api_key(Key::new(KEY).unwrap())
            .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(5)).unwrap())
            .engine(|_| None)
            .unwrap();
        let question = Decide::new("Ready?").unwrap();
        let run = |text| {
            let rows: Vec<_> = engine
                .rows(&question, [Record::text(text)])
                .unwrap()
                .collect::<Result<_, _>>()
                .unwrap();
            assert_eq!(rows.len(), 1);
            serde_json::to_value(&rows[0]).unwrap()
        };
        if !matches!(at, CloseAt::Fresh) {
            assert_eq!(run("first")["status"], "answered");
        }
        let row = run("second");
        let received = peer.finish().unwrap();
        assert_eq!(row["status"], expected, "{at:?}");
        assert_eq!(received, requests, "{at:?}");
    }
}
