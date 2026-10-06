//! Public API work pauses with the actual writer's directory or temporary file alive.
use super::{OBSERVER, Point};
use crate::test_deadline::child;
use child::ChildEnvironment as _;
use conformance_backend::{Canned, Listener};
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::windows::test_support::{ffi as native, process};

const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":5,"output_tokens":1}}"#;

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "thinkthen-native-writer-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).expect("owned scratch");
        Self(root)
    }
    fn usage(&self) -> PathBuf {
        self.0.join("AppData/Local/thinkthen/usage")
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.0);
    }
}
fn launch(root: &Scratch, listener: &Listener, point: &str) -> process::Owned {
    let mut command = Command::new(std::env::current_exe().expect("test binary"));
    command
        .clear_environment()
        .home(&root.0)
        .args([
            "--exact",
            "windows::checkpoint::tests::usage_child",
            "--ignored",
            "--nocapture",
        ])
        .env("THINKTHEN_BASE_URL", listener.base())
        .env("THINKTHEN_API_KEY", "sk-checkpoint-fixture-only")
        .env("THINKTHEN_WINDOWS_CHECKPOINT_ROOT", &root.0)
        .env("THINKTHEN_WINDOWS_CHECKPOINT_POINT", point)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    process::Owned::spawn(&mut command).expect("owned public API child")
}
fn month(root: &Scratch) -> PathBuf {
    fs::read_dir(root.usage())
        .expect("usage directory")
        .map(|entry| entry.expect("entry").path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .expect("recognized month")
}
fn counts(root: &Scratch, sent: u64) {
    let saved: serde_json::Value =
        serde_json::from_slice(&fs::read(month(root)).expect("month bytes")).expect("month JSON");
    assert_eq!(
        saved,
        serde_json::json!({"schema":"thinkthen.usage/1", "requests_sent":sent, "retries":0, "input_tokens":sent*5, "output_tokens":sent, "cache_answers":0})
    );
}
fn release(root: &Scratch) {
    fs::write(root.0.join("release"), b"1").expect("release owned writer");
}
fn descriptors(paths: &[PathBuf]) -> Vec<String> {
    let mut command = child::powershell(
        "foreach($path in (ConvertFrom-Json $env:THINKTHEN_FIXTURE_PATHS)){ $a=Get-Acl -LiteralPath $path; $sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; if($a.GetOwner([System.Security.Principal.SecurityIdentifier]).Value -ne $sid -or !$a.AreAccessRulesProtected){throw 'owner/protection'}; $r=@($a.GetAccessRules($true,$true,[System.Security.Principal.SecurityIdentifier])); if($r.Count -ne 2){throw 'exact user/System rules'}; $ids=@($r | ForEach-Object {$_.IdentityReference.Value} | Sort-Object -Unique); if($ids.Count -ne 2 -or $ids -notcontains $sid -or $ids -notcontains 'S-1-5-18'){throw 'user/System identities'}; foreach($ace in $r){if($ace.AccessControlType -ne 'Allow' -or $ace.FileSystemRights -ne [System.Security.AccessControl.FileSystemRights]::FullControl -or @($sid,'S-1-5-18') -notcontains $ace.IdentityReference.Value){throw 'private rule'}}; $a.Sddl }",
    )
    .expect("native PowerShell environment");
    command.env(
        "THINKTHEN_FIXTURE_PATHS",
        serde_json::to_string(paths).expect("owned paths"),
    );
    let output = crate::test_deadline::output(&mut command).expect("bounded native ACL inspection");
    assert!(
        output.status.success(),
        "native descriptor prerequisite: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("native descriptor UTF-8")
        .lines()
        .map(|line| line.trim().to_owned())
        .collect()
}

#[test]
fn a_real_writer_retains_its_directory_through_a_synchronized_replacement_attempt() {
    let root = Scratch::new();
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("counted loopback");
    assert!(
        launch(&root, &listener, "initialize")
            .finish()
            .expect("initialize")
            .status
            .success()
    );
    counts(&root, 1);
    let before = fs::read(month(&root)).expect("existing exact bytes");
    let mut child = launch(&root, &listener, "directory");
    process::wait(&root.0.join("ready"));
    assert!(child.alive());
    let saved: (u64, [u8; 16]) =
        serde_json::from_slice(&fs::read(root.0.join("identity")).expect("actual writer ID"))
            .expect("full native identity");
    let independently_opened =
        super::super::files::open_read(&root.usage()).expect("independent directory handle");
    assert_eq!(
        native::identity(&independently_opened).expect("independent full ID"),
        saved
    );
    drop(independently_opened);
    let destination = root.0.join("replacement-attempt");
    let failed = fs::rename(root.usage(), &destination)
        .expect_err("the product writer omits directory delete sharing");
    assert!(
        matches!(failed.raw_os_error(), Some(5) | Some(32)),
        "actual native replacement refusal: {failed:?}"
    );
    assert!(!destination.exists());
    assert_eq!(
        fs::read(month(&root)).expect("retained bytes before release"),
        before
    );
    let retained = super::super::files::open_read(&root.usage()).expect("named retained directory");
    assert_eq!(
        native::identity(&retained).expect("retained native ID"),
        saved
    );
    drop(retained);
    release(&root);
    assert!(child.finish().expect("writer finish").status.success());
    counts(&root, 2);
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn newly_created_temporary_has_explicit_protected_user_system_access_before_publication() {
    let root = Scratch::new();
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("counted loopback");
    let mut child = launch(&root, &listener, "temporary");
    process::wait(&root.0.join("ready"));
    assert!(child.alive());
    let temporary = root.usage().join(".update.tmp");
    assert_eq!(
        fs::read(&temporary).expect("empty newly created temporary"),
        b""
    );
    assert!(
        !fs::read_dir(root.usage())
            .expect("usage directory")
            .any(|entry| entry
                .expect("entry")
                .path()
                .extension()
                .is_some_and(|extension| extension == "json"))
    );
    let saved: (u64, [u8; 16]) =
        serde_json::from_slice(&fs::read(root.0.join("identity")).expect("writer temporary ID"))
            .expect("full ID");
    let opened = super::super::files::open_read(&temporary).expect("independent temporary");
    assert_eq!(
        native::identity(&opened).expect("independent temporary ID"),
        saved
    );
    drop(opened);
    let before = descriptors(&[temporary.clone(), root.usage(), root.usage().join(".lock")]);
    assert_eq!(
        before.len(),
        3,
        "inspect temporary, usage directory and lock"
    );
    release(&root);
    assert!(
        child
            .finish()
            .expect("publication and finish")
            .status
            .success()
    );
    counts(&root, 1);
    assert_eq!(
        descriptors(&[month(&root)]),
        vec![before.first().expect("temporary descriptor").clone()]
    );
    assert!(!temporary.exists());
    assert_eq!(listener.requests().len(), 1);
}

#[test]
#[ignore = "subprocess-only synchronized public API writer"]
fn usage_child() {
    let root = PathBuf::from(
        std::env::var_os("THINKTHEN_WINDOWS_CHECKPOINT_ROOT").expect("owned marker root"),
    );
    let selected = match std::env::var("THINKTHEN_WINDOWS_CHECKPOINT_POINT")
        .expect("point")
        .as_str()
    {
        "directory" => Some(Point::Directory),
        "temporary" => Some(Point::Temporary),
        "initialize" => None,
        _ => panic!("unknown fixture point"),
    };
    if let Some(selected) = selected {
        let root = root.clone();
        *OBSERVER.lock().expect("observer") = Some((
            selected,
            Box::new(move |file| {
                let identity = native::identity(file).expect("actual product writer handle");
                fs::write(
                    root.join("identity"),
                    serde_json::to_vec(&identity).expect("identity JSON"),
                )
                .expect("owned identity marker");
                let mut ready = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(root.join("ready"))
                    .expect("exclusive readiness marker");
                ready.write_all(b"1").expect("ready byte");
                ready.flush().expect("ready flush");
                process::wait(&root.join("release"));
            }),
        ));
    }
    let engine = crate::EngineBuilder::from_env()
        .expect("public seeded builder")
        .no_cache()
        .model("local-1")
        .expect("model")
        .build()
        .expect("public engine");
    let question = crate::Question::decide("Accepted?")
        .expect("question")
        .cut();
    let answer = engine
        .decide(&question, "fixture evidence")
        .expect("canned public answer");
    assert_eq!(answer.into_value(), crate::Answer::Yes);
    if selected.is_some() {
        // The writer may pause longer than the public flush deadline while
        // the parent inspects its native descriptor. Keep the host alive.
        process::wait(&root.join("release"));
    }
    engine.finish_usage();
}
