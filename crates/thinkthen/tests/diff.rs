//! `thinkthen diff` matches the prototype's golden files and its hand-checked values.
#![cfg(feature = "cli")]
#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

#[path = "support/measure.rs"]
mod measure_support;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use measure_support::{DIFF_GOLDENS, DIFF_TABLES, fixture, measure, member, ported, run};

fn diff(arguments: &[&str], input: &[u8]) -> (i32, String, String) {
    measure(&[&["diff"], arguments].concat(), input)
}

const NO_PAIR: &str = "thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names\n";

fn digests_differ(count: usize, of: usize) -> String {
    format!(
        "thinkthen: diff: warning: the question digest differs in {count} of {of} paired answers. A different question, threshold, or profile gives a different digest.\n"
    )
}

#[test]
fn goldens_match() {
    for (golden, arguments) in DIFF_GOLDENS {
        let (code, stdout, stderr) = diff(arguments, b"");
        // The soft run rewords the question, so every pair's digest differs.
        let warned = match golden {
            "golden/diff-249-soft.jsonl" => digests_differ(272, 272),
            _ => String::new(),
        };
        assert_eq!((code, stderr), (0, warned), "{golden}");
        let mut expected = fixture(golden);
        if golden == "golden/extra/diff-annotate.jsonl" {
            // The captured rows fall below the 0.75 cut; their move has count 2.
            for end in ["probability\":[0.6", "probability\":[0.7", "count\":2"] {
                let state = format!("\"to\":\"unresolved\",\"{end}");
                expected = expected.replace(&state, &state.replace("unresolved", "unsure"));
            }
        }
        assert_eq!(stdout, expected, "{golden}");
    }
}

#[test]
fn tables_match_byte_for_byte() {
    for (capture, arguments) in DIFF_TABLES {
        let (code, stdout, stderr) = diff(&[arguments, &["--table"]].concat(), b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{capture}");
        assert_eq!(stdout, fixture(capture), "{capture}");
    }
}

#[test]
fn an_option_named_unresolved_stays_an_option() {
    let run = fixture("small/choose.jsonl").replace("\"red\"", "\"unresolved\"");
    let (code, stdout, stderr) = diff(&["-", "small/choose-b.jsonl"], run.as_bytes());
    assert_eq!((code, stderr.as_str()), (0, ""));
    let first = stdout.lines().next().expect("a changed option");
    assert_eq!(member(first, "/from"), "\"unresolved\"");
    assert_eq!(member(first, "/to"), "\"red\"");
    assert_eq!(ported(format!("{first}\n")), format!("{first}\n"));
}

#[test]
fn a_run_line_with_a_member_diff_does_not_use_still_pairs() {
    let wider = fixture("small/decide-b.jsonl").replace("{\"input\"", "{\"later\":[1],\"input\"");
    let (code, stdout, _) = diff(
        &["small/decide.jsonl", "-", "--key", "small/decide-key.jsonl"],
        wider.as_bytes(),
    );
    assert_eq!(code, 0);
    assert_eq!(stdout, fixture("golden/diff-decide-wordings.jsonl"));
}

#[test]
fn the_held_out_half_reads_as_the_prototype_reports() {
    let held: String = fixture("249/key.jsonl")
        .lines()
        .filter(|line| line.contains("\"held\""))
        .map(|line| format!("{line}\n"))
        .collect();
    let summary = |arguments: &[&str]| {
        let (code, stdout, _) = diff(&[arguments, &["--key", "-"]].concat(), held.as_bytes());
        assert_eq!(code, 0);
        stdout.lines().last().expect("a summary").to_owned()
    };
    let cuts = summary(&["249/control.jsonl", "--compare-threshold", "0.42"]);
    let soft = summary(&["249/control.jsonl", "249/soft.jsonl"]);
    let rights = |line: &str| {
        [
            member(line, "/summary/right_a"),
            member(line, "/summary/right_b"),
        ]
    };
    assert_eq!(rights(&cuts), ["87", "89"]);
    let moves: Vec<String> = ["/0/from", "/0/to", "/0/count", "/1"]
        .iter()
        .map(|place| {
            let value: serde_json::Value =
                serde_json::from_str(&member(&cuts, "/summary/moves")).expect("moves");
            value
                .pointer(place)
                .map_or_else(String::new, ToString::to_string)
        })
        .collect();
    assert_eq!(
        moves,
        ["\"no\"", "\"yes\"", &member(&cuts, "/summary/changed"), ""]
    );
    assert_eq!(rights(&soft), ["87", "84"]);
}

#[test]
fn help_names_diff_and_says_what_it_never_does() {
    const ROW: &str = "Show which saved answers changed between two runs or two cuts.";
    let text =
        |arguments: &[&str]| String::from_utf8(run(arguments, b"").stdout).expect("UTF-8 help");
    let row = ROW.trim_end_matches('.');
    let root = text(&["--help"]);
    assert!(root.contains(&format!("\n  diff       {row}\n")), "{root}");
    assert!(text(&["diff", "-h"]).starts_with(&format!("{row}\n\n")));
    let long = text(&["diff", "--help"]);
    assert!(long.starts_with(&format!("{ROW}\n\n")), "{long}");
    for sentence in [
        "diff sends no request and reads no key.",
        "Two cuts on one run cost nothing.",
        "The probabilities are already saved.",
        "An answer inside a band is not sure.",
        "diff pairs answers by record id and answer name only.",
        "It compares question digests only when both runs saved --details.",
    ] {
        assert!(long.contains(sentence), "{sentence}");
    }
}

#[test]
fn the_second_side_reads_under_compare_threshold_when_both_rules_are_given() {
    let cases: [(&[&str], &str); 2] = [
        (
            &[
                "small/decide.jsonl",
                "--key",
                "small/decide-key.jsonl",
                "--threshold",
                "0.5",
            ],
            "r3  no -> yes  p 0.45 -> 0.45  key yes: gained\n0.5 -> 0.4: 1 of 6 changed; no -> yes 1; gained 1, lost 0 (4 -> 5 right of 6); McNemar p 1.000 on right answers\n",
        ),
        (
            &["small/decide-band.jsonl", "--threshold", "0.3:0.6"],
            "r3  unsure -> yes  p 0.45 -> 0.45\nr4  unsure -> no  p 0.30 -> 0.30\n0.3:0.6 -> 0.4: 2 of 6 changed; unsure -> no 1; unsure -> yes 1; McNemar p 1.000 on yes answers\n",
        ),
    ];
    for (arguments, table) in cases {
        let (code, stdout, _) = diff(
            &[arguments, &["--compare-threshold", "0.4", "--table"][..]].concat(),
            b"",
        );
        assert_eq!((code, stdout.as_str()), (0, table), "{arguments:?}");
    }
}

/// A warning row: the case, the first run, the second run, and the expected outputs.
type Row<'a> = (&'a str, &'a str, String, &'a str, String);

