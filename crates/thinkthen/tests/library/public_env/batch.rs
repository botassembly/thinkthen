//! Focused public boundary cases retained in the parent test target.

use super::*;

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn a_malformed_variable_is_usage_and_an_unreadable_configuration_is_local() {
    let said = in_child("refused", &[("THINKTHEN_BASE_URL", "ftp://127.0.0.1/v1")]);
    assert!(
        said.starts_with("Usage: THINKTHEN_BASE_URL: a base address"),
        "{said}"
    );
    let root = folder("unreadable");
    fs::create_dir_all(
        crate::child::Folder::Config
            .under(&root)
            .join("config.json"),
    )
    .unwrap();
    let moved = crate::child::Folder::Config.variable(&root);
    let said = in_child("refused", &[(moved.0, moved.1.as_str())]);
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

#[test]
fn the_rate_variable_paces_an_engine_built_from_the_environment() {
    let starts = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let seen = std::sync::Arc::clone(&starts);
    let listener = Listener::answering(move |_| {
        seen.lock().unwrap().push(std::time::Instant::now());
        Canned::ok(ANSWERED)
    })
    .expect("a loopback listener");
    let address = ("THINKTHEN_BASE_URL", listener.base());
    let key = ("THINKTHEN_API_KEY", "sk-paced-env-fixture");
    let said = in_child(
        "paced",
        &[address, key, ("THINKTHEN_REQUESTS_PER_MINUTE", "600")],
    );
    assert!(
        said.lines()
            .all(|line| line.starts_with("sent 1 cached false")),
        "{said}"
    );
    let starts = starts.lock().unwrap();
    assert_eq!(starts.len(), 4);
    // One start each 100 ms. One interval covers a late first delivery.
    let span = *starts.iter().max().unwrap() - *starts.iter().min().unwrap();
    assert!(span >= std::time::Duration::from_millis(200), "{span:?}");
    let said = in_child("refused", &[("THINKTHEN_REQUESTS_PER_MINUTE", "60001")]);
    assert_eq!(
        said,
        "Usage: THINKTHEN_REQUESTS_PER_MINUTE takes a whole number from 1 to 60000"
    );
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn a_rate_in_the_configuration_file_paces_the_backend_it_names() {
    let starts = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let seen = std::sync::Arc::clone(&starts);
    let listener = Listener::answering(move |_| {
        seen.lock().unwrap().push(std::time::Instant::now());
        Canned::ok(ANSWERED)
    })
    .expect("a loopback listener");
    let root = folder("paced-config");
    let moved = crate::child::Folder::configure(
        &root,
        &format!(
            r#"{{"schema":"thinkthen.config/1","backend":"local","backends":{{"local":{{"url":"{}","key_env":"LOCAL_KEY","model":"local-1","requests_per_minute":600}}}}}}"#,
            listener.base()
        ),
    )
    .unwrap();
    let said = in_child(
        "paced",
        &[
            (moved.0, moved.1.as_str()),
            ("LOCAL_KEY", "sk-paced-config-fixture"),
        ],
    );
    assert!(
        said.lines()
            .all(|line| line.starts_with("sent 1 cached false")),
        "{said}"
    );
    let starts = starts.lock().unwrap();
    assert_eq!(starts.len(), 4);
    // One start each 100 ms from the file alone. One interval covers a late first delivery.
    let span = *starts.iter().max().unwrap() - *starts.iter().min().unwrap();
    assert!(span >= std::time::Duration::from_millis(200), "{span:?}");
}

#[test]
fn the_estimated_input_variable_caps_an_engine_built_from_the_environment() {
    let listener = listener();
    let address = ("THINKTHEN_BASE_URL", listener.base());
    let key = ("THINKTHEN_API_KEY", "sk-capped-env-fixture");
    let capped = ("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL", "10");
    let said = in_child("paced", &[address, key, capped]);
    let refused = "Usage: max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request";
    assert_eq!(said, [refused; 4].join("\n"));
    assert_eq!(listener.count(), 0);
    // The explicit setter outranks the variable.
    let said = in_child("uncapped", &[address, key, capped]);
    assert!(said.starts_with("sent 1 cached false"), "{said}");
    assert_eq!(listener.count(), 1);
    let said = in_child(
        "refused",
        &[("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL", "-1")],
    );
    assert_eq!(
        said,
        "Usage: THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL takes a whole number of 0 or more"
    );
}
