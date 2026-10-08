//! Record context uses the existing stage requests and shared engine workers.
use super::*;
use serde_json::json;

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one counted outside-in case checks record separation, saved requests, replay and pre-send admission through the same command"
)]
fn saved_record_context_exchanges_keep_jobs_eight_separation_and_zero_send_replay() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../conformance/recognition-context.json"
    ))
    .unwrap();
    let exchanges: Vec<Value> = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row["exchanges"].as_array().unwrap().clone())
        .collect();
    let saved = exchanges.clone();
    let listener = Listener::answering(move |body| {
        let request = std::str::from_utf8(body).unwrap();
        let exchange = saved
            .iter()
            .find(|e| e["request"].as_str() == Some(request))
            .unwrap_or_else(|| panic!("unexpected request {request}"));
        Canned::ok(&exchange["response"].to_string())
    })
    .unwrap();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("recognition-record-context-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let question = root.join("question.json");
    fs::write(&question, fixture["question_json"].as_str().unwrap()).unwrap();
    let operand = format!("@{}", question.display());
    let shared = root.join("context.txt");
    fs::write(&shared, fixture["shared_context"].as_str().unwrap()).unwrap();
    let shared = shared.to_string_lossy();
    let input: String = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .take(3)
        .map(|row| {
            format!(
                "{}\n",
                json!({"body":fixture["text"],"context":row["context"],"private":"retain"})
            )
        })
        .collect();
    for jobs in ["1", "8"] {
        let record = root.join(format!("record-{jobs}"));
        let record = record.to_string_lossy();
        let base = [
            "recognize",
            &operand,
            "--model",
            "jev-1.13.0",
            "--url",
            listener.base(),
            "--jsonl",
            "--field",
            "/body",
            "--context-field",
            "/context",
            "--context",
            &shared,
            "--jobs",
            jobs,
            "--details",
            "--no-cache",
        ];
        let before = listener.count();
        let output = spawn(
            &[&base[..], &["--record", &record]].concat(),
            &[("THINKTHEN_API_KEY", "fake")],
            input.as_bytes(),
        )
        .unwrap();
        assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
        assert_eq!(listener.count() - before, 9);
        let rows: Vec<Value> = stdout(&output)
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len(), 3);
        for (at, row) in rows.iter().enumerate() {
            assert_eq!(row["input"]["context"], fixture["rows"][at]["context"]);
            assert_eq!(row["value"], fixture["value"]);
            assert_eq!(row["input"]["private"], "retain");
            assert_eq!(row["meta"].get("context_sha256").is_some(), at != 2);
        }
        let mut sent: Vec<String> = listener
            .requests()
            .iter()
            .map(|r| String::from_utf8(r.body.clone()).unwrap())
            .collect();
        let mut expected: Vec<String> = exchanges
            .iter()
            .take(9)
            .map(|e| e["request"].as_str().unwrap().to_owned())
            .collect();
        sent.sort();
        expected.sort();
        assert_eq!(sent, expected);
        let replay = spawn(
            &[&base[..], &["--replay", &record]].concat(),
            &[],
            input.as_bytes(),
        )
        .unwrap();
        assert_eq!(
            replay.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&replay.stderr)
        );
        assert_eq!(listener.count() - before, 9);
        let changed = input.replace("First context", "Changed context");
        let missed = spawn(
            &[&base[..], &["--replay", &record]].concat(),
            &[],
            changed.as_bytes(),
        )
        .unwrap();
        assert_ne!(missed.status.code(), Some(0));
        assert_eq!(listener.count() - before, 9);
    }
    let before = listener.count();
    for selector in ["invalid", "/missing", "/context"] {
        for value in [Value::Null, json!(4), json!(false)] {
            let input = format!(
                "{}\n{}\n",
                json!({"body":fixture["text"],"context":"First context"}),
                json!({"body":fixture["text"],"context":value})
            );
            let output = spawn(
                &[
                    "recognize",
                    &operand,
                    "--jsonl",
                    "--field",
                    "/body",
                    "--context-field",
                    selector,
                    "--model",
                    "jev-1.13.0",
                    "--url",
                    listener.base(),
                    "--no-cache",
                    "--jobs",
                    "8",
                ],
                &[("THINKTHEN_API_KEY", "fake")],
                input.as_bytes(),
            )
            .unwrap();
            assert_eq!(
                output.status.code(),
                Some(2),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(listener.count(), before);
        }
    }
}
