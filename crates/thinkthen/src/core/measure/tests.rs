//! Hand-checked values from the prototype's own tests, and the cases its review planted.

use super::answer::{self, Rule};
use super::audit::{self, By, Settings, Shown};
use super::key::Key;
use super::{
    SplitMix64, auc, calibration_error, json_lines, python_float_text, rounded, three_places,
    wilson,
};
use crate::core::pointer::Pointer;

/// The six decide answers of `small/decide.jsonl` as `(p(yes), key is yes)`.
const DECIDE: [(f64, bool); 6] = [
    (0.9, true),
    (0.7, false),
    (0.45, true),
    (0.3, false),
    (0.2, false),
    (0.6, true),
];

fn near(value: f64, expected: f64, places: i32) -> bool {
    (value - expected).abs() < 0.5 * 10_f64.powi(-places)
}

/// Audit result lines against key lines, both given as text, with the default settings.
fn audit_text(results: &str, key: &str) -> Vec<audit::Row> {
    let results = json_lines(results.as_bytes()).expect("result lines");
    let key = Key::read(&json_lines(key.as_bytes()).expect("key lines")).expect("a key");
    let pointer = Pointer::new("/id").expect("a pointer");
    let answers = answer::read(&results, &pointer).expect("answers");
    let settings = Settings {
        by: By::Question,
        rule: Rule::AsRun,
        shown: Shown::AsRun,
        seed: 0,
        target: 0.9,
    };
    audit::audit(&answers, &key, &settings).expect("an audit")
}

fn decide_line(id: &str, p: f64) -> String {
    format!(
        r#"{{"input":{{"id":"{id}"}},"value":{},"answer":{{"probability":{p}}}}}"#,
        p >= 0.5
    )
}

#[test]
fn splitmix64_matches_the_published_outputs_and_draws_in_range() {
    let mut generator = SplitMix64::new(0);
    let drawn = [generator.next(), generator.next(), generator.next()];
    assert_eq!(
        drawn,
        [
            0xE220_A839_7B1D_CDAF,
            0x6E78_9E6A_A1B9_65F4,
            0x06C4_5D18_8009_454F
        ]
    );
    // The high 64 bits of next() times n, not next() modulo n.
    assert_eq!(SplitMix64::new(0).index(3), 2);
    assert_eq!(SplitMix64::new(0).index(100), 88);
}

#[test]
fn wilson_gives_the_hand_checked_interval_for_four_of_six() {
    let [low, high] = wilson(4, 6).expect("an interval");
    assert!(
        near(low, 0.3000, 4) && near(high, 0.9032, 4),
        "{low} {high}"
    );
    assert_eq!(wilson(0, 0), None);
}

#[test]
fn auc_counts_ties_as_half_and_matches_the_pairwise_sum() {
    assert!(near(auc(&DECIDE).expect("both kinds"), 7.0 / 9.0, 9));
    assert_eq!(auc(&[(0.5, true), (0.5, false), (0.2, false)]), Some(0.75));
    assert_eq!(auc(&[(0.5, true)]), None);
}

#[test]
fn calibration_error_closes_the_last_bin_at_one() {
    assert!(near(calibration_error(&DECIDE), 0.375, 9));
    let choose = [(0.8, true), (0.7, false), (0.6, true), (0.5, false)];
    assert!(near(calibration_error(&choose), 0.45, 9));
    assert!(near(
        calibration_error(&[(1.0, false), (0.0, false)]),
        0.5,
        9
    ));
}

#[test]
fn numbers_print_as_python_prints_them() {
    let cases = [
        (1.0, "1.0"),
        (0.5, "0.5"),
        (0.42, "0.42"),
        (0.0001, "0.0001"),
        (0.00001, "1e-05"),
    ];
    for (value, text) in cases {
        assert_eq!(python_float_text(value), text, "{value}");
    }
    assert_eq!(three_places(Some(0.0625)), "0.062");
    assert_eq!(three_places(Some(0.6875)), "0.688");
    assert_eq!(three_places(Some(rounded(0.687_499_6))), "0.688");
    assert_eq!(three_places(None), "-");
}

#[test]
fn a_decide_tie_between_cuts_goes_to_the_one_nearer_one_half_then_the_smaller() {
    let results = [decide_line("a", 0.45), decide_line("b", 0.54)].join("\n");
    let key = "{\"id\":\"a\",\"value\":true,\"part\":\"tune\"}\n\
               {\"id\":\"b\",\"value\":false,\"part\":\"tune\"}\n";
    let rows = audit_text(&results, key);
    let suggested = rows[0].suggested.as_ref().expect("a suggested cut");
    assert_eq!(suggested.cut, Some(0.45));
}

#[test]
fn a_seeded_split_over_five_ids_tunes_on_two() {
    let ids = ["a", "b", "c", "d", "e"];
    let results: Vec<String> = ids.iter().map(|id| decide_line(id, 0.6)).collect();
    let key: String = ids
        .iter()
        .map(|id| format!("{{\"id\":\"{id}\",\"value\":true}}\n"))
        .collect();
    let rows = audit_text(&results.join("\n"), &key);
    let suggested = rows[0].suggested.as_ref().expect("a suggested cut");
    let tune = suggested.tune.as_ref().expect("a tuning part");
    let held = suggested.held.as_ref().expect("a held part");
    assert_eq!((suggested.split, tune.n, held.n), ("seeded", 2, 3));
}
