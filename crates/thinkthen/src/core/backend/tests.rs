//! The address rule and the model rule, as edge tables.

use super::{Backend, BackendError};
use crate::core::adapters::built_in::DEFAULT_MODEL;
use crate::core::text::BlankTextError;

/// The address the tool posts to when no base replaces the default one.
const BUILT_IN: &str = "https://api.typesafe.ai/v1/systemone";

/// Resolve from the two sources a case names, with the default model.
fn resolve(url: Option<&str>, base: Option<&str>) -> Result<Backend, BackendError> {
    Backend::resolve(url, base, DEFAULT_MODEL)
}

/// Only the three spellings the clear-text rule names prove an `https://`
/// base is this machine, so only they send with no key.
#[test]
fn only_the_three_loopback_spellings_are_loopback() {
    for (host, loopback) in [
        ("localhost", true),
        ("LOCALHOST", true),
        ("127.0.0.1:9", true),
        ("[::1]", true),
        ("localhost.", false),
        ("localhost.evil.com", false),
        ("127.0.0.1.nip.io", false),
        ("127.0.0.2", false),
        ("0.0.0.0", false),
    ] {
        let backend = resolve(Some(&format!("https://{host}/v1")), None).expect("a base");
        assert_eq!(backend.is_loopback(), loopback, "{host}");
    }
}

#[test]
fn nothing_named_resolves_the_default_base_and_the_default_model() {
    let backend = resolve(None, None).expect("the default base resolves");

    assert_eq!(backend.url().as_str(), BUILT_IN);
    assert_eq!(backend.model().as_str(), DEFAULT_MODEL);
}

#[test]
fn the_option_outranks_the_variable_and_the_variable_outranks_the_default() {
    let option = "http://127.0.0.1:8080/v1";
    let variable = "http://127.0.0.1:9090/v1";
    let cases = [
        (
            Some(option),
            Some(variable),
            "http://127.0.0.1:8080/v1/systemone",
        ),
        (Some(option), None, "http://127.0.0.1:8080/v1/systemone"),
        (None, Some(variable), "http://127.0.0.1:9090/v1/systemone"),
        (None, None, BUILT_IN),
    ];

    for (url, base, expected) in cases {
        let backend = resolve(url, base).expect("a base names an address");
        assert_eq!(backend.url().as_str(), expected, "{url:?} over {base:?}");
    }
}

#[test]
fn a_model_replaces_the_default_and_a_blank_one_is_refused() {
    let backend = Backend::resolve(None, None, "jev-1.13.0").expect("a model names the version");
    assert_eq!(backend.model().as_str(), "jev-1.13.0");

    for blank in ["", " ", "\t\n"] {
        assert_eq!(
            Backend::resolve(None, None, blank),
            Err(BackendError::Blank(BlankTextError::ModelName)),
            "{blank:?}"
        );
    }
}

#[test]
fn a_base_is_read_to_one_address_or_refused_without_showing_itself() {
    let taken = [
        ("http://localhost/v1", "http://localhost/v1/systemone"),
        ("http://localhost/v1/", "http://localhost/v1/systemone"),
        ("http://localhost/v1///", "http://localhost/v1/systemone"),
        ("  http://localhost/v1\n", "http://localhost/v1/systemone"),
        ("HTTPS://host/v1", "https://host/v1/systemone"),
        ("HtTpS://host/v1", "https://host/v1/systemone"),
        ("HTTP://127.0.0.1/v1", "http://127.0.0.1/v1/systemone"),
        ("http://localhost", "http://localhost/systemone"),
    ];
    for (base, expected) in taken {
        let backend = resolve(None, Some(base)).expect("a base names an address");
        assert_eq!(backend.url().as_str(), expected, "{base:?}");
    }

    let refused = [
        ("ftp://host/v1", BackendError::InvalidScheme),
        ("host/v1", BackendError::InvalidScheme),
        ("http:/host/v1", BackendError::InvalidScheme),
        (
            "https://someone:sk-in-the-address@host/v1",
            BackendError::UserInformation,
        ),
        ("http://someone@host/v1", BackendError::UserInformation),
        (
            "http://host/v1?key=sk-in-the-address",
            BackendError::QueryOrFragment,
        ),
        (
            "http://host/v1#sk-in-the-address",
            BackendError::QueryOrFragment,
        ),
        // A port is a number a socket can carry, and none of these is one.
        ("http://localhost:x/v1", BackendError::InvalidPort),
        ("https://host:8080a/v1", BackendError::InvalidPort),
        ("http://localhost:/v1", BackendError::InvalidPort),
        ("http://localhost:99999999999/v1", BackendError::InvalidPort),
        ("https://host:65536/v1", BackendError::InvalidPort),
        ("http://[::1]:+80/v1", BackendError::InvalidPort),
        ("https:///v1", BackendError::NotAnAddress),
    ];
    for (base, expected) in refused {
        let error = resolve(None, Some(base)).expect_err("a base that names no address is refused");
        assert_eq!(error, expected, "{base}");
        assert!(!error.to_string().contains(base), "{error}");
        if expected == BackendError::NotAnAddress {
            assert_eq!(error.to_string(), "a base address has a host");
        }
    }
}

