//! The tier walk and the host rule, as edge tables.

use super::{Named, Tier, choose, find};
use crate::core::BackendError;
use crate::core::plan::Descriptions;

fn configured() -> Vec<Named> {
    vec![
        Named::new(
            "local-d1",
            "http://127.0.0.1:8080/v1",
            "LOCAL_D1_KEY",
            "d1:free",
        ),
        Named::new(
            "stolen",
            "https://API.LIQUID.AI:443/other",
            "TYPESAFE_API_KEY",
            "m",
        ),
        Named::new("second", "https://api.typesafe.ai/v1", "SECOND_KEY", "m"),
        Named::new(
            "stolen-ollama",
            "https://api.liquid.ai/decisions/v1",
            "OLLAMA_API_KEY",
            "m",
        ),
    ]
}

#[test]
fn the_first_tier_naming_anything_decides_and_lower_tiers_are_ignored() {
    let configured = configured();
    let loopback = Some("http://127.0.0.1:9/v1");
    // (tiers, chosen name, chosen address, deciding tier)
    type Case<'a> = (
        [Tier<'a>; 3],
        Option<&'a str>,
        Option<&'a str>,
        Option<usize>,
    );
    let cases: [Case<'_>; 8] = [
        ([(None, None); 3], None, None, None),
        (
            [
                (Some("liquid"), None),
                (None, loopback),
                (Some("typesafe"), None),
            ],
            Some("liquid"),
            None,
            Some(0),
        ),
        (
            [(None, loopback), (Some("liquid"), None), (None, None)],
            None,
            loopback,
            Some(0),
        ),
        (
            [(Some("liquid"), loopback), (None, None), (None, None)],
            Some("liquid"),
            loopback,
            Some(0),
        ),
        (
            [(None, None), (None, loopback), (Some("liquid"), None)],
            None,
            loopback,
            Some(1),
        ),
        (
            [(None, None), (Some("typesafe"), None), (None, loopback)],
            Some("typesafe"),
            None,
            Some(1),
        ),
        (
            [(None, None), (None, None), (Some("local-d1"), None)],
            Some("local-d1"),
            None,
            Some(2),
        ),
        (
            [(None, None), (None, None), (Some("nowhere"), loopback)],
            None,
            None,
            None,
        ),
    ];
    for (index, (tiers, name, url, tier)) in cases.into_iter().enumerate() {
        let Ok(choice) = choose(&tiers, &configured) else {
            assert_eq!(index, 7, "only the unknown name fails");
            continue;
        };
        assert_eq!(choice.named.as_ref().map(Named::name), name, "{index}");
        assert_eq!(choice.url, url, "{index}");
        assert_eq!(choice.tier, tier, "{index}");
    }
}

#[test]
fn names_are_checked_before_they_are_looked_up() {
    let configured = configured();
    for name in ["", "Liquid", "a_b", &"x".repeat(33), "sk live"] {
        assert_eq!(
            find(name, &configured),
            Err(BackendError::InvalidName),
            "{name}"
        );
        assert_eq!(
            BackendError::InvalidName.to_string(),
            "a backend name uses 1 to 32 lowercase letters, digits, and hyphens"
        );
    }
    assert_eq!(
        find("nowhere", &[]).map_err(|error| error.to_string()),
        Err("unknown backend `nowhere`; the built-in backends are `liquid`, `ollama` and `typesafe`, and the configuration file may name more".to_owned())
    );
    assert!(
        find("local-d1", &[]).is_err(),
        "a bare builder knows only the built-ins"
    );
    assert_eq!(
        find(&"x".repeat(32), &[]),
        Err(BackendError::Unknown("x".repeat(32)))
    );
}

/// The section 5 refusal for one backend, variable, owner and other backend.
fn refusal(name: &str, variable: &str, owner: &str, other: &str) -> Option<String> {
    Some(format!(
        "backend `{name}` reads `{variable}`, the key of backend `{owner}`, which never goes to the address of backend `{other}`"
    ))
}

/// One guard case: the backend, the address, and the refusal or none.
type GuardCase<'a> = (&'a str, Option<&'a str>, Option<String>);

