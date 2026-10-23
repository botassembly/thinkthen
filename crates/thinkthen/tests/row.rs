#![cfg(test)]
//! Every function uses the same result row contract.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Annotate, Backend, Decide, Engine, Rank, Record, Toolbox};

#[test]
fn row_probabilities_follow_each_function_and_unsure_answers_are_explicit_nulls() {
    let cases = [
        (
            "decide",
            json!({"decide":"Ready?","threshold":"0.2:0.8"}),
            true,
        ),
        ("filter", json!({"filter":"Ready?"}), true),
        (
            "choose",
            json!({"choose":"Which?","options":["a","b"],"threshold":"0.8"}),
            true,
        ),
        (
            "find",
            json!({"find":"Which line?","threshold":"0.8"}),
            true,
        ),
        ("tag", json!({"tag":"Which labels?","labels":["a"]}), false),
        (
            "score",
            json!({"score":"How much?","levels":["low","high"]}),
            false,
        ),
        ("recognize", json!({"recognize":["person"]}), false),
        (
            "relate",
            json!({"relate":[{"name":"knows","source":"person","target":"person"}]}),
            false,
        ),
    ];
    let questions: serde_json::Map<_, _> = cases
        .iter()
        .map(|(name, question, _)| ((*name).to_owned(), question.clone()))
        .collect();
    let toolbox =
        Toolbox::from_bytes(json!({"questions":questions}).to_string().as_bytes()).unwrap();
    let server = Loopback::answering(|q| {
        if q["type"] == "score" {
            json!({"type":"score","probabilities":{"0":0.4,"1":0.6}})
        } else if q["criteria"].get("BEGIN").is_some() {
            json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":0.,"OUT":1.}})
        } else if q["criteria"].get("u1").is_some() {
            json!({"type":"choice","probabilities":{"u1":0.6,"none":0.4}})
        } else if q["type"] == "choice" {
            json!({"type":"choice","probabilities":{"a":0.6,"b":0.4}})
        } else {
            json!({"type":"noul","noul":0.5})
        }
    }).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let engine = Engine::new(backend);
    let mut rows = Vec::new();
    for (name, _, p) in &cases {
        engine
            .call(
                toolbox.question(Some(name)).unwrap(),
                [Record::text("Ada")],
                |row| {
                    rows.push(((*name).to_owned(), serde_json::to_value(row).unwrap(), *p));
                },
            )
            .unwrap();
    }
    engine
        .rank(
            &Rank::Decide(Decide::new("Ready?").unwrap()),
            [(Record::text("Ada"), ())],
            None,
            |(), row| {
                rows.push(("rank".to_owned(), serde_json::to_value(row).unwrap(), false));
            },
        )
        .unwrap();
    engine
        .annotate(&Annotate::new(&toolbox), [Record::text("Ada")], |row| {
            let row = serde_json::to_value(row).unwrap();
            for (name, _, p) in &cases {
                check(&row["answer"][name], *p, name);
            }
            rows.push(("annotate".to_owned(), row, false));
        })
        .unwrap();
    assert_eq!(rows.len(), 10);
    for (name, row, p) in rows {
        check(&row, p, &name);
        if ["decide", "choose", "find"].contains(&name.as_str()) {
            assert_eq!(row["status"], "unsure", "{name}");
        }
    }
}

fn check(row: &Value, p: bool, name: &str) {
    assert_ne!(row["status"], "failed", "{name}: {row}");
    assert_eq!(row.get("p").is_some(), p, "{name}: {row}");
    assert!(row.get("answer").is_some(), "{name}: {row}");
    if row["status"] == "unsure" {
        assert!(row["answer"].is_null(), "{name}: {row}");
    }
}
