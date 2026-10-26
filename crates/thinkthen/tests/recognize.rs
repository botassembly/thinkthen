#![cfg(test)]
//! Valid name paths, membership support and dependent kinds through the SDK.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Cut, Engine, Key, Recognize, Record, Settings};

fn engine(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap()
}

fn reply(q: &Value, tags: &[(&str, [f64; 5])], kind: &[f64]) -> Value {
    let instructions = q["instructions"].as_str().unwrap();
    if q["criteria"].get("BEGIN").is_some() {
        assert!(q["criteria"]["BEGIN"].as_str().is_some());
        let (_, p) = tags
            .iter()
            .find(|(piece, _)| instructions.contains(&format!("[[{piece}]]")))
            .unwrap();
        json!({"type":"choice", "probabilities":{"BEGIN":p[0],"INSIDE":p[1],"END":p[2],"SINGLE":p[3],"OUT":p[4]}})
    } else {
        assert!(instructions.contains("[["));
        if q["criteria"].get("ENTITY").is_some() {
            json!({"type":"choice", "probabilities":{"ENTITY":1.0,"none of these":0.0}})
        } else {
            json!({"type":"choice", "probabilities":{"person":kind[0],"company":kind[1],"none of these":kind[2]}})
        }
    }
}

#[test]
fn names_keep_offsets_kinds_and_cuts() {
    for (text, tags, kinds, expected) in [
        (
            "Ada",
            vec![("Ada", [0., 0., 0., 1., 0.])],
            vec!["ENTITY"],
            vec![("Ada", 0, 3, "ENTITY")],
        ),
        (
            "New York",
            vec![
                ("New", [1., 0., 0., 0., 0.]),
                ("York", [0., 0., 1., 0., 0.]),
            ],
            vec!["ENTITY"],
            vec![("New York", 0, 8, "ENTITY")],
        ),
        (
            "🙂 Éva met Acme.",
            vec![
                ("🙂", [0., 0., 0., 0., 1.]),
                ("Éva", [0., 0., 0., 1., 0.]),
                ("met", [0., 0., 0., 0., 1.]),
                ("Acme", [0., 0., 0., 1., 0.]),
                (".", [0., 0., 0., 0., 1.]),
            ],
            vec!["person", "company"],
            vec![("Éva", 2, 5, "person"), ("Acme", 10, 14, "company")],
        ),
        (
            "word",
            vec![("word", [0., 0., 0., 0., 1.])],
            vec!["person", "company"],
            vec![],
        ),
        (
            "Ada",
            vec![("Ada", [0., 0., 0., 1., 0.])],
            vec!["person", "company"],
            vec![],
        ),
        // Positional uncertainty does not discard strong name membership.
        (
            "x",
            vec![("x", [0.05, 0.7, 0.05, 0.15, 0.05])],
            vec!["ENTITY"],
            vec![("x", 0, 1, "ENTITY")],
        ),
    ] {
        let drop = text == "Ada" && kinds != ["ENTITY"];
        let tags_for_server = tags.clone();
        let server = Loopback::answering(move |q| {
            reply(
                q,
                &tags_for_server,
                if drop {
                    &[0.1, 0.1, 0.8]
                } else if q["instructions"].as_str().unwrap().contains("[[Acme]]") {
                    &[0.1, 0.8, 0.1]
                } else {
                    &[0.8, 0.1, 0.1]
                },
            )
        })
        .unwrap();
        let question = Recognize::new(kinds.clone()).unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(&question, [Record::text(text)])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let row = &rows[0];
        assert_eq!(row["status"], "answered");
        assert!(row.get("probability").is_none());
        let names = row["answer"].as_array().unwrap();
        assert_eq!(names.len(), expected.len(), "{row}");
        for (name, (text, start, end, kind)) in names.iter().zip(expected) {
            assert_eq!(name["text"], text);
            assert_eq!(name["start"], start);
            assert_eq!(name["end"], end);
            assert_eq!(name["kind"], kind);
            assert!(name["probability"].as_f64().unwrap() >= 0.5);
        }
        assert_eq!(
            row["detail"]["pieces"].as_array().unwrap().len(),
            tags.len()
        );
        let second = tags.iter().any(|(_, p)| p[3] > 0.5) || text == "New York" || text == "x";
        assert_eq!(server.count(), 1 + usize::from(second));
    }
}

