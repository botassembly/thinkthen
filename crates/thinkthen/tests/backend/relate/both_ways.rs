//! A both-ways edge carries `"either":true` last, and a directed edge carries
//! no `either` member (ticket 0344). The request bytes and the question
//! digest stay as main printed them before the change, so cache keys stay.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use super::{answered, run};
use crate::harness::Listener;

const MIXED: &str = r#"{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"},{"name":"partners","source":"organization","target":"person","either":true}]}}"#;

/// Acme comes after Ada, and the both-ways rule names the organization first.
const NAMES: &[u8] = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;

const BARE: &str = concat!(
    r#"{"relation":"works_for","source":{"name":"Ada","kind":"person"},"#,
    r#""target":{"name":"Acme","kind":"organization"},"probability":0.9}"#,
    "\n",
    r#"{"relation":"partners","source":{"name":"Ada","kind":"person"},"#,
    r#""target":{"name":"Acme","kind":"organization"},"probability":0.9,"either":true}"#,
    "\n",
);

/// The request body main sent before ticket 0344.
const BODY: &str = concat!(
    r#"{"state":{"entities":[{"id":"i1","name":"Ada","kind":"person"},"#,
    r#"{"id":"i2","name":"Acme","kind":"organization"}]},"model":"local-1","#,
    r#""questions":{"q1":{"type":"noul","instructions":"Is it true that i1 works for i2?"},"#,
    r#""q2":{"type":"noul","instructions":"Is it true that i1 partners i2, or that i2 partners i1?"}}}"#,
);

fn mixed_file() -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-both-ways");
    fs::create_dir_all(&folder).expect("folder");
    let path = folder.join("mixed.json");
    fs::write(&path, MIXED).expect("relate file");
    format!("@{}", path.display())
}

#[test]
fn a_both_ways_edge_says_so_in_input_order_and_keeps_its_question_keys() {
    let listener = Listener::answering(answered).expect("listener");
    let rules = mixed_file();
    let bare = run(&listener, &[&rules], NAMES);
    assert_eq!(bare.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&bare.stdout), BARE);
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    assert_eq!(String::from_utf8_lossy(&sent[0].body), BODY);
    let details = run(&listener, &[&rules, "--details"], NAMES);
    assert_eq!(details.status.code(), Some(0));
    let result: Value = serde_json::from_slice(&details.stdout).expect("details");
    let edges: Vec<Value> = BARE
        .lines()
        .map(|line| serde_json::from_str(line).expect("edge"))
        .collect();
    assert_eq!(result["value"], Value::Array(edges));
    assert_eq!(
        result["meta"]["question_sha256"],
        "0d69c6b49f18d19ab118eb2ffc8fb5a43e538e1b52148d1f81450d060cb75591"
    );
}
