#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise native settings construction in an isolated child process"
)]
//! SQL rejects unsupported client-lifetime limits before backend resolution.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
#[test]
fn sql_rejects_even_zero_and_never_inherits_the_command_allowance() {
    for value in ["0", "1", "null"] {
        let error =
            thinkthen::native::sql_settings(&format!("{{\"max_requests\":{value}}}")).unwrap_err();
        assert_eq!(error.code(), thinkthen::Code::UsageSettings);
        assert!(error.message().contains("max_requests"));
    }
    let engine = thinkthen::native::sql_engine_unrestricted("{\"backend\":\"stub\"}", |name| {
        assert_ne!(name, "THINKTHEN_MAX_REQUESTS");
        None
    })
    .unwrap();
    assert_eq!(engine.resolved_settings().request_limit(), None);
}

#[test]
fn c_null_settings_inherit_only_the_documented_environment_allowance() {
    const CHILD: &str = "THINKTHEN_REQUEST_LIMIT_TEST_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let server = loopback::Loopback::noul(0.9).unwrap();
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command.env_clear();
        for name in [
            "SystemRoot",
            "COMSPEC",
            "PATHEXT",
            "PATH",
            "TMP",
            "TEMP",
            "LLVM_PROFILE_FILE",
        ] {
            if let Some(value) = std::env::var_os(name) {
                command.env(name, value);
            }
        }
        let output = command
            .args([
                "--exact",
                "c_null_settings_inherit_only_the_documented_environment_allowance",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .env("THINKTHEN_BACKEND", "systemone")
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", loopback::KEY)
            .env("THINKTHEN_MAX_REQUESTS", "0")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(server.count(), 1);
        return;
    }
    use std::ptr;
    use thinkthen_ffi::*;
    // SAFETY: initialized exclusive slots and live handles until their sole cleanup.
    unsafe {
        let mut client = ptr::null_mut();
        assert_eq!(
            thinkthen_client_open(ptr::null(), &mut client, ptr::null_mut()),
            0
        );
        let mut question = ptr::null_mut();
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, ptr::null_mut()),
            0
        );
        let mut row: HostRow = std::mem::zeroed();
        row.struct_size = std::mem::size_of::<HostRow>() as u32;
        assert_eq!(
            thinkthen_run_one(
                client,
                question,
                b"evidence".as_ptr(),
                8,
                0,
                &mut row,
                ptr::null_mut()
            ),
            -thinkthen::Code::LocalRequestLimit.abi()
        );
        thinkthen_row_free(&mut row);
        thinkthen_client_free(client);
        let mut settings = ptr::null_mut();
        assert_eq!(thinkthen_settings_new(&mut settings, ptr::null_mut()), 0);
        assert_eq!(
            thinkthen_settings_set_backend(
                settings,
                thinkthen::BackendName::SystemOne.abi(),
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_client_open(settings, &mut client, ptr::null_mut()),
            0
        );
        assert_eq!(
            thinkthen_run_one(
                client,
                question,
                b"evidence".as_ptr(),
                8,
                0,
                &mut row,
                ptr::null_mut()
            ),
            0
        );
        thinkthen_row_free(&mut row);
        thinkthen_question_free(question);
        thinkthen_client_free(client);
        thinkthen_settings_free(settings);
        let explicit = thinkthen::Settings::default()
            .with_backend(thinkthen::BackendName::Stub)
            .engine(|name| std::env::var(name).ok())
            .unwrap();
        assert_eq!(explicit.resolved_settings().request_limit(), None);
    }
}