#[test]
fn least_piece_membership_multiplies_kind_probability_before_the_cut() {
    let server = Loopback::answering(|q| {
        reply(
            q,
            &[("Ada", [0.15, 0.15, 0.2, 0.3, 0.2])],
            &[0.625, 0.2, 0.175],
        )
    })
    .unwrap();
    // Membership is 0.8, so kind probability 0.625 gives support 0.5.
    for (cut, kept) in [("0.5", true), ("0.50001", false)] {
        let question = Recognize::new(["person", "company"])
            .unwrap()
            .threshold(Cut::new(cut.parse().unwrap()).unwrap());
        let mut rows = Vec::new();
        engine(&server)
            .rows(&question, [Record::text("Ada")])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        assert_eq!(
            rows[0]["answer"].as_array().unwrap().len(),
            usize::from(kept)
        );
        if kept {
            assert!((rows[0]["answer"][0]["probability"].as_f64().unwrap() - 0.5).abs() < 1e-12);
        }
    }
}

#[test]
fn toolbox_empty_text_and_local_limits_keep_record_rows() {
    let server = Loopback::answering(|q| reply(q, &[("Ada", [0., 0., 0., 1., 0.])], &[])).unwrap();
    let question = Recognize::new(["ENTITY"]).unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(
            &question,
            [
                Record::text(""),
                Record::text("Ada"),
                Record::text("x".repeat(100_000)),
            ],
        )
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["answer"], json!([]));
    assert_eq!(rows[1]["answer"][0]["text"], "Ada");
    assert_eq!(rows[2]["error"]["code"], "record.too-large");
    assert_eq!(server.count(), 2);
}

#[test]
fn long_names_keep_the_weakest_piece_support_without_inflation() {
    let server = Loopback::answering(|_| json!({"type":"choice", "probabilities":{"BEGIN":0.1225,"INSIDE":0.1225,"END":0.1225,"SINGLE":0.1225,"OUT":0.51}})).unwrap();
    let question = Recognize::new(["ENTITY"]).unwrap();
    let mut rows = Vec::new();
    // All OUT-majority pieces stay outside, however long the input is.
    engine(&server)
        .rows(&question, [Record::text("x ".repeat(2000))])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["answer"], json!([]));
    let server = Loopback::answering(|q| {
        if q["criteria"].get("ENTITY").is_some() { return json!({"type":"choice","probabilities":{"ENTITY":1.0,"none of these":0.0}}); }
        let weak = q["instructions"].as_str().unwrap().contains("[[weak]]");
        let outside = if weak { 0.375 } else { 0.01 };
        let start = q["instructions"].as_str().unwrap().contains("[[start]]");
        json!({"type":"choice", "probabilities":{"BEGIN":if start {1.0-outside} else {0.0},"INSIDE":if !start && !weak {1.0-outside} else {0.0},"END":if weak {1.0-outside} else {0.0},"SINGLE":0.0,"OUT":outside}})
    }).unwrap();
    let mut rows = Vec::new();
    for text in [
        "start weak".to_owned(),
        format!("start {}weak", "x ".repeat(2000)),
    ] {
        engine(&server)
            .rows(&question, [Record::text(text.clone())])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let row = rows.last().unwrap();
        assert_eq!(row["answer"].as_array().unwrap().len(), 1);
        assert_eq!(row["answer"][0]["text"], text);
        assert_eq!(row["answer"][0]["probability"], 0.625);
    }
}

