//! The synthetic parser fixtures (conformance/fixtures/parser-invalid).
//!
//! These tests lock the CURRENT contract parser's behavior on the invalid
//! cases the production parser must refuse, so the engine swap changes the
//! answers deliberately and never by accident. Each test names the
//! production expectation; the ones marked `#[ignore]` fail today on
//! purpose and run when the production parser lands.

use std::fs;

use thinkthen_contract::{ErrorKind, QuestionSet};

fn fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../conformance/fixtures/parser-invalid")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// A refusal is the usage kind and names the fault.
fn refused(name: &str) -> Option<thinkthen_contract::Error> {
    match QuestionSet::from_json(&fixture(name)) {
        Ok(_) => None,
        Err(error) => {
            assert_eq!(error.kind, ErrorKind::Usage, "{name}: the kind");
            Some(error)
        }
    }
}

#[test]
fn bad_version_is_accepted_today() {
    // Production: refuse a version the grammar does not know.
    assert!(QuestionSet::from_json(&fixture("bad-version.json")).is_ok());
}

#[test]
fn missing_version_is_accepted_today() {
    // Production: refuse a set with no version.
    assert!(QuestionSet::from_json(&fixture("missing-version.json")).is_ok());
}

#[test]
fn duplicate_names_collapse_today() {
    // Production: refuse the same name twice.
    let set = QuestionSet::from_json(&fixture("duplicate-names.json")).expect("parses");
    assert_eq!(set.names().len(), 1, "the JSON duplicate collapses to the last");
}

#[test]
fn unknown_top_key_is_ignored_today() {
    // Production: refuse a top-level key no grammar names.
    assert!(QuestionSet::from_json(&fixture("unknown-top-key.json")).is_ok());
}

#[test]
fn empty_set_is_accepted_today() {
    // Production: refuse a set with no questions.
    let set = QuestionSet::from_json(&fixture("empty-set.json")).expect("parses");
    assert!(set.names().is_empty());
}

#[test]
fn empty_name_is_accepted_today() {
    // Production: refuse a question named with the empty string.
    let set = QuestionSet::from_json(&fixture("empty-name.json")).expect("parses");
    assert_eq!(set.names(), [""]);
}

#[test]
fn on_collision_is_accepted_today() {
    // Production: refuse one `on` group holding the same question twice.
    assert!(QuestionSet::from_json(&fixture("on-collision.json")).is_ok());
}

#[test]
fn file_order_is_sorted_today() {
    // Production: keep file order (zeta, alpha); today the names sort.
    let set = QuestionSet::from_json(&fixture("file-order.json")).expect("parses");
    assert_eq!(set.names(), ["alpha", "zeta"], "today sorts");
}

#[test]
#[ignore = "runs when the production parser lands: each invalid fixture must refuse"]
fn production_refuses_every_invalid_fixture() {
    for name in [
        "bad-version.json",
        "missing-version.json",
        "duplicate-names.json",
        "unknown-top-key.json",
        "empty-set.json",
        "empty-name.json",
        "on-collision.json",
    ] {
        let error = refused(name).unwrap_or_else(|| panic!("{name} must be refused"));
        assert!(
            !error.message.is_empty(),
            "{name}: the refusal names the fault"
        );
    }
}

#[test]
#[ignore = "runs when the production parser lands: names stay in file order"]
fn production_keeps_file_order() {
    let set = QuestionSet::from_json(&fixture("file-order.json")).expect("parses");
    assert_eq!(set.names(), ["zeta", "alpha"]);
}
