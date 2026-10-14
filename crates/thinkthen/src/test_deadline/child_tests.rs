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

#[test]
#[should_panic(
    expected = "a test child may not keep db_password from the parent: set a THINKTHEN_ value or a fake key explicitly"
)]
fn a_secret_shaped_name_is_never_kept_in_any_case() {
    let _ = command("sh", &["db_password"]);
}

#[test]
fn cargo_children_preserve_compiler_flags_without_other_parent_names() {
    let child = command(env!("CARGO"), super::child::CARGO);
    let environment: std::collections::BTreeMap<_, _> = child
        .get_envs()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
        .collect();
    for name in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CFLAGS", "CXXFLAGS"] {
        assert_eq!(
            environment.get(std::ffi::OsStr::new(name)).copied(),
            std::env::var_os(name).as_deref(),
            "Cargo must retain the caller's compiler flags and encoded precedence"
        );
    }
    assert!(!environment.contains_key(std::ffi::OsStr::new("CARGO_PKG_NAME")));
}
