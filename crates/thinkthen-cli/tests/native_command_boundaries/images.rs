use super::{child, folder};
use child::ChildEnvironment as _;
use conformance_backend::{Canned, Listener};
use std::process::{Command, Stdio};
const CLEF: &str =
    include_str!("../../../../specification/fixtures/images/local/clef-profile.json");
const REPLY: &str = r#"{"model":"local","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":123,"output_tokens":0}}"#;

#[test]
fn cli_profile_uses_the_existing_file_door_and_refuses_unknown_ids_without_sends() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let profile = place.join("profile.json");
    let image = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images/red.png");
    let call = || {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .clear_environment()
            .home(&place)
            .env("THINKTHEN_API_KEY", "PRIVATE_LOCAL_KEY")
            .args([
                "decide",
                "Is red visible?",
                "--backend",
                "llamacpp",
                "--model",
                "clef-local-0036",
                "--url",
                listener.base(),
                "--profile",
            ])
            .arg(&profile)
            .arg("--image")
            .arg(&image)
            .args(["--no-cache", "--max-retries", "0"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        child::wait::finish(command.spawn().unwrap(), "local image profile CLI").unwrap()
    };
    std::fs::write(
        &profile,
        CLEF.replace("clef-llamacpp-v0.6.0-0036", "unknown"),
    )
    .unwrap();
    let refused = call();
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(listener.count(), 0);
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("PRIVATE_LOCAL_KEY"));
    std::fs::write(&profile, CLEF).unwrap();
    let admitted = call();
    assert_eq!(
        admitted.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&admitted.stderr)
    );
    assert_eq!(admitted.stdout, b"true\n");
    assert_eq!(listener.count(), 1);
    assert!(!String::from_utf8_lossy(&admitted.stderr).contains("PRIVATE_LOCAL_KEY"));
    std::fs::remove_dir_all(place).unwrap();
}
