//! Public request identity, retry receipts and strict replay at batch one.

use super::*;
use sha2::{Digest as _, Sha256};

/// Every question key of one request body, in wire order, by ADR 0111
/// section 2: the SHA-256 of the adapter, the URL, the model, the state and
/// one question as the body carries them, joined by line feeds.
pub(super) fn question_keys(url: &str, body: &[u8]) -> Vec<String> {
    use serde_json::value::RawValue;
    #[derive(serde::Deserialize)]
    struct Parts<'a> {
        #[serde(borrow)]
        state: &'a RawValue,
        #[serde(borrow)]
        model: &'a RawValue,
        #[serde(borrow)]
        questions: std::collections::BTreeMap<String, &'a RawValue>,
    }
    let parts: Parts<'_> = serde_json::from_slice(body).expect("a request body");
    let mut questions: Vec<_> = parts
        .questions
        .into_iter()
        .map(|(name, question)| (name[1..].parse::<usize>().expect("a qN name"), question))
        .collect();
    questions.sort_by_key(|(place, _)| *place);
    questions
        .into_iter()
        .map(|(_, question)| {
            let joined = [
                "systemone",
                url,
                parts.model.get(),
                parts.state.get(),
                question.get(),
            ]
            .join("\n");
            Sha256::digest(joined.as_bytes())
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .collect()
}

fn retry_listener() -> (Listener, std::sync::Arc<AtomicUsize>) {
    let no = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.1}},"usage":{"input_tokens":3,"output_tokens":1}}"#;
    let yes = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":5,"output_tokens":2}}"#;
    let alpha_attempts = std::sync::Arc::new(AtomicUsize::new(0));
    let attempts = std::sync::Arc::clone(&alpha_attempts);
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("alpha") {
            if attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                Canned::status(503, "busy").asking("retry-after-ms", "0")
            } else {
                Canned::ok(no)
            }
        } else {
            Canned::ok(yes)
        }
    })
    .expect("listener");
    (listener, alpha_attempts)
}

type RetryReceipt = (usize, Vec<String>, Option<(u64, u64)>, String);

fn assert_retry_receipts(listener: &Listener, details: &Mutex<Vec<RetryReceipt>>) {
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let alpha = requests
        .iter()
        .find(|request| String::from_utf8_lossy(&request.body).contains("alpha"))
        .expect("alpha request");
    let beta = requests
        .iter()
        .find(|request| String::from_utf8_lossy(&request.body).contains("beta"))
        .expect("beta request");
    assert_eq!(
        *details.lock().expect("details"),
        [
            (
                0,
                question_keys(listener.url(), &alpha.body),
                Some((3, 1)),
                listener.url().to_owned()
            ),
            (
                1,
                question_keys(listener.url(), &beta.body),
                Some((5, 2)),
                listener.url().to_owned()
            ),
        ]
    );
}

#[test]
fn duplicate_records_share_one_question_key_in_one_literal_request() {
    let _serial = serial();
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .model("jev-latest")
        .expect("model")
        .no_cache()
        .build()
        .expect("engine");
    let asked = Question::decide("The text is the title of a song by the Beatles.")
        .expect("question")
        .cut();
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            seen.lock().expect("observations").push((
                index,
                detail.requests().to_vec(),
                detail.requests_sent(),
            ));
        }
    };
    let mut rows = engine.decide_many_with(
        &asked,
        ["Come Together", "Because", "Come Together"],
        CallOptions::new()
            .batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(3).expect("three"),
            ))
            .observe(&observe),
    );
    assert_eq!(
        rows.by_ref()
            .collect::<Result<Vec<_>, _>>()
            .expect("rows")
            .len(),
        3
    );
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((3, 1))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let expected =
        include_str!("../../../../specification/fixtures/systemone/batch-duplicate.request.json");
    assert_eq!(requests[0].body, expected.trim_end().as_bytes());
    let [come_together, because] =
        <[String; 2]>::try_from(question_keys(listener.url(), &requests[0].body))
            .expect("two distinct questions");
    assert_eq!(
        *seen.lock().expect("observations"),
        [
            (0, vec![come_together.clone()], 1),
            (1, vec![because], 0),
            (2, vec![come_together], 0)
        ]
    );
}

#[test]
fn a_retried_filtered_row_keeps_its_observation_and_call_facts() {
    let _serial = serial();
    let (listener, alpha_attempts) = retry_listener();
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .max_retries(1)
        .no_cache()
        .build()
        .expect("engine");
    let seen = Mutex::new(Vec::new());
    let details = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question { index, detail, .. } => {
            seen.lock()
                .expect("observations")
                .push((index, "question", detail.requests_sent()));
            details.lock().expect("details").push((
                index,
                detail.requests().to_vec(),
                detail
                    .usage()
                    .map(|usage| (usage.input_tokens(), usage.output_tokens())),
                detail.url().to_owned(),
            ));
        }
        RecordObservation::Row { index, .. } => {
            seen.lock().expect("observations").push((index, "row", 0))
        }
    };
    let asked = question();
    let mut rows = engine.filter_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new()
            .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
            .observe(&observe),
    );
    assert_eq!(
        rows.by_ref()
            .collect::<Result<Vec<_>, _>>()
            .expect("filtered"),
        ["beta"]
    );
    let facts = rows.facts().expect("terminal facts");
    assert_eq!(
        (
            facts.records(),
            facts.requests_sent(),
            facts.input_tokens(),
            facts.output_tokens()
        ),
        (2, 3, Some(8), Some(3))
    );
    assert_eq!(alpha_attempts.load(Ordering::SeqCst), 2);
    assert_retry_receipts(&listener, &details);
    assert_eq!(
        *seen.lock().expect("observations"),
        [
            (0, "question", 2),
            (0, "row", 0),
            (1, "question", 1),
            (1, "row", 0)
        ]
    );
}

