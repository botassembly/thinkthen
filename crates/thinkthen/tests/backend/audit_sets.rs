//! `thinkthen audit` grades `recognize` names and `relate` edges by precision,
//! recall, and F1, tunes their cut from the run's cut up, and refuses a run
//! with no label. `tests/fixtures/measure/README.md`, "Ticket 0135 fixtures",
//! works each value by hand.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

use crate::measure_support;

use std::fs;
use std::path::{Path, PathBuf};

use measure_support::{audit, member, replay, repository};

/// Demo 44's four described kinds, as the demo passes them.
const KINDS: [&str; 4] = [
    "PER=Part of a person's name.",
    "ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.",
    "LOC=Part of the name of a place: a country, region, city, or geographic feature.",
    "MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.",
];

/// Demo 44's sentence as two records.
const RECORDS: &str = "{\"id\":1,\"text\":\"Maria Chen joined Northwind Freight in Chicago last spring.\"}\n{\"id\":2,\"text\":\"Maria Chen joined Northwind Freight in Chicago last spring.\"}\n";

/// The key for one demo 44 record, without Northwind Freight, its weakest name, when `whole` is false.
fn names_key(id: usize, whole: bool) -> String {
    let organization = if whole {
        ",{\"kind\":\"ORG\",\"start\":18,\"end\":35}"
    } else {
        ""
    };
    format!(
        "{{\"id\":{id},\"value\":{{\"entities\":[{{\"kind\":\"PER\",\"start\":0,\"end\":10}}{organization},{{\"kind\":\"LOC\",\"start\":39,\"end\":46}}]}}}}\n"
    )
}

/// The key for one demo 45 run: gateway calls billing.
fn edge_key(id: usize) -> String {
    format!(
        "{{\"id\":{id},\"value\":[{{\"relation\":\"calls\",\"source\":{{\"name\":\"gateway\",\"kind\":\"service\"}},\"target\":{{\"name\":\"billing\",\"kind\":\"service\"}}}}]}}\n"
    )
}

/// Replay demo 44 over the records in record mode, from inline kinds or `@FILE`.
fn recognized(question: &[&str], records: &str) -> String {
    let mut arguments = vec!["recognize"];
    arguments.extend_from_slice(question);
    arguments.extend("--replay controlled-0461 --details --jsonl --field /text".split(' '));
    let demo = repository().join("demos/44-recognize-names");
    let url = fs::read_to_string(demo.join("controlled-0461/url.txt")).expect("controlled URL");
    arguments.extend(["--url", url.trim()]);
    replay(&demo, &arguments, records.as_bytes())
}

/// Replay demo 45 once, from its own file or a named one.
fn related(file: &str) -> String {
    let arguments = [
        "relate",
        file,
        "--url",
        "https://api.typesafe.ai/v1",
        "--replay",
        "recording",
        "--details",
    ];
    let demo = repository().join("demos/45-map-relationships");
    let entities = fs::read(demo.join("entities.json")).expect("entities");
    replay(&demo, &arguments, &entities)
}

