//! The parser fixtures (conformance/fixtures/parser-invalid).
//!
//! These tests pin the contract parser's behavior on the fixtures the
//! production parser must refuse. Since 2026-09-22 the contract accepts
//! exactly what the command line's own parser accepts: `from_json` hands
//! the text to `thinkthen_core::QuestionSet::parse` first, so each refusal
//! below is the core's own words and each acceptance is the core's own
//! verdict. `on-collision.json` is accepted and grouped, which is the
//! core's rule; the fixture's README records the measurement.

use std::fs;

use thinkthen_contract::{ErrorKind, QuestionSet};

fn fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../conformance/fixtures/parser-invalid")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// A refusal is the usage kind and names the fault.
fn refused(name: &str) -> thinkthen_contract::Error {
    match QuestionSet::from_json(&fixture(name)) {
        Ok(_) => panic!("{name} must be refused"),
        Err(error) => {
            assert_eq!(error.kind, ErrorKind::Usage, "{name}: the kind");
            assert!(!error.message.is_empty(), "{name}: the refusal names the fault");
            error
        }
    }
}

#[test]
fn a_version_the_grammar_does_not_know_is_refused() {
    let error = refused("bad-version.json");
    assert!(error.message.contains("version"), "{error}");
}

#[test]
fn a_set_with_no_version_is_refused() {
    let error = refused("missing-version.json");
    assert!(error.message.contains("version"), "{error}");
}

#[test]
fn a_name_twice_is_refused() {
    let error = refused("duplicate-names.json");
    assert!(error.message.contains("spam"), "{error}");
}

#[test]
fn an_unknown_top_key_is_refused() {
    let error = refused("unknown-top-key.json");
    assert!(error.message.contains("notes"), "{error}");
}

#[test]
fn an_empty_set_is_refused() {
    let error = refused("empty-set.json");
    assert!(error.message.contains("questions"), "{error}");
}

#[test]
fn an_empty_name_is_refused() {
    let error = refused("empty-name.json");
    assert!(error.message.contains("lowercase"), "{error}");
}

#[test]
fn file_order_is_kept() {
    let set = QuestionSet::from_json(&fixture("file-order.json")).expect("parses");
    assert_eq!(set.names(), ["zeta", "alpha"], "file order, not name order");
}

/// Two members sharing one `on` pointer are the core's grouping form, not a
/// refusal; the CLI accepts them and so does the contract.
#[test]
fn a_shared_on_pointer_is_accepted() {
    let set = QuestionSet::from_json(&fixture("on-collision.json")).expect("parses");
    assert_eq!(set.names(), ["a", "b"]);
}
