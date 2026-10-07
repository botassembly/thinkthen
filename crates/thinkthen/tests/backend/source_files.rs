//! All ten command functions share explicit folder provenance and original evidence.

use crate::harness::{Canned, Listener, spawn};
use crate::input_sources::{folder, text};
use serde_json::{Value, json};
use std::{fs, io, path::PathBuf, process::Output};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../specification/fixtures/files")
}

pub(super) fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let questions = request["questions"].as_object().unwrap();
    if questions.values().any(|q| {
        q["criteria"].get("BEGIN").is_some() || q["criteria"].get("none of these").is_some()
    }) {
        return crate::recognize::automatic(body);
    }
    let answers = questions.iter().map(|(key, question)| {
        let value = if question["type"] == "choice" {
            let labels = question["criteria"].as_object().unwrap();
            let picked = if labels.contains_key("u002") { "u002" }
                else if labels.contains_key("support") && question["instructions"].as_str().unwrap().contains("Support contract") { "support" }
                else { labels.keys().next().unwrap() };
            let probabilities = labels.keys().map(|label| (label.clone(), Value::from(if label == picked { 1.0 } else { 0.0 }))).collect::<serde_json::Map<_, _>>();
            json!({"type":"choice","choice":picked,"probabilities":probabilities})
        } else if question["type"] == "score" {
            json!({"type":"score","score":0.8,"confidence":0.9,"legend":{"0":"a","1":"b"},"probabilities":{"0":0.2,"1":0.8}})
        } else {
            let words = question["instructions"].as_str().unwrap();
            let rejected = (words.ends_with("Does this line describe a refund?") && !words.split(". Does this line").next().unwrap().to_lowercase().contains("refund"))
                || (words.ends_with("Does this document contain a support contract?") && !words.contains("Support contract"))
                || (words.ends_with("Does this document require attention today?") && !words.contains("An urgent"));
            let probability = if rejected { 0.1 } else { 0.9 };
            json!({"type":"noul","noul":probability})
        };
        (key.clone(), value)
    }).collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}

pub(super) fn call(
    listener: &Listener,
    command: &[&str],
    paths: &[&str],
    extra: &[&str],
) -> io::Result<Output> {
    let mut arguments = command.to_vec();
    arguments.extend(["--url", listener.base(), "--model", "local-1", "--no-cache"]);
    for path in paths {
        arguments.extend(["--input", path]);
    }
    arguments.extend(extra);
    spawn(&arguments, &[], b"ignored stdin")
}

