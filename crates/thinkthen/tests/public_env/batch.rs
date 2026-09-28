//! Focused public boundary cases retained in the parent test target.

use super::*;

#[test]
fn a_malformed_variable_is_usage_and_an_unreadable_configuration_is_local() {
    let said = in_child("refused", &[("THINKTHEN_BASE_URL", "ftp://127.0.0.1/v1")]);
    assert!(
        said.starts_with("Usage: THINKTHEN_BASE_URL: a base address"),
        "{said}"
    );
    let config = folder("unreadable");
    fs::create_dir_all(config.join("thinkthen/config.json")).unwrap();
    let said = in_child("refused", &[("XDG_CONFIG_HOME", config.to_str().unwrap())]);
    assert_eq!(said, "Local: the configuration file could not be read");
    for value in ["0", "-1", "1.5", "abc"] {
        let said = in_child("refused", &[("THINKTHEN_MAX_REQUEST_BYTES", value)]);
        assert_eq!(
            said,
            "Usage: THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1"
        );
    }
    assert!(matches!(
        Engine::builder().max_request_bytes(0),
        Err(Error::Usage(_))
    ));
}

#[test]
fn the_batch_environment_is_selected_at_build_and_an_explicit_setter_outranks_it() {
    let listener = listener();
    let address = ("THINKTHEN_BASE_URL", listener.base());
    let key = ("THINKTHEN_API_KEY", "sk-batch-env-fixture");
    let valid = in_child("batch-env", &[address, key, ("THINKTHEN_BATCH", "1")]);
    assert_eq!(valid, "rows 2");
    assert_eq!(listener.count(), 2, "environment batch one sends twice");

    let override_result = in_child(
        "batch-override",
        &[address, key, ("THINKTHEN_BATCH", "invalid")],
    );
    assert_eq!(
        override_result,
        "Usage: THINKTHEN_BATCH takes max or a whole number of at least 1\nrows 1"
    );
    assert_eq!(listener.count(), 3, "only the explicit override sends");

    let conflict = in_child(
        "batch-conflict",
        &[address, key, ("THINKTHEN_BATCH", "max")],
    );
    assert_eq!(conflict, "rows 2");
    assert_eq!(
        listener.count(),
        5,
        "explicit batch one outranks environment max"
    );
}

#[test]
fn with_home_unset_only_the_default_cache_fails_and_at_build() {
    let said = in_child("no-home", &[]);
    let refused =
        "Usage: no default cache folder is available; set THINKTHEN_CACHE or use no_cache";
    assert_eq!(said, ["ok", refused, refused].join("\n"));
}

#[test]
fn no_key_reaches_a_debug_line_or_a_later_error() {
    let refusing = Listener::answering(|_| Canned::status(401, "{}")).expect("a listener");
    let said = in_child(
        "secrecy",
        &[
            ("THINKTHEN_BASE_URL", refusing.base()),
            ("THINKTHEN_API_KEY", "sk-sentinel-from-variable"),
            (ARGUMENT, "sk-sentinel-from-setter"),
        ],
    );
    assert_eq!(said.lines().count(), 9, "{said}");
    assert!(!said.contains("sk-sentinel"), "{said}");
    assert_eq!(refusing.count(), 1, "the error came from a real send");
}

#[test]
fn seeding_and_building_send_nothing_and_seeding_creates_no_file() {
    let served = listener();
    let cache = folder("untouched");
    fs::create_dir_all(&cache).unwrap();
    let said = in_child(
        "effects",
        &[
            ("THINKTHEN_BASE_URL", served.base()),
            ("THINKTHEN_API_KEY", "sk-fake-loopback"),
            ("THINKTHEN_CACHE", cache.to_str().unwrap()),
            ("XDG_CACHE_HOME", cache.to_str().unwrap()),
            (ARGUMENT, cache.to_str().unwrap()),
        ],
    );
    assert_eq!(said, "seeded 0 built 0");
    assert_eq!(served.count(), 0);
}