#[test]
fn batch_one_replays_a_recorded_entry_and_changed_context_misses_without_sending() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let folder =
        std::env::temp_dir().join(format!("thinkthen-batch-replay-{}", std::process::id()));
    std::fs::create_dir_all(&folder).expect("recording folder");
    let asked = question();
    let setting = BatchSetting::Records(std::num::NonZeroUsize::MIN);
    let recorded = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .record(&folder)
        .expect("record")
        .build()
        .expect("engine");
    let rows = recorded
        .decide_many_with(&asked, ["alpha"], CallOptions::new().batch(setting))
        .collect::<Result<Vec<_>, _>>()
        .expect("recorded");
    assert_eq!(rows.len(), 1);
    let replay = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .replay(&folder)
        .expect("replay")
        .build()
        .expect("engine");
    let mut rows = replay.decide_many_with(&asked, ["alpha"], CallOptions::new().batch(setting));
    assert_eq!(
        rows.by_ref()
            .collect::<Result<Vec<_>, _>>()
            .expect("replayed")
            .len(),
        1
    );
    assert_eq!(
        rows.facts().map(|facts| (
            facts.requests_sent(),
            facts.cache_answers(),
            facts.input_tokens()
        )),
        Some((0, 0, None))
    );
    let missed = replay
        .decide_many_with(
            &asked,
            ["alpha"],
            CallOptions::new().batch(setting).context("changed"),
        )
        .next()
        .expect("strict replay miss")
        .expect_err("changed context missed the recorded question");
    assert_eq!(missed.kind(), ErrorKind::Local);
    assert_eq!(
        missed.to_string(),
        "the replay folder holds no answer for this question"
    );
    assert_eq!(listener.count(), 1, "strict replay opened no new send");
    std::fs::remove_dir_all(folder).expect("remove recording");
}

#[test]
fn context_and_later_record_overflow_keep_zero_send_and_ordered_prefix() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .profile_json(
            r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_request_bytes":500}"#,
        )
        .expect("profile")
        .no_cache()
        .build()
        .expect("engine");
    let asked = question();
    let oversized = "x".repeat(600);
    let one = BatchSetting::Records(std::num::NonZeroUsize::MIN);
    let early = engine
        .decide_many_with(
            &asked,
            ["short"],
            CallOptions::new().batch(one).context(&oversized),
        )
        .next()
        .expect("early refusal")
        .expect_err("context is too large");
    assert_eq!(early.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);

    let mut rows = engine.decide_many_with(
        &asked,
        ["short", oversized.as_str()],
        CallOptions::new().batch(one),
    );
    assert_eq!(
        rows.next().expect("prefix").expect("short row").input(),
        &"short"
    );
    let late = rows.next().expect("later refusal").expect_err("long row");
    assert_eq!(late.kind(), ErrorKind::Usage);
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(listener.count(), 1);
}

/// A group request's question count and its count of distinct quoted records.
fn group_request_shape(body: &[u8]) -> (usize, usize) {
    let body: serde_json::Value = serde_json::from_slice(body).expect("group request");
    let questions = body
        .get("questions")
        .and_then(serde_json::Value::as_object)
        .expect("questions");
    let quoted = questions
        .values()
        .map(|question| {
            let asked = question["instructions"]
                .as_str()
                .expect("text instructions");
            asked.rsplit_once(". ").expect("a quoted record").0
        })
        .collect::<std::collections::BTreeSet<_>>();
    (questions.len(), quoted.len())
}

/// Every group of a record shares the fixed state, so a record's questions
/// ride one request, by ADR 0111 section 5. Under a limit of eight
/// questions, each record of five questions closes the request before it.
#[test]
fn a_question_limit_packs_whole_records_and_every_group_together() {
    let _serial = serial();
    let listener = Listener::answering(|body| {
        let request: serde_json::Value = serde_json::from_slice(body).expect("request");
        let answers = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|name| (name.clone(), serde_json::json!({"type":"noul","noul":0.9})))
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&serde_json::json!({"model":"jev-latest","answers":answers}).to_string())
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .throttle(THROTTLE)
        .expect("throttle")
        .profile_json(
            r#"{"schema":"thinkthen.backend-profile/1","name":"eight","max_questions":8}"#,
        )
        .expect("profile")
        .no_cache()
        .build()
        .expect("engine");
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"a1":{"decide":"A1?","on":"/a"},"a2":{"decide":"A2?","on":"/a"},"a3":{"decide":"A3?","on":"/a"},"a4":{"decide":"A4?","on":"/a"},"b":{"decide":"B?","on":"/b"}}}"#).expect("set");
    let records = [
        r#"{"a":"a0","b":"b0"}"#,
        r#"{"a":"a1","b":"b1"}"#,
        r#"{"a":"a2","b":"b2"}"#,
        r#"{"a":"a3","b":"b3"}"#,
        r#"{"a":"a4","b":"b4"}"#,
    ];
    let mut rows = engine.annotate(&set, records);
    let values = rows.by_ref().collect::<Result<Vec<_>, _>>().expect("rows");
    assert_eq!(
        values.iter().map(|row| row.input()).collect::<Vec<_>>(),
        records.iter().collect::<Vec<_>>()
    );
    assert!(values.iter().all(|row| row.values().len() == 5));
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((5, 5))
    );
    let shapes = listener
        .requests()
        .iter()
        .map(|request| group_request_shape(&request.body))
        .collect::<Vec<_>>();
    assert_eq!(shapes, [(5, 2); 5], "five questions over two quoted parts");
}
