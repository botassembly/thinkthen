//! The public rank's whole-input refusal and its question's own model.

use super::*;

#[test]
fn a_rank_refuses_a_blank_record_before_any_send() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let asked = Question::rank("Which asks for a refund?").expect("rank");
    let error = engine
        .rank(&asked, ["one", "two", "  ", "four"])
        .expect_err("a blank record");
    assert_eq!(error.to_string(), "evidence is text, not white space");
    assert_eq!(listener.count(), 0);
}

#[test]
fn a_rank_question_sends_its_own_model() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let asked = Question::rank("Which asks for a refund?")
        .and_then(|asked| asked.with_model("judge-b"))
        .expect("rank with a model");
    let ranked = engine.rank(&asked, ["one"]).expect("ranked");
    assert_eq!(ranked.value().len(), 1);
    let bodies = listener.requests();
    let body: serde_json::Value =
        serde_json::from_slice(&bodies.first().expect("one request").body).expect("JSON body");
    assert_eq!(body["model"], "judge-b");
    let again = Question::rank("Which?")
        .and_then(|asked| asked.with_model("judge-b"))
        .and_then(|asked| asked.with_model("judge-c"))
        .expect_err("a second model");
    assert_eq!(again.to_string(), "the model is already set");
}

#[test]
fn fallible_complete_sets_stop_at_admission_failure_before_any_send() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let limited = Engine::builder()
        .base_url(listener.base())
        .expect("url")
        .api_key("sk-public-batches")
        .expect("key")
        .max_requests(Some(2))
        .expect("limit")
        .no_cache()
        .build()
        .expect("engine");
    let ranked = Question::rank("Q?").expect("rank");
    let found = Question::find("Q?").expect("find");
    for rank in [true, false] {
        let pulls = AtomicUsize::new(0);
        let input = std::iter::from_fn(|| {
            let n = pulls.fetch_add(1, Ordering::SeqCst);
            assert!(n < 3, "tail stays unread");
            Some(Ok("alpha"))
        });
        let error = if rank {
            limited
                .try_rank_with(&ranked, input, CallOptions::new())
                .expect_err("rank cap")
        } else {
            limited
                .try_find_with(&found, input, CallOptions::new())
                .expect_err("find cap")
        };
        assert_eq!(
            error.to_string(),
            "this engine answers at most 2 records in one call"
        );
        assert_eq!(pulls.load(Ordering::SeqCst), 3);
    }
    let engine = engine(listener.base());
    for none in [false, true] {
        let asked = if none {
            found.clone().offering_none().expect("none")
        } else {
            found.clone()
        };
        let maximum = if none { 254 } else { 255 };
        let pulls = AtomicUsize::new(0);
        let input = std::iter::from_fn(|| {
            assert!(
                pulls.fetch_add(1, Ordering::SeqCst) <= maximum,
                "tail stays unread"
            );
            Some(Ok("alpha"))
        });
        assert!(
            engine
                .try_find_with(&asked, input, CallOptions::new())
                .is_err()
        );
        assert_eq!(pulls.load(Ordering::SeqCst), maximum + 1);
    }
    let large = "x".repeat(8 * 1024 * 1024 + 1);
    let pulls = AtomicUsize::new(0);
    let input = std::iter::from_fn(|| {
        assert!(
            pulls.fetch_add(1, Ordering::SeqCst) < 2,
            "byte tail stays unread"
        );
        Some(Ok(large.as_str()))
    });
    let error = engine
        .try_find_with(&found, input, CallOptions::new())
        .expect_err("bytes");
    assert_eq!(error.to_string(), "find input exceeds 16 MiB");
    assert_eq!(pulls.load(Ordering::SeqCst), 2);
    let error = engine
        .try_rank_with(
            &ranked,
            [
                Ok("alpha"),
                Err(Error::new(ErrorKind::Local, "reader failed")),
            ],
            CallOptions::new(),
        )
        .expect_err("reader");
    assert_eq!(error.to_string(), "reader failed");
    assert_eq!(listener.count(), 0);
}
