//! Prune's selection edges, on weighed answers with no file.

use std::time::Duration;

use super::{Prune, Weighed, select};

const NOW: i64 = 1_000;

/// Answers as (key byte, answered_by, taken_at), each 10 bytes of text.
fn answers(rows: &[(u8, &str, i64)]) -> Vec<Weighed> {
    rows.iter()
        .map(|&(key, answered_by, taken_at)| Weighed {
            key: vec![key],
            model: "jev-latest".to_owned(),
            answered_by: answered_by.to_owned(),
            taken_at,
            text: 10,
        })
        .collect()
}

fn prune(max_size: u64, older_than: Option<u64>, other_than: Option<&str>) -> Prune {
    Prune {
        max_size,
        older_than: older_than.map(Duration::from_secs),
        answered_by_other_than: other_than.map(str::to_owned),
    }
}

#[test]
fn age_is_strict_models_form_a_union_and_the_target_keeps_an_exact_fit() {
    let old = [(1, "jev-1.13.0", 900), (2, "jev-1.13.0", 899)];
    let mixed = [
        (1, "jev-1.13.0", 950),
        (2, "jev-1.14.0", 990),
        (3, "jev-1.13.0", 800),
    ];
    let rows: [(&str, &[(u8, &str, i64)], Prune, u64, &[u8]); 5] = [
        // Exactly 100 seconds old stays; one second older leaves.
        (
            "strict age",
            &old,
            prune(u64::MAX, Some(100), None),
            20,
            &[2],
        ),
        // Each selector adds what it selects.
        (
            "union",
            &mixed,
            prune(u64::MAX, Some(100), Some("jev-1.13.0")),
            30,
            &[3, 2],
        ),
        (
            "model alone",
            &mixed,
            prune(u64::MAX, None, Some("jev-1.13.0")),
            30,
            &[2],
        ),
        // A store exactly at its target keeps every answer.
        ("exact fit", &old, prune(20, None, None), 20, &[]),
        // One byte over removes the oldest answer.
        ("one byte over", &old, prune(19, None, None), 20, &[2]),
    ];
    for (name, rows, options, size, expected) in rows {
        let chosen = select(&answers(rows), &options, NOW, size).expect(name);
        let keys: Vec<u8> = chosen.keys.iter().map(|key| key[0]).collect();
        assert_eq!(keys, expected, "{name}");
    }
}
