//! Independent native ownership, identity and read-only library contracts.
use super::{
    privacy,
    process::Owned,
    support::{self, Scratch},
};
use crate::child::ChildEnvironment as _;
use conformance_backend::{Canned, Listener};
use std::fs;
use std::process::{Command, Stdio};

#[test]
fn independent_month_and_lock_full_native_ids_differ() {
    let (scratch, listener) = privacy::initialized();
    let month = fs::File::open(privacy::month(&scratch)).expect("independent month");
    let lock = fs::File::open(scratch.usage().join(".lock")).expect("independent lock");
    let month_id = super::ffi::identity(&month).expect("month full native ID");
    let lock_id = super::ffi::identity(&lock).expect("lock full native ID");
    assert_eq!(month_id.0, lock_id.0);
    assert_ne!(month_id.1, lock_id.1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn public_library_shared_configuration_is_silent_and_refusal_debug_withholds_keys() {
    for named in [false, true] {
        let scratch = Scratch::new();
        fs::create_dir_all(scratch.config().parent().expect("parent")).expect("parent");
        let text = if named {
            r#"{"schema":"thinkthen.config/1","backends":{"fixture":{"url":"http://127.0.0.1:1","key_env":"FIXTURE_NAMED_KEY","model":"local-1"}}}"#
        } else {
            r#"{"schema":"thinkthen.config/1","model":"local-1"}"#
        };
        fs::write(scratch.config(), text).expect("configuration");
        support::plant(&scratch.config(), "(A;;FW;;;WD)", false);
        let before = support::descriptor(&scratch.config());
        let mut command = Command::new(std::env::current_exe().expect("test binary"));
        command
            .clear_environment()
            .home(&scratch.0)
            .args(["--exact", "library_child", "--ignored", "--nocapture"])
            .env("THINKTHEN_API_KEY", "sk-library-fixture-secret")
            .env("FIXTURE_NAMED_KEY", "sk-named-fixture-secret")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let output = Owned::spawn(&mut command)
            .expect("library subprocess")
            .finish()
            .expect("bounded library");
        assert!(output.status.success());
        assert!(output.stderr.is_empty(), "library must write no warning");
        let text = String::from_utf8(output.stdout).expect("library output");
        assert!(text.contains(if named { "refused" } else { "accepted" }));
        for secret in [
            "sk-library-fixture-secret",
            "sk-named-fixture-secret",
            "FIXTURE_NAMED_KEY",
        ] {
            assert!(!text.contains(secret));
        }
        assert_eq!(support::descriptor(&scratch.config()), before);
    }
}

#[test]
fn unreadable_configuration_refuses_locally_with_no_native_details_or_sends() {
    let scratch = Scratch::new();
    fs::create_dir_all(scratch.config().parent().expect("parent")).expect("parent");
    fs::write(scratch.config(), r#"{"schema":"thinkthen.config/1"}"#).expect("config");
    support::powershell(
        &scratch.config(),
        "$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; $a=Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH; $a.SetSecurityDescriptorSddlForm(\"O:$sid`D:P\"); Set-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH -AclObject $a",
    );
    let listener = Listener::answering(|_| Canned::ok(support::ANSWER)).expect("loopback");
    let output = privacy::live(&scratch, &listener);
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(listener.count(), 0);
    assert_eq!(
        output.stderr,
        b"thinkthen: the configuration file could not be read\n"
    );
    support::plant(&scratch.config(), "", false);
}

#[test]
fn replay_keeps_success_and_sends_nothing_beside_unsafe_usage() {
    let (scratch, listener) = privacy::initialized();
    let record = scratch.0.join("recording");
    let record_name = record.to_str().expect("record path");
    let arguments = [
        "decide",
        "Accepted?",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--record",
        record_name,
    ];
    assert_eq!(scratch.run(&arguments).status.code(), Some(0));
    support::plant(&scratch.usage(), "(A;;FR;;;WD)", false);
    let output = scratch.run(&[
        "decide",
        "Accepted?",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--replay",
        record_name,
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert_eq!(listener.count(), 2);
}

#[test]
fn elevated_prerequisite_still_requires_explicit_user_ownership_on_new_usage() {
    let scratch = Scratch::new();
    let elevated = support::powershell(
        &scratch.0,
        "$identity=[System.Security.Principal.WindowsIdentity]::GetCurrent(); $principal=New-Object System.Security.Principal.WindowsPrincipal($identity); if(!$principal.IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)){throw 'elevated token prerequisite'}; 'elevated'",
    );
    assert_eq!(elevated, "elevated");
    let listener = Listener::answering(|_| Canned::ok(support::ANSWER)).expect("loopback");
    assert_eq!(privacy::live(&scratch, &listener).status.code(), Some(0));
    support::assert_private(&scratch.usage());
    support::assert_private(&scratch.usage().join(".lock"));
    support::assert_private(&privacy::month(&scratch));
}
