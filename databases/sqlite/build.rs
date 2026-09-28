//! Give the macOS extension a portable install name in its release archive.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo::rustc-cdylib-link-arg=-Wl,-install_name,@rpath/libthinkthen0.dylib");
    }
}
