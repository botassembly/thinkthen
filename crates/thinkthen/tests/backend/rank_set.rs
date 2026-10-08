//! Independently declared set turns, attribution, intake and original identity.
use crate::harness::{Canned, Listener, spawn};
use crate::input_sources::{folder, text};
use serde_json::{Value, json};
use std::{fs, io};

pub(super) const SET: &str =
    r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#;

pub(super) fn saved(name: &str, set: &str) -> io::Result<String> {
    let path = folder(name)?.join("questions.json");
    fs::write(&path, set)?;
    Ok(format!("@{}", path.display()))
}

pub(super) fn call(
    listener: &Listener,
    question: &str,
    flags: &[&str],
    input: &[u8],
) -> io::Result<std::process::Output> {
    call_at_width(listener, question, flags, input, Some("1"))
}

pub(super) fn call_at_width(
    listener: &Listener,
    question: &str,
    flags: &[&str],
    input: &[u8],
    jobs: Option<&str>,
) -> io::Result<std::process::Output> {
    let width: Vec<_> = jobs.into_iter().flat_map(|jobs| ["--jobs", jobs]).collect();
    let cache = if flags.contains(&"--cache") {
        vec![]
    } else {
        vec!["--no-cache"]
    };
    spawn(
        &[
            &[
                "rank",
                question,
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ][..],
            &width,
            &cache,
            flags,
        ]
        .concat(),
        &[],
        input,
    )
}

