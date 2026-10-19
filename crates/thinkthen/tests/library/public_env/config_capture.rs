//! Configuration capture leaves request environment relevance and key capture to the host.
use super::*;
use serde_json::json;

pub(super) fn capture(override_url: &str) -> Vec<String> {
    let builder = EngineBuilder::from_config().expect("configuration capture");
    let builder = if override_url.is_empty() {
        builder
    } else {
        builder.base_url(override_url).expect("selected override")
    };
    let settings = builder.resolved_settings().expect("selected route");
    let facts = json!({
        "present": settings.configuration_present,
        "shared": settings.configuration_shared,
        "cache": settings.cache_enabled,
        "cache_source": settings.cache_enabled_source,
        "target": settings.cache_target_bytes,
        "url_source": settings.url_source,
        "model_source": settings.model_source,
        "keys": settings.key_variables,
        "api_key_set": settings.api_key_set,
    });
    let mut lines = vec![shown(EngineBuilder::from_env()), facts.to_string()];
    if !override_url.is_empty() {
        let engine = builder.api_key("config-capture-fixture-key").unwrap()
            .no_cache().build().unwrap();
        lines.push(ask(&engine));
    } else {
        let engine = builder.no_cache().build().unwrap();
        lines.push(format!("profile {:?}", engine.profile_name()));
    }
    lines
}

#[test]
fn configuration_capture_ignores_irrelevant_environment_and_allows_an_effective_url_override() {
    let listener = listener();
    let written = in_child("config-only", &[
        ("THINKTHEN_BASE_URL", "bad-relative-address"),
        ("THINKTHEN_MAX_REQUEST_BYTES", "not-a-number"),
        ("THINKTHEN_BATCH", "not-a-batch"),
        ("THINKTHEN_API_KEY", "unselected-private-key-marker"),
        (ARGUMENT, listener.base()),
    ]);
    assert!(written.starts_with("Usage: THINKTHEN_BASE_URL:"));
    assert!(written.contains("\"api_key_set\":false"));
    assert!(written.contains("\"keys\":[\"THINKTHEN_API_KEY\"]"));
    assert!(written.contains("sent 1 cached false"));
    assert!(!written.contains("unselected-private-key-marker"));
    assert_eq!(listener.count(), 1);
}

