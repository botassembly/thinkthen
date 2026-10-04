//! Ticket 0400: setup-only default inheritance and unchanged trust guards.
use super::support::{Home, Proxy, listener, said};
use serde_json::json;

#[test]
fn profile_only_default_builtin_is_inherited_and_an_unnamed_custom_route_is_not() {
    let home = Home::new("default-setup-0400");
    let target = listener();
    home.config(&json!({"schema":"thinkthen.config/1","backends":{"typesafe":{"profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":3}}}}).to_string());
    let input = home.evidence(&["alpha"]);
    let planned = home.run(
        &[
            "decide",
            "a refund?",
            "--input",
            &input,
            "--no-cache",
            "--plan",
        ],
        &[],
    );
    assert_eq!(planned.status.code(), Some(2));
    assert_eq!(
        said(&planned).1,
        "thinkthen: profile small allows at most 3 evidence bytes; this request has 6\n"
    );
    let out = home.run(
        &[
            "decide",
            "a refund?",
            "--input",
            &input,
            "--url",
            target.base(),
            "--no-cache",
        ],
        &[],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    assert_eq!(
        target.count(),
        1,
        "a custom posting URL does not inherit the built-in profile"
    );
}

#[cfg(unix)]
#[test]
fn setup_entries_cannot_bypass_owner_or_named_key_host_guards() {
    use std::os::unix::fs::PermissionsExt as _;
    let home = Home::new("setup-trust-0400");
    let target = listener();
    let profile =
        json!({"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":100});
    home.config(&json!({"schema":"thinkthen.config/1","backends":{"typesafe":{"usd_per_million_input":"2","usd_per_million_output":"0","profile":profile}}}).to_string());
    let input = home.evidence(&["alpha"]);
    let configuration = crate::child::Folder::Config
        .under(&home.root)
        .join("config.json");
    std::fs::set_permissions(&configuration, std::fs::Permissions::from_mode(0o666)).unwrap();
    let out = home.run(
        &[
            "decide",
            "a refund?",
            "--input",
            &input,
            "--url",
            target.base(),
            "--no-cache",
        ],
        &[],
    );
    assert_eq!(out.status.code(), Some(5));
    assert_eq!(
        said(&out).1,
        "thinkthen: the configuration file is writable by another user, so its `backends` are refused; keep it writable by its owner alone\n"
    );
    assert_eq!(target.count(), 0);
    std::fs::set_permissions(&configuration, std::fs::Permissions::from_mode(0o600)).unwrap();
    let proxy = Proxy::start();
    let out = home.run(
        &[
            "decide",
            "a refund?",
            "--input",
            &input,
            "--backend",
            "typesafe",
            "--url",
            "https://api.liquid.ai/decisions/v1",
            "--no-cache",
        ],
        &[("HTTPS_PROXY", &proxy.url)],
    );
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        said(&out).1,
        "thinkthen: backend `typesafe` reads `TYPESAFE_API_KEY`, the key of backend `typesafe`, which never goes to the address of backend `liquid`\n"
    );
    assert_eq!(proxy.count(), 0, "the key host refusal opens no connection");
    assert_eq!(target.count(), 0);
}

#[test]
fn selected_setup_does_not_hide_an_invalid_unselected_entry() {
    let home = Home::new("all-setups-0400");
    let target = listener();
    home.config(&json!({"schema":"thinkthen.config/1","backend":"small","backends":{
        "small":{"url":target.base(),"key_env":"LOCAL_D1_KEY","model":"m","usd_per_million_input":"2","usd_per_million_output":"0"},
        "unused":{"url":target.base(),"key_env":"LOCAL_D1_KEY","model":"m","profile":{"schema":"invalid-marker-0400"}}
    }}).to_string());
    let input = home.evidence(&["alpha"]);
    let out = home.run(
        &["decide", "a refund?", "--input", &input, "--no-cache"],
        &[],
    );
    assert_eq!(out.status.code(), Some(5));
    assert_eq!(
        said(&out).1,
        "thinkthen: configuration backend field `profile` has schema `thinkthen.backend-profile/1`\n"
    );
    assert!(!said(&out).1.contains("invalid-marker"));
    assert_eq!(target.count(), 0);
}
