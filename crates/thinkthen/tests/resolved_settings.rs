//! Explicit settings inspection must neither expose credentials nor build storage.
use conformance_backend::{Canned, Listener};
use thinkthen::{Engine, ErrorKind};

#[test]
fn settings_inspection_withholds_credentials_and_never_sends() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let key = "settings-snapshot-private-key";
    let builder = Engine::builder()
        .base_url(listener.base()).unwrap()
        .model("settings-private-model").unwrap()
        .api_key(key).unwrap()
        .no_cache();
    let settings = builder.resolved_settings().unwrap();
    assert!(settings.api_key_set);
    assert_eq!(settings.model, "settings-private-model");
    assert_eq!(settings.url_source, "command_line");
    assert!(!settings.cache_enabled);
    let shown = format!("{settings:?}");
    for secret in [key, listener.base(), "settings-private-model"] {
        assert!(!shown.contains(secret));
    }
    assert_eq!(listener.count(), 0);
    let unsafe_address = format!("{}/{}", listener.base().trim_end_matches('/'), key);
    let builder = builder.base_url(&unsafe_address).unwrap();
    let error = builder.resolved_settings().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(!error.to_string().contains(key));
    assert_eq!(listener.count(), 0);
}

#[test]
fn explicit_request_rates_retain_the_native_bounds() {
    for (rate, admitted) in [(0, false), (1, true), (60_000, true), (60_001, false)] {
        let selected = Engine::builder().requests_per_minute(rate);
        assert_eq!(selected.is_ok(), admitted);
        if let Err(error) = selected {
            assert_eq!(error.kind(), ErrorKind::Usage);
            assert_eq!(error.detail().message(), "THINKTHEN_REQUESTS_PER_MINUTE takes a whole number from 1 to 60000");
        }
    }
}
