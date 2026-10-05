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

#[test]
fn check_preflights_every_fixed_probe_under_the_selected_setup_before_sending_or_planning() {
    let bodies = include_str!("../../../../../specification/fixtures/check/requests.jsonl");
    let first_bytes = bodies.lines().next().expect("first probe").len();
    let second_bytes = bodies.lines().nth(1).expect("choice probe").len();
    let cases = [
        (
            "max_request_bytes",
            first_bytes,
            "request bytes",
            second_bytes,
        ),
        ("max_request_bytes", 1, "request bytes", first_bytes),
        // The first three probes fit; the fixed mixed probe has five wire questions.
        ("max_questions", 1, "questions", 5),
        // The first probe fits, but the choice probe needs three options.
        ("max_options", 2, "options", 3),
        ("max_evidence_bytes", 1, "evidence bytes", 53),
    ];
    for (field, limit, words, actual) in cases {
        let home = Home::new("check-profile-0400");
        let target = listener();
        let profile =
            json!({"schema":"thinkthen.backend-profile/1","name":"probe-limit",field:limit});
        home.config(
            &json!({"schema":"thinkthen.config/1","backends":{"small":{
                "url":target.base(),"key_env":"LOCAL_D1_KEY","model":"jev-1.13.0","profile":profile
            }}})
            .to_string(),
        );
        for plan in [false, true] {
            let mut args = vec!["check", "--backend", "small"];
            if plan {
                args.push("--plan");
            }
            let out = home.run(&args, &[]);
            assert_eq!(
                out.status.code(),
                Some(2),
                "{field}, plan={plan}: {}",
                said(&out).1
            );
            assert_eq!(said(&out).0, "");
            assert_eq!(
                said(&out).1,
                format!(
                    "thinkthen: profile probe-limit allows at most {limit} {words}; this request has {actual}\n"
                )
            );
            assert_eq!(target.count(), 0, "{field}, plan={plan}");
        }
        // At a non-loopback address a missing key would otherwise fail first.
        home.config(&json!({"schema":"thinkthen.config/1","backends":{"small":{
            "url":"https://probe.example.invalid/v1","key_env":"LOCAL_D1_KEY","model":"jev-1.13.0","profile":profile
        }}}).to_string());
        let proxy = Proxy::start();
        let out = home.run(
            &["check", "--backend", "small"],
            &[("LOCAL_D1_KEY", ""), ("HTTPS_PROXY", &proxy.url)],
        );
        assert_eq!(out.status.code(), Some(2));
        assert_eq!(said(&out).0, "");
        assert_eq!(
            said(&out).1,
            format!(
                "thinkthen: profile probe-limit allows at most {limit} {words}; this request has {actual}\n"
            )
        );
        assert_eq!(proxy.count(), 0);
        home.assert_no_marker_in_files();
    }
}

#[test]
fn an_explicit_command_profile_still_outranks_the_setup_request_limit() {
    let home = Home::new("explicit-request-profile-0400");
    let target = listener();
    home.config(
        &json!({"schema":"thinkthen.config/1","backend":"small","backends":{"small":{
            "url":target.base(),"key_env":"LOCAL_D1_KEY","model":"m",
            "profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_request_bytes":1}
        }}})
        .to_string(),
    );
    let profile = home.path("explicit.json");
    std::fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"explicit","max_request_bytes":4096}"#,
    )
    .expect("explicit profile");
    let input = home.evidence(&["alpha"]);
    let out = home.run(
        &[
            "decide",
            "a refund?",
            "--input",
            &input,
            "--profile",
            &profile,
            "--no-cache",
        ],
        &[],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    assert_eq!(said(&out).0, "true\n");
    assert_eq!(target.count(), 1);
    home.assert_no_marker_in_files();
}