#[test]
fn positional_spans_join_weak_boundaries_and_keep_supported_boundaries() {
    for (text, tags, expected) in [
        (
            "Nacre!",
            vec![
                ("Nacre", [0.34, 0.20, 0.08, 0.27, 0.11]),
                ("!", [0., 0., 0., 1., 0.]),
            ],
            vec![("Nacre!", 0.89)],
        ),
        (
            "Nacre!",
            vec![("Nacre", [0., 0., 0., 1., 0.]), ("!", [0., 0., 0., 0., 1.])],
            vec![("Nacre", 1.)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0.5, 0., 0., 0.5, 0.]),
                ("Skiff", [0., 0., 0., 1., 0.]),
            ],
            vec![("Nacre Skiff", 1.)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0., 0., 0., 1., 0.]),
                ("Skiff", [0., 0.5, 0., 0.5, 0.]),
            ],
            vec![("Nacre Skiff", 1.)],
        ),
        (
            "Nacre Skiff Cedar",
            vec![
                ("Nacre", [0.34, 0.20, 0.08, 0.27, 0.11]),
                ("Skiff", [0.16, 0.16, 0.29, 0.39, 0.0]),
                ("Cedar", [0., 0., 0., 1., 0.]),
            ],
            vec![("Nacre Skiff", 0.89), ("Cedar", 1.)],
        ),
        (
            "Nacre\u{2003}Skiff",
            vec![
                ("Nacre", [0.34, 0.20, 0.08, 0.27, 0.11]),
                ("Skiff", [0.16, 0.16, 0.29, 0.39, 0.0]),
            ],
            vec![("Nacre\u{2003}Skiff", 0.89)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0., 0., 0., 0.8, 0.2]),
                ("Skiff", [0.05, 0.4, 0.3, 0.25, 0.]),
            ],
            vec![("Nacre Skiff", 0.8)],
        ),
        (
            "Nacre-Skiff",
            vec![
                ("Nacre", [0., 0., 0., 1., 0.]),
                ("-", [0.15, 0.15, 0.15, 0.15, 0.4]),
                ("Skiff", [0., 0., 0., 1., 0.]),
            ],
            vec![("Nacre", 1.), ("Skiff", 1.)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0.34, 0.20, 0.08, 0.27, 0.11]),
                ("Skiff", [0.16, 0.16, 0.29, 0.39, 0.0]),
            ],
            vec![("Nacre Skiff", 0.89)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0.20, 0.12, 0.18, 0.40, 0.10]),
                ("Skiff", [0.12, 0.12, 0.43, 0.17, 0.16]),
            ],
            vec![("Nacre Skiff", 0.84)],
        ),
        (
            "Nacre Skiff 707",
            vec![
                ("Nacre", [0.44, 0.37, 0.03, 0.05, 0.11]),
                ("Skiff", [0.15, 0.41, 0.19, 0.04, 0.21]),
                ("707", [0.09, 0.05, 0.18, 0.22, 0.46]),
            ],
            vec![("Nacre Skiff", 0.79)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0., 0., 0., 1., 0.]),
                ("Skiff", [0., 0., 0., 1., 0.]),
            ],
            vec![("Nacre", 1.), ("Skiff", 1.)],
        ),
        (
            "Nacre Skiff Cedar Ferry",
            vec![
                ("Nacre", [1., 0., 0., 0., 0.]),
                ("Skiff", [0., 0., 1., 0., 0.]),
                ("Cedar", [1., 0., 0., 0., 0.]),
                ("Ferry", [0., 0., 1., 0., 0.]),
            ],
            vec![("Nacre Skiff", 1.), ("Cedar Ferry", 1.)],
        ),
        (
            "Nacre passed Skiff",
            vec![
                ("Nacre", [0., 0., 0., 1., 0.]),
                ("passed", [0., 0., 0., 0.4, 0.6]),
                ("Skiff", [0., 0., 0., 1., 0.]),
            ],
            vec![("Nacre", 1.), ("Skiff", 1.)],
        ),
        (
            "vessel Nacre Skiff nearby",
            vec![
                ("vessel", [0.2, 0.2, 0.1, 0.1, 0.4]),
                ("Nacre", [0.9, 0., 0., 0., 0.1]),
                ("Skiff", [0., 0., 0.9, 0., 0.1]),
                ("nearby", [0.1, 0.1, 0.2, 0.2, 0.4]),
            ],
            vec![("Nacre Skiff", 0.9)],
        ),
        (
            "Nacre's Skiff",
            vec![
                ("Nacre", [0.95, 0., 0., 0., 0.05]),
                ("'", [0., 0.95, 0., 0., 0.05]),
                ("s", [0., 0.95, 0., 0., 0.05]),
                ("Skiff", [0., 0., 0.95, 0., 0.05]),
            ],
            vec![("Nacre's Skiff", 0.95)],
        ),
        (
            "Nacre Skiff",
            vec![
                ("Nacre", [0.25, 0.25, 0.25, 0.25, 0.]),
                ("Skiff", [0.25, 0.25, 0.25, 0.25, 0.]),
            ],
            vec![("Nacre Skiff", 1.)],
        ),
    ] {
        let server = Loopback::answering(move |q| reply(q, &tags, &[])).unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(&Recognize::new(["ENTITY"]).unwrap(), [Record::text(text)])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(row));
        let thinkthen::Outcome::Answered { answer: names, .. } = rows[0].outcome() else {
            panic!("expected names")
        };
        assert_eq!(names.len(), expected.len(), "{text}");
        for (name, (text, p)) in names.iter().zip(expected) {
            assert_eq!(name.text(), text);
            assert!((name.probability().get() - p).abs() < 1e-12);
        }
    }
}

