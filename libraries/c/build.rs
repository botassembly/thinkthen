//! Name the shared library's soname `libthinkthen.so.0` (ADR 0047 item 6),
//! or its install name on macOS, whatever file name Cargo gives it.

fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let name = if target == "macos" {
        "-Wl,-install_name,@rpath/libthinkthen.0.dylib"
    } else {
        "-Wl,-soname,libthinkthen.so.0"
    };
    println!("cargo::rustc-cdylib-link-arg={name}");
}
