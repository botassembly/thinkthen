//! A cache that holds one version's answers beside another version's live
//! answer stops the record and names the cache (ticket 0159).

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::PathBuf;
use std::process::Output;

use super::set;
use crate::harness::{Canned, Listener, spawn};

fn reply(model: &str) -> String {
    format!(r#"{{"model":"{model}","answers":{{"q1":{{"type":"noul","noul":0.9}}}}}}"#)
}

#[test]
fn a_cache_that_mixes_versions_stops_the_record() {
    // One listener serves both runs, because a cache folder is bound to its
    // endpoint URL, port included. It answers one reply per connection, in order.
    let replies = [reply("fake-1"), reply("fake-1"), reply("fake-2")];
    let listener =
        Listener::serving(replies.iter().map(|body| Canned::ok(body)).collect()).expect("listener");
    let cache = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-cache-versions");
    let _absent = fs::remove_dir_all(&cache);
    let run = |right: &str| -> Output {
        let file = set(
            "cache-versions",
            &format!(
                r#"{{"version":1,"questions":{{"left_answer":{{"decide":"left?","on":"/left"}},"right_answer":{{"decide":"{right}","on":"/right"}}}}}}"#
            ),
        );
        let arguments = [
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
        ];
        let cache = ["--cache", cache.to_str().expect("a path")];
        spawn(
            &[&arguments[..], &cache[..]].concat(),
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            br#"{"left":"yes","right":"yes"}"#,
        )
        .expect("the command runs")
    };
    let first = run("right?");
    assert_eq!(
        first.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let second = run("is it right?");
    assert_eq!(second.status.code(), Some(4));
    assert_eq!(
        String::from_utf8_lossy(&second.stderr),
        concat!(
            "thinkthen: the replies for one record named different model versions; ",
            "a cache or recording folder may hold answers from the other version, so rerun with ",
            "--no-cache or prune it with thinkthen cache prune DIR --answered-by-other-than VERSION, ",
            "naming the version a --no-cache run returns\n",
        )
    );
    assert!(second.stdout.is_empty());
    assert_eq!(listener.requests().len(), 3);
}