#[test]
fn warnings_go_to_standard_error_and_leave_the_output_and_exit_alone() {
    let table = fixture("golden/table/diff-decide-nokey.txt");
    let b = fixture("small/decide-b.jsonl");
    let annotate = fixture("small/annotate.jsonl");
    // One annotate line holds one digest, so its change flags both answers on it.
    let a1_changed = annotate.replacen(
        "\"questions_sha256\":\"00\"",
        "\"questions_sha256\":\"01\"",
        1,
    );
    let rows: [Row<'_>; 4] = [
        (
            "digest mismatch",
            "small/decide.jsonl",
            b.replace("\"question_sha256\":\"00\"", "\"question_sha256\":\"01\""),
            &table,
            digests_differ(6, 6),
        ),
        (
            "one side has no digest",
            "small/decide.jsonl",
            b.replace("\"question_sha256\":\"00\",", ""),
            &table,
            String::new(),
        ),
        (
            "no pairs",
            "small/decide.jsonl",
            b.replace("\"id\":\"r", "\"id\":\"x"),
            "A -> B: 0 of 0 changed; McNemar p 1.000 on yes answers; only in A 6, only in B 7\n",
            NO_PAIR.to_owned(),
        ),
        (
            "an annotate line's digest",
            "small/annotate.jsonl",
            a1_changed,
            "A -> B: 0 of 5 changed\n",
            digests_differ(2, 5),
        ),
    ];
    for (case, first, second, stdout, stderr) in rows {
        let got = diff(&[first, "-", "--table"], second.as_bytes());
        assert_eq!(got, (0, stdout.to_owned(), stderr), "{case}");
    }
}

#[test]
fn mcnemar_counts_every_pair_that_becomes_right_or_stops_being_right() {
    // c2 moves between tied and right. The old rule left it out and printed p 1.000.
    let rows = [
        (
            ["small/choose.jsonl", "small/choose-b.jsonl"],
            "A -> B: 2 of 5 changed; red -> green 1; tied -> green 1; gained 1, lost 0 (2 -> 4 right of 5); McNemar p 0.500 on right answers",
        ),
        (
            ["small/choose-b.jsonl", "small/choose.jsonl"],
            "A -> B: 2 of 5 changed; green -> red 1; green -> tied 1; gained 0, lost 1 (4 -> 2 right of 5); McNemar p 0.500 on right answers",
        ),
    ];
    for ([a, b], line) in rows {
        let (code, stdout, _) = diff(&[a, b, "--key", "small/choose-key.jsonl", "--table"], b"");
        assert_eq!((code, stdout.lines().last()), (0, Some(line)), "{a} {b}");
    }
}

/// Run `diff` with arguments split on spaces, from the fixture folder.
fn words(arguments: &str) -> (i32, String, String) {
    diff(&arguments.split(' ').collect::<Vec<_>>(), b"")
}