#[test]
fn configuration_capture_retains_configured_nonsecret_metadata() {
    let home = folder("config-capture");
    let config = if cfg!(target_os = "macos") {
        home.join("Library/Application Support/thinkthen/config.json")
    } else {
        home.join("thinkthen/config.json")
    };
    fs::create_dir_all(config.parent().unwrap()).unwrap();
    fs::write(&config, r#"{"schema":"thinkthen.config/1","cache":false,"cache_bytes":17,"model":"configured-model"}"#).unwrap();
    let written = in_child("config-only", &[("XDG_CONFIG_HOME", home.to_str().unwrap()), ("HOME", home.to_str().unwrap()), ("APPDATA", home.to_str().unwrap())]);
    for field in ["\"present\":true", "\"cache\":false", "\"cache_source\":\"configuration\"", "\"target\":17", "\"model_source\":\"configuration\""] {
        assert!(written.contains(field), "{written}");
    }
    fs::remove_dir_all(home).unwrap();
}

#[cfg(unix)]
#[test]
fn configuration_capture_does_not_read_a_non_utf8_credential() {
    use std::os::unix::ffi::OsStringExt as _;
    let output = run::output(
        Command::new(std::env::current_exe().unwrap())
            .args(["public_env::child_case", "--exact", "--ignored", "--test-threads=1"])
            .clear_environment()
            .env(CASE, "config-only")
            .env("THINKTHEN_API_KEY", std::ffi::OsString::from_vec(vec![0xff])),
    ).unwrap();
    assert!(output.status.success());
    let written = String::from_utf8(output.stdout).unwrap();
    assert!(written.contains("THINKTHEN_API_KEY is not valid UTF-8"));
    assert!(written.contains("\"api_key_set\":false"));
}

#[test]
fn configuration_profile_observation_borrows_the_actual_built_profile_without_sending() {
    let listener = listener();
    let home = folder("configured-profile-observation");
    let config = if cfg!(target_os = "macos") {
        home.join("Library/Application Support/thinkthen/config.json")
    } else {
        home.join("thinkthen/config.json")
    };
    fs::create_dir_all(config.parent().unwrap()).unwrap();
    fs::write(&config, serde_json::json!({
        "schema":"thinkthen.config/1", "backend":"local", "backends": {"local": {
            "url":listener.base(), "key_env":"UNSET_CONFIGURED_PROFILE_KEY", "model":"fixed",
            "profile":{"schema":"thinkthen.backend-profile/1","name":"configured-profile","max_questions":3}
        }}
    }).to_string()).unwrap();
    let written = in_child("config-only", &[("XDG_CONFIG_HOME", home.to_str().unwrap()), ("HOME", home.to_str().unwrap()), ("APPDATA", home.to_str().unwrap())]);
    assert!(written.contains("profile Some(\"configured-profile\")"), "{written}");
    assert_eq!(listener.count(), 0);
    fs::remove_dir_all(home).unwrap();
}

/// Observe maintenance metadata before route validation; explicit paths remain usable.
pub(super) fn cache_capture(named: &str) -> Vec<String> {
    let builder = EngineBuilder::from_config().unwrap();
    let cache = builder.cache_settings();
    let mut lines = vec![json!({
        "enabled": cache.cache_enabled,
        "enabled_source": cache.cache_enabled_source,
        "platform": cache.cache_is_platform_default,
        "target": cache.cache_target_bytes,
        "target_source": cache.cache_target_source,
        "path_present": cache.cache_path.is_some(),
    }).to_string(), shown(builder.resolved_settings()),
        shown(EngineBuilder::from_env().and_then(|builder| builder.resolved_settings()))];
    let builder = builder.cache_at(named).unwrap();
    let cache = builder.cache_settings();
    assert_eq!(cache.cache_path.as_deref(), Some(Path::new(named)));
    assert!(cache.cache_enabled);
    assert!(!cache.cache_is_platform_default);
    assert_eq!(cache.cache_path_source, "command_line");
    assert!(!format!("{cache:?}").contains(named));
    lines.push(format!("named target {}", cache.cache_target_bytes));
    lines
}

#[test]
fn cache_metadata_ignores_irrelevant_routes_and_retains_disabled_default_and_target() {
    for (tag, route) in [("unknown", r#""backend":"typesafe""#),
        ("url", r#""url":"https://cache-metadata.example/v1""#)] {
        let home = folder(&format!("cache-route-{tag}"));
        let config = if cfg!(target_os = "macos") {
            home.join("Library/Application Support/thinkthen/config.json")
        } else {
            home.join("thinkthen/config.json")
        };
        fs::create_dir_all(config.parent().unwrap()).unwrap();
        fs::write(&config, format!(r#"{{"schema":"thinkthen.config/1","cache":false,"cache_bytes":17,{route}}}"#)).unwrap();
        let named = home.join("unopened-private-cache-marker");
        let written = in_child("cache-only", &[
            ("XDG_CONFIG_HOME", home.to_str().unwrap()), ("HOME", home.to_str().unwrap()),
            ("APPDATA", home.to_str().unwrap()), ("THINKTHEN_API_KEY", "private-unread-key"),
            ("THINKTHEN_BACKEND", "unknown-private-route"),
            ("THINKTHEN_BASE_URL", if tag == "url" { "invalid-relative-address" } else { "" }),
            (ARGUMENT, named.to_str().unwrap()),
        ]);
        for fact in ["\"enabled\":false", "\"platform\":true", "\"path_present\":true",
            "\"target\":17", "\"target_source\":\"configuration\"", "named target 17", "Usage:"] {
            assert!(written.contains(fact), "{written}");
        }
        assert!(!written.contains("private-unread-key"));
        assert!(!named.exists());
        fs::remove_dir_all(home).unwrap();
    }
}
