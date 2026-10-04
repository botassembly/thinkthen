//! Unix layout, localization, compiler and export proof.
use super::*;

/// The shared library and its link name as `release-pack` lays them out, and
/// the prefix Mach-O puts before every name (ticket 0351).
const MAC: bool = cfg!(target_os = "macos");
const DL: &str = if MAC { "dylib" } else { "so" };
const SONAME: &str = if MAC {
    "libthinkthen.0.dylib"
} else {
    "libthinkthen.so.0"
};
const PREFIX: &str = if MAC { "_" } else { "" };

/// The built door as a release archive lays it out: `libthinkthen.so`, its
/// soname link `libthinkthen.so.0`, and `libthinkthen.a`, in one folder. On
/// macOS the shared library is `libthinkthen.dylib`, linked as
/// `libthinkthen.0.dylib`.
pub(super) fn archive() -> &'static Path {
    static FOLDER: OnceLock<PathBuf> = OnceLock::new();
    FOLDER.get_or_init(|| {
        let built = child::command(env!("CARGO"), child::CARGO)
            .args(["build", "--locked", "--offline", "--lib"])
            .arg("--message-format=json-render-diagnostics")
            .current_dir(crate_dir())
            .output()
            .expect("cargo ran");
        let said = String::from_utf8_lossy(&built.stderr);
        assert!(built.status.success(), "{said}");
        assert!(!said.contains("collision"), "the build warned: {said}");
        // Only `cargo doc` warns when the door's library shares the engine's
        // crate name, so the build's own report of its files is checked.
        let files: Vec<PathBuf> = text(&built.stdout)
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter(|message| message["target"]["name"] == "thinkthen_c")
            .flat_map(|message| message["filenames"].as_array().cloned().unwrap_or_default())
            .filter_map(|file| file.as_str().map(PathBuf::from))
            .collect();
        let folder = scratch("archive");
        let built = |from: &str| {
            files
                .iter()
                .find(|file| file.file_name().is_some_and(|name| name == from))
                .expect("the build reported the door's library under its own name")
        };
        std::fs::copy(
            built(&format!("libthinkthen_c.{DL}")),
            folder.join(format!("libthinkthen.{DL}")),
        )
        .expect("a built library");
        // A release archive holds the localized static library, as `release-pack` writes it.
        let localized = child::command("sh", &[])
            .arg(crate_dir().join("localize.sh"))
            .arg(built("libthinkthen_c.a"))
            .arg(folder.join("libthinkthen.a"))
            .output()
            .expect("sh ran");
        assert!(localized.status.success(), "{}", text(&localized.stderr));
        std::os::unix::fs::symlink(format!("libthinkthen.{DL}"), folder.join(SONAME))
            .expect("the soname link");
        folder
    })
}

/// Compile one C program against the archive, under AddressSanitizer, once.
/// Three tests share `driver.c`, and a relink fails another test's launch.
pub(super) fn compile(source: &Path) -> PathBuf {
    static BUILT: Mutex<BTreeMap<PathBuf, PathBuf>> = Mutex::new(BTreeMap::new());
    let mut built = BUILT.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(binary) = built.get(source) {
        return binary.clone();
    }
    let name = source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("a name");
    let binary = root().join(format!("c-{name}"));
    let folder = archive();
    let linked = child::command("cc", &[])
        .args([
            "-std=c11",
            "-D_GNU_SOURCE",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pthread",
            "-g",
        ])
        .args(["-fsanitize=address", "-fno-omit-frame-pointer", "-I"])
        .arg(crate_dir().join("include"))
        .arg(source)
        .arg("-o")
        .arg(&binary)
        .arg("-L")
        .arg(folder)
        .arg("-lthinkthen")
        .arg(format!("-Wl,-rpath,{}", folder.display()))
        .output()
        .expect("cc ran");
    assert!(
        linked.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&linked.stderr)
    );
    built.insert(source.to_owned(), binary.clone());
    binary
}

/// R2-26 and ADR 0047 item 6: the soname, the exported symbols, and the
/// version macros match the header.
#[test]
fn the_library_carries_its_soname_and_exactly_the_header_symbols() {
    let library = archive().join("libthinkthen.so");
    let dynamic = child::command("readelf", &[])
        .arg("-d")
        .arg(&library)
        .output()
        .expect("readelf");
    assert!(
        text(&dynamic.stdout).contains("Library soname: [libthinkthen.so.0]"),
        "{}",
        text(&dynamic.stdout)
    );
    let header =
        std::fs::read_to_string(crate_dir().join("include/thinkthen.h")).expect("the header");
    let exported = child::command("nm", &[])
        .args(["-D", "--defined-only"])
        .arg(&library)
        .output()
        .expect("nm");
    // ADR 0111: a bundled SQLite stays private, so no `sqlite3_` name leaks.
    let leaked = text(&exported.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().nth(2))
        .filter(|name| name.starts_with("sqlite3_"))
        .count();
    assert_eq!(leaked, 0, "the door exports sqlite3_ symbols");
    let mut symbols: Vec<String> = text(&exported.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().nth(2))
        .filter(|name| name.starts_with("thinkthen_"))
        .map(str::to_owned)
        .collect();
    symbols.sort();
    assert_eq!(symbols, declared(&header));
}

/// ADR 0111's 2026-09-30 amendment: the static library defines exactly the
/// header's functions as global names. The bundled SQLite's `sqlite3_` names
/// and Rust's runtime names stay local, so a program can link its own SQLite
/// or a second Rust static library beside it. Gates every release.
#[test]
fn the_static_library_exports_exactly_the_header_symbols() {
    let exported = child::command("nm", &[])
        .args(["-g", "--defined-only"])
        .arg(archive().join("libthinkthen.a"))
        .output()
        .expect("nm");
    let mut names: Vec<String> = text(&exported.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().nth(2))
        .map(|name| name.strip_prefix(PREFIX).unwrap_or(name))
        .map(str::to_owned)
        .collect();
    assert!(
        !names.iter().any(|name| name.starts_with("sqlite3_")),
        "the static library exports sqlite3_ symbols"
    );
    names.sort();
    let header =
        std::fs::read_to_string(crate_dir().join("include/thinkthen.h")).expect("the header");
    assert_eq!(names, declared(&header));
}
