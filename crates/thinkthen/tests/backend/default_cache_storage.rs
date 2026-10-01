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

/// Release QA ran under a read-only `XDG_CACHE_HOME`: main sent one request,
/// paid for it, then exited 5 and dropped the answer. A writing store now
/// opens before the first send, so each unwritable folder refuses unsent
/// (ticket 0367). A plan still sends nothing and creates nothing.
#[test]
fn an_unwritable_cache_folder_refuses_before_the_first_send() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("unwritable-cache-home");
    if root.exists() {
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("writable again");
        fs::remove_dir_all(&root).expect("old fixture");
    }
    fs::create_dir_all(&root).expect("cache home");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o500)).expect("read-only home");
    let root_text = root.to_str().expect("a UTF-8 path");
    let named = root.join("named");
    let named_text = named.to_str().expect("a UTF-8 path");
    let listener =
        Listener::answering(|_| Canned::status(500, "unused")).expect("a loopback listener");
    let base = listener.base().to_owned();
    let cases = [
        (vec![], ("XDG_CACHE_HOME", root_text), DEFAULT),
        (vec!["--jsonl"], ("XDG_CACHE_HOME", root_text), DEFAULT),
        (vec![], ("THINKTHEN_CACHE", named_text), NAMED),
        (
            vec!["--record", named_text],
            ("XDG_CACHE_HOME", root_text),
            NAMED,
        ),
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
        assert_eq!(output.stdout, b"", "{options:?} {folder:?}");
    }
    let planned = spawn(
        &["decide", "asks for a refund", "--url", &base, "--plan"],
        &[("XDG_CACHE_HOME", root_text)],
        b"evidence",
    )
    .expect("plan");
    assert_eq!(planned.status.code(), Some(0));
    assert_eq!(listener.connections(), 0);
    assert_eq!(fs::read_dir(&root).expect("home").count(), 0);
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("writable again");
}

/// The usage check runs before the cache check, so a malformed usage month
/// names the usage file over an unwritable cache, and a refused run makes no
/// cache folder in a writable home (ticket 0367).
#[test]
fn an_unreadable_usage_month_outranks_the_cache_and_creates_no_cache() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("usage-before-cache");
    if root.exists() {
        fs::set_permissions(root.join("cache"), fs::Permissions::from_mode(0o700))
            .expect("writable again");
        fs::remove_dir_all(&root).expect("old fixture");
    }
    let (cache, usage) = (root.join("cache"), root.join("state/thinkthen"));
    fs::create_dir_all(&cache).expect("cache home");
    fs::create_dir_all(&usage).expect("usage folder");
    fs::set_permissions(&usage, fs::Permissions::from_mode(0o700)).expect("private folder");
    let month = usage.join("2026-08.json");
    fs::write(&month, b"garbage\n").expect("malformed month");
    fs::set_permissions(&month, fs::Permissions::from_mode(0o600)).expect("private month");
    let listener =
        Listener::answering(|_| Canned::status(500, "unused")).expect("a loopback listener");
    let base = listener.base().to_owned();
    let state = root.join("state");
    for mode in [0o700, 0o500] {
        fs::set_permissions(&cache, fs::Permissions::from_mode(mode)).expect("cache mode");
        let output = spawn(
            &["decide", "asks for a refund", "--url", &base],
            &[
                ("THINKTHEN_API_KEY", "sk-test-value"),
                ("XDG_CACHE_HOME", cache.to_str().expect("a UTF-8 path")),
                ("XDG_STATE_HOME", state.to_str().expect("a UTF-8 path")),
            ],
            b"evidence",
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(5), "{mode:o}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: cannot read the usage totals: 2026-08.json has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again.\n",
            "{mode:o}"
        );
        assert_eq!(
            fs::read_dir(&cache).expect("cache home").count(),
            0,
            "{mode:o}"
        );
    }
    assert_eq!(listener.connections(), 0);
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o700)).expect("writable again");
}