#[test]
fn recognize_and_relate_rows_match_the_hand_worked_outputs() {
    let rows = [
        "items/recognize-a.jsonl items/recognize-b.jsonl = strict.jsonl",
        "items/recognize-a.jsonl items/recognize-b.jsonl --key items/recognize-key.jsonl = strict-key.jsonl",
        "items/recognize-a.jsonl items/recognize-b.jsonl --key items/recognize-key.jsonl --match overlap = overlap-key.jsonl",
        "items/recognize-a.jsonl items/recognize-none.jsonl --key items/recognize-key.jsonl --match overlap = none-key.jsonl",
        "items/recognize-a.jsonl items/recognize-b.jsonl --key items/recognize-key.jsonl --table = strict-key.txt",
        "items/recognize-a.jsonl items/recognize-b.jsonl --key items/recognize-key.jsonl --match overlap --table = overlap-key.txt",
        "items/abbey-work.jsonl items/recognize-b.jsonl --match overlap --table = abbey-work.txt",
        "items/relate-a.jsonl items/relate-b.jsonl --key items/relate-key.jsonl = relate-key.jsonl",
        "items/relate-a.jsonl items/relate-b.jsonl --key items/relate-key.jsonl --table = relate-key.txt",
        "items/decide-c1.jsonl small/choose.jsonl --table = decide-choose.txt",
    ];
    for (arguments, golden) in rows.iter().filter_map(|row| row.split_once(" = ")) {
        let expected = fixture(&format!("golden/items/{golden}"));
        assert_eq!(words(arguments), (0, expected, String::new()), "{golden}");
    }
}

#[test]
fn two_cuts_on_a_replayed_recognize_run_lose_the_names_between_them() {
    let (code, stdout, _) =
        words("diff-recognize-key-run.jsonl --threshold 0.5 --compare-threshold 0.8");
    let summary = r#"{"summary":{"records":200,"changed":52,"only_a":0,"only_b":0,"moves":[],"labeled":null,"right_a":null,"right_b":null,"gained":null,"lost":null,"mcnemar_on":null,"mcnemar_p":null,"key_items":null,"items_gained":0,"items_lost":64,"items_changed_kind":0,"extra_a":null,"extra_b":null,"compare":"cuts","a":0.5,"b":0.8}}"#;
    assert_eq!((code, stdout.lines().last()), (0, Some(summary)));
    let rows = stdout
        .lines()
        .filter_map(|row| serde_json::from_str(row).ok());
    let lost: Vec<serde_json::Value> = rows
        .flat_map(|row: serde_json::Value| row["lost"].as_array().cloned())
        .flatten()
        .collect();
    assert!(lost.iter().all(|name| {
        name["strength"]
            .as_f64()
            .is_some_and(|s| (0.5..0.8).contains(&s))
    }));
}

#[test]
fn mcnemar_on_key_names_leaves_the_extras_out() {
    let (code, stdout, _) =
        words("items/mcnemar-a.jsonl items/mcnemar-b.jsonl --key items/mcnemar-key.jsonl");
    let last = stdout.lines().last().expect("a summary");
    let got = ["mcnemar_p", "gained", "lost", "extra_a", "extra_b"]
        .map(|name| member(last, &format!("/summary/{name}")));
    assert_eq!((code, got.join(" ")), (0, "0.145996 9 3 2 0".to_owned()));
}

#[test]
fn item_verbs_refuse_what_diff_cannot_compare() {
    let cut = "recognize and relate take a single --threshold at or above the cut they ran with";
    let grade =
        "holds an answer diff cannot grade; diff grades decide, choose, recognize and relate";
    let rows = [
        format!("items/recognize-a.jsonl --compare-threshold 0.3 = {cut}"),
        format!("items/recognize-a.jsonl items/recognize-b.jsonl --threshold 0.4:0.6 = {cut}"),
        "items/recognize-a.jsonl items/pair-decide.jsonl = second run line 1 pairs recognize or relate with another verb".to_owned(),
        "items/mixed-decide.jsonl --compare-threshold 0.6 = first run line 2 mixes recognize or relate with other verbs".to_owned(),
        "items/mixed-relate.jsonl --compare-threshold 0.6 = first run line 2 mixes recognize with relate".to_owned(),
        "items/recognize-a.jsonl items/person-only.jsonl --key items/recognize-key.jsonl = key line 1 names a level, label, or unit the question does not have".to_owned(),
        format!("items/record-mode.jsonl items/recognize-b.jsonl = first run line 1 {grade}"),
        format!("items/tag.jsonl items/recognize-b.jsonl = first run line 1 {grade}"),
        "items/bare.jsonl items/recognize-b.jsonl = first run line 1 has no string or integer id at the --id pointer".to_owned(),
        "small/decide.jsonl small/decide-b.jsonl --match overlap = --match applies to recognize and relate".to_owned(),
    ];
    for (arguments, sentence) in rows.iter().filter_map(|row| row.split_once(" = ")) {
        let refused = (2, String::new(), format!("thinkthen: diff: {sentence}\n"));
        assert_eq!(words(arguments), refused, "{sentence}");
    }
}