#[test]
fn only_marks_touching_both_names_can_bridge_an_outside_piece() {
    for (middle, outside, expected) in [
        ("'", 0.4, 1),
        ("-", 0.4, 1),
        ("’", 0.4, 1),
        ("'", 0.6, 0),
        (" (", 0.4, 2),
        ("- ", 0.4, 2),
        (" -", 0.4, 2),
        (" passed ", 0.4, 2),
    ] {
        let mark = middle.trim();
        let server = Loopback::answering(move |q| {
            reply(
                q,
                &[
                    ("Nacre", [0.34, 0.20, 0.08, 0.27, 0.11]),
                    (
                        mark,
                        [
                            (1.0 - outside) / 4.0,
                            (1.0 - outside) / 4.0,
                            (1.0 - outside) / 4.0,
                            (1.0 - outside) / 4.0,
                            outside,
                        ],
                    ),
                    ("Skiff", [0.16, 0.16, 0.29, 0.39, 0.0]),
                ],
                &[],
            )
        })
        .unwrap();
        let text = format!("Nacre{middle}Skiff");
        let mut rows = Vec::new();
        engine(&server)
            .rows(
                &Recognize::new(["ENTITY"]).unwrap(),
                [Record::text(text.clone())],
            )
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let names = rows[0]["answer"].as_array().unwrap();
        assert_eq!(names.len(), expected, "{text}: {names:?}");
        if expected == 1 {
            assert_eq!(names[0]["text"], text);
            assert_eq!(names[0]["probability"], 0.6);
        } else if expected == 2 {
            assert_eq!(names[0]["text"], "Nacre");
            assert_eq!(names[1]["text"], "Skiff");
        }
    }
}

#[test]
fn invalid_kinds_fail_explicitly() {
    for kinds in [vec![""], vec!["person", "person"], vec!["none of these"]] {
        assert_eq!(
            Recognize::new(kinds).unwrap_err().code().as_str(),
            "usage.question"
        );
    }
}

