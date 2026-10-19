//! Proposal facts survive settlement and share ordinary cached stage answers.
use super::*;
use thinkthen::{Kind, RecognitionSeedSpan, Recognize};

#[path = "../support/recognition_proposals.rs"]
mod fixture;
use fixture::response;

#[test]
fn judged_proposals_retain_declines_duplicate_winners_and_under_cut_seed_facts() {
    let listener = Listener::answering(response).unwrap();
    let cache = folder();
    let engine = crate::api::fixed(listener.base(), "fake")
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .build()
        .unwrap()
        .with_seed_spans(vec![RecognitionSeedSpan {
            start: 11,
            end: 14,
            kind: None,
        }]);
    let first = crate::api::recognize(&engine, &ask, "!! Ada Bob Åda", CallOptions::new()).unwrap();
    let result = crate::api::recognition(&first).unwrap();
    let proposals = result
        .probabilities()
        .judged_proposals()
        .collect::<Vec<_>>();
    assert_eq!(proposals.len(), 5);
    assert_eq!(
        proposals.iter().map(|p| p.range()).collect::<Vec<_>>(),
        [0..1, 1..2, 3..6, 7..10, 11..14]
    );
    assert_span_probabilities(&proposals);
    assert_eq!(
        proposals.iter().map(|p| p.strength()).collect::<Vec<_>>(),
        [Some(0.54), Some(0.72), None, Some(0.42), Some(0.36)]
    );
    assert_eq!(
        proposals.iter().map(|p| p.kept()).collect::<Vec<_>>(),
        [false, true, false, false, false]
    );
    assert_eq!(proposals[0].selected(), Some(0..2));
    assert_eq!(proposals[1].selected(), Some(0..2));
    assert_eq!(proposals[2].selected(), None);
    assert_eq!(proposals[2].kind(), None);
    assert_eq!(proposals[4].kind(), Some("person"));
    let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert!(document["answer"]["proposals"][2].get("strength").is_none());
    assert!(document["answer"]["proposals"][2].get("selected").is_none());
    assert_eq!(
        document["value"],
        json!({"entities":[{"text":"!!","start":0,"end":2,"length":2,"kind":"person","strength":0.72}]})
    );
    schema::call(&first, "completeRecognition");
    let replay =
        crate::api::recognize(&engine, &ask, "!! Ada Bob Åda", CallOptions::new()).unwrap();
    let replay_value = crate::api::recognition(&replay).unwrap();
    assert_eq!(first.facts().requests_sent(), 2);
    assert_eq!(replay.facts().requests_sent(), 0);
    assert_eq!(result.answer_id(), replay_value.answer_id());
    assert_eq!(
        result.identity().observations(),
        replay_value.identity().observations()
    );
    let cached: Value = serde_json::from_str(&replay_value.to_json().unwrap()).unwrap();
    assert_eq!(document["answer"], cached["answer"]);
    assert_eq!(listener.count(), 2);
    assert_eq!(listener.questions(), 12);
    std::fs::remove_dir_all(cache).unwrap();
}

#[cfg(test)]
fn assert_span_probabilities(proposals: &[thinkthen::RecognitionProposal<'_>]) {
    for (p, expected) in proposals.iter().zip([0.9, 0.8, 0.9, 0.6, 0.4]) {
        assert!((p.span_probability() - expected).abs() < 0.00001);
    }
}
