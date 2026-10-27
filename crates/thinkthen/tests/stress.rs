#![cfg(test)]
//! Short gate coverage and separately selected loopback volume/soak workloads.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

#[path = "stress/axes.rs"]
mod axes;
#[cfg(feature = "command")]
mod support;
use loopback::{KEY, StressFixture, stress_table};
use serde_json::Value;
#[cfg(feature = "command")]
use std::io::{BufRead, BufReader, Read, Write};
#[cfg(feature = "command")]
use std::process::Stdio;
use thinkthen::{Decide, Record, Settings};

fn assert_row(row: &Value, position: usize, ordinal: usize, scenario: &Value) {
    assert_eq!(row["position"], position);
    assert_eq!(row["id"], identifier(ordinal));
    match row["status"].as_str().unwrap() {
        "answered" => {
            assert_eq!(row["answer"], true);
            assert_eq!(row["probability"], 0.9);
        }
        "failed" => assert_eq!(row["error"]["code"], scenario["code"]),
        status => panic!("unexpected status {status}"),
    }
    if scenario["status"] == "answered" {
        assert_eq!(row["status"], "answered");
    }
    if scenario["name"] == "refused" {
        assert_eq!(row["status"], "failed");
    }
}
fn rust_call(table: &Value, scenario: &Value, backend: &str, count: usize, rounds: usize) {
    let fixture = StressFixture::open(scenario["name"].as_str().unwrap()).unwrap();
    let engine = stress_engine(table, backend, fixture.base());
    for round in 0..rounds {
        let widths = rust_rows_at(&engine, scenario, count, "Ready?", round * count);
        if rounds > 1 {
            resource_checkpoint("rust", backend, &fixture, std::process::id(), widths, count);
        }
    }
    assert!(fixture.stats()["recorded_requests"].as_u64().unwrap() <= 8);
}
fn stress_engine(table: &Value, backend: &str, url: &str) -> thinkthen::Engine {
    let mut settings = table["client"].clone();
    settings["backend"] = backend.into();
    Settings::from_json(&settings.to_string())
        .unwrap()
        .url(url)
        .api_key(KEY.parse().unwrap())
        .engine(|_| None)
        .unwrap()
}
fn rust_rows(engine: &thinkthen::Engine, scenario: &Value, count: usize, question: &str) {
    rust_rows_at(engine, scenario, count, question, 0);
}
fn rust_rows_at(
    engine: &thinkthen::Engine,
    scenario: &Value,
    count: usize,
    question: &str,
    offset: usize,
) -> u64 {
    let question = Decide::new(question).unwrap();
    let rows = engine.rows(&question, records_at(count, offset)).unwrap();
    let mut received = 0;
    let mut failed = 0;
    let mut widths = 0_u64;
    for row in rows {
        received += 1;
        let value = serde_json::to_value(row.unwrap()).unwrap();
        failed += usize::from(value["status"] == "failed");
        widths |= 1_u64 << value["id"].as_str().unwrap().len();
        assert_row(&value, received, offset + received - 1, scenario);
    }
    assert_eq!(received, count);
    assert_eq!(failed > 0, scenario["status"] == "failed");
    widths
}
fn records(count: usize) -> impl Iterator<Item = Record> + Send + 'static {
    records_at(count, 0)
}
fn records_at(count: usize, offset: usize) -> impl Iterator<Item = Record> + Send + 'static {
    (offset..offset + count)
        .map(|i| Record::text(format!("record {}", record_number(i))).with_id(identifier(i).into()))
}
fn record_number(ordinal: usize) -> usize {
    static BASIS: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    let basis = BASIS.get_or_init(|| {
        stress_table().unwrap()["records"]["identifier_base"]
            .as_u64()
            .unwrap() as usize
    });
    basis + ordinal
}
fn identifier(ordinal: usize) -> String {
    format!("record-{}", record_number(ordinal))
}
#[cfg(feature = "command")]
fn command_process(
    table: &Value,
    backend: &str,
    url: &str,
) -> (std::process::Child, std::thread::JoinHandle<Vec<u8>>) {
    spawn_command(table, backend, url, "Ready?", Stdio::piped())
}
#[cfg(feature = "command")]
fn spawn_command(
    table: &Value,
    backend: &str,
    url: &str,
    question: &str,
    input: Stdio,
) -> (std::process::Child, std::thread::JoinHandle<Vec<u8>>) {
    let mut child = support::command()
        .args([
            "decide",
            question,
            "--from",
            "jsonl",
            "--evidence",
            "/body",
            "--json",
            "--backend",
            backend,
            "--url",
            url,
            "--model",
            table["client"]["model"].as_str().unwrap(),
            "--batch",
            &table["client"]["batch"].to_string(),
        ])
        .env("THINKTHEN_API_KEY", KEY)
        .env("OPENAI_API_KEY", KEY)
        .stdin(input)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stderr = child.stderr.take().unwrap();
    // Drain diagnostics while reading rows; failed volume cases exceed a pipe's capacity.
    let diagnostics = std::thread::spawn(move || {
        let mut retained = Vec::new();
        let mut buffer = [0; 4096];
        loop {
            let count = stderr.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            let keep = count.min(4096 - retained.len());
            retained.extend_from_slice(&buffer[..keep]);
        }
        retained
    });
    (child, diagnostics)
}
#[cfg(feature = "command")]
fn command_call(table: &Value, scenario: &Value, backend: &str, count: usize) {
    let fixture = StressFixture::open(scenario["name"].as_str().unwrap()).unwrap();
    let (mut child, diagnostics) = command_process(table, backend, fixture.base());
    let mut input = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        for i in 0..count {
            writeln!(
                input,
                "{}",
                serde_json::json!({"id":identifier(i),"body":format!("record {}", record_number(i))})
            )
            .unwrap();
        }
    });
    let mut received = 0;
    for line in BufReader::new(child.stdout.take().unwrap()).lines() {
        received += 1;
        assert_row(
            &serde_json::from_str::<Value>(&line.unwrap()).unwrap(),
            received,
            received - 1,
            scenario,
        );
    }
    writer.join().unwrap();
    let status = child.wait().unwrap();
    let stderr = diagnostics.join().unwrap();
    let wanted = if scenario["status"] == "answered" {
        0
    } else {
        6
    };
    assert_eq!(
        status.code(),
        Some(wanted),
        "{}",
        String::from_utf8_lossy(&stderr)
    );
    assert_eq!(received, count);
    assert!(fixture.stats()["recorded_requests"].as_u64().unwrap() <= 8);
}
#[cfg(feature = "command")]
fn command_soak(table: &Value, backend: &str, count: usize, rounds: usize) {
    let fixture = StressFixture::open("healthy").unwrap();
    let scenario = &table["scenarios"][0];
    let batch = table["client"]["batch"].as_u64().unwrap() as usize;
    let count = count.div_ceil(batch) * batch;
    let (mut child, diagnostics) = command_process(table, backend, fixture.base());
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap()).lines();
    let (release, next) = std::sync::mpsc::channel();
    let writer = std::thread::spawn(move || {
        for round in 0..rounds {
            for i in round * count..(round + 1) * count {
                writeln!(
                    input,
                    "{}",
                    serde_json::json!({"id":identifier(i),"body":format!("record {}", record_number(i))})
                )
                .unwrap();
            }
            input.flush().unwrap();
            // Keep stdin and the same command alive through its final sample.
            next.recv().unwrap();
        }
    });
    for round in 0..rounds {
        let mut widths = 0_u64;
        for position in round * count + 1..=(round + 1) * count {
            let row: Value = serde_json::from_str(&output.next().unwrap().unwrap()).unwrap();
            widths |= 1_u64 << row["id"].as_str().unwrap().len();
            assert_row(&row, position, position - 1, scenario);
        }
        assert!(child.try_wait().unwrap().is_none());
        resource_checkpoint("command", backend, &fixture, child.id(), widths, count);
        release.send(()).unwrap();
    }
    writer.join().unwrap();
    assert!(output.next().is_none());
    let status = child.wait().unwrap();
    let stderr = diagnostics.join().unwrap();
    assert!(status.success(), "{}", String::from_utf8_lossy(&stderr));
}
fn resource_checkpoint(
    surface: &str,
    backend: &str,
    fixture: &StressFixture,
    pid: u32,
    widths: u64,
    verified_records: usize,
) {
    if let Ok(hook) = std::env::var("THINKTHEN_STRESS_CHECKPOINT_COMMAND") {
        let port = fixture
            .base()
            .strip_prefix("http://127.0.0.1:")
            .unwrap()
            .split('/')
            .next()
            .unwrap();
        let mut context = fixture.stats();
        context["identifier_width_mask"] = widths.into();
        context["verified_records"] = verified_records.into();
        let status = std::process::Command::new("sh")
            .args(["-c", &hook])
            .env("THINKTHEN_STRESS_BACKEND_PORT", port)
            .env("THINKTHEN_STRESS_GROUP", format!("{surface}/{backend}"))
            .env("THINKTHEN_STRESS_REQUIRED_PID", pid.to_string())
            .env("THINKTHEN_STRESS_CONCURRENCY", "1")
            .env("THINKTHEN_STRESS_CONTEXT", context.to_string())
            .status()
            .unwrap();
        assert!(status.success(), "resource checkpoint failed");
    }
}
fn backends() -> Vec<&'static str> {
    vec!["systemone", "openai"]
}
fn cases(full: bool) {
    let table = stress_table().unwrap();
    let count = table["records"][if full { "full" } else { "smoke" }]
        .as_u64()
        .unwrap() as usize;
    for scenario in table["scenarios"].as_array().unwrap() {
        for backend in backends() {
            rust_call(&table, scenario, backend, count, 1);
            #[cfg(feature = "command")]
            command_call(&table, scenario, backend, count);
        }
    }
    if full {
        axes::cases(&table, count);
        let rounds = table["records"]["soak_rounds"].as_u64().unwrap();
        let healthy = &table["scenarios"][0];
        for backend in backends() {
            rust_call(&table, healthy, backend, count, rounds as usize);
            #[cfg(feature = "command")]
            command_soak(&table, backend, count, rounds as usize);
        }
    }
}
#[test]
fn loopback_stress_smoke_preserves_every_input_position() {
    cases(false);
}
#[test]
#[ignore = "separate full stress job after landing"]
fn loopback_stress_full_volume_and_soak() {
    cases(true);
}

