//! Public counted constructors and immutable image ownership under the existing sanitizer.
use super::{compile, crate_dir, run_with, text};
use conformance_backend::Backend;
#[test]
fn immutable_image_bytes_dimensions_and_filename_outlive_the_engine() {
    let backend = Backend::start().expect("loopback");
    let image = crate_dir().join("../../specification/fixtures/images/red.png");
    let jpeg = crate_dir().join("../../specification/fixtures/images/red.jpg");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/carriers.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[("TYPED_IMAGE", &image), ("TYPED_JPEG", &jpeg)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0);
}
#[test]
fn all_ten_question_constructors_and_invalid_counted_descriptors_send_nothing() {
    let backend = Backend::start().expect("loopback");
    let path = crate_dir().join("../../specification/fixtures/files/questions.json");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/carrier_inputs.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[("TYPED_QUESTION", &path)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0);
}
