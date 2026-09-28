//! `filter` and `rank` over a loopback backend: what is kept, and in what order.

use std::io;
use std::process::Output;

use crate::harness::{Canned, Listener, spawn_one as spawn};

mod default_framing;
mod graded_rank;
mod model_versions;
mod rank_top;

/// The question both commands ask of each record.
const QUESTION: &str = "Does this report a payment failure?";

/// Four JSON records, with odd spacing and an id no request ever carries.
///
/// The second record is written with spaces the tool must not tidy away, so a
/// page that says "byte for byte" is held to it.
pub(crate) const RECORDS: &str = concat!(
    "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
    "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
    "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
    "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
);

/// Each record of [`RECORDS`] as `--details` prints it back under `input`.
///
/// A row writes the record through a JSON encoder, so the second record's odd
/// spacing is gone here. A kept record keeps the bytes that arrived.
pub(crate) const RECORDS_AS_SENT: [&str; 4] = [
    "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}",
    "{\"id\":\"R-2\",\"body\":\"Thanks for the quick fix.\"}",
    "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}",
    "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}",
];

/// The response a backend gives, with the probability the case names.
pub(crate) fn answered(probability: &str) -> String {
    format!(
        concat!(
            r#"{{"model":"jev-1.13.0","answers":{{"q1":{{"type":"noul","noul":{probability}}}}},"#,
            r#""usage":{{"input_tokens":88,"output_tokens":12}}}}"#,
        ),
        probability = probability
    )
}

/// A listener that answers these probabilities, one per request, in order.
fn serving(probabilities: &[&str]) -> io::Result<Listener> {
    Listener::serving(
        probabilities
            .iter()
            .map(|p| Canned::ok(&answered(p)))
            .collect(),
    )
}

/// A listener that answers each record's body with the probability it earns.
///
/// The answers go by content, so a run with several requests in flight gets
/// the same answer for a record whatever order the requests arrive in.
fn by_body(odds: &'static [(&'static str, &'static str)]) -> io::Result<Listener> {
    Listener::answering(move |body| {
        let sent = String::from_utf8_lossy(body).into_owned();
        let found = odds
            .iter()
            .find_map(|(word, probability)| sent.contains(word).then_some(*probability))
            .unwrap_or("0.0");
        Canned::ok(&answered(found))
    })
}

/// The probability each record earns, keyed by a word only that record holds.
const BY_BODY: &[(&str, &str)] = &[
    ("payout", "0.91"),
    ("quick fix", "0.02"),
    ("checkout", "0.77"),
    ("refund never", "0.55"),
];

