#![cfg(feature = "command")]
//! The command reports its version.
#![cfg(test)]

mod support;

#[test]
fn version_prints_name_and_version() {
    let output = support::command().arg("--version").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    let banner = String::from_utf8(output.stdout).unwrap();
    assert!(banner.starts_with(&format!(
        "thinkthen {} (contract {}; backends ",
        thinkthen::VERSION,
        thinkthen::VERSION
    )));
    for backend in thinkthen::BackendName::ALL {
        assert!(banner.contains(backend.as_str()));
        assert!(banner.contains(&backend.default_model()));
    }
}
