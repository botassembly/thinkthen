//! Captured named authority is an additional predicate on the one opened descriptor.
use super::{CheckedReadError, read_checked};
#[test]
fn a_replaced_named_root_cannot_authorize_outside_content_even_for_a_privileged_reader() {
    let held = super::tests::scratch("named-root");
    let base = held.join("base");
    let outside = held.join("outside");
    std::fs::write(base.join("question.json"), "original").expect("original writes");
    std::fs::write(outside.join("question.json"), "private outside content")
        .expect("outside writes");
    let captured = base.canonicalize().expect("root resolves");
    assert_eq!(
        read_checked(&base.join("question.json"), 1024, None, Some(&captured)).as_deref(),
        Ok("original")
    );
    std::fs::rename(&base, held.join("old-base")).expect("root moves");
    std::os::unix::fs::symlink(&outside, &base).expect("replacement links");
    assert_eq!(
        read_checked(&base.join("question.json"), 1024, None, Some(&captured)),
        Err(CheckedReadError::Refused)
    );
    // Privileged explicit reads retain their existing rules; only named authority adds this predicate.
    assert_eq!(
        read_checked(&base.join("question.json"), 1024, None, None).as_deref(),
        Ok("private outside content")
    );
    let _ = std::fs::remove_dir_all(held);
}
