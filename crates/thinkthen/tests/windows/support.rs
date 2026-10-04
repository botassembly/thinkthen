use crate::child::ChildEnvironment as _;
use crate::process::Owned;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":5,"output_tokens":1}}"#;
pub(crate) const ADVICE: &str = "Make it owned by your user and restrict its Windows access permissions to your user and Windows SYSTEM, or move it aside.";
pub(crate) struct Scratch(pub(crate) PathBuf);
impl Scratch {
    pub(crate) fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "thinkthen-windows-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).expect("disposable folder");
        Self(path)
    }
    pub(crate) fn usage(&self) -> PathBuf {
        self.0.join("AppData/Local/thinkthen/usage")
    }
    pub(crate) fn config(&self) -> PathBuf {
        self.0.join("AppData/Roaming/thinkthen/config.json")
    }
    pub(crate) fn command(&self, arguments: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .clear_environment()
            .home(&self.0)
            .args(arguments)
            .env("THINKTHEN_API_KEY", "sk-fixture-only")
            .env("THINKTHEN_BATCH", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command
    }
    pub(crate) fn run(&self, arguments: &[&str]) -> Output {
        self.start(arguments, b"fixture evidence")
            .finish()
            .expect("bounded CLI")
    }
    pub(crate) fn start(&self, arguments: &[&str], input: &[u8]) -> Owned {
        let mut child = Owned::spawn(&mut self.command(arguments)).expect("isolated CLI");
        let mut stdin = child
            .0
            .as_mut()
            .expect("child")
            .stdin
            .take()
            .expect("stdin");
        stdin.write_all(input).expect("fixture input");
        drop(stdin);
        child
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        // Restore only our disposable tree so denial plants cannot prevent cleanup.
        let _restored = std::panic::catch_unwind(|| {
            powershell(
                &self.0,
                "Get-ChildItem -LiteralPath $env:THINKTHEN_FIXTURE_PATH -Recurse -Force | ForEach-Object { try { $acl=Get-Acl -LiteralPath $_.FullName; $acl.SetAccessRuleProtection($false,$false); Set-Acl -LiteralPath $_.FullName -AclObject $acl } catch {} }",
            )
        });
        let _removed = fs::remove_dir_all(&self.0);
    }
}
pub(crate) fn powershell(path: &Path, script: &str) -> String {
    let mut command = Command::new("powershell.exe");
    command
        .clear_environment()
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("$ErrorActionPreference='Stop'; {script}"),
        ])
        .env("THINKTHEN_FIXTURE_PATH", path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let output = Owned::spawn(&mut command)
        .expect("native PowerShell prerequisite")
        .finish()
        .expect("bounded PowerShell");
    assert!(
        output.status.success(),
        "native fixture prerequisite failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("PowerShell UTF-8")
        .trim()
        .to_owned()
}
pub(crate) fn descriptor(path: &Path) -> String {
    powershell(
        path,
        "(Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH).Sddl",
    )
}
pub(crate) fn plant(path: &Path, extra: &str, foreign_owner: bool) {
    let owner = if foreign_owner { "BA" } else { "$sid" };
    powershell(
        path,
        &format!(
            "$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; $sddl=\"O:{owner}D:P(A;;FA;;;$sid)(A;;FA;;;SY){extra}\"; $acl=Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH; $acl.SetSecurityDescriptorSddlForm($sddl); Set-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH -AclObject $acl"
        ),
    );
}
pub(crate) fn assert_private(path: &Path) {
    let inspection = powershell(
        path,
        "$a=Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH; $sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; if($a.GetOwner([System.Security.Principal.SecurityIdentifier]).Value -ne $sid -or !$a.AreAccessRulesProtected){throw 'owner/protection'}; foreach($r in $a.GetAccessRules($true,$true,[System.Security.Principal.SecurityIdentifier])){if($r.AccessControlType -eq 'Allow' -and $r.IdentityReference.Value -ne $sid -and $r.IdentityReference.Value -ne 'S-1-5-18'){throw 'foreign grant'}}; 'private'",
    );
    assert_eq!(inspection, "private");
}
pub(crate) fn status(scratch: &Scratch) -> serde_json::Value {
    let output = scratch.run(&["status", "--json"]);
    assert_eq!(output.status.code(), Some(0));
    serde_json::from_slice(&output.stdout).expect("status JSON")
}
