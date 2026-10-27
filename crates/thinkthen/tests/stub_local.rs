#![cfg(test)]
//! The standalone crate runs deterministic judgments without network or credentials.
use thinkthen::{BackendName, Decide, Record, Settings};

#[test]
fn stub_ignores_remote_settings_and_never_reads_credentials() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .url("http://remote.invalid")
        .model("ignored")
        .engine(|name| panic!("the stub read environment variable {name}"))
        .unwrap();
    let question = Decide::new("Ready?").unwrap();
    let mut rows = Vec::new();
    engine
        .rows(&question, ["alpha", "beta", "alpha"].map(Record::text))
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["probability"], rows[2]["probability"]);
    assert_ne!(rows[0]["probability"], rows[1]["probability"]);
    assert!(rows.iter().all(|row| row["status"] == "answered"));
}
