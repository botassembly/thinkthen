//! Private panic scope for the C++ bridge.

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

thread_local! {
    static BRIDGE_DEPTH: Cell<usize> = const { Cell::new(0) };
}

static HOOK: Once = Once::new();

fn install_hook() {
    HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !BRIDGE_DEPTH
                .try_with(|depth| depth.get() > 0)
                .unwrap_or(false)
            {
                previous(info);
            }
        }));
    });
}

struct BridgeDepth(usize);

impl Drop for BridgeDepth {
    fn drop(&mut self) {
        BRIDGE_DEPTH.with(|depth| depth.set(self.0));
    }
}

fn in_bridge<T>(call: impl FnOnce() -> T) -> T {
    install_hook();
    let prior = BRIDGE_DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = BridgeDepth(prior);
    call()
}

/// Dispose of opaque panic payloads before the marked scope is restored.
/// The outer catch also contains a panic during hook or scope setup.
pub(super) fn caught<T>(call: impl FnOnce() -> T) -> Result<T, ()> {
    match catch_unwind(AssertUnwindSafe(|| {
        in_bridge(|| match catch_unwind(AssertUnwindSafe(call)) {
            Ok(value) => Ok(value),
            Err(payload) => {
                std::mem::forget(payload);
                Err(())
            }
        })
    })) {
        Ok(result) => result,
        Err(payload) => {
            std::mem::forget(payload);
            Err(())
        }
    }
}
