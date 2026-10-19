//! Binary behavior belongs to the command crate; library behavior stays with the library.
#[path = "../../thinkthen/src/test_deadline/child.rs"]
mod child;
fn folder() -> std::path::PathBuf {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let path = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "native-command-boundaries-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir(&path).unwrap();
    path
}

#[path = "native_command_boundaries/cache.rs"]
mod cache;
#[path = "native_command_boundaries/images.rs"]
mod images;
#[path = "native_command_boundaries/named.rs"]
mod named;
#[path = "native_command_boundaries/recognize.rs"]
mod recognize;
