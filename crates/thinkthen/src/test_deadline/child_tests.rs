use super::child::command;

#[test]
fn a_child_sees_only_path_and_the_names_it_keeps() {
    assert!(std::env::var_os("CARGO_PKG_NAME").is_some());
    let probe = r#"test -z "${CARGO_PKG_NAME+x}" && test -n "$CARGO_MANIFEST_DIR""#;
    let status = command("sh", &["CARGO_MANIFEST_DIR"])
        .args(["-c", probe])
        .status();
    assert!(status.is_ok_and(|status| status.success()));
}

#[test]
#[should_panic(
    expected = "a test child may not keep THINKTHEN_BASE_URL from the parent: set a THINKTHEN_ value or a fake key explicitly"
)]
fn a_thinkthen_name_is_never_kept() {
    let _ = command("sh", &["THINKTHEN_BASE_URL"]);
}
