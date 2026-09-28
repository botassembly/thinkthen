//! One run cannot compare probabilities from different actual reply models.

#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "an unexpected request shape stops the loopback fixture"
)]

use super::{Canned, Listener, RECORDS, code, over, printed, said};
use serde_json::Value;
use std::io;

fn answer(model: &str) -> String {
    format!(r#"{{"model":"{model}","answers":{{"q1":{{"type":"noul","noul":0.9}}}}}}"#)
}

fn reply_for(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request JSON");
    if request["questions"]
        .as_object()
        .map_or(0, serde_json::Map::len)
        == 2
    {
        return Canned::status(413, "{}");
    }
    let model = if String::from_utf8_lossy(body).contains("quick fix") {
        "jev-1.14.0"
    } else {
        "jev-1.13.0"
    };
    Canned::ok(&answer(model))
}

#[test]
fn ordinary_and_split_replies_keep_only_a_consistent_filter_prefix() -> io::Result<()> {
    let input = RECORDS.lines().take(2).collect::<Vec<_>>().join("\n") + "\n";
    for split in [false, true] {
        for verb in ["filter", "rank"] {
            let listener = Listener::answering(reply_for)?;
            let batch = if split { "2" } else { "1" };
            let output = over(
                verb,
                listener.base(),
                &[
                    "--jsonl",
                    "--field",
                    "/body",
                    "--batch",
                    batch,
                    "--jobs",
                    "1",
                    "--max-retries",
                    "0",
                    "--no-cache",
                    "--facts",
                ],
                &input,
            )?;
            assert_eq!(code(&output), 4, "{verb}/{split}: {}", said(&output));
            let expected = if verb == "filter" {
                format!("{}\n", RECORDS.lines().next().expect("first record"))
            } else {
                String::new()
            };
            assert_eq!(printed(&output), expected, "{verb}/{split}");
            assert!(
                said(&output).contains(
                    "the replies for one filter or rank run named different model versions"
                ),
                "{verb}/{split}: {}",
                said(&output)
            );
            let attempts = if split { 3 } else { 2 };
            assert_eq!(listener.requests().len(), attempts, "{verb}/{split}");
            let facts: Value =
                serde_json::from_str(said(&output).lines().last().expect("run facts"))
                    .expect("run facts JSON");
            assert_eq!(facts["requests_sent"], attempts, "{verb}/{split}");
        }
    }
    Ok(())
}