#[cfg(feature = "command")]
#[test]
fn command_soak_samples_one_live_stream() {
    let table = stress_table().unwrap();
    command_soak(
        &table,
        "systemone",
        table["records"]["smoke"].as_u64().unwrap() as usize,
        table["records"]["soak_rounds"].as_u64().unwrap() as usize,
    );
}

#[test]
fn soak_identifiers_keep_their_type_width_and_unique_ordinals() {
    let table = stress_table().unwrap();
    let fixture = StressFixture::open("healthy").unwrap();
    let engine = stress_engine(&table, "systemone", fixture.base());
    let full = table["records"]["full"].as_u64().unwrap() as usize;
    let rounds = table["records"]["soak_rounds"].as_u64().unwrap() as usize;
    let batch = table["client"]["batch"].as_u64().unwrap() as usize;
    let full = full.div_ceil(batch) * batch;
    let basis = record_number(0);
    let last = basis + full * rounds * 2 - 1;
    assert_eq!(basis.to_string().len(), last.to_string().len());
    assert!(last < i32::MAX as usize);
    let mut identifiers = std::collections::BTreeSet::new();
    let mut widths = std::collections::BTreeSet::new();
    // Sample actual records at the first round, both sides of the warm-up split,
    // and the final concurrent call without running the intervening volume.
    for offset in [
        0,
        full * (rounds / 2) - 2,
        full * (rounds / 2),
        full * rounds * 2 - 2,
    ] {
        let question = Decide::new("Ready?").unwrap();
        for (index, row) in engine
            .rows(&question, records_at(2, offset))
            .unwrap()
            .enumerate()
        {
            let row = serde_json::to_value(row.unwrap()).unwrap();
            assert_row(&row, index + 1, offset + index, &table["scenarios"][0]);
            let id = row["id"].as_str().unwrap();
            assert!(identifiers.insert(id.to_owned()));
            widths.insert(id.len());
            assert!(row["position"].is_u64());
            assert!(row["answer"].is_boolean());
            assert!(row["probability"].is_number());
        }
    }
    assert_eq!(identifiers.len(), 8);
    assert_eq!(widths.len(), 1);
}
