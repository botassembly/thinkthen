//! `thinkthen audit` matches the prototype's golden files and its hand-checked values.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

use crate::child::ChildEnvironment as _;
use crate::measure_support;
use crate::wait;

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use measure_support::{
    GOLDENS, TABLES, audit, compact, fixture, fixtures, member, members, ported, without_added,
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
        assert_eq!(without_added(&stdout), ported(fixture(golden)), "{golden}");
    }
    for (capture, [results, key]) in TABLES {
        let (code, stdout, stderr) = audit(&[results, key, "--table"], b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{capture}");
        assert_eq!(
            without_added_lines(&stdout),
            ported(fixture(capture)),
            "{capture}"
        );
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
                // The list names each file with `/`, which Windows writes as `\`.
                let name = path
                    .strip_prefix(fixtures())
                    .expect("inside")
                    .components()
                    .map(|part| part.as_os_str().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("/");
                found.push((name, digest));
            }
        }
    }
    found.sort();
    assert_eq!(found, listed);
    assert_eq!(listed.len(), 118);
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
    assert_eq!(
        without_added(&stdout),
        ported(fixture("golden/audit-decide.jsonl"))
    );

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
            "--batch",
            "1",
            "--replay",
        ])
        .arg(rows.join("recording"))
        .arg("--input")
        .arg(rows.join("cases.jsonl"))
        .clear_environment()
        .stdout(Stdio::piped())
        .spawn()
        .expect("decide runs");
    let audit = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["audit", "-", "replay/key.jsonl"])
        .clear_environment()
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
        ported(fixture("replay/audit.jsonl"))
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
    assert_eq!(interval(&both), "[0.017405,0.12316]");
    assert_eq!(interval(&second), "[0.017405,0.12316]");

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
    assert_eq!(numbers, ["0.630", "0.311", "0.397", "0.725", "0.086"]);
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

/// The yes/no and `choose` answers of `given/`, graded by hand in the fixture README.
const YES_NO: [&str; 2] = ["given/yesno.jsonl", "given/key.jsonl"];
const CHOSEN: [&str; 2] = ["given/choose.jsonl", "given/key.jsonl"];

/// A table line equal to this, or the whole table.
fn has_line(table: &str, line: &str) {
    assert!(table.lines().any(|held| held == line), "{table}");
}

/// A tie that holds the key earns one over the tied options.
#[test]
fn tie_share() {
    let (code, stdout, _) = audit(&CHOSEN, b"");
    assert_eq!(code, 0);
    let pointers = [
        "/tied",
        "/tied_holding_key",
        "/tie_share",
        "/right",
        "/wrong",
    ];
    assert_eq!(members(&stdout, pointers), ["3", "2", "0.75", "1", "1"]);
    let (_, table, _) = audit(&[&CHOSEN[..], &["--table"]].concat(), b"");
    has_line(&table, "  ties holding the key: 2 of 3, share 0.750");
}

/// Calibration pairs each answer's confidence in the answer it gave.
#[test]
fn calibration_pairs_the_answer_given() {
    let calibration = |arguments: &[&str]| {
        let (code, stdout, _) = audit(arguments, b"");
        assert_eq!(code, 0);
        members(&stdout, ["/calibration/error", "/calibration/interval"])
    };
    assert_eq!(calibration(&YES_NO), ["0.45", "[0.22419,0.71444]"]);
    assert_eq!(calibration(&CHOSEN), ["0.11", "[0.0,0.33641]"]);
    let (_, stdout, _) = audit(&CHOSEN, b"");
    let bins: Vec<String> =
        serde_json::from_str::<serde_json::Value>(&member(&stdout, "/calibration/by_bin"))
            .expect("bins")
            .as_array()
            .expect("a list")
            .iter()
            .filter(|bin| bin["n"] != 0)
            .map(|bin| {
                ["low", "n", "right", "confidence"]
                    .map(|name| bin[name].to_string())
                    .join(" ")
            })
            .collect();
    assert_eq!(
        bins,
        ["0.2 1 0.25 0.25", "0.4 2 0.5 0.425", "0.6 2 1.0 0.6"]
    );
}

