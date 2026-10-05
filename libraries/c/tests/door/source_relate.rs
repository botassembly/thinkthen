//! Complete source relation sets have bounds on rows, evidence and expanded output.
use super::sources::ask_base;
use crate::scratch;
use conformance_backend::{Backend, Canned, Listener};
use serde_json::{Value, json};
use std::fs;

fn request(paths: &[&std::path::Path], unit: &str) -> Value {
    json!({"version":1,"relate":{"relations":[{"name":"met","source":"*","target":"*"}]},"source":{"paths":paths,"unit":unit}})
}

fn yes(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).expect("request");
    let answers = body["questions"]
        .as_object()
        .expect("questions")
        .keys()
        .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"jev-latest","answers":answers,"usage":{"input_tokens":1,"output_tokens":1}}).to_string())
}

#[test]
fn source_relation_admission_refuses_duplicate_rows_and_aggregate_evidence_without_sending() {
    let backend = Backend::start().unwrap();
    let base = format!("{}/generic/v1", backend.origin());
    let place = scratch("source-relate-bounds");
    let path = place.join("names.txt");
    let many = place.join("many.txt");
    let first = place.join("first.txt");
    let second = place.join("second.txt");
    let tail = place.join("tail.txt");
    let mut names = format!("{}Bea\n", "Ada\n".repeat(255)).into_bytes();
    names.push(0xff);
    fs::write(&path, names).unwrap();
    let mut names = format!("{}{}", "Ada\n".repeat(10_000), "Bea\n".repeat(10_000)).into_bytes();
    names.push(0xff);
    fs::write(&many, names).unwrap();
    fs::write(&first, "x".repeat(8 * 1024 * 1024)).unwrap();
    fs::write(&second, "y".repeat(8 * 1024 * 1024 + 1)).unwrap();
    fs::write(&tail, [0xff]).unwrap();
    let replies = ask_base(
        &base,
        &[
            request(&[&path], "line"),
            request(&[&many], "line"),
            request(&[&first, &second, &tail], "file"),
        ],
        false,
    );
    for ((code, reply), message) in replies.iter().zip([
        "source relate takes at most 255 source records",
        "source relate takes at most 255 source records",
        "source relate input exceeds 16 MiB",
    ]) {
        assert_eq!(*code, 1, "{reply}");
        assert_eq!(reply["message"], message);
        assert!(reply.get("value").is_none());
        assert!(reply["facts"].is_null());
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn source_relation_expansion_refuses_excess_without_a_partial_value_and_keeps_call_facts() {
    let place = scratch("source-relate-expansion");
    let path = place.join("names.txt");
    let suffix = "\u{0001}".repeat(200);
    let names = format!(
        "{}{}",
        format!("PRIVATE_INPUT_MARKER-Ada{suffix}\n").repeat(65),
        format!("Bea{suffix}\n").repeat(65)
    );
    fs::write(&path, names).unwrap();
    let listener = Listener::answering(yes).unwrap();
    let replies = ask_base(listener.base(), &[request(&[&path], "line")], false);
    let (code, reply) = &replies[0];
    assert_eq!(*code, 1, "{reply}");
    assert_eq!(reply["message"], "source relate output exceeds 16 MiB");
    assert!(reply.get("value").is_none());
    assert!(listener.count() > 0);
    assert_eq!(reply["facts"]["requests_sent"], listener.count());
}

#[test]
fn source_relation_admits_all_255_rows_before_judging_the_distinct_set() {
    let place = scratch("source-relate-boundary");
    let path = place.join("names.txt");
    fs::write(&path, format!("{}Bea\n", "Ada\n".repeat(254))).unwrap();
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        let answers = body["questions"].as_object().unwrap().keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.1})))
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&json!({"model":"jev-latest","answers":answers,"usage":{"input_tokens":1,"output_tokens":1}}).to_string())
    }).unwrap();
    let replies = ask_base(listener.base(), &[request(&[&path], "line")], false);
    assert_eq!(replies[0].0, 0);
    assert_eq!(replies[0].1["value"]["edges"], json!([]));
    assert!(listener.count() > 0);
    assert_eq!(replies[0].1["facts"]["requests_sent"], listener.count());
}
