//! Name the shared library's soname `libthinkthen.so.0` (ADR 0047 item 6),
//! or its install name on macOS, whatever file name Cargo gives it.

fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let name = match target.as_str() {
        "macos" => "-Wl,-install_name,@rpath/libthinkthen.0.dylib",
        // A Windows DLL has no soname, and the MSVC linker refuses `-Wl` (ticket 0373).
        "windows" => return,
        _ => "-Wl,-soname,libthinkthen.so.0",
    };
    println!("cargo::rustc-cdylib-link-arg={name}");
}