/// Assert each case against the guard.
fn assert_guard(cases: &[GuardCase<'_>]) {
    let configured = configured();
    for (name, url, expected) in cases {
        let choice = choose(&[(Some(name), *url)], &configured).expect("a known backend");
        let backend = choice.backend(None, "unused").expect("an address");
        assert_eq!(
            choice.guard(&backend).err().map(|error| error.to_string()),
            *expected,
            "{name} {url:?}"
        );
    }
}

#[test]
fn a_built_in_key_never_goes_to_the_other_built_in_host() {
    let configured = configured();
    // (backend, address, refusal)
    let cases = [
        (
            "typesafe",
            Some("https://api.liquid.ai/decisions/v1"),
            refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "typesafe",
            Some("https://API.Liquid.ai:8443/elsewhere"),
            refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "liquid",
            Some("https://api.typesafe.ai/v1"),
            refusal("liquid", "LIQUIDAI_API_KEY", "liquid", "typesafe"),
        ),
        (
            "stolen",
            None,
            refusal("stolen", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "typesafe",
            Some("https://api.liquid.ai./decisions/v1"),
            refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "typesafe",
            Some("https://api.liquid.ai../v1"),
            refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "typesafe",
            Some("https://api%2Eliquid.ai/x"),
            refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "typesafe",
            Some("https://api%2eLIQUID%2Eai%2e/x"),
            refusal("typesafe", "TYPESAFE_API_KEY", "typesafe", "liquid"),
        ),
        (
            "liquid",
            Some("https://api.typesafe.ai./v1"),
            refusal("liquid", "LIQUIDAI_API_KEY", "liquid", "typesafe"),
        ),
        ("typesafe", Some("https://sub.api.liquid.ai/v1"), None),
        ("typesafe", Some("https://gateway.example/v1"), None),
        ("liquid", Some("http://127.0.0.1:9/v1"), None),
        ("liquid", None, None),
        ("typesafe", None, None),
        ("second", None, None),
        ("local-d1", Some("https://api.liquid.ai/decisions/v1"), None),
    ];
    assert_guard(&cases);
    let unnamed = choose(
        &[(None, Some("https://api.liquid.ai/decisions/v1"))],
        &configured,
    )
    .expect("unnamed");
    let backend = unnamed.backend(None, "m").expect("an address");
    assert_eq!(
        unnamed.guard(&backend),
        Ok(()),
        "THINKTHEN_API_KEY is no built-in's variable"
    );
}

#[test]
fn the_ollama_key_stays_off_other_built_in_hosts_and_loopback_is_no_built_in_host() {
    assert_guard(&[
        // ADR 0115: `OLLAMA_API_KEY` never goes to another built-in's host,
        // and a loopback built-in base is no other built-in's host.
        (
            "ollama",
            Some("https://api.typesafe.ai/v1"),
            refusal("ollama", "OLLAMA_API_KEY", "ollama", "typesafe"),
        ),
        (
            "ollama",
            Some("https://API.liquid.ai./decisions/v1"),
            refusal("ollama", "OLLAMA_API_KEY", "ollama", "liquid"),
        ),
        (
            "stolen-ollama",
            None,
            refusal("stolen-ollama", "OLLAMA_API_KEY", "ollama", "liquid"),
        ),
        ("ollama", None, None),
        ("ollama", Some("https://ollama.example/v1"), None),
        ("liquid", Some("http://localhost:8080/v1"), None),
        ("liquid", Some("http://localhost:11434/v1"), None),
        ("typesafe", Some("http://localhost:11434/v1"), None),
    ]);
}

#[test]
fn models_and_keys_follow_the_path() {
    let configured = configured();
    let liquid = choose(&[(Some("liquid"), None)], &configured).expect("liquid");
    assert_eq!(liquid.keys(), ["LIQUIDAI_API_KEY", "LIQUID_API_KEY"]);
    assert_eq!(liquid.key_variable(), "LIQUIDAI_API_KEY");
    let backend = liquid.backend(None, "configured-model").expect("liquid");
    assert_eq!(
        backend.url().as_str(),
        "https://api.liquid.ai/decisions/v1/systemone"
    );
    assert_eq!(backend.model().as_str(), "d1:free");
    assert_eq!(
        liquid
            .backend(Some("asked"), "configured-model")
            .expect("liquid")
            .model()
            .as_str(),
        "asked"
    );
    let unnamed = choose(&[(None, None)], &configured).expect("unnamed");
    assert_eq!(unnamed.keys(), ["THINKTHEN_API_KEY"]);
    assert_eq!(
        unnamed
            .backend(None, "configured-model")
            .expect("default")
            .model()
            .as_str(),
        "configured-model"
    );
    // ADR 0115: only `ollama` sends descriptions as text, at any address.
    for (name, url, form) in [
        ("ollama", None, Descriptions::Text),
        (
            "ollama",
            Some("https://ollama.example/v1"),
            Descriptions::Text,
        ),
        ("liquid", None, Descriptions::Authored),
        ("typesafe", None, Descriptions::Authored),
        ("local-d1", None, Descriptions::Authored),
    ] {
        let choice = choose(&[(Some(name), url)], &configured).expect("a backend");
        let backend = choice.backend(None, "unused").expect("an address");
        assert_eq!(backend.descriptions(), form, "{name} {url:?}");
    }
    let ollama = choose(&[(Some("ollama"), None)], &configured).expect("ollama");
    assert_eq!(ollama.keys(), ["OLLAMA_API_KEY"]);
    let backend = ollama.backend(None, "unused").expect("ollama");
    assert_eq!(
        backend.url().as_str(),
        "http://localhost:11434/v1/systemone"
    );
    assert_eq!(backend.model().as_str(), "nimble");
    assert_eq!(
        unnamed.backend(None, "m").expect("default").descriptions(),
        Descriptions::Authored
    );
    assert!(format!("{:?}", configured[0]).contains("<withheld>"));
    assert!(!format!("{:?}", configured[0]).contains("8080"));
}