/// `--pooled` adds one last line over every paired verb and leaves the rows alone.
#[test]
fn pooled() {
    let input = [
        fixture("given/yesno.jsonl"),
        fixture("given/choose.jsonl"),
        fixture("verbs/tag.jsonl"),
    ]
    .concat();
    let run = |extra: &[&str]| {
        let (code, stdout, stderr) = audit(
            &[&["-", "given/key.jsonl"][..], extra].concat(),
            input.as_bytes(),
        );
        assert_eq!((code, stderr.as_str()), (0, ""));
        stdout
    };
    let (rows, pooled) = (run(&[]), run(&["--pooled"]));
    let (before, last) = pooled.trim_end().rsplit_once('\n').expect("a last line");
    assert_eq!(format!("{before}\n"), rows);
    let pointers = [
        "/pooled",
        "/answers",
        "/calibration/error",
        "/calibration/interval",
    ];
    assert_eq!(
        members(last, pointers),
        ["\"every verb\"", "10", "0.17", "[0.0,0.40036]"]
    );
    has_line(
        &run(&["--pooled", "--table"]),
        "  every verb pooled: calibration error 0.170 (95% 0.000 to 0.400) over 10 answers",
    );
}

/// Each part is scored at the bar the other part tuned.
#[test]
fn crossed() {
    let crossed = |arguments: &[&str]| {
        let (code, stdout, _) = audit(arguments, b"");
        assert_eq!(code, 0);
        let pointers = [
            "/suggested/crossed/cuts",
            "/suggested/crossed/held/right",
            "/suggested/crossed/held/agreement",
        ];
        members(&stdout, pointers)
    };
    let hand = ["given/crossed.jsonl", "given/crossed-key.jsonl"];
    assert_eq!(crossed(&hand), ["[0.4,0.5]", "4", "0.5"]);
    assert_eq!(
        crossed(&[&hand[..], &["--optimize", "f1"]].concat()),
        ["[0.4,0.3]", "5", "0.625"]
    );
    let abbey = [
        "abbey/rows.jsonl",
        "abbey/key.jsonl",
        "--id",
        "/input",
        "--seed",
        "0",
    ];
    assert_eq!(crossed(&abbey), ["[0.85,0.73]", "65", "0.928571"]);
    let (_, table, _) = audit(&[&hand[..], &["--table"]].concat(), b"");
    has_line(
        &table,
        "  crossed: cuts 0.4 and 0.5, each checked on the other part: agreement 0.500, 4 right of 8 answered",
    );
}

/// `--curve` lists the kept answers and their summed right at each confidence.
#[test]
fn curve() {
    let curve = |arguments: &[&str]| {
        let (code, stdout, _) = audit(&[arguments, &["--curve"]].concat(), b"");
        assert_eq!(code, 0);
        member(&stdout, "/curve")
    };
    let steps = |points: [(f64, usize, f64); 4]| {
        let listed: Vec<String> = points
            .iter()
            .map(|(cut, kept, right)| {
                format!("{{\"cut\":{cut},\"kept\":{kept},\"right\":{right:?}}}")
            })
            .collect();
        compact(&format!("[{}]", listed.join(",")))
    };
    assert_eq!(
        curve(&CHOSEN),
        steps([
            (0.6, 2, 1.0),
            (0.45, 3, 1.0),
            (0.4, 4, 1.5),
            (0.25, 5, 1.75)
        ])
    );
    assert_eq!(
        curve(&YES_NO),
        steps([(0.9, 2, 1.0), (0.65, 3, 2.0), (0.5, 4, 2.0), (0.4, 5, 3.0)])
    );
    let (_, stdout, _) = audit(&CHOSEN, b"");
    assert_eq!(member(&stdout, "/curve"), "null");
}

/// `--by POINTER` groups by a field of each input, then by verb, in first-seen order.
#[test]
fn by_pointer() {
    let rows = |pointer: &str| -> Vec<String> {
        let (code, stdout, stderr) = audit(
            &["given/by.jsonl", "given/by-key.jsonl", "--by", pointer],
            b"",
        );
        assert_eq!((code, stderr.as_str()), (0, ""), "{pointer}");
        stdout
            .lines()
            .map(|line| members(line, ["/group", "/verb", "/rows", "/right", "/wrong"]).join(" "))
            .collect()
    };
    assert_eq!(
        rows("/category"),
        [
            "\"lead\" \"decide\" 2 1 1",
            "\"lead\" \"choose\" 1 1 0",
            "\"tail\" \"decide\" 2 1 1",
            "\"3\" \"choose\" 1 0 1",
        ]
    );
    let groups: Vec<String> = rows("/a~1b")
        .iter()
        .map(|row| row.split(' ').take(3).collect::<Vec<_>>().join(" "))
        .collect();
    assert_eq!(
        groups,
        [
            "\"x\" \"decide\" 1",
            "\"x\" \"choose\" 2",
            "\"y\" \"decide\" 3"
        ]
    );
}