/// Run one of the two commands against one address over the records given.
fn over(verb: &str, base: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = [verb, QUESTION, "--url", base, "--model", "local-1"];
    spawn(
        &[&asked[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

/// What the run printed on standard output.
pub(crate) fn printed(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// What the run said on standard error.
pub(crate) fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The exit code the run earned.
pub(crate) fn code(output: &Output) -> i32 {
    output.status.code().unwrap_or(-1)
}

#[test]
fn filter_prints_every_kept_record_byte_for_byte_and_in_input_order() -> io::Result<()> {
    let listener = serving(&["0.91", "0.02", "0.77", "0.55"])?;
    let output = over(
        "filter",
        listener.base(),
        &["--jsonl", "--field", "/body", "--jobs", "1"],
        RECORDS,
    )?;

    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(
        printed(&output),
        concat!(
            "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
            "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
            "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
        )
    );
    assert_eq!(said(&output), "");
    Ok(())
}

#[test]
fn a_cut_moves_the_line_and_a_record_with_odd_spacing_survives_it() -> io::Result<()> {
    let listener = serving(&["0.91", "0.99", "0.77", "0.55"])?;
    let output = over(
        "filter",
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--threshold",
            "0.8",
            "--jobs",
            "1",
        ],
        RECORDS,
    )?;

    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(
        printed(&output),
        concat!(
            "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
            "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
        )
    );
    Ok(())
}

#[test]
fn a_text_line_keeps_its_trailing_spaces_and_its_line_ending_becomes_a_feed() -> io::Result<()> {
    let listener = serving(&["0.91", "0.02"])?;
    let output = over(
        "filter",
        listener.base(),
        &["--lines", "--jobs", "1"],
        "the payout failed again.   \r\nthanks for the quick fix.\n",
    )?;

    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(printed(&output), "the payout failed again.   \n");
    Ok(())
}

#[test]
fn rank_prints_every_record_with_the_most_likely_yes_first() -> io::Result<()> {
    let listener = serving(&["0.55", "0.02", "0.91", "0.77"])?;
    let output = over(
        "rank",
        listener.base(),
        &["--jsonl", "--field", "/body", "--jobs", "1"],
        RECORDS,
    )?;

    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(
        printed(&output),
        concat!(
            "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
            "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
            "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
            "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
        )
    );
    Ok(())
}

#[test]
fn an_exact_tie_keeps_input_order_and_top_prints_the_first_places() -> io::Result<()> {
    let listener = serving(&["0.4", "0.9", "0.4", "0.9"])?;
    let output = over(
        "rank",
        listener.base(),
        &["--jsonl", "--field", "/body", "--jobs", "1"],
        RECORDS,
    )?;
    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(
        printed(&output),
        concat!(
            "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
            "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
            "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
            "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
        )
    );

    let listener = serving(&["0.55", "0.02", "0.91", "0.77"])?;
    let output = over(
        "rank",
        listener.base(),
        &["--jsonl", "--field", "/body", "--top", "2", "--jobs", "1"],
        RECORDS,
    )?;
    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(
        printed(&output),
        concat!(
            "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n",
            "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
        )
    );
    assert_eq!(listener.requests().len(), 4, "--top saves no request");
    Ok(())
}

#[test]
fn every_number_of_jobs_prints_the_bytes_one_job_prints() -> io::Result<()> {
    for verb in ["filter", "rank"] {
        let mut answers = Vec::new();
        for jobs in ["1", "4", "8"] {
            let listener = by_body(BY_BODY)?;
            let output = over(
                verb,
                listener.base(),
                &["--jsonl", "--field", "/body", "--jobs", jobs],
                RECORDS,
            )?;
            assert_eq!(code(&output), 0, "{verb} --jobs {jobs}: {}", said(&output));
            answers.push(printed(&output));
        }
        assert_eq!(answers.first(), answers.get(1), "{verb}");
        assert_eq!(answers.first(), answers.get(2), "{verb}");
        assert!(
            answers
                .first()
                .is_some_and(|first| first.lines().count() > 1),
            "{verb} printed nothing to compare"
        );
    }
    Ok(())
}

#[test]
fn details_keeps_filter_membership_and_rank_top_membership() -> io::Result<()> {
    let listener = serving(&["0.91", "0.02", "0.77", "0.55"])?;
    let output = over(
        "filter",
        listener.base(),
        &["--jsonl", "--field", "/body", "--details", "--jobs", "1"],
        RECORDS,
    )?;
    assert_eq!(code(&output), 0, "{}", said(&output));
    let rows: Vec<(String, String)> = printed(&output)
        .lines()
        .filter_map(|line| {
            let (_, rest) = line.split_once("\"input\":")?;
            let (record, _) = rest.split_once(",\"question\":")?;
            let (value, _) = line.split_once("\"value\":")?.1.split_once(',')?;
            Some((record.to_owned(), value.to_owned()))
        })
        .collect();
    assert_eq!(
        rows,
        [
            (RECORDS_AS_SENT[0].to_owned(), "true".to_owned()),
            (RECORDS_AS_SENT[2].to_owned(), "true".to_owned()),
            (RECORDS_AS_SENT[3].to_owned(), "true".to_owned()),
        ],
        "filter --details prints only kept records, in input order"
    );
    let listener = serving(&["0.55", "0.02", "0.91", "0.77"])?;
    let output = over(
        "rank",
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--details",
            "--top",
            "2",
            "--jobs",
            "1",
        ],
        RECORDS,
    )?;
    assert_eq!(code(&output), 0, "{}", said(&output));
    let held: Vec<String> = printed(&output)
        .lines()
        .filter_map(|line| {
            let (_, rest) = line.split_once("\"input\":")?;
            let (record, _) = rest.split_once(",\"question\":")?;
            Some(record.to_owned())
        })
        .collect();
    assert_eq!(
        held,
        [
            "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}",
            "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}",
        ]
    );
    assert!(
        printed(&output)
            .lines()
            .all(|line| line.contains("\"value\":null,") && line.contains("\"threshold\":null")),
        "a ranked row carries no rule and makes no selection"
    );
    Ok(())
}

#[test]
fn an_empty_record_stream_prints_nothing_and_sends_nothing_for_either_command() -> io::Result<()> {
    for verb in ["filter", "rank"] {
        let listener = serving(&[])?;
        let output = over(verb, listener.base(), &["--jsonl", "--field", "/body"], "")?;
        assert_eq!(code(&output), 0, "{verb}: {}", said(&output));
        assert_eq!(printed(&output), "", "{verb}");
        assert_eq!(said(&output), "", "{verb}");
        assert_eq!(listener.requests().len(), 0, "{verb}");
    }
    Ok(())
}

#[test]
fn a_failed_record_stops_filter_after_a_prefix_and_leaves_rank_printing_nothing() -> io::Result<()>
{
    let listener = Listener::serving(vec![
        Canned::ok(&answered("0.91")),
        Canned::status(500, "{}"),
        Canned::ok(&answered("0.77")),
    ])?;
    let output = over(
        "filter",
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--jobs",
            "1",
            "--max-retries",
            "0",
        ],
        RECORDS,
    )?;
    assert_eq!(code(&output), 4, "{}", said(&output));
    assert_eq!(
        printed(&output),
        "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n"
    );
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n",
            "thinkthen: stopped at record 2; 1 record finished\n",
        )
    );

    let listener = Listener::serving(vec![
        Canned::ok(&answered("0.91")),
        Canned::status(500, "{}"),
        Canned::ok(&answered("0.77")),
    ])?;
    let output = over(
        "rank",
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--jobs",
            "1",
            "--max-retries",
            "0",
        ],
        RECORDS,
    )?;
    assert_eq!(code(&output), 4, "{}", said(&output));
    assert_eq!(printed(&output), "");
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n",
            "thinkthen: stopped at record 2; 1 record finished,",
            " and nothing was printed because an order needs every record\n",
        )
    );
    Ok(())
}

