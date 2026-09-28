//! Suppress this binding's owned panic text while preserving host callbacks.

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

/// Mark only work this binding owns on the current thread.
pub(crate) fn owned<T>(body: impl FnOnce() -> T) -> T {
    install();
    let prior = DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = Restore(prior);
    body()
}

/// Let an interpreter or foreign producer run under the previous hook.
pub(crate) fn host<T>(body: impl FnOnce() -> T) -> T {
    let prior = DEPTH.with(|depth| depth.replace(0));
    let _restore = Restore(prior);
    body()
}
