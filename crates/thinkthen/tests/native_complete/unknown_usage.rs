//! Missing counts and reported zero stay distinct across actual storage routes.
use super::*;

#[test]
fn empty_usage_stays_absent_and_explicit_zero_stays_reported_through_cache_and_replay() {
    let mut documents = Vec::new();
    for (usage, expected) in [
        (json!({}), None),
        (json!({"input_tokens":0}), Some(json!({"input_tokens":0}))),
        (
            json!({"input_tokens":0,"output_tokens":0}),
            Some(json!({"input_tokens":0,"output_tokens":0})),
        ),
    ] {
        let body =
            json!({"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":usage})
                .to_string();
        let listener = Listener::answering(move |_| Canned::ok(&body)).unwrap();
        let root = folder();
        let build = || crate::api::fixed(listener.base(), "unknown-usage-private").max_retries(0);
        let engine = build().cache_at(&root).unwrap().build().unwrap();
        let question = Question::decide("Good?").unwrap().cut();
        let live = crate::api::decide(&engine, &question, "Text.", CallOptions::new()).unwrap();
        let held = crate::api::decide(&engine, &question, "Text.", CallOptions::new()).unwrap();
        let replay = build().replay(&root).unwrap().build().unwrap();
        let replayed = crate::api::decide(&replay, &question, "Text.", CallOptions::new()).unwrap();
        for call in [&live, &held, &replayed] {
            let result = crate::api::decision(call).unwrap();
            let row: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
            assert_eq!(row["meta"].get("usage"), expected.as_ref());
            assert_eq!(result.reported_usage().is_some(), expected.is_some());
            assert_eq!(
                result.answer_id(),
                crate::api::decision(&live).unwrap().answer_id()
            );
            documents.push(serde_json::to_value(call).unwrap());
        }
        assert_eq!(
            live.facts().input_tokens(),
            expected.as_ref().and_then(|u| u["input_tokens"].as_u64())
        );
        assert_eq!(
            live.facts().output_tokens(),
            expected.as_ref().and_then(|u| u["output_tokens"].as_u64())
        );
        assert_eq!(held.facts().requests_sent(), 0);
        assert_eq!(replayed.facts().requests_sent(), 0);
        assert_eq!(listener.count(), 1);
        assert_eq!(
            serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
            json!({"model":"fixed","state":"Each question quotes the text it asks about.","questions":{"q1":{"type":"noul","instructions":"The text is \"Text.\". Good?"}}})
        );
        drop(replay);
        drop(engine);
        std::fs::remove_dir_all(root).unwrap();
    }
    schema::check_batch(&documents, "completeDecide");
}
