#![cfg(test)]
#![cfg(feature = "command")]
//! Backend rejection stops a command without consuming every batch.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;

#[test]
fn rejected_key_stops_a_thousand_record_call() {
    let server = loopback::Loopback::start(|_| {
        loopback::Reply::json(401, &serde_json::json!({"error":loopback::KEY}))
    })
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("records.txt");
    std::fs::write(&input, "evidence\n".repeat(1000)).unwrap();
    let output = support::command()
        .args([
            "decide", "Ready?", "--json", "--from", "lines", "--batch", "10",
        ])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .stdin(std::fs::File::open(input).unwrap())
        .output()
        .unwrap();
    assert!(server.count() <= 8, "sent {} requests", server.count());
    assert_eq!(output.status.code(), Some(4), "{output:?}");
    assert_eq!(support::reported(&output), "backend.status");
    assert!(output.stdout.is_empty());
    assert!(
        !String::from_utf8(output.stderr)
            .unwrap()
            .contains(loopback::KEY)
    );
}
