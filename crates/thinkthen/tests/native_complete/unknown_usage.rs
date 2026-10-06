//! Missing counts and reported zero stay distinct across actual storage routes.
use super::*;

#[test]
fn empty_usage_stays_absent_and_explicit_zero_stays_reported_through_cache_and_replay() {
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
        let build = || {
            Engine::builder()
                .base_url(listener.base())
                .unwrap()
                .model("fixed")
                .unwrap()
                .api_key("unknown-usage-private")
                .unwrap()
                .max_retries(0)
        };
        let engine = build().cache_at(&root).unwrap().build().unwrap();
        let question = Question::decide("Good?").unwrap().cut();
        let live = engine
            .decide_complete_with(&question, "Text.", CallOptions::new())
            .unwrap();
        let held = engine
            .decide_complete_with(&question, "Text.", CallOptions::new())
            .unwrap();
        let replay = build().replay(&root).unwrap().build().unwrap();
        let replayed = replay
            .decide_complete_with(&question, "Text.", CallOptions::new())
            .unwrap();
        for call in [&live, &held, &replayed] {
            let row: Value = serde_json::from_str(&call.value().to_json().unwrap()).unwrap();
            assert_eq!(row["meta"].get("usage"), expected.as_ref());
            assert_eq!(call.value().reported_usage().is_some(), expected.is_some());
            assert_eq!(call.value().answer_id(), live.value().answer_id());
            schema::call(call, "completeDecide");
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
}
