//! `filter` and `rank` with no framing flag: lines, or JSON Lines under a pointer.

use std::io;

use super::{BY_BODY, QUESTION, RECORDS, by_body, code, over, printed, said};
use crate::harness::{Listener, spawn};

/// Three text lines, each earning its own probability from [`BY_BODY`].
const LINES: &str =
    "The payout failed again.\nThanks for the quick fix.\nThe card was refused at checkout.\n";

/// The `state` of each request the listener read, in the order it read them.
fn states(listener: &Listener) -> Vec<String> {
    listener
        .requests()
        .iter()
        .map(|request| {
            let body: serde_json::Value =
                serde_json::from_slice(&request.body).expect("a JSON request body");
            body["state"].as_str().expect("a text state").to_owned()
        })
        .collect()
}

#[test]
fn filter_and_rank_read_lines_when_no_framing_is_named() -> io::Result<()> {
    let cases = [
        (
            "filter",
            "The payout failed again.\nThe card was refused at checkout.\n",
        ),
        (
            "rank",
            "The payout failed again.\nThe card was refused at checkout.\nThanks for the quick fix.\n",
        ),
    ];
    for (verb, wanted) in cases {
        let listener = by_body(BY_BODY)?;
        let output = over(verb, listener.base(), &[], LINES)?;
        assert_eq!(code(&output), 0, "{verb}: {}", said(&output));
        // Requests run in parallel, so they arrive in any order.
        let mut sent = states(&listener);
        sent.sort();
        assert_eq!(
            sent,
            [
                "Thanks for the quick fix.",
                "The card was refused at checkout.",
                "The payout failed again.",
            ],
            "{verb}: one request per line"
        );
        assert_eq!(said(&output), "", "{verb}");
        assert_eq!(printed(&output), wanted, "{verb}");
    }
    Ok(())
}

#[test]
fn a_pointer_with_no_framing_reads_json_lines() -> io::Result<()> {
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("default-framing");
    std::fs::create_dir_all(&folder)?;
    let file = folder.join("on-body.json");
    std::fs::write(&file, format!(r#"{{"decide":"{QUESTION}","on":"/body"}}"#))?;
    let asked = format!("@{}", file.display());
    let rows: [&[&str]; 2] = [
        &["filter", QUESTION, "--field", "/body"],
        &["filter", &asked],
    ];
    for row in rows {
        let listener = by_body(BY_BODY)?;
        let arguments = [
            row,
            &[
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--jobs",
                "1",
            ],
        ]
        .concat();
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            RECORDS.as_bytes(),
        )?;
        assert_eq!(code(&output), 0, "{row:?}: {}", said(&output));
        assert_eq!(said(&output), "", "{row:?}");
        assert_eq!(
            printed(&output),
            concat!(
                "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
                "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
                "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
            ),
            "{row:?}"
        );
        assert_eq!(
            states(&listener),
            [
                "The payout failed again.",
                "Thanks for the quick fix.",
                "The card was refused at checkout.",
                "The refund never arrived.",
            ],
            "{row:?}: each request sends the body alone"
        );
    }
    Ok(())
}

#[test]
fn the_plan_names_a_framing_the_default_chose() -> io::Result<()> {
    let listener = by_body(BY_BODY)?;
    let base = listener.base();
    let request = |state: &str| {
        format!(
            r#""request":{{"state":"{state}","model":"local-1","questions":{{"q1":{{"type":"noul","instructions":"{QUESTION}"}}}}}}}}"#
        )
    };
    let cases = [
        (
            "filter",
            &[][..],
            LINES,
            format!(
                r#"{{"url":"{base}/systemone","model":"local-1","key_env":"THINKTHEN_API_KEY","input":{{"framing":"lines","field":[],"from":"default"}},{}"#,
                request("The payout failed again.")
            ),
        ),
        (
            "rank",
            &["--field", "/body"][..],
            RECORDS,
            format!(
                r#"{{"url":"{base}/systemone","model":"local-1","key_env":"THINKTHEN_API_KEY","input":{{"framing":"jsonl","field":["/body"],"from":"default"}},{}"#,
                request("The payout failed again.")
            ),
        ),
    ];
    for (verb, extra, input, wanted) in cases {
        let arguments = [
            &[
                verb,
                QUESTION,
                "--dry-run",
                "--url",
                base,
                "--model",
                "local-1",
            ][..],
            extra,
        ]
        .concat();
        let output = spawn(&arguments, &[], input.as_bytes())?;
        assert_eq!(code(&output), 0, "{verb}: {}", said(&output));
        assert_eq!(printed(&output), format!("{wanted}\n"), "{verb}");
    }
    assert_eq!(listener.connections(), 0, "a plan opens no connection");
    Ok(())
}