fn rows(output: &Output) -> Vec<Value> {
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn ten_folder_examples_retain_original_records_and_physical_sources() -> io::Result<()> {
    let place = fixture();
    let documents = place.join("documents");
    let documents = documents.to_str().unwrap();
    let set = place.join("questions.json");
    let commands: Vec<Vec<&str>> = vec![
        vec!["decide", "Does this document contain a support contract?"],
        vec![
            "choose",
            "Which category fits this document?",
            "billing",
            "support",
        ],
        vec!["tag", "Which labels apply?", "refund", "contract"],
        vec!["score", "How urgent is this document?", "a", "b"],
        vec!["filter", "Does this line describe a refund?"],
        vec!["rank", "Does this document discuss a billing dispute?"],
        vec!["find", "Which line gives the refund policy?"],
        vec!["annotate", set.to_str().unwrap()],
        vec!["recognize", "person", "organization"],
        vec!["relate", "connected"],
    ];
    let listener = Listener::answering(answer)?;
    let examples =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../site/examples/learn/read-files");
    let golden = [
        "5-decide",
        "6-choose",
        "7-tag",
        "8-score",
        "1-filter",
        "3-rank",
        "4-find",
        "9-annotate",
        "a-recognize",
        "b-relate",
    ];
    for (command, golden) in commands.iter().zip(golden) {
        let verb = command[0];
        let unit = if matches!(verb, "filter" | "find") {
            "line"
        } else {
            "file"
        };
        let output = call(&listener, command, &[documents], &["--unit", unit])?;
        let rows = rows(&output);
        let prefix =
            serde_json::to_string(&format!("{}{}", place.display(), std::path::MAIN_SEPARATOR))?;
        let normalized = text(&output.stdout)
            .replace(prefix.trim_matches('"'), "")
            .replace(r"\\", "/");
        let expected = fs::read_to_string(examples.join(format!("{golden}.out")))?;
        let parse = |text: &str| {
            text.lines()
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            parse(&normalized),
            parse(&expected),
            "{verb} published output"
        );
        assert!(!rows.is_empty(), "{verb}");
        let requests = listener.requests();
        assert!(!requests.is_empty(), "{verb}");
        check_rows(verb, &rows, documents);
    }
    let output = call(&listener, &commands[4], &[documents], &["--files-only"])?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(
        text(&output.stdout),
        format!(
            "{}\n",
            place.join("documents").join("01-policy.txt").display()
        )
    );
    Ok(())
}

fn check_rows(verb: &str, rows: &[Value], documents: &str) {
    if verb == "relate" {
        for row in rows {
            check_relation(row, documents);
        }
        return;
    }
    for row in rows {
        assert!(
            row["file"].as_str().unwrap().starts_with(documents),
            "{verb}: {row}"
        );
        assert!(row["input"].is_string(), "{verb}: {row}");
    }
    if verb == "find" {
        assert_eq!(rows[0]["first_line"], 2);
        assert_eq!(
            rows[0]["input"],
            "Customers may request a refund within 30 days."
        );
    }
    if verb == "recognize" {
        let ada = rows[0]["value"]["entities"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["text"] == "Ada")
            .unwrap();
        assert_eq!(
            (ada["first_line"].as_u64(), ada["last_line"].as_u64()),
            (Some(3), Some(3))
        );
    }
}

fn check_relation(row: &Value, documents: &str) {
    for endpoint in ["source", "target"] {
        assert!(
            row[endpoint]["file"]
                .as_str()
                .unwrap()
                .starts_with(documents)
        );
        assert_eq!(row[endpoint]["first_line"], 1);
        assert_eq!(row[endpoint]["last_line"], 4);
        assert!(row[endpoint]["record"].as_str().unwrap().contains('\n'));
    }
}

#[test]
fn physical_paths_do_not_change_provider_requests_or_replay_keys() -> io::Result<()> {
    let place = folder("source-identity")?;
    let one = place.join("β-one");
    let two = place.join("two");
    let recording = place.join("recording");
    fs::write(&one, "same original evidence\r\n")?;
    fs::write(&two, "same original evidence\r\n")?;
    let listener = Listener::answering(answer)?;
    let command = ["decide", "Does this contain evidence?"];
    let output = call(
        &listener,
        &command,
        &[one.to_str().unwrap()],
        &["--unit", "line", "--record", recording.to_str().unwrap()],
    )?;
    assert_eq!(rows(&output)[0]["first_line"], 1);
    assert_eq!(listener.requests().len(), 1);
    let replay = call(
        &listener,
        &command,
        &[two.to_str().unwrap()],
        &["--unit", "line", "--replay", recording.to_str().unwrap()],
    )?;
    assert_eq!(rows(&replay)[0]["file"], two.to_str().unwrap());
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn invalid_reader_options_and_missing_operands_admit_no_requests() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    let path = fixture().join("documents");
    for extra in [
        vec!["--unit", "file", "--window", "2"],
        vec!["--window", "0"],
        vec!["--unit", "line", "--jsonl"],
    ] {
        let output = call(
            &listener,
            &["decide", "Clear?"],
            &[path.to_str().unwrap()],
            &extra,
        )?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(listener.requests().is_empty());
    }
    let output = call(
        &listener,
        &["decide", "Clear?"],
        &[path.to_str().unwrap(), "/missing-source-0420"],
        &[],
    )?;
    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn source_rank_withholds_output_and_stops_before_the_invalid_tail_at_excess() -> io::Result<()> {
    let place = folder("source-rank-budget")?;
    let first = place.join("01-first.txt");
    let excess = place.join("02-excess.txt");
    let tail = place.join("03-unread.txt");
    fs::write(&first, "good\n")?;
    fs::write(&excess, "y".repeat(16 * 1024 * 1024))?;
    fs::write(&tail, b"\xff")?;
    let listener = Listener::answering(answer)?;
    for paths in [
        vec![place.to_str().unwrap()],
        vec![
            first.to_str().unwrap(),
            excess.to_str().unwrap(),
            tail.to_str().unwrap(),
        ],
    ] {
        let output = call(
            &listener,
            &["rank", "Relevant?"],
            &paths,
            &["--unit", "file", "--facts"],
        )?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        let stderr = text(&output.stderr);
        assert_eq!(
            stderr.lines().next(),
            Some("thinkthen: source rank reads at most 16 MiB across all input records")
        );
        assert!(output.stdout.is_empty());
        let facts: Value = serde_json::from_str(stderr.lines().last().unwrap()).unwrap();
        let requests = listener.requests();
        assert_eq!(facts["requests_sent"], requests.len());
        assert!(requests.len() <= 1, "only the preceding record may send");
        for request in requests {
            let request: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(
                request["questions"]["q1"]["instructions"],
                "The text is \"good\\n\". Relevant?"
            );
        }
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn unrepresentable_folder_names_refuse_the_complete_manifest_without_sending() -> io::Result<()> {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt as _;
    let place = folder("source-filename")?;
    fs::write(place.join("01-unread.txt"), b"\xff")?;
    fs::write(
        place.join(OsString::from_vec(b"02-\xff.txt".to_vec())),
        "original evidence",
    )?;
    let error = thinkthen::read_files([&place], thinkthen::ReaderOptions::default()).unwrap_err();
    assert_eq!(error.detail().message(), "source path is not valid UTF-8");
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &["decide", "Clear?"],
        &[place.to_str().unwrap()],
        &[],
    )?;
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert_eq!(
        text(&output.stderr),
        "thinkthen: --input could not be opened: source path is not valid UTF-8\n"
    );
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn recognize_and_relate_windows_keep_original_units_and_physical_spans() -> io::Result<()> {
    let place = folder("source-names-windows")?;
    let path = place.join("names.txt");
    let original = "café 😀\r\nAda at Acme\r\nlast";
    fs::write(&path, original)?;
    let listener = Listener::answering(answer)?;
    for verb in ["recognize", "relate"] {
        let help = spawn(&[verb, "--help"], &[], b"")?;
        assert!(text(&help.stdout).contains("--window <N>"), "{verb}");
        let command = if verb == "recognize" {
            vec![verb, "person", "organization"]
        } else {
            vec![verb, "connected"]
        };
        let output = call(
            &listener,
            &command,
            &[path.to_str().unwrap()],
            &["--window", "2"],
        )?;
        let values = rows(&output);
        assert!(!values.is_empty(), "{verb}");
        check_window_requests(&listener, path.to_str().unwrap());
        if verb == "recognize" {
            assert_eq!(values.len(), 2);
            assert_eq!(values[0]["input"], "café 😀\r\nAda at Acme");
            assert_eq!(values[0]["first_line"], 1);
            assert_eq!(values[0]["last_line"], 2);
            assert_eq!(values[1]["input"], "last");
            assert_eq!(values[1]["first_line"], 3);
            let ada = values[0]["value"]["entities"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entity| entity["text"] == "Ada")
                .unwrap();
            assert_eq!(ada["first_line"], 2);
            assert_eq!(ada["last_line"], 2);
        } else {
            check_window_endpoints(&values, path.to_str().unwrap());
        }
        for extra in [
            vec!["--window", "0"],
            vec!["--unit", "file", "--window", "2"],
        ] {
            let rejected = call(&listener, &command, &[path.to_str().unwrap()], &extra)?;
            assert_eq!(rejected.status.code(), Some(2));
            assert!(listener.requests().is_empty());
        }
    }
    Ok(())
}

fn check_window_endpoints(values: &[Value], path: &str) {
    for edge in values {
        for endpoint in ["source", "target"] {
            let row = &edge[endpoint];
            assert_eq!(row["file"], path);
            if row["first_line"] == 1 {
                assert_eq!(row["record"], "café 😀\r\nAda at Acme");
                assert_eq!(row["last_line"], 2);
            } else {
                assert_eq!(row["first_line"], 3);
                assert_eq!(row["record"], "last");
                assert_eq!(row["last_line"], 3);
            }
        }
    }
}

fn check_window_requests(listener: &Listener, path: &str) {
    let requests = listener.requests();
    assert!(!requests.is_empty(), "window calls send native requests");
    for request in requests {
        let body = text(&request.body);
        assert!(!body.contains(path));
        assert!(!body.contains("first_line"));
        assert!(!body.contains("last_line"));
    }
}
