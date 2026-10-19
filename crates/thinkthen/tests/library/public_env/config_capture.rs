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
    assert!(written.contains("sent 1 cached 0"));
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
