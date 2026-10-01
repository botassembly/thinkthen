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
