//! macOS links an extension before PostgreSQL loads it, so the server's
//! symbols resolve at load time (ticket 0336). This is the flag `cargo pgrx
//! new` writes to `.cargo/config.toml`; a build script keeps it when the
//! package step sets `RUSTFLAGS`, which replaces that file's flags.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo::rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup");
    }
}
