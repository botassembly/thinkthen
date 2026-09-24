use crate::core::{RecognizeSpec, RecognizedName};
use crate::engine::Cancel;
use crate::engine::facade_tests::engine;
use crate::engine::prepared_request::PREPARATIONS;

use super::Aggregate;

fn entity(name: &str, kind: &str, start: usize) -> RecognizedName {
    RecognizedName {
        name: name.to_owned(),
        kind: kind.to_owned(),
        start,
        end: start + name.len(),
        strength: 0.9,
    }
}

#[test]
fn recognition_sends_the_chunks_the_settled_relation_prepared() {
    let spec = RecognizeSpec::parse(
        r#"{"version":1,"recognize":{"kinds":{"person":"A person.","organization":"An organization."},"relations":[{"name":"works_for","source":"person","target":"organization"}]}}"#,
    )
    .expect("spec");
    let engine = engine("http://127.0.0.1:9");
    let entities = [
        entity("Ada", "person", 0),
        entity("Acme", "organization", 8),
    ];
    PREPARATIONS.with(|count| count.set(0));
    let sent = engine.relations(
        &spec,
        "Ada met Acme.",
        &entities,
        &mut Aggregate::default(),
        &Cancel::default(),
    );
    assert!(sent.is_err(), "nothing answers the closed local port");
    assert_eq!(PREPARATIONS.with(std::cell::Cell::get), 1);
}
