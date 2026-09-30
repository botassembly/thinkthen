//! `filter` and `rank` with no framing flag: lines, or JSON Lines under a pointer.

use std::io;
use std::process::Output;

use super::{BY_BODY, QUESTION, RECORDS, by_body, code, printed, said};
use crate::harness::{Listener, spawn_one as spawn};

/// Three text lines, each earning its own probability from [`BY_BODY`].
const LINES: &str =
    "The payout failed again.\nThanks for the quick fix.\nThe card was refused at checkout.\n";

/// Run a command line at one listener, with a key set so any leak would show.
fn ask(line: &[&str], base: &str, input: &str) -> io::Result<Output> {
    let at = ["--url", base, "--model", "local-1"];
    let key = [("THINKTHEN_API_KEY", "sk-test-value")];
    spawn(&[line, &at[..]].concat(), &key, input.as_bytes())
}

/// The text each request quotes in its one question, sorted, because
/// requests run in parallel and arrive in any order.
fn sent(listener: &Listener) -> Vec<String> {
    let mut states: Vec<String> = listener
        .requests()
        .iter()
        // A body under another state or with no quote reads as empty and
        // fails the comparison.
        .map(|request| {
            let body = serde_json::from_slice::<serde_json::Value>(&request.body).ok();
            body.as_ref()
                .filter(|body| body["state"] == "Each question quotes the text it asks about.")
                .and_then(|body| body["questions"]["q1"]["instructions"].as_str())
                .and_then(|asked| asked.strip_prefix("The text is "))
                .and_then(|asked| {
                    let mut quoted = serde_json::Deserializer::from_str(asked).into_iter();
                    quoted.next()?.ok()
                })
                .and_then(|quoted: serde_json::Value| quoted.as_str().map(str::to_owned))
                .unwrap_or_default()
        })
        .collect();
    states.sort();
    states
}

#[test]
fn filter_and_rank_read_lines_when_no_framing_is_named() -> io::Result<()> {
    let kept = "The payout failed again.\nThe card was refused at checkout.\n";
    let ordered = format!("{kept}Thanks for the quick fix.\n");
    for (verb, wanted) in [("filter", kept), ("rank", ordered.as_str())] {
        let listener = by_body(BY_BODY)?;
        let output = ask(&[verb, QUESTION], listener.base(), LINES)?;
        assert_eq!(code(&output), 0, "{verb}: {}", said(&output));
        assert_eq!(
            sent(&listener),
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
        let output = ask(row, listener.base(), RECORDS)?;
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
            sent(&listener),
            [
                "Thanks for the quick fix.",
                "The card was refused at checkout.",
                "The payout failed again.",
                "The refund never arrived.",
            ],
            "{row:?}: each request quotes the body alone"
        );
    }
    Ok(())
}

#[test]
fn the_plan_names_a_framing_the_default_chose() -> io::Result<()> {
    let listener = by_body(BY_BODY)?;
    let base = listener.base();
    let plan = |input: &str, counts: &str| {
        format!(
            r#"{{"url":"{base}/systemone","model":"local-1","key_env":"THINKTHEN_API_KEY","input":{input},"request":{{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{{"q1":{{"type":"noul","instructions":"The text is \"The payout failed again.\". {QUESTION}"}}}}}}}}"#
        ) + "\n"
            + counts
            + "\n"
    };
    let cases = [
        (
            &["filter", QUESTION, "--plan"][..],
            LINES,
            plan(
                r#"{"framing":"lines","field":[],"from":"default"}"#,
                r#"{"records":3,"requests":3,"estimated_bytes":622,"estimated_input_tokens":{"lower":320,"upper":565},"upper_bound":false}"#,
            ),
        ),
        (
            &["rank", QUESTION, "--field", "/body", "--plan"][..],
            RECORDS,
            plan(
                r#"{"framing":"jsonl","field":["/body"],"from":"default"}"#,
                r#"{"records":4,"requests":4,"estimated_bytes":827,"estimated_input_tokens":{"lower":426,"upper":751},"upper_bound":false}"#,
            ),
        ),
    ];
    for (line, input, wanted) in cases {
        let output = ask(line, base, input)?;
        assert_eq!(code(&output), 0, "{line:?}: {}", said(&output));
        assert_eq!(printed(&output), wanted, "{line:?}");
    }
    assert_eq!(listener.connections(), 0, "a plan opens no connection");
    Ok(())
}
