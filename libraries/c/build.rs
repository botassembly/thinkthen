//! Name the shared library's soname `libthinkthen.so.0` (ADR 0047 item 6),
//! or its install name on macOS, and hide dependency exports where supported.

#[path = "tests/door/header.rs"]
mod header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=include/thinkthen.h");
    println!("cargo::rerun-if-changed=tests/door/header.rs");
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let name = match target.as_str() {
        "macos" => {
            let input = std::fs::read_to_string("include/thinkthen.h")?;
            let names = header::declarations(&input)?;
            let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default())
                .join("thinkthen.exports");
            std::fs::write(&out, format!("_{}\n", names.join("\n_")))?;
            println!(
                "cargo::rustc-cdylib-link-arg=-Wl,-exported_symbols_list,{}",
                out.display()
            );
            "-Wl,-install_name,@rpath/libthinkthen.0.dylib"
        }
        "windows" => {
            if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() != "msvc" {
                return Ok(());
            }
            let input = std::fs::read_to_string("include/thinkthen.h")?;
            let names = header::declarations(&input)?;
            let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default())
                .join("thinkthen.def");
            std::fs::write(
                &out,
                format!(
                    "LIBRARY thinkthen.dll\nEXPORTS\n    {}\n",
                    names.join("\n    ")
                ),
            )?;
            println!("cargo::rustc-cdylib-link-arg=/DEF:{}", out.display());
            return Ok(());
        }
        "linux" => {
            println!("cargo::rustc-cdylib-link-arg=-Wl,--exclude-libs,ALL");
            "-Wl,-soname,libthinkthen.so.0"
        }
        _ => return Ok(()),
    };
    println!("cargo::rustc-cdylib-link-arg={name}");
    Ok(())
}
