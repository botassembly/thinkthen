# ThinkThen Swift C-door package prototype

This Swift 6.4.0 SwiftPM-shaped source package is a local experiment, not a published product. Install a separately verified `thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz` native archive (Linux x86_64 glibc, SONAME `libthinkthen.so.0`) alongside this source package. Verify its SHA-256 and exact header version 0.0.1 and check its manifest before use. Do not substitute an adjacent header version. The native library is not bundled in SwiftPM.

Example: unpack the native archive into `$native`, then run `swift build --package-path "$package" --jobs 2 -Xlinker -L -Xlinker "$native/lib" -Xlinker -rpath -Xlinker "$native/lib"`. Set `LD_LIBRARY_PATH="$native/lib"` for execution if the linker does not embed the desired runtime path. Use a clean SwiftPM cache on each isolated install. Link the system-library target to the supplied native binary, not a system-wide unverified installation. `Examples/main.swift` exercises the installed package.

`Outcome` is a checked backed enum (no=0, yes=1, unsure=2); `Answer` and `DoorFailure` expose typed scalar outputs/errors. `CancelToken` owns the native token and exposes `cancel()` for another Swift thread. Pass its `handle` to engine calls; keep the token alive and join all in-flight calls before releasing it. JSON-only verbs, including annotate, return a JSON string. Callers must join every in-flight user before `Engine.close()`. Native returned strings are copied then freed using `thinkthen_free_string`. This experiment provides no native ABI handshake, registry release, signing, other host target, or model-quality proof.

Verified host: Ubuntu 24.04.3 LTS x86_64, glibc 2.39. Swift 6.4.0 upstream Ubuntu 24.04 x86_64 tarball was unpacked under `~/.local/opt/swift-6.4.0` from `https://download.swift.org/swift-6.4.0-release/ubuntu2404/swift-6.4.0-RELEASE/swift-6.4.0-RELEASE-ubuntu24.04.tar.gz`; local download SHA-256 `69f7b2b4dbce6090b6c92194b71149a057e16269093b5a9dded38afc4a05094e` (no independent upstream sidecar was available). Native build uses rustup's pinned Rust 1.95.0 (`rustup toolchain install 1.95.0`), an already populated copied Cargo registry, and `cargo build --locked --offline --release -j2` against the pinned C crate. Verify fixtures with Ubuntu apt packages `clang` (observed 18.1.3), `bubblewrap` (0.9.0), `python3` (3.12.3), and `flock` from `util-linux`. These are build/test prerequisites, not transitive Swift runtime packages. A future product ticket must define GH Actions `ubuntu-24.04` release jobs and direct/Swift registry installation, checksums and supported platforms.

## Clone, build, and use locally (re-pin verification copy)

Install Rust 1.95.0 and Swift 6.4.0 on the tested Linux x86_64 host, then clone and build the native door from its pinned source:

```sh
git clone https://github.com/botassembly/thinkthen.git
cd thinkthen
git checkout 71f25087b72bc1876ea178790b72c35df824658d
cargo build --locked --release -j2 --manifest-path libraries/c/Cargo.toml
native="$HOME/.local/thinkthen-c-71f25087"
mkdir -p "$native/lib" "$native/include"
cp target/release/libthinkthen_c.so "$native/lib/libthinkthen.so"
ln -s libthinkthen.so "$native/lib/libthinkthen.so.0"
cp libraries/c/include/thinkthen.h "$native/include/thinkthen.h"
```

Obtain this SwiftPM package from the verified source archive, then in its directory run `swift build --jobs 2 -Xlinker -L -Xlinker "$native/lib" -Xlinker -rpath -Xlinker "$native/lib"`. To try its packaged example against your configured backend and API token: `LD_LIBRARY_PATH="$native/lib" .build/debug/ThinkThenExample`. To use it in your own project, add the package to your SwiftPM `dependencies` as `.package(path: "/absolute/path/to/thinkthen-swift-0.0.1")`, depend on its `.product(name: "ThinkThen", package: "thinkthen-swift-0.0.1")` (SwiftPM uses the extracted folder name as the local package identity), `import ThinkThen`, and link to this *same* verified native archive with your build's native-library search and runtime paths. The native binary is **not** included in the SwiftPM package. These are source-build steps for the tested Linux host, not permission to ship the rehearsal archive. At a release pin, verify the asset checksum and the header/native export parity.