/// A scratch folder for one test.
fn scratch(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("thinkthen-0135-{name}-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&folder);
    fs::create_dir(&folder).expect("a folder");
    folder
}

fn write(folder: &Path, name: &str, text: &str) -> String {
    let path = folder.join(name);
    fs::write(&path, text).expect("a file");
    path.to_string_lossy().into_owned()
}

/// Check each named member of the first row.
fn row_has(arguments: &[&str], input: &str, expected: &[(&str, &str)]) {
    let (code, stdout, stderr) = audit(arguments, input.as_bytes());
    assert_eq!((code, stderr.as_str()), (0, ""), "{arguments:?}");
    let row = stdout.lines().next().expect("a row");
    for (pointer, value) in expected {
        assert_eq!(&member(row, pointer), value, "{arguments:?} {pointer}");
    }
}

#[test]
fn names_match_strictly_or_by_overlap() {
    let nulls = [
        ("/true_no", "null"),
        ("/agreement", "null"),
        ("/calibration", "null"),
    ];
    let strict = [
        ("/right", "2"),
        ("/false_yes", "8"),
        ("/false_no", "10"),
        ("/precision", "0.2"),
        ("/yes_recall", "0.166667"),
        ("/f1", "0.181818"),
    ];
    let overlap = [
        ("/right", "6"),
        ("/false_yes", "4"),
        ("/false_no", "6"),
        ("/precision", "0.6"),
        ("/yes_recall", "0.5"),
        ("/f1", "0.545455"),
    ];
    let files = ["sets/names.jsonl", "sets/names-key.jsonl"];
    row_has(&files, "", &[&strict[..], &nulls].concat());
    row_has(
        &[&files[..], &["--match", "overlap"]].concat(),
        "",
        &overlap,
    );
}

#[test]
fn edges_match_by_direction() {
    let expected = [
        ("/rows", "2"),
        ("/failed", "1"),
        ("/labeled", "2"),
        ("/right", "3"),
        ("/false_yes", "1"),
        ("/false_no", "1"),
        ("/f1", "0.75"),
    ];
    row_has(&["sets/edges.jsonl", "sets/edges-key.jsonl"], "", &expected);
}

#[test]
fn cuts_start_at_the_run_cut() {
    let objective = ("/suggested/objective", "\"most f1 on the tuning part\"");
    let edges = ["sets/edges.jsonl", "sets/edges-key.jsonl"];
    row_has(&edges, "", &[("/suggested/cut", "0.6"), objective]);
    let names = ["sets/cut.jsonl", "sets/cut-key.jsonl"];
    row_has(&names, "", &[("/suggested/cut", "0.5"), objective]);
}

#[test]
fn edge_cases() {
    let question = |cut: &str| {
        format!(
            "\"question\":{{\"verb\":\"recognize\",\"kinds\":{{\"PER\":\"A person.\"}},\"threshold\":{cut}}}"
        )
    };
    let said = |id: &str, cut: &str| {
        format!(
            "{{\"input\":{{\"id\":\"{id}\"}},\"value\":{{\"entities\":[{{\"kind\":\"PER\",\"start\":0,\"end\":1,\"strength\":1}}]}},{}}}\n",
            question(cut)
        )
    };
    let key = |id: &str, kind: &str| {
        format!(
            "{{\"id\":\"{id}\",\"value\":{{\"entities\":[{{\"kind\":\"{kind}\",\"start\":0,\"end\":1}}]}},\"part\":\"tune\"}}\n"
        )
    };
    let folder = scratch("edges");
    let empty = write(
        &folder,
        "empty.jsonl",
        "{\"id\":\"a\",\"value\":{\"entities\":[]}}\n",
    );
    let empty_row = [
        ("/false_yes", "1"),
        ("/precision", "0.0"),
        ("/yes_recall", "null"),
    ];
    row_has(&["-", &empty], &said("a", "0.3"), &empty_row);
    let whole = write(
        &folder,
        "whole.jsonl",
        &(key("a", "PER") + &key("b", "PER")),
    );
    let top = said("a", "1") + &said("b", "1");
    row_has(
        &["-", &whole],
        &top,
        &[("/right", "2"), ("/suggested/cut", "null")],
    );
    let failed = "{\"input\":{\"id\":\"a\"},\"failure\":{\"kind\":\"backend\"}}\n";
    row_has(
        &["-", &whole],
        failed,
        &[("/failed", "1"), ("/labeled", "0")],
    );
    let uncut = "{\"input\":{\"id\":\"a\"},\"value\":{\"entities\":[]},\"question\":{\"verb\":\"recognize\",\"kinds\":{\"PER\":\"A person.\"}}}\n";
    let unknown = write(&folder, "unknown.jsonl", &key("a", "PERSON"));
    let refused = [
        (
            uncut.to_owned(),
            whole.clone(),
            "results line 1 holds an answer audit cannot grade; audit grades decide, filter, choose, tag, score, rank, find, recognize, and relate",
        ),
        (
            said("a", "0.3"),
            unknown,
            "key line 1 names a level, label, or unit the question does not have",
        ),
    ];
    for (input, key, sentence) in refused {
        let (code, stdout, stderr) = audit(&["-", &key], input.as_bytes());
        let expected = format!("thinkthen: audit: {sentence}\n");
        assert_eq!(
            (code, stdout.as_str(), stderr.as_str()),
            (2, "", expected.as_str())
        );
    }
    fs::remove_dir_all(folder).expect("the scratch folder goes");
}

#[test]
fn demos_grade() {
    let names = recognized(
        &[
            &["--threshold", "0.01"][..],
            &KINDS.map(|kind| ["--kind", kind]).concat(),
        ]
        .concat(),
        &RECORDS[..RECORDS.find('\n').expect("a line") + 1],
    );
    let perfect = [
        ("/precision", "1.0"),
        ("/yes_recall", "1.0"),
        ("/f1", "1.0"),
    ];
    let folder = scratch("demos");
    let key = write(&folder, "names-key.jsonl", &names_key(1, true));
    row_has(&["-", &key], &names, &perfect);
    let file = repository().join("demos/45-map-relationships/relations.json");
    let text = fs::read_to_string(&file).expect("the relate file");
    let low = write(&folder, "low.json", &text.replace("0.5", "0.01"));
    let edges = related(&format!("@{low}"));
    let key = write(&folder, "edge-key.jsonl", &edge_key(1));
    let half = [
        ("/right", "1"),
        ("/false_yes", "1"),
        ("/precision", "0.5"),
        ("/f1", "0.666667"),
    ];
    row_has(&["-", &key], &edges, &half);
    let key = write(&folder, "edges-key.jsonl", &(edge_key(1) + &edge_key(2)));
    let (_, table, _) = audit(&["-", &key, "--table"], (edges.clone() + &edges).as_bytes());
    for line in [
        "  matched 2, extra 2, missed 0: precision 0.500   recall 1.000   f1 0.667",
        "  at the suggested cut on the held part: precision 1.000, recall 1.000, f1 1.000",
    ] {
        assert!(table.lines().any(|held| held == line), "{table}");
    }
    fs::remove_dir_all(folder).expect("the scratch folder goes");
}

#[test]
fn write_puts_the_cut_in_recognize_and_relate_files() {
    let folder = scratch("write");
    let kinds: Vec<String> = KINDS
        .iter()
        .map(|kind| {
            let (name, text) = kind.split_once('=').expect("a described kind");
            format!("\"{name}\": \"{text}\"")
        })
        .collect();
    let names = format!(
        "{{\n  \"version\": 1,\n  \"recognize\": {{\"kinds\": {{{}}}}},\n  \"threshold\": 0.01\n}}\n",
        kinds.join(", ")
    );
    let file = write(&folder, "names.json", &names);
    // Every name of demo 44's recording scores above 0.99, past the bar's
    // grid, so the name the key leaves out is lowered to 0.6 by hand.
    let lines = recognized(&[&format!("@{file}")], RECORDS)
        .replace("\"strength\":0.997", "\"strength\":0.6");
    let key = write(
        &folder,
        "names-key.jsonl",
        &(names_key(1, false) + &names_key(2, false)),
    );
    let edges_text =
        fs::read_to_string(repository().join("demos/45-map-relationships/relations.json"))
            .expect("the relate file")
            .replace("0.5", "0.01");
    let edges = write(&folder, "edges.json", &edges_text);
    let run = related(&format!("@{edges}"));
    let edge_keys = write(&folder, "edge-key.jsonl", &(edge_key(1) + &edge_key(2)));
    // The names file gains the model its lines name (ticket 0159). The relate
    // file already names `local-1`, the model its lines name.
    let named = names.replacen("0.01\n}", "0.61,\n  \"model\": \"jev-1.13.0\"\n}", 1);
    let edged = edges_text.replacen("0.01", "0.5", 1);
    for (path, lines, key, cut, expected) in [
        (&file, lines, &key, "0.61", named),
        (&edges, run.clone() + &run, &edge_keys, "0.5", edged),
    ] {
        let (code, _, stderr) = audit(&["-", key, "--write", path], lines.as_bytes());
        let said =
            format!("thinkthen: audit: wrote threshold {cut} for the question; it was 0.01\n");
        assert_eq!((code, stderr.as_str()), (0, said.as_str()));
        let after = fs::read_to_string(path).expect("the written file");
        assert_eq!(after, expected);
    }
    // No recording holds a `--lines` run, so this line is written by hand. Its
    // digest is the sha256sum of the lines-form question in the fixture README.
    let lines_file = write(
        &folder,
        "lines.json",
        "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"calls\",\"source\":\"*\",\"target\":\"*\"}]},\"threshold\":0.01}\n",
    );
    let line = "{\"value\":[{\"relation\":\"calls\",\"source\":{\"name\":\"x\",\"kind\":\"*\"},\"target\":{\"name\":\"y\",\"kind\":\"*\"},\"probability\":0.9}],\"question\":{\"verb\":\"relate\",\"relations\":[{\"name\":\"calls\"}],\"threshold\":0.01},\"meta\":{\"question_sha256\":\"f2c4e88c6a7b11bd98a7fed97bcc8412f0e6ca7da0b45b0c9bb2014cd41da3fc\",\"failed_questions\":0}}\n";
    let key = "{\"id\":1,\"value\":[{\"relation\":\"calls\",\"source\":{\"name\":\"x\",\"kind\":\"*\"},\"target\":{\"name\":\"y\",\"kind\":\"*\"}}]}\n";
    let key = write(
        &folder,
        "lines-key.jsonl",
        &(key.to_owned() + &key.replace(":1,", ":2,")),
    );
    let (code, _, stderr) = audit(
        &["-", &key, "--write", &lines_file],
        line.repeat(2).as_bytes(),
    );
    let kept = "thinkthen: audit: kept the bar for the question; the steady bar beat it on 0 of 20 held parts\n";
    assert_eq!((code, stderr.as_str()), (0, kept));
    fs::remove_dir_all(folder).expect("the scratch folder goes");
}

#[test]
fn refusals() {
    let secret = "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":{\"entities\":[{\"name\":\"secret-name\",\"kind\":\"PER\",\"start\":0,\"end\":1,\"strength\":0.9}]},\"question\":{\"verb\":\"recognize\",\"kinds\":{\"PER\":\"A person.\"},\"threshold\":0.5}}\n";
    let folder = scratch("refusals");
    let key = write(
        &folder,
        "key.jsonl",
        "{\"id\":\"secret-id-5150\",\"value\":{\"entities\":[{\"kind\":\"PER\",\"start\":0,\"end\":1,\"name\":\"secret-value\"}]}}\n",
    );
    let bad = write(
        &folder,
        "bad.jsonl",
        "{\"id\":\"secret-id-5150\",\"value\":[\"secret-value\"]}\n",
    );
    let other = write(
        &folder,
        "other.jsonl",
        "{\"id\":\"secret-other\",\"value\":true}\n",
    );
    let unlabeled = "no answer has a label in the key; check that --id points at the key's ids and that its values fit the verb";
    let set_cut =
        "recognize and relate take a single --threshold at or above the cut they ran with";
    let cases: [(&[&str], &str); 4] = [
        (&["-", &key, "--threshold", "0.4:0.6"], set_cut),
        (&["-", &key, "--threshold", "0.4"], set_cut),
        (
            &["-", &bad],
            "key line 1 gives recognize or relate a value unlike the command's own",
        ),
        (&["-", &other], unlabeled),
    ];
    for (arguments, sentence) in cases {
        let (code, stdout, stderr) = audit(arguments, secret.as_bytes());
        let expected = format!("thinkthen: audit: {sentence}\n");
        assert_eq!(
            (code, stdout.as_str(), stderr.as_str()),
            (2, "", expected.as_str()),
            "{arguments:?}"
        );
        assert!(!stderr.contains("secret"), "{stderr}");
    }
    // The refusal comes before --write reads the question file. A write first
    // would refuse this line for lacking the file's digest.
    let question = "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"calls\",\"source\":\"*\",\"target\":\"*\"}]},\"threshold\":0.01}\n";
    let file = write(&folder, "lines.json", question);
    let (code, _, stderr) = audit(&["-", &other, "--write", &file], secret.as_bytes());
    assert_eq!(
        (code, stderr),
        (2, format!("thinkthen: audit: {unlabeled}\n"))
    );
    assert_eq!(fs::read_to_string(&file).expect("the file"), question);
    // A key that labels one of two questions grades the one and exits 0.
    let two = "{\"input\":{\"id\":\"r1\"},\"value\":true,\"question\":{\"text\":\"A?\"}}\n{\"input\":{\"id\":\"r2\"},\"value\":true,\"question\":{\"text\":\"B?\"}}\n";
    let one = write(&folder, "one.jsonl", "{\"id\":\"r1\",\"value\":true}\n");
    let (code, stdout, _) = audit(&["-", &one], two.as_bytes());
    assert_eq!((code, stdout.lines().count()), (0, 2));
    fs::remove_dir_all(folder).expect("the scratch folder goes");
}
