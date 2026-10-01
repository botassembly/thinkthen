use std::path::PathBuf;

use super::{Config, Platform, resolve_cache, resolve_config, resolve_usage};

#[test]
fn raw_configuration_debug_withholds_the_address() {
    let config = Config::parse(
        br#"{"schema":"thinkthen.config/1","url":"http://localhost/config-marker-0210"}"#,
    )
    .expect("valid configuration");
    let shown = format!("{config:?}");
    assert!(shown.contains("url: Some(\"<withheld>\")"));
    assert!(!shown.contains("config-marker-0210"));
}

#[cfg(unix)]
#[test]
fn another_owner_or_other_write_warns_and_group_write_stays_quiet() {
    use super::metadata_writable_by_another;

    for (mode, owner, effective, warned) in [
        (0o644, 1001, 1000, true),
        (0o600, 1000, 1000, false),
        (0o664, 1000, 1000, false),
        (0o775, 1000, 1000, false),
        (0o707, 1000, 1000, true),
        (0o700, 1001, 1000, true),
    ] {
        assert_eq!(metadata_writable_by_another(mode, owner, effective), warned);
    }
}

#[test]
fn each_refusal_names_its_field_and_never_its_value() {
    assert!(Config::parse(br#"{"schema":"thinkthen.config/1"}"#).is_ok());
    let not_closed = "the configuration file is not valid closed JSON";
    let schema = "configuration field `schema` must be `thinkthen.config/1`";
    let bytes = "configuration field `cache_bytes` must be a whole number greater than zero";
    for (text, sentence) in [
        (r#"{"schema":"thinkthen.config/1""#, not_closed),
        (r#"["schema"]"#, not_closed),
        (
            r#"{"schema":"thinkthen.config/1","cache":true,"cache":false}"#,
            not_closed,
        ),
        (r#"{}"#, schema),
        (r#"{"schema":"wrong"}"#, schema),
        (r#"{"schema":1}"#, schema),
        (
            r#"{"schema":"thinkthen.config/1","extra":true}"#,
            "the configuration file holds a field other than `schema`, `url`, `model`, `cache`, `cache_bytes`, `usd_per_million_input`, `usd_per_million_output`, `backend`, and `backends`",
        ),
        (
            r#"{"schema":"thinkthen.config/1","cache":"/folder"}"#,
            "configuration field `cache` must be true or false",
        ),
        (
            r#"{"schema":"thinkthen.config/1","url":7}"#,
            "configuration field `url` must be a string",
        ),
        (
            r#"{"schema":"thinkthen.config/1","model":["m"]}"#,
            "configuration field `model` must be a string",
        ),
        (r#"{"schema":"thinkthen.config/1","cache_bytes":0}"#, bytes),
        (r#"{"schema":"thinkthen.config/1","cache_bytes":-1}"#, bytes),
        (
            r#"{"schema":"thinkthen.config/1","model":"  "}"#,
            "configuration field `model` must not be blank",
        ),
        (
            r#"{"schema":"thinkthen.config/1","url":"http://example.com"}"#,
            "configuration field `url` must be a safe backend base",
        ),
    ] {
        let refused = Config::parse(text.as_bytes()).expect_err(text);
        assert_eq!(refused.message, sentence, "{text}");
        assert!(!refused.unreadable, "{text}");
    }
}

// The Linux and macOS rows use Unix absolute paths, which Windows reads as relative.
#[cfg(unix)]
#[test]
fn linux_and_macos_resolve_config_and_cache_independently() {
    let cases = [
        (
            Platform::Linux,
            Some("/config"),
            Some("/cache"),
            None,
            "/config/thinkthen/config.json",
            "/cache/thinkthen",
        ),
        (
            Platform::Linux,
            Some("relative"),
            Some("relative"),
            Some("/home/person"),
            "/home/person/.config/thinkthen/config.json",
            "/home/person/.cache/thinkthen",
        ),
        (
            Platform::Macos,
            Some("/ignored"),
            Some("/ignored"),
            Some("/Users/person"),
            "/Users/person/Library/Application Support/thinkthen/config.json",
            "/Users/person/Library/Caches/thinkthen",
        ),
    ];
    for (platform, config, cache, home, expected_config, expected_cache) in cases {
        assert_eq!(
            resolve_config(platform, config.map(str::to_owned), home.map(str::to_owned)),
            Some(PathBuf::from(expected_config))
        );
        assert_eq!(
            resolve_cache(platform, cache.map(str::to_owned), home.map(str::to_owned)),
            Some(PathBuf::from(expected_cache))
        );
    }
    assert_eq!(
        resolve_config(Platform::Linux, Some("/config".to_owned()), None),
        Some(PathBuf::from("/config/thinkthen/config.json"))
    );
    assert_eq!(resolve_cache(Platform::Linux, None, None), None);
    for platform in [Platform::Linux, Platform::Macos] {
        for unusable in ["", "relative"] {
            assert_eq!(
                resolve_config(
                    platform,
                    Some(unusable.to_owned()),
                    Some(unusable.to_owned())
                ),
                None
            );
            assert_eq!(
                resolve_cache(
                    platform,
                    Some(unusable.to_owned()),
                    Some(unusable.to_owned())
                ),
                None
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn usage_lives_in_the_state_folder_and_never_follows_the_cache() {
    let home = Some("/home/person");
    for (platform, state, home, expected) in [
        (
            Platform::Linux,
            Some("/state"),
            home,
            Some("/state/thinkthen"),
        ),
        (
            Platform::Linux,
            None,
            home,
            Some("/home/person/.local/state/thinkthen"),
        ),
        (
            Platform::Linux,
            Some("relative"),
            home,
            Some("/home/person/.local/state/thinkthen"),
        ),
        (
            Platform::Linux,
            Some("/state"),
            None,
            Some("/state/thinkthen"),
        ),
        (Platform::Linux, None, None, None),
        (Platform::Linux, Some(""), Some("relative"), None),
        (
            Platform::Macos,
            Some("/ignored"),
            Some("/Users/person"),
            Some("/Users/person/Library/Application Support/thinkthen/usage"),
        ),
        (Platform::Macos, None, Some("relative"), None),
    ] {
        assert_eq!(
            resolve_usage(platform, state.map(str::to_owned), home.map(str::to_owned)),
            expected.map(PathBuf::from),
            "{state:?} {home:?}"
        );
    }
}

/// `absolute()` uses the host's rule, so a `C:\` base is absolute only on
/// Windows. Windows reads no `HOME` and no `XDG_` name (ticket 0373).
#[cfg(windows)]
#[test]
fn windows_resolves_config_from_appdata_and_cache_and_usage_from_localappdata() {
    let roaming = Some(r"C:\Users\person\AppData\Roaming".to_owned());
    let local = Some(r"C:\Users\person\AppData\Local".to_owned());
    let home = Some(r"C:\Users\person".to_owned());
    assert_eq!(
        resolve_config(Platform::Windows, roaming, home.clone()),
        Some(PathBuf::from(
            r"C:\Users\person\AppData\Roaming\thinkthen\config.json"
        ))
    );
    assert_eq!(
        resolve_cache(Platform::Windows, local.clone(), home.clone()),
        Some(PathBuf::from(
            r"C:\Users\person\AppData\Local\thinkthen\cache"
        ))
    );
    assert_eq!(
        resolve_usage(Platform::Windows, local, home.clone()),
        Some(PathBuf::from(
            r"C:\Users\person\AppData\Local\thinkthen\usage"
        ))
    );
    for unusable in [None, Some(String::new()), Some("relative".to_owned())] {
        assert_eq!(
            resolve_config(Platform::Windows, unusable.clone(), home.clone()),
            None
        );
        assert_eq!(
            resolve_cache(Platform::Windows, unusable.clone(), home.clone()),
            None
        );
        assert_eq!(
            resolve_usage(Platform::Windows, unusable, home.clone()),
            None
        );
    }
}
