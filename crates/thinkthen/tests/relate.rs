#![cfg(test)]
//! Stated relation rules, ordering and all dependent rounds through the SDK.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use thinkthen::{Annotate, Engine, Key, Question, Record, Relate, RelationRule, Settings};

fn engine(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap()
}
fn answer(q: &Value, p: f64, empty: bool) -> Value {
    let words = q["instructions"].as_str().unwrap();
    if q["criteria"].get("BEGIN").is_some() {
        let named = !empty
            && ["Ada", "Áda", "Bea", "Acme"]
                .iter()
                .any(|name| words.contains(&format!("[[{name}]]")));
        json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":if named {1.} else {0.},"OUT":if named {0.} else {1.}}})
    } else if q["criteria"].get("person").is_some() {
        let company = words.contains("[[Acme]]");
        let mut probabilities = serde_json::Map::new();
        for kind in q["criteria"].as_object().unwrap().keys() {
            probabilities.insert(
                kind.clone(),
                json!(
                    if (kind == "company" && company) || (kind == "person" && !company) {
                        1.
                    } else {
                        0.
                    }
                ),
            );
        }
        json!({"type":"choice","probabilities":probabilities})
    } else {
        json!({"type":"noul","noul":p})
    }
}
#[test]
fn rules_ask_only_allowed_unique_pairs_and_keep_cut_inclusive() {
    for (text, rule, p, empty, asked, kept) in [
        (
            "Ada joined Acme",
            "works_for=person:company",
            0.5,
            false,
            1,
            1,
        ),
        (
            "Acme hired Ada",
            "works_for=person:company",
            0.49,
            false,
            1,
            0,
        ),
        (
            "Acme hired Ada",
            "works_for=person:company",
            0.8,
            false,
            1,
            1,
        ),
        ("Ada met Bea", "knows=person:person", 0.9, false, 2, 2),
        ("Ada met Bea", "knows=person:person", 0.9, true, 0, 0),
        (
            "Ada Ada joined Acme",
            "works_for=person:company",
            0.9,
            false,
            1,
            1,
        ),
        ("Ada", "knows=person:person", 0.9, false, 0, 0),
        ("", "works_for=person:company", 0.9, false, 0, 0),
    ] {
        let questions = Arc::new(Mutex::new(Vec::new()));
        let captured = questions.clone();
        let server = Loopback::answering(move |q| {
            if q.get("criteria").is_none() {
                captured
                    .lock()
                    .unwrap()
                    .push(q["instructions"].as_str().unwrap().to_owned());
            }
            answer(q, p, empty)
        })
        .unwrap();
        let (relation, kinds) = rule.split_once('=').unwrap();
        let (source, target) = kinds.split_once(':').unwrap();
        let question =
            Relate::new(vec![RelationRule::new(relation, source, target).unwrap()]).unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(&question, [Record::text(text)])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let row = &rows[0];
        assert_eq!(row["status"], "answered", "{row}");
        assert!(row.get("probability").is_none());
        assert_eq!(row["answer"].as_array().unwrap().len(), kept);
        assert_eq!(row["detail"]["pairs"].as_array().unwrap().len(), asked);
        let questions = questions.lock().unwrap();
        assert_eq!(questions.len(), asked);
        for (i, question) in questions.iter().enumerate() {
            let statement = question
                .strip_prefix(&format!(
                    "The text is {}. Does the text itself state that ",
                    serde_json::to_string(text).unwrap()
                ))
                .unwrap()
                .strip_suffix('?')
                .unwrap();
            let (source, target) = statement
                .split_once(if rule.starts_with("works_for=") {
                    " works for "
                } else {
                    " knows "
                })
                .unwrap();
            let source: Value = serde_json::from_str(source).unwrap();
            let target: Value = serde_json::from_str(target).unwrap();
            let expected = if rule.starts_with("works_for=") {
                ("Ada", "Acme")
            } else if i == 0 {
                ("Ada", "Bea")
            } else {
                ("Bea", "Ada")
            };
            assert_eq!(source["text"], expected.0);
            assert_eq!(target["text"], expected.1);
            assert_eq!(source["kind"], "person");
            assert_eq!(
                target["kind"],
                if rule.starts_with("works_for=") {
                    "company"
                } else {
                    "person"
                }
            );
        }
        if kept > 0 {
            let relation = &row["answer"][0];
            assert_eq!(relation["source"]["text"], "Ada");
            let start = text.split("Ada").next().unwrap().chars().count();
            assert_eq!(relation["source"]["start"], start);
            assert_eq!(relation["source"]["end"], start + 3);
            assert_eq!(relation["probability"], p);
            assert!(relation["source"].get("probability").is_none());
            assert!(relation.get("either").is_none());
        }
    }
}
#[test]
fn unordered_pairs_keep_text_order_and_rule_order() {
    let questions = Arc::new(Mutex::new(Vec::new()));
    let captured = questions.clone();
    let server = Loopback::answering(move |q| {
        if q.get("criteria").is_none() {
            captured
                .lock()
                .unwrap()
                .push(q["instructions"].as_str().unwrap().to_owned());
        }
        answer(q, 0.8, false)
    })
    .unwrap();
    let question = Relate::new(vec![
        RelationRule::new("works_for", "person", "company")
            .unwrap()
            .either(true),
        RelationRule::new("near", "company", "person")
            .unwrap()
            .either(true)
            .reads("is near")
            .unwrap(),
    ])
    .unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(&question, [Record::text("Acme hired Ada")])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["answer"].as_array().unwrap().len(), 2);
    for (relation, name) in rows[0]["answer"]
        .as_array()
        .unwrap()
        .iter()
        .zip(["works_for", "near"])
    {
        assert_eq!(relation["relation"], name);
        assert_eq!(relation["source"]["text"], "Acme");
        assert_eq!(relation["target"]["text"], "Ada");
        assert_eq!(relation["either"], true);
    }
    let questions = questions.lock().unwrap();
    assert_eq!(questions.len(), 2);
    for (question, reads) in questions.iter().zip(["works for", "is near"]) {
        let statement = question
            .split_once("Does the text itself state that ")
            .unwrap()
            .1
            .strip_suffix('?')
            .unwrap();
        let (forward, reverse) = statement.split_once(", or that ").unwrap();
        for (statement, source, target) in [(forward, "Acme", "Ada"), (reverse, "Ada", "Acme")] {
            let (one, two) = statement.split_once(&format!(" {reads} ")).unwrap();
            let one: Value = serde_json::from_str(one).unwrap();
            let two: Value = serde_json::from_str(two).unwrap();
            assert_eq!(one["text"], source);
            assert_eq!(two["text"], target);
        }
    }
}
#[test]
fn toolbox_relations_mix_with_independent_and_two_round_annotations() {
    let toolbox = serde_json::from_str::<Annotate>(r#"{"questions":{"decision":{"function":"decide","question":"Is hiring stated?"},"names":{"function":"recognize","kinds":["person","company"]},"relations":{"function":"relate","rules":[{"relation":"works_for","source":"person","target":"company"}]}}}"#).unwrap();
    let server = Loopback::answering(|q| answer(q, 0.8, false)).unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(
            &toolbox,
            [Record::text("Ada joined Acme"), Record::text("nobody")],
        )
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["answer"]["decision"]["answer"], true);
    assert_eq!(
        rows[0]["answer"]["names"]["answer"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        rows[0]["answer"]["relations"]["answer"][0]["relation"],
        "works_for"
    );
    assert_eq!(rows[1]["answer"]["relations"]["answer"], json!([]));
}
#[test]
fn invalid_rules_toolbox_fields_and_thresholds_fail_before_calls() {
    for (relation, source, target) in [
        ("", "person", "company"),
        ("near", "", "company"),
        ("near", "person", ""),
        ("near", "*", "company"),
    ] {
        assert_eq!(
            RelationRule::new(relation, source, target)
                .unwrap_err()
                .code()
                .as_str(),
            "usage.question"
        );
    }
    assert!(Relate::new(vec![]).is_err());
    let rule = RelationRule::new("near", "person", "person").unwrap();
    assert!(Relate::new(vec![rule.clone(), rule.clone()]).is_err());
    assert!(Question::from_json(r#"{"function":"relate","rules":[{"relation":"near","source":"person","target":"person"}],"threshold":{"low":0.2,"high":0.8}}"#).is_err());
    for body in [
        r#"{"function":"relate","rules":[]}"#,
        r#"{"function":"relate","rules":[{"relation":"near","source":"person","target":"person","reads":" "}]}"#,
        r#"{"function":"relate","rules":[{"relation":"near","source":"person","target":"person","single":true}]}"#,
        r#"{"function":"relate","rules":[{"relation":"near","source":"person"}]}"#,
        r#"{"function":"relate","rules":[{"relation":"near","source":"person","source":"place","target":"person"}]}"#,
        r#"{"function":"relate","rules":[{"relation":"near","source":"person","target":"person","reads":""}]}"#,
        r#"{"function":"relate","rules":[{"relation":"near","source":"person","target":"person"}],"options":[]}"#,
    ] {
        assert_eq!(
            Question::from_json(body).unwrap_err().code().as_str(),
            "usage.question"
        );
    }
}
#[test]
fn failed_relation_round_has_no_partial_answer() {
    let server = Loopback::answering(|q| {
        if q.get("criteria").is_none() {
            json!({"type":"choice","probabilities":{}})
        } else {
            answer(q, 0.8, false)
        }
    })
    .unwrap();
    let question = Relate::new(vec![
        RelationRule::new("works_for", "person", "company").unwrap(),
    ])
    .unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(&question, [Record::text("Ada joined Acme")])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["error"]["code"], "backend.reply");
    assert!(rows[0].get("answer").is_none());
}

#[test]
fn annotation_rounds_keep_each_selected_text_and_failure_separate() {
    let toolbox = serde_json::from_str::<Annotate>(r#"{"questions":{"decision":{"function":"decide","question":"Is hiring stated?","evidence":"/decision"},"names":{"function":"recognize","kinds":["person","company"],"evidence":"/names"},"relations":{"function":"relate","rules":[{"relation":"works_for","source":"person","target":"company"}],"evidence":"/relations"},"missing":{"function":"relate","rules":[{"relation":"near","source":"person","target":"company"}],"evidence":"/absent"}}}"#).unwrap();
    let server = Loopback::answering(|q| {
        let words = q["instructions"].as_str().unwrap();
        if q.get("criteria").is_none() {
            assert!(
                words.contains("hiring stated")
                    || words.starts_with("The text is \"é Áda joined Acme\".")
            );
        }
        answer(q, 0.8, false)
    })
    .unwrap();
    let record = json!({"decision":"hiring", "names":"Bea", "relations":"é Áda joined Acme"});
    let mut rows = Vec::new();
    engine(&server)
        .rows(&toolbox, [Record::from_json(&record.to_string())])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    let entries = &rows[0]["answer"];
    assert_eq!(entries["decision"]["answer"], true);
    assert_eq!(entries["names"]["answer"][0]["text"], "Bea");
    assert_eq!(entries["names"]["answer"].as_array().unwrap().len(), 1);
    assert_eq!(entries["missing"]["status"], "failed");
    let relation = &entries["relations"]["answer"][0];
    assert_eq!(
        relation["source"],
        json!({"text":"Áda", "start":2, "end":5, "kind":"person"})
    );
    assert_eq!(
        relation["target"],
        json!({"text":"Acme", "start":13, "end":17, "kind":"company"})
    );
}

#[test]
fn membership_support_admits_names_before_relations_keep_their_own_probability() {
    for (membership, expected) in [(0.8, 1), (0.799, 0)] {
        let server = Loopback::answering(move |q| {
            let words = q["instructions"].as_str().unwrap();
            if q["criteria"].get("BEGIN").is_some() {
                let probabilities = if words.contains("[[Ada]]") {
                    [membership-0.54,0.25,0.04,0.25,1.0-membership]
                } else if words.contains("[[Vale]]") {
                    [0.16,0.16,0.29,0.39,0.0]
                } else if words.contains("[[Acme]]") {
                    [0.0,0.0,0.0,1.0,0.0]
                } else {
                    [0.0,0.0,0.0,0.0,1.0]
                };
                json!({"type":"choice","probabilities":{"BEGIN":probabilities[0],"INSIDE":probabilities[1],"END":probabilities[2],"SINGLE":probabilities[3],"OUT":probabilities[4]}})
            } else if q["criteria"].get("person").is_some() {
                let company = words.contains("[[Acme]]");
                assert!(company || words.contains("[[Ada Vale]]"), "kind question must judge the joined name: {words}");
                json!({"type":"choice","probabilities":{"person":if company {0.0} else {0.625},"company":if company {0.625} else {0.0},"none of these":0.375}})
            } else {
                json!({"type":"noul","noul":0.7})
            }
        }).unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .rows(
                &Relate::new(vec![
                    RelationRule::new("works_for", "person", "company").unwrap(),
                ])
                .unwrap(),
                [Record::text("Ada Vale joined Acme")],
            )
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
        let row = &rows[0];
        assert_eq!(row["status"], "answered");
        assert_eq!(row["answer"].as_array().unwrap().len(), expected);
        assert_eq!(row["detail"]["pairs"].as_array().unwrap().len(), expected);
        assert_eq!(server.count(), 2 + expected);
        if expected == 1 {
            assert_eq!(row["answer"][0]["source"]["text"], "Ada Vale");
            assert_eq!(row["answer"][0]["target"]["text"], "Acme");
            assert_eq!(row["answer"][0]["probability"], 0.7);
        }
    }
}

#[test]
fn punctuation_proposals_cannot_become_relation_endpoints() {
    let server = Loopback::answering(|q| {
        if q["criteria"].get("BEGIN").is_some() {
            json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":1.,"OUT":0.}})
        } else {
            panic!("punctuation must not reach kind or relation questions: {q}")
        }
    })
    .unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(
            &Relate::new(vec![RelationRule::new("near", "thing", "thing").unwrap()]).unwrap(),
            [Record::text("! ★")],
        )
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "answered");
    assert_eq!(rows[0]["answer"], json!([]));
    assert_eq!(rows[0]["detail"]["pairs"], json!([]));
    assert_eq!(server.count(), 1);
}
