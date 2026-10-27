#![cfg(test)]
//! Command trust, file permissions and terminal controls at the process boundary.
#![cfg(feature = "command")]
#![cfg(unix)]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;
use serde_json::json;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn shell_steps_require_consent_and_pass_only_named_backend_keys() {
    let directory = tempfile::tempdir().unwrap();
    let toolbox = directory.path().join("thinkthen.json");
    let rewrite = |env: Vec<&str>| {
        let mut step = json!({"shell":["/bin/sh","-c","printf ran > marker; system=false; openai=false; test -z \"$THINKTHEN_API_KEY\" || system=true; test -z \"$OPENAI_API_KEY\" || openai=true; printf '{\"system\":%s,\"openai\":%s}\\n' \"$system\" \"$openai\""]});
        if !env.is_empty() {
            step["env"] = json!(env);
        }
        fs::write(
            &toolbox,
            json!({"entries":{"inspect":{"run":[step]}}}).to_string(),
        )
        .unwrap();
    };
    rewrite(Vec::<&str>::new());
    let refused = support::command()
        .current_dir(directory.path())
        .args(["run", "inspect", "--backend", "stub"])
        .env("THINKTHEN_API_KEY", "secret-system")
        .env("OPENAI_API_KEY", "secret-openai")
        .output()
        .unwrap();
    assert_eq!(refused.status.code(), Some(5));
    assert!(refused.stdout.is_empty());
    assert!(!directory.path().join("marker").exists());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("--allow-shell"));
    for (names, expected) in [
        (vec![], json!({"system":false,"openai":false})),
        (
            vec!["OPENAI_API_KEY"],
            json!({"system":false,"openai":true}),
        ),
    ] {
        rewrite(names.clone());
        let output = support::command()
            .current_dir(directory.path())
            .args(["run", "inspect", "--backend", "stub", "--allow-shell"])
            .env("THINKTHEN_API_KEY", "secret-system")
            .env("OPENAI_API_KEY", "secret-openai")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
            expected
        );
        let notice = String::from_utf8_lossy(&output.stderr);
        assert_eq!(notice.contains("OPENAI_API_KEY"), !names.is_empty());
        assert!(!notice.contains("secret-"));
    }
}

#[test]
fn new_output_preserves_source_restrictions_under_umask() {
    let directory = tempfile::tempdir().unwrap();
    for (mode, mask, expected) in [
        (0o600, "022", 0o600),
        (0o666, "077", 0o600),
        (0o640, "022", 0o640),
    ] {
        let input = directory.path().join(format!("{mode}-{mask}.csv"));
        let output = input.with_extension("out.csv");
        fs::write(&input, "evidence\nready\n").unwrap();
        fs::set_permissions(&input, fs::Permissions::from_mode(mode)).unwrap();
        let mut result = support::command();
        result
            .args([
                "decide",
                "Ready?",
                "--backend",
                "stub",
                "--column",
                "evidence",
                "--into",
                "ready",
                "--output",
            ])
            .arg(&output)
            .arg(&input);
        // Set umask in a child shell so the test process and parallel tests stay isolated.
        let mut shell = std::process::Command::new("/bin/sh");
        let executable = result.get_program().to_owned();
        let arguments: Vec<_> = result.get_args().map(std::ffi::OsStr::to_owned).collect();
        shell.env_clear().envs(
            result
                .get_envs()
                .filter_map(|(key, value)| value.map(|value| (key, value))),
        );
        let actual = shell
            .args(["-c", &format!("umask {mask}; exec \"$@\""), "mode-test"])
            .arg(executable)
            .args(arguments)
            .output()
            .unwrap();
        assert!(
            actual.status.success(),
            "{}",
            String::from_utf8_lossy(&actual.stderr)
        );
        assert_eq!(
            fs::metadata(&output).unwrap().permissions().mode() & 0o777,
            expected
        );
    }
}

#[test]
fn terminal_text_escapes_controls_while_pipes_keep_evidence_bytes() {
    use std::process::Command;
    let server = loopback::Loopback::answering(
        |_| json!({"type":"choice","probabilities":{"u1":0.9,"none":0.1}}),
    )
    .unwrap();
    let mut command = support::command();
    command
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", loopback::KEY);
    let script = r#"
import errno, os, pty, subprocess, sys
binary = sys.argv[1]
text = 'safe\x1b[31m\x07\x85\u202eend\n'
args = ([binary, 'rank', 'Order?', '--backend', 'stub', '--from', 'lines', '--level', 'low', '--level', 'high'] if sys.argv[2] == 'rank' else [binary, 'find', 'Which?', '--from', 'lines'])
plain = subprocess.run(args, input=text.encode(), stdout=subprocess.PIPE, stderr=subprocess.PIPE)
assert plain.returncode == 0, plain.stderr
assert plain.stdout == text.encode(), plain.stdout
master, slave = pty.openpty()
child = subprocess.Popen(args, stdin=subprocess.PIPE, stdout=slave, stderr=subprocess.PIPE)
os.close(slave)
child.stdin.write(text.encode()); child.stdin.close()
chunks = []
while True:
    try:
        chunk = os.read(master, 4096)
        if not chunk: break
        chunks.append(chunk)
    except OSError as error:
        if error.errno == errno.EIO: break
        raise
os.close(master)
assert child.wait() == 0, child.stderr.read()
actual = b''.join(chunks)
assert actual == b'safe\\u001b[31m\\u0007\\u0085\\u202eend\r\n', actual
"#;
    for surface in ["rank", "find"] {
        let result = Command::new("python3")
            .env_clear()
            .envs(
                command
                    .get_envs()
                    .filter_map(|(key, value)| value.map(|value| (key, value))),
            )
            .args(["-c", script])
            .arg(command.get_program())
            .arg(surface)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn chosen_labels_escape_terminal_controls() {
    let label = "safe\u{1b}[31m\u{7}\u{85}\u{202e}end";
    let server = loopback::Loopback::answering(
        move |_| json!({"type":"choice","probabilities":{label:0.9,"other":0.1}}),
    )
    .unwrap();
    let command = support::command();
    let script = r#"
import errno, os, pty, subprocess, sys
master, slave = pty.openpty()
label = 'safe\x1b[31m\x07\x85\u202eend'
child = subprocess.Popen([sys.argv[1], 'choose', 'Which?', '--choice', label, '--choice', 'other', '--url', sys.argv[2]], stdin=subprocess.DEVNULL, stdout=slave, stderr=subprocess.PIPE)
os.close(slave)
chunks = []
while True:
    try:
        chunk = os.read(master, 4096)
        if not chunk: break
        chunks.append(chunk)
    except OSError as error:
        if error.errno == errno.EIO: break
        raise
os.close(master)
assert child.wait() == 0, child.stderr.read()
assert b''.join(chunks) == b'safe\\u001b[31m\\u0007\\u0085\\u202eend\r\n', chunks
"#;
    let result = std::process::Command::new("python3")
        .env_clear()
        .envs(
            command
                .get_envs()
                .filter_map(|(key, value)| value.map(|value| (key, value))),
        )
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .args(["-c", script])
        .arg(command.get_program())
        .arg(server.base())
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
