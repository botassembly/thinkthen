//! Parsed questions retain metadata independently of caller buffers and engines.
use super::{compile, crate_dir, run, successful};
use conformance_backend::Backend;
#[test]
fn parsed_question_authorship_outlives_inputs_and_refusals_preserve_outputs() {
    let backend = Backend::start().expect("loopback");
    let output = run(
        &compile(&crate_dir().join("tests/c/authored.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
    );
    successful(&output);
    assert_eq!(backend.count(), 0);
}
