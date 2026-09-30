//! A storage failure in the platform default cache names that cache (ticket 0138).

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;

use crate::harness::{Canned, Listener, spawn};

const DEFAULT: &str = "thinkthen: the default cache folder could not be read or written; check its permissions and free space, use --no-cache, or set THINKTHEN_CACHE to another folder\n";
const NAMED: &str = "thinkthen: the recording folder could not be read or written; check its permissions and free space\n";

#[test]
fn a_default_cache_that_fails_names_the_default_cache() {
    // A private default cache whose store is no database fails every run.
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("default-cache-storage");
    let _absent = fs::remove_dir_all(&root);
    let named = root.join("thinkthen");
    fs::create_dir_all(&named).expect("cache folder");
    fs::set_permissions(&named, fs::Permissions::from_mode(0o700)).expect("private folder");
    fs::write(
        named.join("thinkthen.sqlite"),
        b"not a database, padded well past the length of one sqlite header",
    )
    .expect("store");
    let root_text = root.to_str().expect("a UTF-8 path");
    let named_text = named.to_str().expect("a UTF-8 path");
    let listener =
        Listener::answering(|_| Canned::status(500, "unused")).expect("a loopback listener");
    let base = listener.base().to_owned();
    let cases = [
        (vec![], ("XDG_CACHE_HOME", root_text), DEFAULT),
        (vec!["--jsonl"], ("XDG_CACHE_HOME", root_text), DEFAULT),
        (vec![], ("THINKTHEN_CACHE", named_text), NAMED),
    ];
    for (options, folder, expected) in cases {
        let mut arguments = vec!["decide", "asks for a refund", "--url", &base];
        arguments.extend(&options);
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value"), folder],
            b"{\"a\":1}\n{\"a\":2}\n",
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(5), "{options:?} {folder:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            expected,
            "{options:?} {folder:?}"
        );
    }
    assert_eq!(listener.connections(), 0);
}
