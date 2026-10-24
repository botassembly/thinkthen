#![cfg(feature = "command")]
//! The command reports its version.
#![cfg(test)]

mod support;

#[test]
fn version_prints_name_and_version() {
    let output = support::command().arg("--version").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("thinkthen {}\n", env!("CARGO_PKG_VERSION"))
    );
}
