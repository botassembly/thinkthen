//! Eager admission shares duplicate work independently of caller observation.
use super::*;
use thinkthen::{BatchSetting, RecordObservation, RequestFunction};

#[test]
fn eager_duplicates_across_the_window_keep_one_send_per_unique_question_with_every_observer() {
    for width in [1, 2] {
        for observed in 0..3 {
            let listener = Listener::answering(|body| {
                let request: Value = serde_json::from_slice(body).unwrap();
                let answers: serde_json::Map<String, Value> = request["questions"]
                    .as_object().unwrap().keys().map(|key| (key.clone(), json!({"type":"noul","noul":0.9}))).collect();
                Canned::ok(&json!({"model":"fixed","answers":answers,"usage":{"input_tokens":3,"output_tokens":1}}).to_string())
            }).unwrap();
            let engine = engine(&listener);
            let observed_rows = AtomicUsize::new(0);
            let observer = |_: RecordObservation<'_>| {
                observed_rows.fetch_add(1, Ordering::SeqCst);
            };
            let observed_attempts = AtomicUsize::new(0);
            let attempt_observer = |_: thinkthen::AttemptObservation| {
                observed_attempts.fetch_add(1, Ordering::SeqCst);
            };
            let controls = CallOptions::new().batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(width).unwrap(),
            ));
            let controls = match observed {
                1 => controls.observe(&observer),
                2 => controls.attempts(true).observe_attempt(&attempt_observer),
                _ => controls,
            };
            // Twenty unique records exceed the largest tested scheduler window.
            let mut originals: Vec<String> =
                (0..20).map(|index| format!("record {index}")).collect();
            originals.push(originals[0].clone());
            let call = crate::api::whole(
                &engine,
                RequestFunction::Decide,
                Question::decide("Refund?").unwrap().cut().into(),
                crate::api::plain_records(originals),
                controls,
            )
            .unwrap();
            assert_eq!(call.facts().records(), 21);
            assert_eq!(
                listener.questions(),
                20,
                "width {width}, observed {observed}"
            );
            assert_eq!(call.facts().requests_sent(), listener.count() as u64);
            assert_eq!(
                observed_attempts.load(Ordering::SeqCst),
                if observed == 2 { 20 / width } else { 0 }
            );
            assert_eq!(
                call.facts().attempts().map(<[_]>::len),
                (observed == 2).then_some(20 / width)
            );
            assert_eq!(
                listener.count(),
                20 / width,
                "width {width}, observed {observed}"
            );
            assert_eq!(
                observed_rows.load(Ordering::SeqCst),
                if observed == 1 { 42 } else { 0 }
            );
        }
    }
}
