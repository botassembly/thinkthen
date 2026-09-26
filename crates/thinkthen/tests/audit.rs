//! `thinkthen audit` matches the prototype's golden files and its hand-checked values.
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

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use measure_support::{
    GOLDENS, TABLES, audit, compact, fixture, fixtures, member, members, without_added,
    without_added_lines,
};
use sha2::{Digest as _, Sha256};

/// Every golden line and table capture of 0113 holds, byte for byte, once
/// the members and lines ticket 0125 added are removed.
#[test]
fn old_goldens_hold() {
    for (golden, arguments) in GOLDENS {
        let (code, stdout, stderr) = audit(arguments, b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{golden}");
        assert_eq!(without_added(&stdout), fixture(golden), "{golden}");
    }
    for (capture, [results, key]) in TABLES {
        let (code, stdout, stderr) = audit(&[results, key, "--table"], b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{capture}");
        assert_eq!(without_added_lines(&stdout), fixture(capture), "{capture}");
    }
}

#[test]
fn every_fixture_keeps_its_checksum() {
    let listed: Vec<(String, String)> = fixture("README.md")
        .lines()
        .skip_while(|line| *line != "| File | SHA-256 |")
        .skip(2)
        .map(|line| {
            let cells: Vec<&str> = line.split('`').collect();
            (cells[1].to_owned(), cells[3].to_owned())
        })
        .collect();
    let mut found = Vec::new();
    let mut pending = vec![fixtures()];
    while let Some(folder) = pending.pop() {
        for entry in fs::read_dir(folder).expect("a folder") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.file_name().is_some_and(|name| name != "README.md") {
                let bytes = fs::read(&path).expect("a fixture");
                let digest: String = Sha256::digest(&bytes)
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect();
                let name = path
                    .strip_prefix(fixtures())
                    .expect("inside")
                    .to_string_lossy()
                    .into_owned();
                found.push((name, digest));
            }
        }
    }
    found.sort();
    assert_eq!(found, listed);
    assert_eq!(listed.len(), 72);
}

#[test]
fn readers_ignore_members_they_do_not_use_and_a_band_prints_as_typed() {
    let extra = |text: String| {
        text.replace("{\"id\"", "{\"later\":[1],\"id\"")
            .replace("\"input\"", "\"later\":2,\"input\"")
    };
    let results = extra(fixture("small/decide.jsonl"));
    let key = fixtures().join("small/decide-key.jsonl");
    let wider = std::env::temp_dir().join(format!("thinkthen-0113-key-{}", std::process::id()));
    fs::write(&wider, extra(fixture("small/decide-key.jsonl"))).expect("a key");
    let (code, stdout, _) = audit(&["-", wider.to_str().expect("a path")], results.as_bytes());
    fs::remove_file(&wider).expect("cleanup");
    assert_eq!(code, 0);
    assert_eq!(without_added(&stdout), fixture("golden/audit-decide.jsonl"));

    let key = key.to_str().expect("a path");
    let (_, band, _) = audit(
        &["small/decide.jsonl", key, "--threshold", "0.40:0.60"],
        b"",
    );
    assert_eq!(member(&band, "/threshold"), "\"0.40:0.60\"");
    let (code, stdout, stderr) = audit(&["-", key], b"");
    assert_eq!((code, stdout.as_str(), stderr.as_str()), (0, "", ""));
}

#[test]
fn a_replayed_recording_piped_to_audit_grades_as_the_prototype_does() {
    let rows = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../transforms/rows");
    let question = fs::read_to_string(rows.join("question.txt")).expect("the question");
    let mut decide = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args([
            "decide",
            question.trim(),
            "--jsonl",
            "--field",
            "/body",
            "--details",
            "--replay",
        ])
        .arg(rows.join("recording"))
        .arg("--input")
        .arg(rows.join("cases.jsonl"))
        .env_clear()
        .stdout(Stdio::piped())
        .spawn()
        .expect("decide runs");
    let audit = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["audit", "-", "replay/key.jsonl"])
        .env_clear()
        .current_dir(fixtures())
        .stdin(decide.stdout.take().expect("the pipe"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("audit runs");
    let output = wait::finish(audit, "thinkthen audit").expect("audit finishes");
    let decided = wait::finish(decide, "thinkthen decide").expect("decide finishes");
    assert_eq!(decided.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(
        without_added(&String::from_utf8_lossy(&output.stdout)),
        fixture("replay/audit.jsonl")
    );
}

#[test]
fn each_question_draws_its_own_bootstrap_and_the_held_out_half_reads_as_reported() {
    let control = fixture("249/control.jsonl");
    let (mut both, mut second) = (String::new(), String::new());
    for (place, line) in control.lines().enumerate() {
        let mut row: serde_json::Value = serde_json::from_str(line).expect("a line");
        row["question"]["text"] = if place % 2 == 1 { "B" } else { "A" }.into();
        let line = format!("{row}\n");
        both.push_str(&line);
        if place % 2 == 1 {
            second.push_str(&line);
        }
    }
    let interval = |input: &str| {
        let (code, stdout, _) = audit(&["-", "249/key.jsonl"], input.as_bytes());
        assert_eq!(code, 0);
        member(
            stdout.lines().last().expect("the second group"),
            "/calibration/interval",
        )
    };
    assert_eq!(interval(&both), "[0.049765,0.164636]");
    assert_eq!(interval(&second), "[0.049765,0.164636]");

    let held: String = fixture("249/key.jsonl")
        .lines()
        .filter(|l| l.contains("\"held\""))
        .map(|l| format!("{l}\n"))
        .collect();
    let path = std::env::temp_dir().join(format!("thinkthen-0113-held-{}", std::process::id()));
    fs::write(&path, held).expect("a key");
    let (_, stdout, _) = audit(
        &[
            "249/control.jsonl",
            path.to_str().expect("a path"),
            "--by",
            "verb",
        ],
        b"",
    );
    fs::remove_file(&path).expect("cleanup");
    let read = |pointer: &str| {
        member(&stdout, pointer)
            .parse::<f64>()
            .map(|v| format!("{v:.3}"))
            .expect("a number")
    };
    assert_eq!(member(&stdout, "/labeled"), "138");
    let numbers = [
        "/agreement",
        "/yes_recall",
        "/mean_probability",
        "/auc",
        "/calibration/error",
    ]
    .map(read);
    assert_eq!(numbers, ["0.630", "0.311", "0.397", "0.725", "0.092"]);
}

/// The Abbey Road rows, keyed with every record in the tuning part.
const ABBEY: [&str; 4] = ["abbey/rows.jsonl", "abbey/key-tune.jsonl", "--id", "/input"];

/// Each measure picks its own bar by its own tie rule. Ian's ruling of
/// 2026-09-25 gives the four picks, and experiment 259 checked two by hand.
#[test]
fn four_measures_pick_four_bars() {
    let cut = |measure: &str| {
        let (code, stdout, stderr) = audit(&[&ABBEY[..], &["--optimize", measure]].concat(), b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{measure}");
        member(&stdout, "/suggested/cut")
    };
    let picks = ["accuracy", "precision", "recall", "f1"].map(cut);
    assert_eq!(picks, ["0.85", "0.95", "0.75", "0.73"]);
    let (_, stdout, _) = audit(&[&ABBEY[..], &["--threshold", "0.75"]].concat(), b"");
    let measures = members(&stdout, ["/precision", "/yes_recall", "/f1"]);
    assert_eq!(measures, ["0.583333", "1.0", "0.736842"]);
}

/// Twenty splits from `--seed` tune the bars experiment 259 measured one
/// seed at a time with the landed binary.
#[test]
fn steady_counts_twenty_seeds() {
    let steady = |seed: &str| {
        let arguments = [
            "abbey/rows.jsonl",
            "abbey/key.jsonl",
            "--id",
            "/input",
            "--seed",
            seed,
        ];
        let (code, stdout, _) = audit(&arguments, b"");
        assert_eq!(code, 0);
        members(
            &stdout,
            ["/suggested/steady/cut", "/suggested/steady/counts"],
        )
    };
    let counts = |pairs: [(f64, usize); 6]| {
        let listed: Vec<String> = pairs
            .iter()
            .map(|(cut, count)| format!("{{\"cut\":{cut},\"count\":{count}}}"))
            .collect();
        compact(&format!("[{}]", listed.join(",")))
    };
    let seed_0 = [
        (0.58, 3),
        (0.68, 3),
        (0.73, 5),
        (0.83, 4),
        (0.85, 4),
        (0.92, 1),
    ];
    let seed_1 = [
        (0.58, 3),
        (0.68, 3),
        (0.73, 5),
        (0.83, 4),
        (0.85, 3),
        (0.92, 2),
    ];
    assert_eq!(steady("0"), ["0.73".to_owned(), counts(seed_0)]);
    assert_eq!(steady("1"), ["0.73".to_owned(), counts(seed_1)]);
}

/// The steady bar and its `better` count over one `better/` fixture.
fn steady_better(name: &str, target: &str) -> [String; 2] {
    let results = format!("better/{name}.jsonl");
    let key = format!("better/{name}-key.jsonl");
    let (code, stdout, _) = audit(&[&results, &key, "--target", target], b"");
    assert_eq!(code, 0, "{name}");
    members(
        &stdout,
        ["/suggested/steady/cut", "/suggested/steady/better"],
    )
}

/// `better` counts held parts where the steady bar beats the run's rule.
/// The fixture README grades each of the twenty held parts by hand.
#[test]
fn better_reads_steady_cut() {
    assert_eq!(steady_better("decide", "0.9"), ["0.61", "17"]);
    assert_eq!(steady_better("tie", "0.9"), ["0.5", "0"]);
}

/// A `choose` bar wins a held part by reaching the target, or by answering
/// more at no lower agreement.
#[test]
fn better_for_choose() {
    assert_eq!(steady_better("choose-reach", "0.9"), ["0.61", "17"]);
    assert_eq!(steady_better("choose-more", "0.8"), ["0.01", "20"]);
}
