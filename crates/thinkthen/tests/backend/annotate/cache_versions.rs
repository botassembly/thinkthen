//! A cache that holds one version's answers beside another version's live
//! answer: model checks compare live replies only, by ADR 0111 section 4,
//! so an old stored answer never stops a record.

#![allow(
    clippy::indexing_slicing,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]
#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{Map, Value, json};

use super::set;
use crate::harness::{Canned, Listener, spawn};

/// A listener that answers every question of a request yes, naming the
/// first model for its first `first` requests and the second after.
fn versioned(first: usize, models: [&'static str; 2]) -> (Listener, Arc<AtomicUsize>) {
    let sent = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&sent);
    let listener = Listener::answering(move |body| {
        let model = models[usize::from(observed.fetch_add(1, Ordering::SeqCst) >= first)];
        let request: Value = serde_json::from_slice(body).expect("a request");
        let answers: Map<String, Value> = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":model,"answers":answers}).to_string())
    })
    .expect("listener");
    (listener, sent)
}

fn run(listener: &Listener, cache: &Path, name: &str, right: &str, model: &[&str]) -> Output {
    let file = set(
        name,
        &format!(
            r#"{{"version":1,"questions":{{"left_answer":{{"decide":"left?","on":"/left"}},"right_answer":{{"decide":"{right}","on":"/right"}}}}}}"#
        ),
    );
    let arguments = [
        "annotate",
        &file.to_string_lossy(),
        "--url",
        listener.base(),
        "--cache",
        cache.to_str().expect("a path"),
    ];
    spawn(
        &[&arguments[..], model].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"left":"yes","right":"yes"}"#,
    )
    .expect("the command runs")
}

fn questions(listener: &Listener) -> Vec<usize> {
    listener
        .requests()
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("a body");
            body["questions"].as_object().map_or(0, Map::len)
        })
        .collect()
}

/// A changed question sends only itself. Its live answer names a new
/// version beside the old version's stored answer, and the record prints.
#[test]
fn a_cached_answer_from_another_version_takes_no_part_in_the_model_check() {
    let (listener, _sent) = versioned(1, ["fake-1", "fake-2"]);
    let cache = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-cache-versions");
    let _absent = fs::remove_dir_all(&cache);
    let first = run(&listener, &cache, "cache-versions", "right?", &[]);
    assert_eq!(
        first.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let second = run(&listener, &cache, "cache-versions", "is it right?", &[]);
    assert_eq!(
        second.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(second.stderr.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&second.stdout),
        "{\"left\":\"yes\",\"right\":\"yes\",\"left_answer\":true,\"right_answer\":true}\n"
    );
    assert_eq!(
        questions(&listener),
        [2, 1],
        "only the changed question goes out"
    );
}

#[test]
fn a_mutable_alias_refreshes_old_groups_and_the_new_group_together() {
    let (listener, sent) = versioned(1, ["jev-1.13.0", "jev-1.14.0"]);
    let cache = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-alias-refresh");
    let _absent = fs::remove_dir_all(&cache);
    let alias = ["--model", "jev-latest"];
    let first = run(&listener, &cache, "alias-refresh", "right?", &alias);
    assert_eq!(first.status.code(), Some(0));
    let second = run(&listener, &cache, "alias-refresh", "is it right?", &alias);
    assert_eq!(
        second.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(!second.stdout.is_empty());
    assert_eq!(questions(&listener), [2, 2], "both questions refresh");
    assert_eq!(sent.load(Ordering::SeqCst), 2);
    assert_eq!(
        String::from_utf8_lossy(&second.stderr)
            .matches("sends each planned cache request live")
            .count(),
        1
    );
}
