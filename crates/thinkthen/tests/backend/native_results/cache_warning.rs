//! Model freshness warnings are safe text and occur only on excluded online hits.
use super::*;
#[test]
fn a_held_model_mismatch_warns_once_without_echoing_the_stored_model_or_credential() {
    let next = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let sent = next.clone();
    let listener = Listener::answering(move |_| {
        let model = if sent.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 0 {
            "stored-private-model"
        } else {
            "fixed"
        };
        Canned::ok(&format!(
            r#"{{"model":"{model}","answers":{{"q1":{{"type":"noul","noul":0.7}}}}}}"#
        ))
    })
    .unwrap();
    let root = crate::input_sources::folder("complete-command-model-warning").unwrap();
    let run = |question| {
        spawn(
            &[
                "decide",
                question,
                "--details",
                "--cache",
                root.to_str().unwrap(),
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--max-retries",
                "0",
            ],
            &[("THINKTHEN_API_KEY", "warning-private")],
            b"Text.",
        )
        .unwrap()
    };
    let first = run("Good?");
    assert_eq!(
        first.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stderr.is_empty());
    let second = run("Good?");
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(
        second.stderr,
        b"thinkthen: warning: a held answer names a different model and cannot be reused online\n"
    );
    withheld(&second, "warning-private");
    assert!(!String::from_utf8_lossy(&second.stdout).contains("stored-private-model"));
    let hit = run("Good?");
    assert!(hit.stderr.is_empty());
    assert_eq!(hit.status.code(), Some(0));
    assert_eq!(listener.count(), 2);
    let unrelated = run("Different?");
    assert_eq!(unrelated.status.code(), Some(0));
    assert!(unrelated.stderr.is_empty());
    assert_eq!(listener.count(), 3);
}
