//! Suppress this addon's caught Rust payload while preserving host diagnostics.

use std::cell::Cell;
use std::sync::Once;

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

static HOOK: Once = Once::new();

fn install() {
    HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !DEPTH.try_with(|depth| depth.get() != 0).unwrap_or(false) {
                previous(info);
            }
        }));
    });
}

struct Restore(usize);

impl Drop for Restore {
    fn drop(&mut self) {
        let _ = DEPTH.try_with(|depth| depth.set(self.0));
    }
}

/// Mark only this addon's Rust work on the current thread.
pub(super) fn owned<T>(body: impl FnOnce() -> T) -> T {
    install();
    let prior = DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = Restore(prior);
    body()
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use super::super::guarded;

    const CHILD: &str = "THINKTHEN_NODE_PANIC_CHILD";
    const STRING: &str = "node-owned-string-payload-marker";
    const DROP: &str = "node-owned-drop-payload-marker";

    struct Exploding;

    impl Drop for Exploding {
        fn drop(&mut self) {
            panic!("{DROP}");
        }
    }

    #[test]
    fn a_caught_panic_stays_out_of_node_diagnostics() {
        if std::env::var_os(CHILD).is_some() {
            child();
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .args([
                "--exact",
                "door::diagnostics::tests::a_caught_panic_stays_out_of_node_diagnostics",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .output()
            .expect("isolated Node proof");
        assert!(output.status.success());
        for stream in [&output.stdout, &output.stderr] {
            let text = String::from_utf8_lossy(stream);
            assert!(!text.contains(STRING), "{text}");
            assert!(!text.contains(DROP), "{text}");
        }
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "host-thread-marker\n"
        );
    }

    fn child() {
        std::panic::set_hook(Box::new(|info| {
            let text = info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
                .unwrap_or(if info.payload().is::<Exploding>() {
                    DROP
                } else {
                    "other panic"
                });
            let _ = std::io::stderr().write_all(format!("{text}\n").as_bytes());
        }));
        let string = guarded(|| panic!("{STRING}"));
        let dropped = guarded(|| std::panic::panic_any(Exploding));
        for envelope in [string, dropped] {
            assert_eq!(
                envelope,
                r#"{"err":{"kind":"defect","message":"defect: the Node binding panicked","retryable":false}}"#
            );
        }
        assert_eq!(guarded(|| Ok("7".to_owned())), r#"{"ok":7}"#);
        let _ = std::thread::spawn(|| panic!("host-thread-marker")).join();
    }
}