#[test]
fn failed_rounds_stop_each_record_without_partial_names() {
    for fail_boundary in [false, true] {
        let server = Loopback::answering(move |q| {
            let boundary = q["criteria"].get("BEGIN").is_some();
            if boundary == fail_boundary { json!({"type":"choice", "probabilities":{}}) }
            else { json!({"type":"choice", "probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":1.,"OUT":0.}}) }
        }).unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(
                &Recognize::new(["person"]).unwrap(),
                [Record::text("Ada"), Record::text("Ada")],
            )
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        assert_eq!(rows.len(), 2);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row["position"], i + 1);
            assert_eq!(row["error"]["code"], "backend.reply");
            assert!(row.get("answer").is_none());
        }
        assert_eq!(server.count(), if fail_boundary { 1 } else { 2 });
    }
}

#[test]
fn unicode_pieces_preserve_names_and_original_scalar_offsets() {
    for (text, expected, start, end) in [
        ("A\u{301}da met Acme.", "A\u{301}da", 0, 4),
        ("\u{feff}Ada met Acme.", "Ada", 1, 4),
        (
            "\u{feff}\u{200b}\u{200c}\u{200d}\u{2060}Ada met Acme.",
            "Ada",
            5,
            8,
        ),
    ] {
        let question = Recognize::new(["ENTITY"]).unwrap();
        let expected_owned = expected.to_owned();
        let server = Loopback::answering(move |q| {
            reply(
                q,
                &[
                    (&expected_owned, [0., 0., 0., 1., 0.]),
                    ("met", [0., 0., 0., 0., 1.]),
                    ("Acme", [0., 0., 0., 0., 1.]),
                    (".", [0., 0., 0., 0., 1.]),
                ],
                &[],
            )
        })
        .unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(&question, [Record::text(text)])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let names = rows[0]["answer"].as_array().unwrap();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0]["text"], expected);
        assert_eq!(names[0]["start"], start);
        assert_eq!(names[0]["end"], end);
    }
    for (text, expected) in [
        ("a_b+$", vec!["a", "_", "b", "+", "$"]),
        (
            "!\u{301}\u{20dd}\u{200d}a",
            vec!["!\u{301}\u{20dd}\u{200d}", "a"],
        ),
        (" \u{301}a", vec!["\u{301}a"]),
        ("a\u{200b}b", vec!["a\u{200b}b"]),
        ("a\u{0903}b\u{e000}c", vec!["a\u{0903}b\u{e000}c"]),
        (" \u{feff}a", vec!["\u{feff}a"]),
        ("\u{feff}\u{2060}", vec![]),
    ] {
        let q = Recognize::new(["ENTITY"]).unwrap();
        let server = Loopback::answering(|q| {
            let probabilities = q["criteria"]
                .as_object()
                .unwrap()
                .keys()
                .map(|label| (label.clone(), json!(if label == "OUT" { 1.0 } else { 0.0 })))
                .collect::<serde_json::Map<_, _>>();
            json!({"type":"choice", "probabilities":probabilities})
        })
        .unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(&q, [Record::text(text)])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let row = &rows[0];
        let pieces: Vec<_> = row["detail"]["pieces"]
            .as_array()
            .unwrap()
            .iter()
            .map(|piece| piece["text"].as_str().unwrap())
            .collect();
        assert_eq!(pieces, expected, "{text:?}");
    }
}

#[test]
fn decode_ties_choose_the_earliest_previous_tag_at_each_step() {
    let server = Loopback::answering(|q| {
        reply(
            q,
            &[
                ("A", [1., 1., 0.5, 0.5, 1.]),
                ("B", [0.5, 0.5, 0.5, 0., 0.]),
                ("C", [0.5, 0.5, 0.5, 0.5, 0.]),
            ],
            &[],
        )
    })
    .unwrap();
    let question = Recognize::new(["ENTITY"])
        .unwrap()
        .threshold(Cut::new(0.01).unwrap());
    let mut rows = Vec::new();
    engine(&server)
        .rows(&question, [Record::text("A B C")])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    let names = rows[0]["answer"].as_array().unwrap();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0]["text"], "B C");
    assert_eq!(names[0]["start"], 2);
    assert_eq!(names[0]["end"], 5);
}

