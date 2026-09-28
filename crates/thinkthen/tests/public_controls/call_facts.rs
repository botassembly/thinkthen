//! Focused public boundary cases retained in the parent test target.

use super::*;

#[test]
fn counters_and_cache_answers_match_the_real_attempts() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let folder = std::env::temp_dir().join(format!("thinkthen-counters-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let engine = Engine::builder()
        .base_url(listener.base())
        .and_then(|b| b.api_key("sk-public-controls"))
        .and_then(|b| b.cache_at(&folder))
        .and_then(thinkthen::EngineBuilder::build)
        .expect("engine");
    let asked = question();
    let sent = engine.decide(&asked, "Refund me.").expect("live answer");
    assert_eq!(*sent.value(), Answer::Yes);
    assert_eq!(
        (sent.facts().records(), sent.facts().requests_sent()),
        (1, 1)
    );
    assert_eq!(
        (sent.facts().input_tokens(), sent.facts().output_tokens()),
        (Some(3), Some(1))
    );
    let cached = engine.decide(&asked, "Refund me.").expect("cached answer");
    assert_eq!(*cached.value(), Answer::Yes);
    assert_eq!(
        (
            cached.facts().records(),
            cached.facts().requests_sent(),
            cached.facts().cache_answers()
        ),
        (1, 0, 1)
    );
    assert_eq!(cached.facts().input_tokens(), None);
    let token = CancelToken::new();
    token.cancel();
    let options = CallOptions::new().cancel(&token);
    assert!(engine.decide_with(&asked, "Another.", options).is_err());
    let usage = engine.usage();
    assert_eq!(listener.count(), 1);
    assert_eq!((usage.requests_sent(), usage.cache_answers()), (1, 1));
    let _gone = std::fs::remove_dir_all(&folder);
}

/// Ticket 0132: a reply over 1 MiB plus 8 bytes per request byte is refused by name, once.
#[test]
fn a_reply_over_its_limit_names_it() {
    let _serial = serial();
    let listener = Listener::answering(|body| {
        let size = 1_048_576 + 8 * body.len() + 1;
        let (head, tail) = DECIDED.split_at(DECIDED.len() - 1);
        Canned::ok(&format!("{head}{}{tail}", " ".repeat(size - DECIDED.len())))
    })
    .expect("listener");
    let result = engine(listener.base()).decide(&question(), "Refund me.");
    let limit = 1_048_576 + 8 * listener.requests()[0].body.len();
    assert_eq!(kind(&result), Some(ErrorKind::Backend));
    assert_eq!(result.as_ref().err().map(Error::retryable), Some(false));
    assert_eq!(
        message(result),
        format!(
            "the backend's reply passed this request's limit of {limit} bytes, so the answer was not kept; the request was not sent again"
        )
    );
    assert_eq!(listener.count(), 1);
}
