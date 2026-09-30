//! The 4,096-input cap closes a request while later records keep coming.

use serde_json::Value;

use super::{all_yes, rows, set};
use crate::harness::{Listener, spawn};

#[test]
fn the_input_cap_closes_a_request_at_4096_records() {
    let file = set(
        "batch-cap-records",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let listener = Listener::answering(all_yes).expect("listener");
    let input: String = (1..=4097).map(|at| format!("r{at}\n")).collect();
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--lines",
            "--batch",
            "max",
            "--max-request-bytes",
            "1000000",
            "--jobs",
            "2",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        input.as_bytes(),
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut sizes: Vec<usize> = listener
        .requests()
        .iter()
        .map(|request| {
            let body = serde_json::from_slice::<Value>(&request.body).expect("request JSON");
            body["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len)
        })
        .collect();
    sizes.sort_unstable();
    assert_eq!(sizes, [1, 4096]);
    let rows = rows(&output.stdout);
    assert_eq!(rows.len(), 4097);
    assert_eq!(rows[4096]["value"]["ready"], true);
}
