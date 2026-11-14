#![cfg(test)]
//! Result formatting follows command string leaves without rewriting saved answers.
use serde_json::{Value, json};
use thinkthen::{AnyRow, Choose, Row};
thinkthen::choices! { enum Topic { Billing = "bill\ting", Shipping = "ship\nping" } }

fn printed(row: &AnyRow) -> String {
    match row {
        AnyRow::Choose(row) => row.to_string(),
        AnyRow::Tag(row) => row.to_string(),
        AnyRow::Find(row) => row.to_string(),
        AnyRow::Recognize(row) => row.to_string(),
        AnyRow::Relate(row) => row.to_string(),
        AnyRow::Annotate(row) => row.to_string(),
        AnyRow::Decide(row) => row.to_string(),
        _ => panic!("unhandled printing case"),
    }
}

#[test]
fn string_leaves_fold_controls_while_answers_and_saved_rows_stay_exact() {
    let name =
        json!({"kind":"person","text":"Ada\t\n\rLovelace","start":0,"end":14,"probability":1.0});
    let cases = [
        ("choose", json!("bill\ting"), "bill ing"),
        (
            "tag",
            json!(["bill\ting", "ship\nping"]),
            "bill ing\tship ping",
        ),
        (
            "find",
            json!({"text":"refund\tme","start":0,"end":9}),
            "refund me",
        ),
        ("recognize", json!([name]), "person: Ada   Lovelace (0..14)"),
        (
            "relate",
            json!([{"source":{"kind":"person","text":"Ada\nLovelace","start":0,"end":12},"target":{"kind":"organization","text":"Example\tGroup","start":13,"end":26},"relation":"works\rfor","probability":1.0}]),
            "Ada Lovelace --works for--> Example Group",
        ),
        (
            "annotate",
            json!({"a\tb":{"function":"choose","status":"answered","answer":"bill\ting","probability":1.0}}),
            "a b=bill ing",
        ),
    ];
    for (function, answer, expected) in cases {
        let mut saved = json!({"function":function,"position":1,"source":{"question_identity":"fixture","backend":"stub","model":"stub"},"status":"answered","answer":answer});
        if matches!(function, "choose" | "find") {
            saved["probability"] = json!(1.0);
        }
        let row: AnyRow = serde_json::from_value(saved.clone()).unwrap();
        assert_eq!(printed(&row), expected);
        assert_eq!(serde_json::to_value(&row).unwrap(), saved);
        if function == "recognize" {
            let AnyRow::Recognize(row) = row else {
                panic!("wrong function")
            };
            assert_eq!(row.answer().unwrap()[0].text(), "Ada\t\n\rLovelace");
        } else if function == "choose" {
            let typed: Row<Choose<Topic>> = serde_json::from_value(saved.clone()).unwrap();
            assert_eq!(typed.answer().unwrap(), Topic::Billing);
            assert_eq!(typed.to_string(), expected);
            assert_eq!(serde_json::to_value(typed).unwrap(), saved);
        }
    }
    let failed: Value = json!({"function":"decide","position":1,"source":{"question_identity":"fixture","backend":"stub","model":"stub"},"status":"failed","error":{"code":"backend.status","message":"bad\t\nservice","http_status":503}});
    let row: AnyRow = serde_json::from_value(failed.clone()).unwrap();
    // Error admission already escapes control characters; formatting preserves that canonical message.
    let saved = serde_json::to_value(&row).unwrap();
    assert_eq!(saved["error"]["message"], "bad\\t\\nservice");
    assert_eq!(printed(&row), "failed: backend.status: bad\\t\\nservice");
    assert_eq!(serde_json::to_value(row).unwrap(), saved);
}