#[test]
fn the_plan_under_dry_run_shows_the_first_record_and_opens_no_connection() -> io::Result<()> {
    // The plan names the listener, so a run that started asking is counted.
    let listener = serving(&["0.91", "0.02", "0.77", "0.55"])?;
    for verb in ["filter", "rank"] {
        let output = spawn(
            &[
                verb,
                QUESTION,
                "--jsonl",
                "--field",
                "/body",
                "--dry-run",
                "--url",
                listener.base(),
            ],
            &[],
            RECORDS.as_bytes(),
        )?;
        assert_eq!(code(&output), 0, "{verb}: {}", said(&output));
        let plan = printed(&output);
        assert_eq!(plan.lines().count(), 1, "{verb}");
        assert!(
            plan.contains(r#""input":{"framing":"jsonl","field":["/body"]}"#),
            "{verb}: {plan}"
        );
        assert!(plan.contains("The payout failed again."), "{verb}: {plan}");
        assert!(!plan.contains("quick fix"), "{verb}: {plan}");
        assert_eq!(listener.requests().len(), 0, "{verb} sent a request");
    }
    assert_eq!(listener.connections(), 0, "a plan opens no connection");
    Ok(())
}

/// A small spread of probabilities, taken from one multiplier per record.
///
/// `proptest` is a development dependency of the core alone, and adding it to
/// this crate would be a new dependency. The loop below drives the same claim
/// over the counts and the cuts that matter, and `order.rs` in the core holds
/// the permutation property itself as a `proptest` case.
fn odds(place: usize) -> f64 {
    let stepped = (place * 37 % 13) as f64 / 13.0;
    (stepped * 100.0).round() / 100.0
}

/// A file of `count` JSON records, each naming its own place.
fn spread(count: usize) -> String {
    (0..count)
        .map(|place| format!("{{\"id\":\"P-{place}\",\"body\":\"record {place}\"}}\n"))
        .collect()
}

/// A listener that answers each record with the probability its place earns.
fn by_place() -> io::Result<Listener> {
    Listener::answering(move |body| {
        let sent = String::from_utf8_lossy(body).into_owned();
        let place = (0..64)
            .find(|place| {
                sent.contains(&format!("record {place}\\\""))
                    || sent.contains(&format!("record {place}\""))
            })
            .unwrap_or(0);
        Canned::ok(&answered(&format!("{}", odds(place))))
    })
}

#[test]
fn filter_prints_a_subsequence_of_its_input_and_rank_a_permutation_of_it() -> io::Result<()> {
    for count in 0..8_usize {
        let input = spread(count);
        let lines: Vec<&str> = input.lines().collect();

        for cut in ["0.1", "0.5", "0.9"] {
            let listener = by_place()?;
            let output = over(
                "filter",
                listener.base(),
                &["--jsonl", "--field", "/body", "--threshold", cut],
                &input,
            )?;
            assert_eq!(code(&output), 0, "filter {count} {cut}: {}", said(&output));
            let kept = printed(&output);
            let kept: Vec<&str> = kept.lines().collect();
            let mut next = lines.iter();
            for line in &kept {
                assert!(
                    next.any(|held| held == line),
                    "filter {count} {cut} printed a line that is not the next input line"
                );
            }
            assert!(kept.len() <= count, "filter {count} {cut} printed too much");
        }

        for top in [None, Some(1_usize), Some(3), Some(99)] {
            let listener = by_place()?;
            let asked = top.map(|n| n.to_string());
            let mut arguments = vec!["--jsonl", "--field", "/body"];
            if let Some(number) = asked.as_deref() {
                arguments.extend(["--top", number]);
            }
            let output = over("rank", listener.base(), &arguments, &input)?;
            assert_eq!(code(&output), 0, "rank {count} {top:?}: {}", said(&output));
            let ordered = printed(&output);
            let mut ordered: Vec<&str> = ordered.lines().collect();
            assert_eq!(
                ordered.len(),
                top.unwrap_or(count).min(count),
                "rank {count} {top:?} printed the wrong number of records"
            );
            ordered.sort_unstable();
            ordered.dedup();
            assert!(
                ordered.iter().all(|line| lines.contains(line)),
                "rank {count} {top:?} printed a line that was never read"
            );
            assert_eq!(
                ordered.len(),
                top.unwrap_or(count).min(count),
                "rank {count} {top:?} printed one record twice"
            );
        }
    }
    Ok(())
}
