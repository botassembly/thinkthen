#![cfg(test)]
//! Record-store admission, storage and correction preserve their public codes.

use thinkthen::Answer;
use thinkthen_proxy::{ActorKind, Attribution, Evidence, Service};

#[test]
fn store_boundaries_keep_their_own_codes() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/boundaries.json"
    ))
    .unwrap();
    let cases = &cases["store"];
    let directory = tempfile::tempdir().unwrap();
    assert_eq!(
        Service::open(directory.path()).unwrap_err().code(),
        cases["storage"]["code"]
    );
    let mut service = Service::open(directory.path().join("records.sqlite")).unwrap();
    assert_eq!(
        service
            .admit_inline(cases["question"]["request"].as_str().unwrap())
            .unwrap_err()
            .code(),
        cases["question"]["code"]
    );
    let question = service
        .admit_inline(r#"{"function":"decide","question":{"type":"inline","value":"Ready?"}}"#)
        .unwrap();
    let evidence = service
        .admit_evidence(&Evidence::Text("alpha".to_owned()))
        .unwrap();
    let author = || Attribution {
        actor: "curator".to_owned(),
        kind: ActorKind::Person,
        reason: "Review evidence".to_owned(),
    };
    service
        .set_gold(
            &question.question,
            &evidence,
            None,
            &Answer::Bool(false),
            author(),
        )
        .unwrap();
    let error = service
        .set_gold(
            &question.question,
            &evidence,
            None,
            &Answer::Bool(true),
            author(),
        )
        .unwrap_err();
    assert_eq!(error.code(), cases["conflict"]["code"]);
}
