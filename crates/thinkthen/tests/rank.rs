#![cfg(test)]
//! Scale ranking preserves stable ties, original positions and failed records.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use std::num::NonZeroUsize;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use thinkthen::{Cancel, Key, Rank, Record, Request, Settings};

fn settings(server: &Loopback) -> Settings {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch("1".parse().unwrap())
}

#[test]
fn rank_orders_stably_with_failures_last_and_limits_successful_rows() {
    for top in [None, Some(1), Some(2), Some(20)] {
        let server = Loopback::answering(|q| {
            let text = q["instructions"].as_str().unwrap();
            if text.contains("broken") {
                return json!({});
            }
            let p = if text.contains("low") {
                0.1
            } else if text.contains("middle") {
                0.5
            } else {
                0.9
            };
            json!({"type":"score","probabilities":{"0":1.0-p,"1":p}})
        })
        .unwrap();
        let engine = settings(&server).engine(|_| None).unwrap();
        let mut question = Rank::new("Priority?", ["low", "high"])
            .unwrap()
            .field("/body".parse().unwrap());
        if let Some(top) = top {
            question = question.top(NonZeroUsize::new(top).unwrap()).unwrap();
        }
        let inputs = [
            "broken",
            "low",
            "high-a",
            "middle",
            "high-b",
            "broken-again",
        ];
        let rows = engine
            .rows(
                &question,
                inputs.map(|text| Record::json(json!({"id":text,"body":text}))),
            )
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let successful = ["high-a", "high-b", "middle", "low"];
        let length = top.unwrap_or(4).min(4);
        let expected: Vec<_> = successful[..length]
            .iter()
            .copied()
            .chain(["broken", "broken-again"])
            .collect();
        assert_eq!(
            rows.iter()
                .map(|row| row.id().unwrap().as_str().unwrap())
                .collect::<Vec<_>>(),
            expected
        );
        for (index, row) in rows.iter().enumerate() {
            let id = row.id().unwrap().as_str().unwrap();
            assert_eq!(
                row.position(),
                (inputs.iter().position(|input| *input == id).unwrap() + 1) as u64
            );
            assert!(row.probability().is_none());
            let original = row.record().expect("rank rows retain original input");
            assert_eq!(original.id(), row.id());
            let (kind, bytes) = original.original();
            assert_eq!(kind, thinkthen::records::RecordKind::Json);
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(bytes).unwrap(),
                json!({"id":id,"body":id})
            );
            assert!(serde_json::to_value(row).unwrap().get("original").is_none());
            if index < length {
                assert_eq!(row.clone().into_answer().unwrap().get(), index + 1);
            } else {
                assert_eq!(row.error().unwrap().code().as_str(), "backend.reply");
            }
        }
        assert_eq!(server.count(), 6);
    }
}

#[test]
fn empty_rank_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = settings(&server).engine(|_| None).unwrap();
    assert!(
        engine
            .rows(
                &Rank::new("Priority?", ["low", "high"]).unwrap(),
                Vec::<Record>::new()
            )
            .unwrap()
            .next()
            .is_none()
    );
    assert_eq!(server.count(), 0);
}

#[test]
fn cancelled_rank_does_not_read_input() {
    let server = Loopback::noul(0.9).unwrap();
    let cancel = Cancel::default();
    cancel.cancel();
    let read = Arc::new(AtomicUsize::new(0));
    let seen = read.clone();
    let records = (0..3).map(move |_| {
        seen.fetch_add(1, Ordering::SeqCst);
        Record::text("record")
    });
    let engine = settings(&server).engine(|_| None).unwrap().cancel(cancel);
    let failure = engine
        .rows(&Rank::new("Priority?", ["low", "high"]).unwrap(), records)
        .unwrap_err();
    assert_eq!(failure.code().as_str(), "local.cancelled");
    assert_eq!(read.load(Ordering::SeqCst), 0);
    assert_eq!(server.count(), 0);
}

#[test]
fn rank_keeps_zero_scores_tied_below_positive_answers() {
    for top in [None, Some(2)] {
        let server = Loopback::answering(|q| {
            let text = q["instructions"].as_str().unwrap();
            let p = if text.contains("negative-zero") {
                -0.0
            } else if text.contains("positive-zero") {
                0.0
            } else {
                0.1
            };
            json!({"type":"score","probabilities":{"0":1.0-p,"1":p}})
        })
        .unwrap();
        let engine = settings(&server).engine(|_| None).unwrap();
        let mut question = Rank::new("Priority?", ["low", "high"]).unwrap();
        if let Some(top) = top {
            question = question.top(NonZeroUsize::new(top).unwrap()).unwrap();
        }
        let records = ["negative-zero", "positive-zero", "positive"]
            .map(|text| Record::text(text).with_id(text.into()));
        let rows = engine
            .rows(&question, records)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let expected = ["positive", "negative-zero", "positive-zero"];
        assert_eq!(
            rows.iter()
                .map(|row| row.id().unwrap().as_str().unwrap())
                .collect::<Vec<_>>(),
            expected[..top.unwrap_or(3)]
        );
    }
}

#[test]
fn zero_top_is_refused_at_json_admission_without_sending() {
    let server = Loopback::noul(0.9).unwrap();
    assert!(NonZeroUsize::new(0).is_none());
    let error = Request::from_json(r#"{"question":{"function":"rank","question":"Priority?","levels":["low","high"],"top":0},"records":["evidence"]}"#).unwrap_err();
    assert_eq!(error.code().as_str(), "usage.question");
    assert_eq!(server.count(), 0);
}

#[cfg(target_os = "linux")]
#[test]
fn ranked_top_one_retains_bounded_original_storage_for_a_long_stream() {
    fn resident_bytes() -> usize {
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        status
            .lines()
            .find_map(|line| {
                line.strip_prefix("VmRSS:")
                    .and_then(|value| value.split_whitespace().next())
                    .map(|value| value.parse::<usize>().unwrap() * 1024)
            })
            .unwrap()
    }
    let samples = Arc::new(std::sync::Mutex::new(Vec::new()));
    let observed = Arc::clone(&samples);
    let engine = Settings::default()
        .with_backend(thinkthen::BackendName::Stub)
        .batch("1".parse().unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Rank::new("Priority?", ["low", "high"])
        .unwrap()
        .top(NonZeroUsize::new(1).unwrap())
        .unwrap();
    let records = (0..8192).map(move |index| {
        if index >= 1024 && index % 512 == 0 {
            observed.lock().unwrap().push(resident_bytes());
        }
        thinkthen::records::Record::text(format!("{index}:{}", "x".repeat(32768)))
    });
    let rows = engine
        .rows(&question, records)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].record().unwrap().original().1.len() >= 32768);
    let samples = samples.lock().unwrap();
    let first = samples[0];
    let maximum = *samples.iter().max().unwrap();
    assert!(
        maximum.saturating_sub(first) < 64 * 1024 * 1024,
        "top one retained more than 64 MiB while consuming 256 MiB: first={first}, peak={maximum}"
    );
}
