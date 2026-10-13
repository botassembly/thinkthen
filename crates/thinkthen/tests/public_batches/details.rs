//! Token overflow through the admitted public Request tag path.

use super::*;

/// Ticket 0358: a `tag` row whose label shares overflow together fails, as
/// every row does, instead of dropping its usage.
#[test]
fn a_tag_row_whose_label_shares_overflow_fails() {
    let _serial = serial();
    let first = std::sync::atomic::AtomicBool::new(true);
    let listener = Listener::answering(move |_| {
        let input = if first.swap(false, Ordering::SeqCst) { u64::MAX } else { 1 };
        Canned::ok(&format!(
            r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":0.9}}}},"usage":{{"input_tokens":{input},"output_tokens":1}}}}"#
        ))
    })
    .expect("listener");
    let engine = crate::api::builder(listener.base(), "sk-public-batches")
        .throttle(THROTTLE)
        .expect("throttle")
        .profile_json(r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#)
        .expect("one question per request")
        .no_cache()
        .build()
        .expect("engine");
    let tag =
        Question::from_json(r#"{"tag":"Which labels?","labels":["first","second"]}"#).expect("tag");
    let error = crate::api::pull_tags(
        &engine,
        &tag,
        crate::api::plain_records(["one"]),
        CallOptions::new(),
    )
    .next()
    .expect("a row")
    .expect_err("the overflow");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(
        error.to_string(),
        "the backend reported token counts whose total is too large"
    );
    assert_eq!(listener.count(), 2);
}
