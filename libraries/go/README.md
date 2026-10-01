# ThinkThen for Go

This Go 1.22 cgo source module calls the separately installed ThinkThen C library. Rust retains question grammar, judgment, scheduling and transport. The package gate runs on Linux x86_64. Each GitHub release tags the module as `libraries/go/v0.1.0` and ships the matching C archive.

## Build and use a matching local copy

From this repository checkout, build the C door and install its header and libraries into one prefix. The build using `libraries/c/Cargo.toml` writes to `libraries/c/target`, not the root `target` folder.

```sh
cargo build --locked --offline --manifest-path libraries/c/Cargo.toml --lib
NATIVE="$HOME/.local/thinkthen-c"
mkdir -p "$NATIVE/include" "$NATIVE/lib/pkgconfig"
cp libraries/c/include/thinkthen.h "$NATIVE/include/"
cp libraries/c/target/debug/libthinkthen_c.so "$NATIVE/lib/libthinkthen.so"
sh libraries/c/localize.sh libraries/c/target/debug/libthinkthen_c.a "$NATIVE/lib/libthinkthen.a"
ln -sfn libthinkthen.so "$NATIVE/lib/libthinkthen.so.0"
printf 'prefix=%s\nName: thinkthen\nDescription: ThinkThen C ABI\nVersion: 0.1.0\nLibs: -L${prefix}/lib -lthinkthen\nCflags: -I${prefix}/include\n' "$NATIVE" > "$NATIVE/lib/pkgconfig/thinkthen.pc"
```

In a separate Go project, `go get github.com/botassembly/thinkthen/libraries/go@v0.1.0` adds the released module. To build against this checkout instead, add `require github.com/botassembly/thinkthen/libraries/go v0.1.0` and `replace github.com/botassembly/thinkthen/libraries/go => /absolute/path/to/this/checkout/libraries/go` to its `go.mod`. Set `PKG_CONFIG_PATH="$NATIVE/lib/pkgconfig"`, `CGO_ENABLED=1`, `GOPROXY=off`, `GOTOOLCHAIN=local`, and `LD_LIBRARY_PATH="$NATIVE/lib"` when building and running. The [example](examples/decide/main.go) imports the product module path. Match the native header and library to the same source revision. Shared-library relocation needs an explicit loader policy. Static-C mode links the C archive but still depends on Linux system libraries.

`sh libraries/go/check.sh 0` runs the offline product gate with a counted loopback backend. It checks the current 31-export header/library match, four copied module consumers, exact request bodies, the public result corpus, settings precedence, and planted failures. It does not call a paid backend.

For Linux x86-64, `sdlc/scripts/release-pack TARGET OUT c go cpp` produces versioned Go, C++ and C archives from one clean source commit. Run `sdlc/scripts/release-go-cpp-pair OUT` before using the files together. The Go archive carries source only; unpack it and the matching C archive into separate folders. Point `PKG_CONFIG_PATH` to the C archive's `lib/pkgconfig`, and set `LD_LIBRARY_PATH` to its `lib` when running the shared build. The Go archive's `THINKTHEN-PACKAGE-INPUTS` records the exact C archive digest. The release ships these archives, and an installed-consumer check covers them.

## API and ownership

`New()` reads the native environment. `NewWith(settingsJSON)` applies the accepted settings object; its `base_url` overrides a valid but unusable environment route. An invalid settings object fails before sending. `Call` returns a Go-owned JSON string whose successful asking result contains `{"value":VALUE,"facts":FACTS}`. The four direct typed methods `Decide`, `DecideMany`, `Recognize`, and `Relate` return `Result[T]` with the former value in `.Value` and the call's facts object in `.Facts` as `json.RawMessage`. After `Decide`, read `result.Value.Outcome`, one of `Yes`, `No` and `Unsure` (1, 0 and 2); `Recognize` and `Relate` return their result JSON in `result.Value`. `specification/result.schema.json` describes every JSON value; decode the members you need and ignore the rest. `ReadField` reads one annotate answer member as `Unresolved` (JSON null), `Answered` with its JSON value, or `Failed` with the failure's kind and cause. `Error` carries numeric `Code`, an `ErrorKind` (`KindUsage` through `KindDefect`, codes 1 to 6), `Retryable`, a copied `Message`, and copied final `Facts` when the failed native call started.

`Plan(verb, question, input, settings)` previews a `decide`, `choose`, `score` or `tag` call through `thinkthen_plan_json` and returns the result schema's `plan` object as `json.RawMessage`. The question is bare text, or one question object when it starts with `{`; settings is nil or a `thinkthen.settings/1` object. It needs no key, reads no cache and sends nothing. A context deadline reaches every sending method as `deadline_ms`, and `NewWith` passes engine settings such as `max_requests_total` to the C constructor unchanged. The package offers no probability option on score or tag, so it has no probability refusal to make.

Each failing cgo call and its borrowed error reads stay on one OS thread with `runtime.LockOSThread`; a goroutine can migrate between those calls otherwise. Returned answers, JSON and error data are Go-owned. `Engine.Close` waits for in-flight calls and then refuses new work. A caller must join its own users of the engine before closing it. The binding rejects interior NUL in C-string inputs, keeps evidence byte lengths, bounds native result lengths before `GoStringN`, and frees native result strings after copying.

Each call owns a one-shot cancellation token. Its watcher joins before token free. A context deadline becomes a bounded native millisecond budget; explicit cancellation fires the token. Deadline and cancellation return different native error kinds, including after a held reply drains. Fresh tokens can recover. The Go race checker covers Go code, not Rust allocations. The local synthetic backend checks calling mechanics, not model quality or final-release packaging.
