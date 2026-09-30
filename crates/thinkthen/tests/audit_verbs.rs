//! `thinkthen audit` grades every verb it reads, from hand fixtures and from
//! real saved output.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

#[path = "support/measure.rs"]
mod measure_support;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use measure_support::{audit, found, member, ranked, replay, repository, with_ids};

/// Each hand fixture under `verbs/`, the row it checks, and each member's hand-computed value.
/// `tests/fixtures/measure/README.md` shows the arithmetic.
const ROWS: [&str; 13] = [
    r#"tag 0 {"/group": "Which apply?", "/rows": 12, "/labeled": 10, "/right": 8, "/wrong": 2, "/precision": 0.833333, "/yes_recall": 0.833333, "/f1": 0.833333, "/interval": null, "/calibration": null, "/coverage": null, "/auc": null, "/mean_probability": null, "/suggested/crossed": null}"#,
    r#"tag 1 {"/group": "Which apply?/x", "/labeled": 5, "/right": 4, "/false_no": 1, "/precision": 1.0, "/yes_recall": 0.75, "/f1": 0.857143, "/suggested/crossed/cuts": [0.4, 0.5]}"#,
    r#"tag 2 {"/group": "Which apply?/y", "/labeled": 5, "/right": 4, "/false_yes": 1, "/precision": 0.666667, "/yes_recall": 1.0, "/f1": 0.8, "/suggested/crossed/cuts": [0.5, 0.61]}"#,
    r#"score 0 {"/right": 4, "/wrong": 2, "/mean_level_distance": 0.333333, "/disagreements": [{"key": "low", "said": "mid", "count": 1}, {"key": "mid", "said": "high", "count": 1}], "/suggested/cut": null, "/suggested/cuts": [0.71, 1.61], "/suggested/tune/at_cut/right": 6}"#,
    r#"rank 0 {"/verb": "rank", "/labeled": 6, "/unsure": 6, "/r_precision": 0.5}"#,
    r#"rank 1 {"/labeled": 3, "/r_precision": 1.0}"#,
    r#"rank 2 {"/labeled": 2, "/r_precision": null}"#,
    r#"rank 3 {"/labeled": 0, "/r_precision": null}"#,
    r#"rank 4 {"/labeled": 3, "/r_precision": 0.5, "/suggested": null, "/calibration": null}"#,
    r#"find 0 {"/rows": 5, "/right": 3, "/wrong": 1, "/tied": 1, "/tied_holding_key": 1, "/tie_share": 0.5, "/disagreements": [{"key": "u001", "said": "u003", "count": 1}], "/suggested": null}"#,
    r#"annotate 0 {"/group": "urgent", "/right": 1, "/false_yes": 1, "/precision": 0.5}"#,
    r#"annotate 1 --optimize precision {"/verb": "choose", "/suggested": {"cut": null, "objective": "precision does not apply to choose", "split": "seeded", "seed": 0}}"#,
    r#"annotate 2 {"/group": "effort", "/verb": "score", "/right": 2, "/mean_level_distance": 0.333333}"#,
];

/// Every verb in the table of `specification/audit.md` grades by its rule.
#[test]
fn each_verb_grades() {
    for case in ROWS {
        let (name, rest) = case.split_once(' ').expect("a name");
        let (place, rest) = rest.split_once(' ').expect("a place");
        let (options, expected) = rest.split_at(rest.find('{').expect("the expected members"));
        let (results, key) = (
            format!("verbs/{name}.jsonl"),
            format!("verbs/{name}-key.jsonl"),
        );
        let arguments = [
            &[results.as_str(), &key][..],
            &options.split_whitespace().collect::<Vec<_>>(),
        ]
        .concat();
        let (code, stdout, stderr) = audit(&arguments, b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{name}");
        let row = stdout
            .lines()
            .nth(place.parse().expect("a place"))
            .expect("the row");
        let expected: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(expected).expect("JSON");
        for (pointer, value) in expected {
            assert_eq!(
                member(row, &pointer),
                value.to_string(),
                "{name} {place} {pointer}"
            );
        }
    }
    let arguments = "verbs/annotate.jsonl verbs/annotate-key.jsonl --optimize precision --table";
    let (_, table, _) = audit(&arguments.split(' ').collect::<Vec<_>>(), b"");
    let line = "  suggested cut: none; precision does not apply to choose";
    assert!(table.lines().any(|held| held == line), "{table}");
}

/// Real saved `tag`, `score`, `find`, and `rank` output reads as the verb it is.
#[test]
fn real_output_reads() {
    let demos = repository().join("demos");
    let options = "tag @hazards.json --details --replay recording --input message.txt";
    let arguments: Vec<&str> = options.split(' ').collect();
    let tag = replay(&demos.join("39-screen-a-message"), &arguments, b"");
    let levels = [
        "A canned reply answers it.",
        "One person can answer it after a look at the account.",
        "It needs a specialist and more than one system.",
    ];
    let mut score = String::new();
    for request in ["password", "receipt", "tax-split", "two-seats"] {
        let text = std::fs::read(demos.join(format!("17-rate-and-sort/requests/{request}.txt")))
            .expect("a request");
        let arguments = [
            &["score", "How hard is this request to answer?"],
            &levels[..],
            &["--details", "--replay", "recording"],
        ]
        .concat();
        score.push_str(&replay(&demos.join("17-rate-and-sort"), &arguments, &text));
    }
    let find = found();
    let rank = ranked("The passage answers the query.");
    let (one, two) = (levels[1], levels[2]);
    let cases = [
        (
            with_ids(&tag),
            r#"{"id":1,"value":["urgent"]}"#.to_owned(),
            "/id",
            ["\"tag\"", "3", "3", "3"],
        ),
        (
            with_ids(&score),
            format!(
                "{{\"id\":1,\"value\":\"{one}\"}}\n{{\"id\":2,\"value\":\"{}\"}}\n{{\"id\":3,\"value\":\"{two}\"}}\n{{\"id\":4,\"value\":\"{one}\"}}",
                levels[0]
            ),
            "/id",
            ["\"score\"", "4", "4", "3"],
        ),
        (
            find,
            r#"{"id":1,"value":"u007"}"#.to_owned(),
            "/id",
            ["\"find\"", "1", "1", "1"],
        ),
        (
            rank,
            ["database.md", "login.md"]
                .map(|path| format!("{{\"id\":\"runbooks/{path}\",\"value\":\"yes\"}}\n"))
                .concat()
                + r#"{"id":"notes/2025-11-outage.md","value":"yes"}"#,
            "/path",
            ["\"rank\"", "6", "3", "0"],
        ),
    ];
    let folder = std::env::temp_dir().join(format!("thinkthen-0125-real-{}", std::process::id()));
    std::fs::create_dir_all(&folder).expect("a folder");
    for (results, key, pointer, expected) in cases {
        let path = folder.join("key.jsonl");
        std::fs::write(&path, key).expect("a key");
        let key = path.to_str().expect("a path");
        let (code, stdout, stderr) = audit(&["-", key, "--id", pointer], results.as_bytes());
        assert_eq!((code, stderr.as_str()), (0, ""), "{expected:?}");
        let row = stdout.lines().next().expect("a row");
        let found = ["/verb", "/rows", "/labeled", "/right"].map(|p| member(row, p));
        assert_eq!(found, expected.map(str::to_owned));
    }
    std::fs::remove_dir_all(&folder).expect("cleanup");
}
