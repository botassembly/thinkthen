use super::*;
use thinkthen::{CancelToken, InputEvidence, QuestionInput, RankSet};
struct Original<'a> {
    token: &'a CancelToken,
    snapshots: &'a AtomicUsize,
}
impl InputEvidence for Original<'_> {
    fn question_input(&self) -> QuestionInput {
        self.snapshots.fetch_add(1, Ordering::Relaxed);
        self.token.cancel();
        QuestionInput::Text("Plain.".to_owned())
    }
}
#[cfg(test)]
fn call(
    function: &str,
    engine: &Engine,
    token: &CancelToken,
    snapshots: &AtomicUsize,
) -> Result<(), thinkthen::Error> {
    let records = || (0..3).map(|_| crate::api::record(Original { token, snapshots }));
    let options = CallOptions::new().cancel(token);
    let question = Question::decide("Ready?").unwrap().cut();
    let rank = Question::rank("Ready?").unwrap();
    let set = r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#;
    match function {
        "atomic" => crate::api::eager_decisions(engine,&question,records(),options).map(|_|()),
        "annotation" => crate::api::whole(engine,thinkthen::RequestFunction::Annotate,thinkthen::QuestionSet::from_json(set).unwrap().into(),records().map(Ok),options).map(|_|()),
        "rank" => crate::api::eager_rank(engine,&rank,records(),options).map(|_|()),
        "rank-set" => crate::api::whole(engine,thinkthen::RequestFunction::Rank,RankSet::from_json(set).unwrap().into(),records().map(Ok),options).map(|_|()),
        "find" => crate::api::find(engine, &Question::find("Which?").unwrap(), records().map(Ok), options).map(|_|()),
        "recognize" => crate::api::recognize_records(engine,&thinkthen::Recognize::from_json(r#"{"version":1,"recognize":{}}"#).unwrap(),records(),options).map(|_|()),
        "relate" => crate::api::whole(engine,thinkthen::RequestFunction::Relate,thinkthen::Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"*","target":"*","reads":"follows"}]}}"#).unwrap().into(),records().map(Ok),options).map(|_|()),
        _=>panic!("function"),
    }
}
#[test]
fn finite_native_admission_stops_at_the_cancelled_input_without_preparing_the_suffix_or_sending() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    for function in [
        "atomic",
        "annotation",
        "rank",
        "rank-set",
        "find",
        "recognize",
        "relate",
    ] {
        let token = CancelToken::new();
        let snapshots = AtomicUsize::new(0);
        let error = call(function, &engine, &token, &snapshots).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Cancelled, "{function}");
        assert_eq!(snapshots.load(Ordering::Relaxed), 1, "{function}");
        assert!(error.facts().is_none());
        assert_eq!(listener.count(), 0);
        let snapshots = AtomicUsize::new(0);
        let error = call(function, &engine, &token, &snapshots).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Cancelled, "{function}");
        assert_eq!(snapshots.load(Ordering::Relaxed), 0, "{function}");
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn native_finite_source_cancel_stops_before_pulling_the_next_record() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let token = CancelToken::new();
    let pulls = AtomicUsize::new(0);
    let records = std::iter::from_fn(|| {
        let at = pulls.fetch_add(1, Ordering::Relaxed);
        assert_eq!(at, 0, "a cancelled native source was pulled again");
        token.cancel();
        Some(Ok(crate::api::record("Plain.")))
    });
    let error = crate::api::whole(
        &engine,
        thinkthen::RequestFunction::Rank,
        Question::rank("Ready?").unwrap().into(),
        records,
        CallOptions::new().cancel(&token),
    )
    .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(pulls.load(Ordering::Relaxed), 1);
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 0);
}