#[test]
fn only_literal_ascii_letters_in_an_unbracketed_host_become_lowercase() {
    let cases = [
        (
            "HTTPS://XN--BCHER-KVA.ExAmPlE:00443/MiXeD/%2F",
            "https://xn--bcher-kva.example:00443/MiXeD/%2F/systemone",
        ),
        (
            "https://[2001:DB8::A]:00443/MiXeD/%2F",
            "https://[2001:DB8::A]:00443/MiXeD/%2F/systemone",
        ),
        (
            "https://MiXeD%2EHoSt/MiXeD/%2F",
            "https://mixed%2Ehost/MiXeD/%2F/systemone",
        ),
        (
            "https://BÜCHER.ExAmPlE/MiXeD/%2F",
            "https://bÜcher.example/MiXeD/%2F/systemone",
        ),
    ];

    for (base, expected) in cases {
        let backend = resolve(None, Some(base)).expect("a base names an address");
        assert_eq!(backend.url().as_str(), expected, "{base:?}");
    }
}

#[test]
fn plain_http_reaches_loopback_alone_and_every_other_host_is_refused() {
    let loopback = [
        "http://localhost/v1",
        "http://LocalHost:8080/v1",
        "http://127.0.0.1/v1",
        "http://127.0.0.1:8721/v1",
        "http://[::1]/v1",
        "http://[::1]:9/v1",
    ];
    for base in loopback {
        resolve(None, Some(base)).expect("a loopback base is taken");
    }

    let refused = [
        "http://host/v1",
        "http://example.com/v1",
        "http://localhost.example.com/v1",
        "http://localhost./v1",
        "http://127.0.0.1./v1",
        "http://127.1/v1",
        "http://0.0.0.0/v1",
        "http://10.0.0.5/v1",
        "http://192.168.1.4:8080/v1",
        "http://[::ffff:127.0.0.1]/v1",
        "http://[::ffff:7f00:1]/v1",
        "http://[0:0:0:0:0:0:0:1]/v1",
        "HTTP://host/v1",
        "http://2130706433/v1",
    ];
    for base in refused {
        let error = resolve(None, Some(base)).expect_err("a plain http base is refused");
        assert_eq!(error, BackendError::KeyInClear, "{base}");
        assert_eq!(
            error.to_string(),
            "`http://` sends the key across the network in clear text, \
             so it reaches localhost, 127.0.0.1, and [::1] alone"
        );
    }

    for base in [
        "https://host/v1",
        "HTTPS://example.com/v1",
        "https://[::1]/v1",
    ] {
        resolve(None, Some(base)).expect("every https base is untouched");
    }
}

#[test]
fn an_address_is_secure_under_https_and_never_under_plain_http() {
    for base in ["https://host/v1", "HTTPS://host/v1", "https://[::1]/v1"] {
        let backend = resolve(None, Some(base)).expect("a base names an address");
        assert!(backend.is_secure(), "{base}");
    }

    for base in ["http://localhost/v1", "HTTP://127.0.0.1/v1", "http://[::1]"] {
        let backend = resolve(None, Some(base)).expect("a base names an address");
        assert!(!backend.is_secure(), "{base}");
    }
}

#[test]
fn a_base_that_holds_only_white_space_is_refused_as_a_blank_address() {
    for blank in ["", " ", "\t", "\n", "  \t\r\n "] {
        for (url, base) in [(Some(blank), None), (None, Some(blank))] {
            assert_eq!(
                resolve(url, base),
                Err(BackendError::Blank(BlankTextError::Url)),
                "{blank:?}"
            );
        }
    }
}
