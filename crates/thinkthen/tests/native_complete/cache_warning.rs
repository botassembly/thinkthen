//! Excluded held models warn without exposing stored values or crossing routes.
use super::*;
#[test]
fn excluded_held_model_warning_is_call_scoped_and_corrected_cache_hits_remain_silent() {
    let next = std::sync::Arc::new(AtomicUsize::new(0));
    let sent = next.clone();
    let listener = Listener::answering(move |_| {
        let model = if sent.fetch_add(1, Ordering::SeqCst) == 0 {
            "stored-private-model"
        } else {
            "fixed"
        };
        Canned::ok(&format!(
            r#"{{"model":"{model}","answers":{{"q1":{{"type":"noul","noul":0.7}}}}}}"#
        ))
    })
    .unwrap();
    let folder = folder();
    let engine = crate::api::fixed(listener.base(), "complete-private")
        .cache_at(&folder)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let first = crate::api::decide(&engine, &question, "Refund me.", CallOptions::new()).unwrap();
    assert!(!first.facts().held_model_mismatch());
    assert_eq!(
        crate::api::decision(&first)
            .unwrap()
            .identity()
            .answered_by(),
        Some("stored-private-model")
    );
    let corrected =
        crate::api::decide(&engine, &question, "Refund me.", CallOptions::new()).unwrap();
    let corrected_result = crate::api::decision(&corrected).unwrap();
    assert!(corrected.facts().held_model_mismatch());
    assert_eq!(corrected.facts().requests_sent(), 1);
    assert_eq!(corrected_result.identity().answered_by(), Some("fixed"));
    let row = serde_json::to_value(corrected.complete().unwrap()).unwrap();
    assert_eq!(row["facts"]["held_model_mismatch"], true);
    schema::check(&row, "completeDecide");
    assert!(!row.to_string().contains("stored-private-model"));
    let hit = crate::api::decide(&engine, &question, "Refund me.", CallOptions::new()).unwrap();
    let hit_result = crate::api::decision(&hit).unwrap();
    assert!(!hit.facts().held_model_mismatch());
    assert_eq!(hit.facts().requests_sent(), 0);
    assert_eq!(hit_result.identity().origin(), Some(Origin::Cache));
    assert_eq!(
        hit_result.identity().answer_id(),
        corrected_result.identity().answer_id()
    );
    assert_eq!(listener.count(), 2);
    let held = crate::api::decide(
        &Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .replay(&folder)
            .unwrap()
            .build()
            .unwrap(),
        &question,
        "Refund me.",
        CallOptions::new(),
    )
    .unwrap_err();
    assert_eq!(held.kind(), ErrorKind::Local);
    assert!(!held.facts().unwrap().held_model_mismatch());
    assert_eq!(held.facts().unwrap().requests_sent(), 0);
    assert_eq!(listener.count(), 2);
}
