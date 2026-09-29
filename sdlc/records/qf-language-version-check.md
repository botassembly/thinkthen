# Quick Fix: language version checking

The original failure on main `24e48a22d` was `versions` reporting eleven nonexistent wrapper `Cargo.toml` files and `versions --self-test` raising `FileNotFoundError` at the first copy. The registry listed landed PHP, C#, JVM, Dart, Swift, Zig, Go, C++, Ada, Objective-C and COBOL wrappers whose actual package formats are not Cargo.

## Metadata and retained checks

The checker now maps the nine landed Cargo surfaces to their exact manifests, including the R nested crate and the three existing database manifests. It reads each non-Cargo package's actual required metadata. C# uses `ThinkThen.csproj` plus its static `.nuspec` version; JVM uses `pom.xml`; Dart uses its package and both Flutter pubspec versions; Zig uses `build.zig.zon`; C++ uses `CMakeLists.txt`; Objective-C and COBOL use their `source-package.json` versions. PHP Composer, Swift Package.swift, Go go.mod and Ada project files carry package identity but no product version field; their releases derive version from the tag or paired C artifact. No version field was added to them. The copied Swift and Objective-C C headers retain all five existing C-header version checks.

The root crate version, other original static copies, every tracked readable Cargo lock package entry, `--tag vX.Y.Z`, and byte equality of the two installer copies remain checked. Unknown landed surfaces, missing or malformed required metadata, and unreadable locks now report a failure. `--set` reads and validates the registry, metadata, lock contents, installer equality, and every version slot before writing any file. The existing refused-update byte comparison is retained. The check does not parse or build any language package, prove release archive provenance, or qualify SQL/DataFrame hosts.

## Focused proof

`CARGO_NET_OFFLINE=true RUSTC_WRAPPER= CARGO_BUILD_RUSTC_WRAPPER= CARGO_BUILD_JOBS=2 python3 sdlc/scripts/versions` exited 0 with `versions: 64 places read 0.0.1`. The same environment with `--self-test` exited 0, 21/21 copied-source cases. The new cases pin C# package mismatch, nested Dart mismatch, copied-header mismatch, missing versionless PHP metadata, an unknown landed surface, and unchanged bytes after refused updates for missing Swift metadata, malformed Cargo package name, malformed lock, a valid lock whose package fields defeat the updater, and differing installer. Original Cargo, lock, tag, other static mismatch and `--set` cases remain. `python3 -m py_compile sdlc/scripts/versions`, `python3 sdlc/scripts/pages` and `python3 sdlc/scripts/tickets` passed. Offline policy passed after overriding the machine's configured `sccache` with `/usr/bin/env`; the first policy run with blank wrapper variables failed only in its planted `cargo tree` checks because `sccache` was sandbox-denied.

## What the build taught us

The registry is a surface list, not a Cargo manifest list. A versionless wrapper still needs a checked metadata identity, and a failed read must be found before `--set` writes. The copied package and header version fields are real version copies, so a root-only or Cargo-only check cannot keep release metadata aligned. No wrapper build or publication was run for this internal gate repair.
