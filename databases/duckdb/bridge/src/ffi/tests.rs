//! One isolated check through the actual product bridge boundaries.

use std::sync::atomic::{AtomicUsize, Ordering};

use super::{BridgeStop, guarded, reply_boundary, run_detached};

const CHILD: &str = "THINKTHEN_DUCKDB_PANIC_CHILD";
const STRING_MARKER: &str = "bridge-string-marker-0254";
const DROP_MARKER: &str = "bridge-drop-marker-0254";

static PRIOR_HOOK_CALLS: AtomicUsize = AtomicUsize::new(0);
static PAYLOAD_DROPS: AtomicUsize = AtomicUsize::new(0);

struct PanickingDrop;

impl Drop for PanickingDrop {
    fn drop(&mut self) {
        PAYLOAD_DROPS.fetch_add(1, Ordering::SeqCst);
        std::panic::panic_any(DROP_MARKER);
    }
}

fn child() {
    std::panic::set_hook(Box::new(|info| {
        PRIOR_HOOK_CALLS.fetch_add(1, Ordering::SeqCst);
        use std::io::Write as _;
        let label = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .unwrap_or("opaque payload");
        let _ = writeln!(std::io::stderr(), "prior-host-hook: {label}");
    }));

    let plain = reply_boundary(|| std::panic::panic_any(STRING_MARKER));
    assert_eq!(
        (plain.status, plain.bytes.is_null(), plain.len),
        (4, true, 0)
    );

    let opaque = reply_boundary(|| std::panic::panic_any(PanickingDrop));
    assert_eq!(
        (opaque.status, opaque.bytes.is_null(), opaque.len),
        (4, true, 0)
    );

    let fixed = guarded(|| -> Result<(), String> { std::panic::panic_any(STRING_MARKER) });
    assert_eq!(
        fixed,
        Err("thinkthen defect: the bridge panicked".to_owned())
    );

    let stop = BridgeStop {
        context: std::ptr::null_mut(),
        interrupted: None,
    };
    let worker = run_detached(stop, |_| -> Result<(), String> {
        std::panic::panic_any(PanickingDrop)
    });
    assert_eq!(
        worker,
        Err("thinkthen defect: the engine worker panicked".to_owned())
    );

    assert_eq!(guarded(|| Ok::<_, String>(7)), Ok(7));
    assert_eq!(run_detached(stop, |_| Ok::<_, String>(9)), Ok(9));
    assert_eq!(PAYLOAD_DROPS.load(Ordering::SeqCst), 0);

    let host = std::thread::spawn(|| std::panic::panic_any("unrelated-host-panic"));
    assert!(host.join().is_err());
    assert_eq!(PRIOR_HOOK_CALLS.load(Ordering::SeqCst), 1);
}

fn parent() {
    let executable = std::env::current_exe().expect("test executable");
    let output = std::process::Command::new(executable)
        .args([
            "--exact",
            "ffi::tests::caught_payloads_stay_in_bridge_scope",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .output()
        .expect("child output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(0),
        "child failed: {stdout}\n{stderr}"
    );
    for marker in [STRING_MARKER, DROP_MARKER] {
        assert!(!stdout.contains(marker), "stdout copied {marker}: {stdout}");
        assert!(!stderr.contains(marker), "stderr copied {marker}: {stderr}");
    }
    assert!(stderr.contains("prior-host-hook: unrelated-host-panic"));
    assert_eq!(stderr.matches("prior-host-hook: ").count(), 1);
}

#[test]
fn caught_payloads_stay_in_bridge_scope() {
    if std::env::var_os(CHILD).is_some() {
        child();
    } else {
        parent();
    }
}
