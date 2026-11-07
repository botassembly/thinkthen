#![cfg(test)]
//! Record-store admission, storage and correction preserve their public codes.

use serde_json::json;
use thinkthen_proxy::{ActorKind, Attribution, Evidence, Service};

#[test]
fn store_boundaries_keep_their_own_codes() {
    let directory = tempfile::tempdir().unwrap();
    assert_eq!(
        Service::open(directory.path()).unwrap_err().code(),
        "proxy.storage"
    );
    let mut service = Service::open(directory.path().join("records.sqlite")).unwrap();
    assert_eq!(
        service.admit_inline("{").unwrap_err().code(),
        "proxy.question"
    );
    let question = service
        .admit_inline(r#"{"question":{"function":"decide","question":"Ready?"}}"#)
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
        .set_gold(&question.question, &evidence, None, &json!(false), author())
        .unwrap();
    let error = service
        .set_gold(&question.question, &evidence, None, &json!(true), author())
        .unwrap_err();
    assert_eq!(error.code(), "proxy.conflict");
}
