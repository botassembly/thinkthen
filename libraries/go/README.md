# ThinkThen for Go

This Go 1.22 cgo source module calls the separately installed ThinkThen C library. Rust retains question grammar, judgment, scheduling and transport. The local package gate has passed on Linux x86_64; a release module and native archives are separate work.

## Build and use a matching local copy

From this repository checkout, build the C door and install its header and libraries into one prefix. The build using `libraries/c/Cargo.toml` writes to `libraries/c/target`, not the root `target` folder.

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
NATIVE="$HOME/.local/thinkthen-c"
mkdir -p "$NATIVE/include" "$NATIVE/lib/pkgconfig"
cp libraries/c/include/thinkthen.h "$NATIVE/include/"
cp libraries/c/target/debug/libthinkthen_c.so "$NATIVE/lib/libthinkthen.so"
cp libraries/c/target/debug/libthinkthen_c.a "$NATIVE/lib/libthinkthen.a"
ln -sfn libthinkthen.so "$NATIVE/lib/libthinkthen.so.0"
printf 'prefix=%s\nName: thinkthen\nDescription: ThinkThen C ABI\nVersion: 0.0.1\nLibs: -L${prefix}/lib -lthinkthen\nCflags: -I${prefix}/include\n' "$NATIVE" > "$NATIVE/lib/pkgconfig/thinkthen.pc"
```

In a separate local Go project, add `require github.com/botassembly/thinkthen/libraries/go v0.0.1` and `replace github.com/botassembly/thinkthen/libraries/go => /absolute/path/to/this/checkout/libraries/go` to its `go.mod`. The `require` version is a local placeholder, not a published tag. Set `PKG_CONFIG_PATH="$NATIVE/lib/pkgconfig"`, `CGO_ENABLED=1`, `GOPROXY=off`, `GOTOOLCHAIN=local`, and `LD_LIBRARY_PATH="$NATIVE/lib"` when building and running. The [example](examples/decide/main.go) imports the product module path. Match the native header and library to the same source revision. Shared-library relocation needs an explicit loader policy. Static-C mode links the C archive but still depends on Linux system libraries.

`sh libraries/go/check.sh 0` runs the offline product gate with a counted loopback backend. It checks the current 30-export header/library match, four copied module consumers, exact request bodies, the public result corpus, settings precedence, and planted failures. It does not call a paid backend.

For the local Linux x86-64 release pilot, `sdlc/scripts/release-pack TARGET OUT c go cpp` produces versioned Go, C++ and C archives from one clean source commit. Run `sdlc/scripts/release-go-cpp-pair OUT` before using the files together. The Go archive carries source only; unpack it and the matching C archive into separate folders. Point `PKG_CONFIG_PATH` to the C archive's `lib/pkgconfig`, and set `LD_LIBRARY_PATH` to its `lib` when running the shared build. The Go archive's `THINKTHEN-PACKAGE-INPUTS` records the exact C archive digest. This local package has an installed-consumer check, but no published Go module tag or final release asset.

## API and ownership

`New()` reads the native environment. `NewWith(settingsJSON)` applies the accepted settings object; its `base_url` overrides a valid but unusable environment route. An invalid settings object fails before sending. `Call` returns a Go-owned JSON string whose successful asking result contains `{"value":VALUE,"facts":FACTS}`. The four direct typed methods `Decide`, `DecideMany`, `Recognize`, and `Relate` return `Result[T]` with the former value in `.Value` and owned final call facts in `.Facts`. After `Decide`, read `result.Value.Outcome`; `Recognize` and `Relate` preserve their result JSON bytes in `result.Value`. Required facts are records, requests sent, cache answers and seconds; optional token counts and model are nil when unreported. Decode an allowed `null` answer separately from a failed call. `Error` carries numeric `Code`, one of six named `Kind` values, `Retryable`, a copied `Message`, and copied final `Facts` when the failed native call started.

Each failing cgo call and its borrowed error reads stay on one OS thread with `runtime.LockOSThread`; a goroutine can migrate between those calls otherwise. Returned answers, JSON and error data are Go-owned. `Engine.Close` waits for in-flight calls and then refuses new work. A caller must join its own users of the engine before closing it. The binding rejects interior NUL in C-string inputs, keeps evidence byte lengths, bounds native result lengths before `GoStringN`, and frees native result strings after copying.

Each call owns a one-shot cancellation token. Its watcher joins before token free. A context deadline becomes a bounded native millisecond budget; explicit cancellation fires the token. Deadline and cancellation return different native error kinds, including after a held reply drains. Fresh tokens can recover. The Go race checker covers Go code, not Rust allocations. The local synthetic backend checks calling mechanics, not model quality or final-release packaging.
