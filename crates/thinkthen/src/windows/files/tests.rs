//! Native identities are checked through independently opened distinct files.
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../../../tests/windows/ffi.rs"]
mod independent;

#[test]
fn identity_check_refuses_a_distinct_named_leaf_and_keeps_both_files() {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "thinkthen-native-identity-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    super::make_private_directory(&root).expect("private disposable root");
    let first = root.join("first.json");
    let second = root.join("second.json");
    let (a, _) = super::usage_write(&first, true, true).expect("first object");
    let (b, _) = super::usage_write(&second, true, true).expect("second object");
    let a_id = independent::identity(&a).expect("independent first full ID");
    let b_id = independent::identity(&b).expect("independent second full ID");
    assert_eq!(a_id.0, b_id.0);
    assert_ne!(a_id.1, b_id.1);
    let failure =
        super::verify_identity(&second, &a, false).expect_err("distinct file must refuse");
    assert_eq!(failure.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(failure.to_string(), "usage identity changed");
    assert!(super::verify_identity(&first, &a, false).is_ok());
    assert!(super::verify_identity(&second, &b, false).is_ok());
    drop((a, b));
    assert_eq!(fs::read(first).expect("retained first bytes"), b"");
    assert_eq!(fs::read(second).expect("retained second bytes"), b"");
    fs::remove_dir_all(root).expect("remove only owned fixture");
}
