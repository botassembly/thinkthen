#![cfg(test)]
//! Name punctuation and category descriptions through the public SDK.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Key, Recognize, Record, Settings};

#[test]
fn names_complete_only_touching_brackets_opened_inside_the_span() {
    for (text, tags, expected) in [
        (
            "the record Lantern",
            vec![("the", "OUT"), ("record", "OUT"), ("Lantern", "SINGLE")],
            "Lantern",
        ),
        (
            "the record Echo (Live)",
            vec![
                ("the", "OUT"),
                ("record", "SINGLE"),
                ("Echo", "BEGIN"),
                ("(", "INSIDE"),
                ("Live", "END"),
                (")", "OUT"),
            ],
            "Echo (Live)",
        ),
        (
            "Echo [Live]",
            vec![
                ("Echo", "BEGIN"),
                ("[", "INSIDE"),
                ("Live", "END"),
                ("]", "OUT"),
            ],
            "Echo [Live]",
        ),
        (
            "Echo {[Live]}",
            vec![
                ("Echo", "BEGIN"),
                ("{", "INSIDE"),
                ("[", "INSIDE"),
                ("Live", "END"),
                ("]", "OUT"),
                ("}", "OUT"),
            ],
            "Echo {[Live]}",
        ),
        (
            "Echo (Live]",
            vec![
                ("Echo", "BEGIN"),
                ("(", "INSIDE"),
                ("Live", "END"),
                ("]", "OUT"),
            ],
            "Echo (Live",
        ),
        (
            "Echo ] (Live)",
            vec![
                ("Echo", "BEGIN"),
                ("]", "INSIDE"),
                ("(", "INSIDE"),
                ("Live", "END"),
                (")", "OUT"),
            ],
            "Echo ] (Live",
        ),
        (
            "(Echo)",
            vec![("(", "OUT"), ("Echo", "SINGLE"), (")", "OUT")],
            "Echo",
        ),
        (
            "Echo (Live)",
            vec![
                ("Echo", "BEGIN"),
                ("(", "INSIDE"),
                ("Live", "INSIDE"),
                (")", "END"),
            ],
            "Echo (Live)",
        ),
        (
            "Echo (Live )",
            vec![
                ("Echo", "BEGIN"),
                ("(", "INSIDE"),
                ("Live", "END"),
                (")", "OUT"),
            ],
            "Echo (Live",
        ),
        (
            "💿 Écho (Lïve)",
            vec![
                ("💿", "OUT"),
                ("Écho", "BEGIN"),
                ("(", "INSIDE"),
                ("Lïve", "END"),
                (")", "OUT"),
            ],
            "Écho (Lïve)",
        ),
        (
            "Echo ([Live]",
            vec![
                ("Echo", "BEGIN"),
                ("(", "INSIDE"),
                ("[", "INSIDE"),
                ("Live", "END"),
                ("]", "OUT"),
            ],
            "Echo ([Live]",
        ),
        (
            "Echo (Live) tail",
            vec![
                ("Echo", "BEGIN"),
                ("(", "INSIDE"),
                ("Live", "END"),
                (")", "SINGLE"),
                ("tail", "OUT"),
            ],
            "Echo (Live",
        ),
        (
            "Record House",
            vec![("Record", "BEGIN"), ("House", "END")],
            "Record House",
        ),
    ] {
        // The descriptive word still reaches kind rejection; a bare closing mark does not.
        let expected_kind_count = if text == "the record Echo (Live)" {
            2
        } else {
            1
        };
        let server = Loopback::answering(move |question| {
            let words = question["instructions"].as_str().unwrap();
            if question["criteria"].get("BEGIN").is_some() {
                assert!(words.contains("A category word before a name is OUT"));
                let (_, tag) = tags.iter().find(|(piece,_)| words.contains(&format!("[[{piece}]]"))).unwrap();
                let probabilities: serde_json::Map<String, Value> = ["BEGIN","INSIDE","END","SINGLE","OUT"].into_iter()
                    .map(|label| (label.to_owned(), json!(f64::from(label == *tag)))).collect();
                json!({"type":"choice","probabilities":probabilities})
            } else {
                assert!(words.contains("A word that merely names a category is not a proper name"));
                assert!(question["criteria"]["none of these"].as_str().unwrap().contains("merely describe a category"));
                let category = words.contains("[[record]]") || words.contains("[[)]]");
                json!({"type":"choice","probabilities":{"work":if category {0.} else {0.8},"none of these":if category {1.} else {0.2}}})
            }
        }).unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let rows: Vec<_> = engine
            .rows(&Recognize::new(["work"]).unwrap(), [Record::text(text)])
            .unwrap()
            .map(|row| serde_json::to_value(row.unwrap()).unwrap())
            .collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["status"], "answered");
        let names = rows[0]["answer"].as_array().unwrap();
        assert_eq!(names.len(), 1, "{text}: {names:?}");
        let start = text[..text.find(expected).unwrap()].chars().count();
        let end = start + expected.chars().count();
        assert_eq!(
            names[0],
            json!({"text":expected,"start":start,"end":end,"kind":"work","probability":0.8})
        );
        assert!(
            rows[0]["detail"]["names"]
                .as_array()
                .unwrap()
                .iter()
                .any(|name| {
                    name["start"] == start && name["end"] == end && name["kinds"]["work"] == 0.8
                })
        );
        let asks = server.requests();
        let kind_questions: Vec<_> = asks
            .iter()
            .flat_map(|request| {
                let body = request.json().unwrap();
                body["questions"]
                    .as_object()
                    .unwrap()
                    .values()
                    .filter(|question| question["criteria"].get("work").is_some())
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(kind_questions.len(), expected_kind_count);
        assert!(kind_questions.iter().any(|question| {
            question["instructions"]
                .as_str()
                .unwrap()
                .contains(&format!("[[{expected}]]"))
        }));
    }
}
