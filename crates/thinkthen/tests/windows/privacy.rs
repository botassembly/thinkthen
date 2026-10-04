use super::support::{self, Scratch};
use conformance_backend::{Canned, Listener};
use std::fs;

const SHARED_WARNING: &str = "thinkthen: another user owns the configuration file or its Windows access permissions allow another user to change it; it decides where the key and evidence go\n";
const NAMED_REFUSAL: &str = "thinkthen: another user owns the configuration file or its Windows access permissions allow another user to change it, so its `backends` are refused; keep it owned by your user and writable only by your user and Windows SYSTEM\n";

pub(super) fn live(scratch: &Scratch, listener: &Listener) -> std::process::Output {
    scratch.run(&[
        "decide",
        "Accepted?",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ])
}
pub(super) fn initialized() -> (Scratch, Listener) {
    let scratch = Scratch::new();
    let listener = Listener::answering(|_| Canned::ok(support::ANSWER)).expect("counted loopback");
    let output = live(&scratch, &listener);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert_eq!(listener.requests().len(), 1);
    (scratch, listener)
}
pub(super) fn month(scratch: &Scratch) -> std::path::PathBuf {
    fs::read_dir(scratch.usage())
        .expect("usage folder")
        .map(|entry| entry.expect("entry").path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .expect("recognized month")
}
#[test]
fn native_creation_and_monthly_replacement_keep_exact_counts_and_private_access() {
    let (scratch, listener) = initialized();
    for path in [
        scratch.usage(),
        scratch.usage().join(".lock"),
        month(&scratch),
    ] {
        support::assert_private(&path);
    }
    assert_eq!(live(&scratch, &listener).status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    let totals = support::status(&scratch);
    assert_eq!(totals["usage"]["total"]["requests_sent"], 2);
    assert_eq!(totals["usage"]["total"]["input_tokens"], 10);
    assert_eq!(totals["usage"]["total"]["output_tokens"], 2);
    support::assert_private(&month(&scratch));
}
#[test]
fn foreign_usage_grants_and_owners_refuse_before_any_additional_request() {
    for object in ["root", "lock", "month"] {
        for grant in [
            "(A;;FR;;;WD)",
            "(A;;FW;;;WD)",
            "(A;;0x4;;;WD)",
            "(A;;GA;;;WD)",
            "(A;OICI;FR;;;WD)",
            "(D;;FW;;;WD)(A;;FR;;;WD)",
            "owner",
        ] {
            let (scratch, listener) = initialized();
            let path = match object {
                "root" => scratch.usage(),
                "lock" => scratch.usage().join(".lock"),
                _ => month(&scratch),
            };
            support::plant(
                &path,
                if grant == "owner" { "" } else { grant },
                grant == "owner",
            );
            let before = support::descriptor(&path);
            let bytes = if path.is_file() {
                Some(fs::read(&path).expect("original bytes"))
            } else {
                None
            };
            let output = live(&scratch, &listener);
            assert_eq!(output.status.code(), Some(5), "{object} {grant}");
            assert_eq!(listener.requests().len(), 1);
            let subject = match object {
                "root" => "the usage folder that thinkthen status names",
                "lock" => ".lock",
                _ => path
                    .file_name()
                    .expect("month filename")
                    .to_str()
                    .expect("month name"),
            };
            let ending = if object == "root" {
                "aside"
            } else {
                "out of the usage folder that thinkthen status names"
            };
            let sentence = format!(
                "thinkthen: cannot read the usage totals: {subject} has unsafe or unreadable state. Make it owned by your user and restrict its Windows access permissions to your user and Windows SYSTEM, or move it {ending}.\n"
            );
            assert_eq!(output.stderr, sentence.as_bytes());
            assert_eq!(support::descriptor(&path), before);
            if let Some(bytes) = bytes {
                assert_eq!(fs::read(&path).expect("unchanged bytes"), bytes);
            }
        }
    }
}
#[test]
fn unsafe_temporary_is_not_truncated_and_write_failure_keeps_the_judgment() {
    let (scratch, listener) = initialized();
    let temporary = scratch.usage().join(".update.tmp");
    fs::write(&temporary, b"retain temporary fixture").expect("temporary");
    support::plant(&temporary, "(A;;FR;;;WD)", false);
    let before = support::descriptor(&temporary);
    let output = live(&scratch, &listener);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert_eq!(listener.requests().len(), 2);
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage"));
    assert_eq!(
        fs::read(&temporary).expect("retained temporary"),
        b"retain temporary fixture"
    );
    assert_eq!(support::descriptor(&temporary), before);
}
#[test]
fn status_and_plan_keep_their_no_send_contract_beside_unsafe_usage() {
    let (scratch, listener) = initialized();
    support::plant(&scratch.usage(), "(A;;FR;;;WD)", false);
    let output = scratch.run(&["status", "--json"]);
    assert_eq!(output.status.code(), Some(0));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON");
    assert!(report["usage"]["total"].is_null());
    let sentence = format!(
        "thinkthen: cannot read the usage totals: {} has unsafe or unreadable state. {}\n",
        scratch.usage().display(),
        support::ADVICE
    );
    assert_eq!(output.stderr, sentence.as_bytes());
    assert_eq!(
        scratch
            .run(&[
                "decide",
                "Accepted?",
                "--plan",
                "--no-cache",
                "--url",
                listener.base()
            ])
            .status
            .code(),
        Some(0)
    );
    assert_eq!(listener.requests().len(), 1);
}
#[test]
fn malformed_private_month_keeps_invalid_content_advice() {
    let (scratch, listener) = initialized();
    fs::write(month(&scratch), b"malformed fixture").expect("malformed");
    let output = live(&scratch, &listener);
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(listener.requests().len(), 1);
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid contents"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(support::ADVICE));
}
#[test]
fn native_configuration_writer_policy_warns_and_refuses_named_backends_without_leaking_values() {
    for (grant, foreign_owner, shared) in [
        ("(A;;FR;;;WD)", false, false),
        ("(A;;FW;;;WD)", false, true),
        ("(A;;0x4;;;WD)", false, true),
        ("(A;;GA;;;WD)", false, true),
        ("", true, true),
    ] {
        let scratch = Scratch::new();
        fs::create_dir_all(scratch.config().parent().expect("config parent"))
            .expect("config parent");
        fs::write(
            scratch.config(),
            r#"{"schema":"thinkthen.config/1","model":"local-1"}"#,
        )
        .expect("config");
        support::plant(&scratch.config(), grant, foreign_owner);
        let output = scratch.run(&["status", "--json"]);
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(
            output.stderr,
            if shared {
                SHARED_WARNING.as_bytes()
            } else {
                b""
            }
        );
        fs::write(scratch.config(), r#"{"schema":"thinkthen.config/1","backends":{"fixture":{"url":"http://127.0.0.1:1","key_env":"FIXTURE_NAMED_KEY","model":"secret-model-fixture"}}}"#).expect("named config");
        let listener = Listener::answering(|_| Canned::ok(support::ANSWER)).expect("loopback");
        let output = live(&scratch, &listener);
        if shared {
            assert_eq!(output.status.code(), Some(5));
            assert_eq!(output.stderr, NAMED_REFUSAL.as_bytes());
        }
        assert_eq!(listener.requests().len(), usize::from(!shared));
        let printed = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!printed.contains("sk-fixture-only"));
        assert!(!printed.contains("FIXTURE_NAMED_KEY"));
        assert!(!printed.contains("secret-model-fixture"));
    }
}

#[test]
fn null_usage_dacl_is_unrestricted_and_empty_dacl_is_unreadable() {
    for dacl in ["D:NO_ACCESS_CONTROL", "D:P"] {
        let (scratch, listener) = initialized();
        let path = month(&scratch);
        let retained = fs::read(&path).expect("original month");
        support::powershell(
            &path,
            &format!(
                "$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; $a=Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH; $a.SetSecurityDescriptorSddlForm(\"O:${{sid}}{dacl}\"); Set-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH -AclObject $a"
            ),
        );
        // Inspect the actual descriptor representation. An empty rule list alone
        // cannot distinguish an unrestricted null list from a denying empty list.
        let descriptor = support::descriptor(&path);
        assert!(descriptor.contains(dacl), "native descriptor {descriptor}");
        let output = live(&scratch, &listener);
        assert_eq!(output.status.code(), Some(5));
        assert_eq!(listener.requests().len(), 1);
        support::plant(&path, "", false);
        assert_eq!(
            fs::read(&path).expect("retained month after restoration"),
            retained
        );
    }
}

#[test]
fn null_configuration_dacl_is_shared_and_named_backends_are_refused() {
    let scratch = Scratch::new();
    fs::create_dir_all(scratch.config().parent().expect("parent")).expect("parent");
    fs::write(scratch.config(), r#"{"schema":"thinkthen.config/1","backends":{"fixture":{"url":"http://127.0.0.1:1","key_env":"FIXTURE_NAMED_KEY","model":"local-1"}}}"#).expect("configuration");
    support::powershell(
        &scratch.config(),
        "$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; $a=Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH; $a.SetSecurityDescriptorSddlForm(\"O:$sid`D:NO_ACCESS_CONTROL\"); Set-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH -AclObject $a",
    );
    assert!(support::descriptor(&scratch.config()).contains("NO_ACCESS_CONTROL"));
    let listener = Listener::answering(|_| Canned::ok(support::ANSWER)).expect("counted backend");
    let output = live(&scratch, &listener);
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(listener.requests().len(), 0);
    assert_eq!(output.stderr, NAMED_REFUSAL.as_bytes());
}

#[test]
fn a_usage_root_junction_refuses_without_touching_its_target() {
    let (scratch, listener) = initialized();
    let target = scratch.0.join("junction-target");
    fs::rename(scratch.usage(), &target).expect("move owned target");
    let target_month = fs::read_dir(&target)
        .expect("target")
        .map(|entry| entry.expect("entry").path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .expect("month");
    let bytes = fs::read(&target_month).expect("target bytes");
    let security = support::descriptor(&target_month);
    support::powershell(
        &scratch.usage(),
        "$target=Join-Path (Split-Path (Split-Path (Split-Path (Split-Path $env:THINKTHEN_FIXTURE_PATH)))) 'junction-target'; New-Item -ItemType Junction -Path $env:THINKTHEN_FIXTURE_PATH -Target $target | Out-Null; if(!((Get-Item -LiteralPath $env:THINKTHEN_FIXTURE_PATH).Attributes -band [System.IO.FileAttributes]::ReparsePoint)){throw 'junction prerequisite'}",
    );
    let output = live(&scratch, &listener);
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        fs::read(&target_month).expect("unchanged target bytes"),
        bytes
    );
    assert_eq!(support::descriptor(&target_month), security);
    // Remove only the link we made before Scratch traverses its own tree.
    fs::remove_dir(scratch.usage()).expect("remove owned junction");
}

#[test]
fn recognized_month_and_configuration_symlinks_refuse_without_target_mutation() {
    for configuration in [false, true] {
        let (scratch, listener) = initialized();
        let leaf = if configuration {
            fs::create_dir_all(scratch.config().parent().expect("parent")).expect("parent");
            fs::write(scratch.config(), r#"{"schema":"thinkthen.config/1"}"#).expect("config");
            scratch.config()
        } else {
            month(&scratch)
        };
        let target = scratch.0.join("symlink-target");
        fs::rename(&leaf, &target).expect("move owned leaf");
        let bytes = fs::read(&target).expect("target bytes");
        let descriptor = support::descriptor(&target);
        std::os::windows::fs::symlink_file(&target, &leaf)
            .expect("native symlink prerequisite; inability leaves this proof open");
        let output = live(&scratch, &listener);
        assert_eq!(output.status.code(), Some(5));
        assert_eq!(listener.requests().len(), 1);
        assert_eq!(fs::read(&target).expect("retained target"), bytes);
        assert_eq!(support::descriptor(&target), descriptor);
        fs::remove_file(&leaf).expect("remove owned link");
    }
}
