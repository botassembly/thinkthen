//! Saved image fixture and isolated folders shared by public and ownership tests.
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::Question;

pub(crate) const RED: &[u8] = include_bytes!("../../../../specification/fixtures/images/red.png");
pub(crate) const LIQUID_DECIDE: &str =
    include_str!("../../../../specification/fixtures/images/liquid-decide-reply.json");
#[cfg(test)]
pub(crate) fn question() -> Question {
    Question::decide("Is red visible?")
        .expect("valid saved fixture or loopback configuration")
        .cut()
}
static NEXT: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
pub(crate) fn folder() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "thinkthen-native-images-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&path).expect("valid saved fixture or loopback configuration");
    path
}