#[test]
fn names_require_a_unicode_letter_or_decimal_digit() {
    for (text, pieces, kept) in [
        ("!", vec!["!"], false),
        ("★", vec!["★"], false),
        ("\u{301}", vec!["\u{301}"], false),
        ("⅓", vec!["⅓"], false),
        ("É", vec!["É"], true),
        ("東京", vec!["東京"], true),
        ("７", vec!["７"], true),
        ("٣", vec!["٣"], true),
        ("A\u{301}", vec!["A\u{301}"], true),
        ("Lark!", vec!["Lark", "!"], true),
    ] {
        for kinds in [vec!["ENTITY"], vec!["person", "company"]] {
            let tags: Vec<_> = pieces
                .iter()
                .enumerate()
                .map(|(i, piece)| {
                    let mut probabilities = [0.; 5];
                    probabilities[if pieces.len() == 1 {
                        3
                    } else if i == 0 {
                        0
                    } else {
                        2
                    }] = 1.;
                    (*piece, probabilities)
                })
                .collect();
            let server = Loopback::answering(move |q| reply(q, &tags, &[1., 0., 0.])).unwrap();
            let mut rows = Vec::new();
            engine(&server)
                .rows(
                    &Recognize::new(kinds.clone()).unwrap(),
                    [Record::text(text)],
                )
                .unwrap()
                .map(|row| row.unwrap())
                .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0]["status"], "answered");
            let names = rows[0]["answer"].as_array().unwrap();
            assert_eq!(names.len(), usize::from(kept), "{text}: {names:?}");
            assert_eq!(
                rows[0]["detail"]["names"].as_array().unwrap().len(),
                usize::from(kept && !kinds.is_empty())
            );
            if kept {
                assert_eq!(names[0]["text"], text);
                assert_eq!(names[0]["start"], 0);
                assert_eq!(names[0]["end"], text.chars().count());
                assert_eq!(names[0]["probability"], 1.);
            }
        }
    }
}

#[test]
fn dependent_rounds_preserve_names_through_split_retry_and_deduplication() {
    use loopback::Reply;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use thinkthen::Retry;
    let retried = AtomicBool::new(false);
    let server = Loopback::start(move |request| {
        let body = request.json().unwrap();
        let questions = body["questions"].as_object().unwrap();
        let boundary = questions.values().next().unwrap()["criteria"].get("BEGIN").is_some();
        if boundary && questions.len() > 1 {return Reply::json(413, &json!({}));}
        if !boundary && !retried.swap(true, Ordering::SeqCst) {return Reply::json(503, &json!({}));}
        let answers = questions.keys().map(|key| (key.clone(), if boundary {
            json!({"type":"choice","probabilities":{"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"SINGLE":0.9,"OUT":0.1}})
        } else {json!({"type":"choice","probabilities":{"person":0.8,"none of these":0.2}})})).collect::<serde_json::Map<_,_>>();
        Reply::json(200, &json!({"answers":answers}))
    }).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .retry(Retry::new(1, Duration::ZERO, Duration::from_secs(5)).unwrap())
        .engine(|_| None)
        .unwrap();
    let rows = engine
        .rows(
            &Recognize::new(["person"]).unwrap(),
            ["Ada Bea", "Ada Bea", "Ada Bea", "Ada Bea"].map(Record::text),
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 4);
    for (position, row) in rows.into_iter().enumerate() {
        assert_eq!(row.position(), (position + 1) as u64);
        let names = row.into_result().unwrap().unwrap();
        assert_eq!(
            names
                .iter()
                .map(|name| name.text().as_str())
                .collect::<Vec<_>>(),
            ["Ada", "Bea"]
        );
        assert!(
            names
                .iter()
                .all(|name| (name.probability().get() - 0.72).abs() < 1e-12)
        );
    }
    assert_eq!(server.count(), 5);
}