pub(super) fn answer(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).expect("request JSON");
    let answers = body["questions"]
        .as_object()
        .expect("questions")
        .iter()
        .map(|(key, question)| {
            let instructions = question["instructions"].as_str().expect("instructions");
            // First independently ranks [a,b,c]. Second ranks [a,c,b].
            // A duplicate-refill merge would incorrectly return [a,c,b].
            let probability = match (
                instructions.contains("Second?"),
                instructions.contains("\"a\""),
                instructions.contains("\"b\""),
            ) {
                (false, true, _) => 0.7,
                (false, _, true) => 0.6,
                (false, _, _) => 0.5,
                (true, true, _) => 1.0,
                (true, _, true) => 0.98,
                (true, _, _) => 0.99,
            };
            (
                key.clone(),
                json!({"type":"noul","noul":probability,"confidence":0.01}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(
        &json!({"model":"local-1","answers":answers,"usage":{"input_tokens":6,"output_tokens":2}})
            .to_string(),
    )
}

#[test]
fn duplicates_consume_visits_without_refill_and_top_follows_the_merge() -> io::Result<()> {
    let question = saved("rank-set-order", SET)?;
    for (top, expected) in [
        (None, "0.7 1:a\n0.6 2:b\n0.99 3:c\n"),
        (Some("1"), "0.7 1:a\n"),
        (Some("2"), "0.7 1:a\n0.6 2:b\n"),
        (Some("3"), "0.7 1:a\n0.6 2:b\n0.99 3:c\n"),
    ] {
        let listener = Listener::answering(answer)?;
        let mut flags = vec!["--batch", "1", "-n", "--scores"];
        if let Some(top) = top {
            flags.extend(["--top", top]);
        }
        let output = call(&listener, &question, &flags, b"a\nb\nc\n")?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        assert_eq!(output.stdout, expected.as_bytes());
        assert_eq!(
            listener.requests().len(),
            3,
            "all records are judged even at top one"
        );
    }
    Ok(())
}

#[test]
fn common_jsonl_field_and_named_windows_use_shared_intake_display() -> io::Result<()> {
    let question = saved("rank-set-intake", SET)?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &question,
        &["--jsonl", "--field", "/body", "-n", "--scores"],
        b"{\"body\":\"a\", \"private\":\"hidden\"}\n{\"body\":\"b\"}\n{\"body\":\"c\"}\n",
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(output.stdout, b"0.7 1:{\"body\":\"a\", \"private\":\"hidden\"}\n0.6 2:{\"body\":\"b\"}\n0.99 3:{\"body\":\"c\"}\n");
    for request in listener.requests() {
        assert!(!text(&request.body).contains("hidden"));
    }
    let file = folder("rank-set-window")?.join("input");
    fs::write(&file, "a\n\nb\n\nc\n")?;
    let output = call(
        &listener,
        &question,
        &[
            "--window",
            "2",
            "-n",
            "--around",
            "0",
            file.to_str().expect("file"),
        ],
        b"",
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(output.stdout, b"--\n1:a\n2:\n--\n3:b\n4:\n--\n5:c\n");
    Ok(())
}

#[test]
fn set_facts_count_originals_and_details_sum_only_their_actual_member_receipts() -> io::Result<()> {
    let question = saved("rank-set-facts", SET)?;
    let listener = Listener::answering(answer)?;
    let result = call(
        &listener,
        &question,
        &["--details", "--facts", "--batch", "1"],
        b"a\nb\nc\n",
    )?;
    assert_eq!(result.status.code(), Some(0), "{}", text(&result.stderr));
    let rows: Vec<Value> = text(&result.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("detail"))
        .collect();
    assert_eq!(
        rows.iter()
            .map(|row| row["meta"]["requests_sent"].as_u64())
            .collect::<Vec<_>>(),
        [Some(1), Some(1), Some(1)]
    );
    for row in rows {
        assert_eq!(
            row["meta"]["usage"],
            json!({"input_tokens":6,"output_tokens":2})
        );
    }
    let facts: Value = serde_json::from_slice(&result.stderr)?;
    assert_eq!(facts["records"], 3);
    assert_eq!(facts["requests_sent"], 3);
    assert_eq!(facts["input_tokens"], 18);
    assert_eq!(facts["output_tokens"], 6);
    Ok(())
}

#[test]
fn authored_cuts_and_on_are_refused_before_normalization_and_sends() -> io::Result<()> {
    let cases = [
        (
            r#"{"version":1,"questions":{}}"#,
            "`questions` holds at least one named question",
        ),
        (
            r#"{"version":2,"questions":{"first":{"decide":"secret wording"}}}"#,
            "`version` is the number 1",
        ),
        (
            r#"{"version":1,"threshold":0.5,"questions":{"first":{"decide":"secret wording"}}}"#,
            "`threshold`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","threshold":0.5}}}"#,
            "`questions.first.threshold`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","on":""}}}"#,
            "`questions.first.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","on":[""]}}}"#,
            "`questions.first.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","on":"/private"}}}"#,
            "`questions.first.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"first":{"score":"secret wording","levels":["low","high"]}}}"#,
            "`questions.first`: rank takes only decide questions",
        ),
        (
            r#"{"version":1,"questions":{"first":{"choose":"secret wording","options":["a","b"]}}}"#,
            "`questions.first`: rank takes only decide questions",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","unknown":"secret value"}}}"#,
            "the question set holds no key `questions.first.unknown`",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","model":"secret value"}}}"#,
            "the question set holds no key `questions.first.model`",
        ),
        (
            r#"{"version":1,"questions":{"first":{"decide":"secret wording","profile":"secret value"}}}"#,
            "the question set holds no key `questions.first.profile`",
        ),
        (
            r#"{"version":1,"questions":{"Bad":{"decide":"secret wording"}}}"#,
            "`questions.Bad` uses lowercase letters, digits, and underscores, and is not empty",
        ),
        (
            r#"{"version":1,"batch":0,"questions":{"first":{"decide":"secret wording"}}}"#,
            "`batch` in the question file takes max or a whole number of at least 1",
        ),
        (
            r#"{"version":1,"questions":{"first":"secret value"}}"#,
            "`questions.first` is one question object",
        ),
    ];
    let listener = Listener::answering(answer)?;
    for (index, (set, why)) in cases.into_iter().enumerate() {
        let question = saved(&format!("rank-set-invalid-{index}"), set)?;
        let output = call(&listener, &question, &[], b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
        assert_eq!(output.stderr, format!("thinkthen: {why}\n").as_bytes());
        assert!(output.stdout.is_empty());
        assert_eq!(listener.connections(), 0);
    }
    Ok(())
}

