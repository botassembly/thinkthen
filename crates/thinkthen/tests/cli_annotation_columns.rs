#![cfg(feature = "command")]
#![cfg(test)]
//! Feature: annotation tables retain inputs and expose child answers in one batch.
mod support;

use loopback::{KEY, Loopback};
use serde_json::Value;
use std::fs;

#[test]
fn annotation_table_columns_follow_the_shared_four_record_case() {
    let case: Value = serde_json::from_str(include_str!(
        "../../../conformance/typed/annotate-columns.json"
    ))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.csv");
    let toolbox = directory.path().join("questions.json");
    let mut definition = case["request"]["question"].clone();
    definition.as_object_mut().unwrap().remove("function");
    fs::write(&toolbox, definition.to_string()).unwrap();
    let mut writer = csv::Writer::from_path(&input).unwrap();
    writer.write_record(["id", "body"]).unwrap();
    for record in case["request"]["records"].as_array().unwrap() {
        writer
            .write_record([
                record["id"].as_str().unwrap(),
                record["body"].as_str().unwrap(),
            ])
            .unwrap();
    }
    writer.flush().unwrap();
    let server = Loopback::case(&case).unwrap();
    let output = support::command()
        .arg("annotate")
        .arg(&toolbox)
        .arg(&input)
        .args(["--from", "csv", "--column", "body", "--batch", "32"])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(server.count(), case["requests"].as_u64().unwrap() as usize);
    let mut reader = csv::Reader::from_path(directory.path().join("input.thinkthen.csv")).unwrap();
    let headers = reader.headers().unwrap().clone();
    let rows: Vec<_> = reader.records().map(Result::unwrap).collect();
    assert_eq!(rows.len(), 4);
    for (index, expected) in case["expected"].as_array().unwrap().iter().enumerate() {
        assert_eq!(
            &rows[index][0],
            case["request"]["records"][index]["id"].as_str().unwrap()
        );
        assert_eq!(
            &rows[index][1],
            case["request"]["records"][index]["body"].as_str().unwrap()
        );
        for (name, child) in expected["answer"].as_object().unwrap() {
            let column = headers.iter().position(|header| header == name).unwrap();
            let answer = match &child["answer"] {
                Value::Bool(value) => value.to_string().to_uppercase(),
                Value::String(value) => value.clone(),
                value => value.to_string(),
            };
            assert_eq!(rows[index][column], answer);
            assert_eq!(&rows[index][column + 1], child["status"].as_str().unwrap());
            if let Some(probability) = child["probability"].as_f64() {
                assert_eq!(rows[index][column + 2].parse::<f64>().unwrap(), probability);
            } else {
                assert!(rows[index][column + 2].is_empty());
            }
        }
    }
}
