//! Native borrowed diagnostics keep independent domains and stable roots.
use super::{compile, crate_dir, run, successful};
use conformance_backend::Backend;
#[test]
fn native_error_views_keep_thread_slots_and_stable_borrowed_roots() {
    let backend = Backend::start().expect("loopback");
    let output = run(
        &compile(&crate_dir().join("tests/c/error_views.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
    );
    successful(&output);
    assert_eq!(backend.count(), 0, "diagnostic operations send nothing");
}