#[test]
fn set_owned_meanings_and_existing_display_conflicts_fail_without_sends() -> io::Result<()> {
    let question = saved("rank-set-conflicts", SET)?;
    let listener = Listener::answering(answer)?;
    for flags in [
        vec!["--true", "secret override"],
        vec!["--false", "secret override"],
        vec!["--true", "secret override", "--false", "secret override"],
    ] {
        let output = call(&listener, &question, &flags, b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stderr, b"thinkthen: `rank` with a question set takes no --true or --false; put meanings in each member\n");
        assert!(output.stdout.is_empty());
    }
    for flags in [
        vec!["--details", "--scores"],
        vec!["--window", "2", "--field", "/body"],
        vec!["--top", "0"],
        vec!["--quiet"],
        vec!["--raw"],
    ] {
        let output = call(&listener, &question, &flags, b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert!(!text(&output.stderr).contains("secret evidence"));
    }
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn malformed_or_duplicate_sets_never_fall_back_and_file_load_is_capped() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    for (index, set) in [
        r#"{"version":1,"questions":{broken"#,
        r#"{"version":1,"questions":{"first":{"decide":"secret"},"first":{"decide":"secret"}}}"#,
        r#"{"version":1,"questions":{"first":{"decide":"secret","decide":"secret"}}}"#,
    ]
    .into_iter()
    .enumerate()
    {
        let question = saved(&format!("rank-set-json-{index}"), set)?;
        let output = call(&listener, &question, &[], b"secret evidence\n")?;
        assert_eq!(output.status.code(), Some(5));
        assert!(output.stdout.is_empty());
        assert!(!text(&output.stderr).contains("secret"));
    }
    let question = saved(
        "rank-set-file-cap",
        &format!("{SET}{}", " ".repeat(1_048_576)),
    )?;
    let output = call(&listener, &question, &[], b"secret evidence\n")?;
    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    Ok(())
}

#[test]
fn a_late_failed_member_discards_every_held_rank_and_keeps_secrets_out() -> io::Result<()> {
    let question = saved("rank-set-late-failure", SET)?;
    for view in [
        vec![],
        vec!["--details"],
        vec!["-n", "--scores", "--around", "1", "--top", "1"],
    ] {
        let listener = Listener::answering(|body| {
            if text(body).contains("late") {
                // q1 succeeds but q2 is absent; annotate permits this partial
                // member failure. Set rank must reject the whole result.
                Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
            } else {
                answer(body)
            }
        })?;
        let flags = [&["--batch", "1"][..], &view].concat();
        let output = call(&listener, &question, &flags, b"a\nlate secret evidence\n")?;
        assert_eq!(output.status.code(), Some(4), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.requests().len(), 2);
        assert!(!text(&output.stderr).contains("secret evidence"));
    }
    Ok(())
}

pub(super) fn text_members() -> io::Result<()> {
    for jobs in [None, Some("1"), Some("4"), Some("8")] {
        preview_matches_counted_requests("rank-set-preview-text-0400", SET, false, jobs)?;
    }
    Ok(())
}

pub(super) fn mixed_members() -> io::Result<()> {
    for (name, set) in [
        (
            "rank-set-preview-text-json-0400",
            r#"{"version":1,"questions":{"text":{"decide":"Ready?"},"json":{"decide":{"check":"Ready?"}}}}"#,
        ),
        (
            "rank-set-preview-json-text-0400",
            r#"{"version":1,"questions":{"json":{"decide":{"check":"Ready?"}},"text":{"decide":"Ready?"}}}"#,
        ),
    ] {
        for jobs in [None, Some("1"), Some("4"), Some("8")] {
            preview_matches_counted_requests(name, set, true, jobs)?;
        }
    }
    Ok(())
}

fn preview_matches_counted_requests(
    name: &str,
    set: &str,
    mixed: bool,
    jobs: Option<&str>,
) -> io::Result<()> {
    let question = saved(name, set)?;
    let listener = Listener::answering(crate::input_sources::answer)?;
    let preview = call_at_width(
        &listener,
        &question,
        &["--batch", "1", "--plan"],
        b"alpha\nbeta\ngamma\n",
        jobs,
    )?;
    assert_eq!(preview.status.code(), Some(0), "{}", text(&preview.stderr));
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
    let lines: Vec<_> = text(&preview.stdout).lines().map(str::to_owned).collect();
    assert_eq!(lines.len(), 2);
    let document: serde_json::Map<String, Value> = serde_json::from_str(&lines[0])?;
    let raw: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(&lines[0])?;
    assert!(contains_record(&document["request"], "alpha"));
    let expected = if mixed { "6" } else { "3" };
    let runtime = call_at_width(
        &listener,
        &question,
        &["--batch", "1", "--max-requests-total", expected, "--facts"],
        b"alpha\nbeta\ngamma\n",
        jobs,
    )?;
    assert_eq!(runtime.status.code(), Some(0), "{}", text(&runtime.stderr));
    assert_eq!(runtime.stdout, b"alpha\nbeta\ngamma\n");
    let requests = listener.requests();
    let per_record = if mixed { 2 } else { 1 };
    assert_eq!(requests.len(), 3 * per_record);
    let first = requests
        .iter()
        .find(|request| request.body == raw["request"].get().as_bytes())
        .expect("the exact preview body was sent");
    assert_eq!(
        document["request"],
        serde_json::from_slice::<Value>(&first.body)?
    );
    let bodies: Vec<Value> = requests
        .iter()
        .map(|request| serde_json::from_slice(&request.body))
        .collect::<Result<_, _>>()?;
    for record in ["alpha", "beta", "gamma"] {
        let own: Vec<Value> = bodies
            .iter()
            .filter(|body| contains_record(body, record))
            .cloned()
            .collect();
        assert_eq!(own.len(), per_record);
        assert_member_evidence(&own, record, mixed);
    }
    let facts: Value = serde_json::from_str(text(&runtime.stderr).trim())?;
    assert_eq!(facts["records"], 3);
    assert_eq!(facts["requests_sent"], 3 * per_record);
    let bytes: usize = requests.iter().map(|request| request.body.len()).sum();
    let largest = requests
        .iter()
        .map(|request| request.body.len())
        .max()
        .unwrap_or(0);
    let summary: Value = serde_json::from_str(&lines[1])?;
    assert_eq!(
        summary,
        serde_json::json!({
            "records":3,"requests":6,"estimated_bytes":bytes,
            "largest_request_bytes":largest,"largest_request_estimated_input_tokens":(largest * 908).div_ceil(1000),
            "token_estimate_method":"encoded-body-bytes-908-v1",
            "estimated_input_tokens":{"lower":bytes * 516 / 1000,"upper":(bytes * 908).div_ceil(1000)},
            "upper_bound":!mixed
        })
    );
    Ok(())
}

fn contains_record(body: &Value, record: &str) -> bool {
    body["state"] == record
        || body["questions"]
            .as_object()
            .expect("questions")
            .values()
            .any(|q| {
                q["instructions"]
                    .as_str()
                    .is_some_and(|text| text.contains(&format!("\"{record}\"")))
            })
}

fn assert_member_evidence(bodies: &[Value], record: &str, mixed: bool) {
    let instructions: Vec<_> = bodies
        .iter()
        .flat_map(|body| {
            body["questions"]
                .as_object()
                .expect("questions")
                .values()
                .map(|q| &q["instructions"])
        })
        .collect();
    assert_eq!(
        instructions.len(),
        2,
        "both members judge each original record"
    );
    let texts: Vec<_> = instructions.iter().filter_map(|q| q.as_str()).collect();
    assert_eq!(texts.len(), if mixed { 1 } else { 2 });
    assert!(texts.iter().all(|q| q.contains(&format!("\"{record}\""))));
    if mixed {
        let structured = bodies
            .iter()
            .find(|body| {
                body["questions"]
                    .as_object()
                    .expect("questions")
                    .values()
                    .any(|q| q["instructions"].is_object())
            })
            .expect("structured request");
        assert_eq!(structured["state"], record);
        assert!(
            instructions
                .iter()
                .any(|q| **q == serde_json::json!({"check":"Ready?"}))
        );
        assert!(texts[0].contains("Ready?"));
    } else {
        for member in ["First?", "Second?"] {
            assert!(texts.iter().any(|q| q.contains(member)));
        }
    }
}
