//! `thinkthen diff` matches the prototype's golden files and its hand-checked values.
#![cfg(feature = "cli")]
#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

#[path = "support/measure.rs"]
mod measure_support;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use measure_support::{DIFF_GOLDENS, DIFF_TABLES, fixture, measure, member, run};

fn diff(arguments: &[&str], input: &[u8]) -> (i32, String, String) {
    measure(&[&["diff"], arguments].concat(), input)
}

#[test]
fn goldens_match() {
    for (golden, arguments) in DIFF_GOLDENS {
        let (code, stdout, stderr) = diff(arguments, b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{golden}");
        assert_eq!(stdout, fixture(golden), "{golden}");
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
            "r3  unresolved -> yes  p 0.45 -> 0.45\nr4  unresolved -> no  p 0.30 -> 0.30\n0.3:0.6 -> 0.4: 2 of 6 changed; unresolved -> no 1; unresolved -> yes 1; McNemar p 1.000 on yes answers\n",
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
