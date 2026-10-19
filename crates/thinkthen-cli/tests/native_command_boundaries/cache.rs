use super::child;
use child::ChildEnvironment as _;
use conformance_backend::{Canned, Listener};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const YES: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const NO: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.1}}}"#;
const BODY: &[u8] = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#;

struct Folder(PathBuf);

impl Folder {
    fn new() -> std::io::Result<Self> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "native-cache-policy-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(Self(path))
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _removed = std::fs::remove_dir_all(&self.0);
    }
}

fn command(
    listener: &Listener,
    folder: &Folder,
    options: &[&str],
) -> std::io::Result<std::process::Output> {
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .clear_environment()
        .isolated_home(&folder.0)
        .env("THINKTHEN_API_KEY", "fixture-storage-policy")
        .args([
            "decide",
            "Refund?",
            "--model",
            "fixed",
            "--url",
            listener.base(),
            "--facts",
        ])
        .args(options)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::other("missing stdin"))?
        .write_all(b"Refund me.")?;
    child.wait_with_output()
}

#[test]
fn refresh_evicts_only_after_a_valid_nonstorable_reply_and_failed_refresh_keeps_the_old_answer() {
    for failed in [false, true] {
        let folder = Folder::new().unwrap();
        let listener = Listener::serving(vec![
            Canned::ok(YES),
            if failed {
                Canned::status(400, "{}")
            } else {
                Canned::ok(NO).asking("Cache-Control", "no-store")
            },
            Canned::ok(NO),
        ])
        .unwrap();
        let cache = folder.0.join("cache");
        let path = cache.to_str().unwrap();
        let first = command(&listener, &folder, &["--cache", path]).unwrap();
        assert_eq!(first.status.code(), Some(0));
        assert_eq!(first.stdout, b"true\n");
        let refreshed = command(&listener, &folder, &["--cache", path, "--refresh-cache"]).unwrap();
        assert_eq!(refreshed.status.code(), Some(if failed { 4 } else { 1 }));
        let again = command(&listener, &folder, &["--cache", path]).unwrap();
        assert_eq!(
            again.stdout,
            if failed {
                b"true\n".as_slice()
            } else {
                b"false\n".as_slice()
            }
        );
        let requests = listener.requests();
        assert_eq!(requests.len(), if failed { 2 } else { 3 });
        assert_eq!(requests[0].header("Cache-Control"), None);
        assert_eq!(requests[1].header("Cache-Control"), Some("no-cache"));
        assert!(requests.iter().all(|request| request.body == BODY));
    }
}

#[test]
fn command_record_refusal_reports_local_exit_and_final_started_facts_without_repeating_headers() {
    let folder = Folder::new().unwrap();
    let listener = Listener::answering(|_| {
        Canned::ok(YES)
            .asking("Cache-Control", "no-store")
            .asking("X-Private-Marker", "secret-sentinel")
    })
    .unwrap();
    let path = folder.0.join("record");
    let output = command(&listener, &folder, &["--record", path.to_str().unwrap()]).unwrap();
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    let lines: Vec<_> = stderr.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(
        lines[0],
        "thinkthen: the reply forbids storage, so the recording cannot be written"
    );
    let facts: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(facts["requests_sent"], 1);
    assert_eq!(facts["stopped"]["cause"], "local");
    assert!(facts["call_id"].as_str().is_some());
    assert!(!stderr.contains("secret-sentinel"));
    assert_eq!(listener.count(), 1);
    assert!(!path.join("thinkthen.sqlite").exists());
}
