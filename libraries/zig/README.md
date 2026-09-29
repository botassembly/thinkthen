# ThinkThen Zig 0.0.1 rehearsal

Zig 0.15.2, x86_64 Linux with glibc only. This module wraps the unchanged 0.0.1 C ABI. This is not a published release. The C archive is supplied separately; no Rust or Cargo is required by a consumer. The archive must contain `include/thinkthen.h`, `lib/libthinkthen.so`, `lib/libthinkthen.so.0` and `lib/libthinkthen.a`. Use a C archive built from the matching pinned release. `build.zig` checks version macros, selected library and platform, not the binary's runtime ABI identity. Verify archive hashes out of band.

## Clone, build, and use from source

The commands below start from a local clone at the **same commit as the native archive**, on the supported x86_64 Linux/glibc host. The rehearsal package here is local and not published. Cargo's `--offline` requires its dependencies to be cached already; omit that flag only when a download is authorized.

```sh
git clone https://github.com/botassembly/thinkthen.git
cd thinkthen
git checkout 71f25087b72bc1876ea178790b72c35df824658d
cargo build --manifest-path libraries/c/Cargo.toml --release --locked --offline -j2
mkdir -p "$HOME/thinkthen-c/include" "$HOME/thinkthen-c/lib"
cp libraries/c/include/thinkthen.h "$HOME/thinkthen-c/include/"
cp target/release/libthinkthen_c.so "$HOME/thinkthen-c/lib/libthinkthen.so"
cp target/release/libthinkthen_c.a "$HOME/thinkthen-c/lib/libthinkthen.a"
ln -s libthinkthen.so "$HOME/thinkthen-c/lib/libthinkthen.so.0"
```

From a separate clone or a copy of this `package/` folder, build the Zig package example with `zig build -Dnative="$HOME/thinkthen-c" -Dlink-mode=shared -j2` (this builds the executable without sending a request). Set `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE` as specified in the C header, then `zig build -Dnative="$HOME/thinkthen-c" -Dlink-mode=shared -j2 example` to send a request to a local backend or an authorized service. For static C linkage use `-Dlink-mode=static`. Never send production inputs to the example by default.

To use it in another Zig project, run `zig fetch --save=thinkthen /absolute/path/to/this/package` from that project's root. In the project's `build.zig` use the dependency/module/link lines below and pass the absolute `native` archive path from its build options; `zig build -Dnative="$HOME/thinkthen-c" -j2` then builds your own executable. This native archive is an unshipped local build, not the reviewed release archive. The release integration must publish and verify a matching archive before promising installations on other machines.

For a dependent `build.zig`, add `const tt = b.dependency("thinkthen", .{ .target = target, .optimize = optimize });`, `exe.root_module.addImport("thinkthen", tt.module("thinkthen"));`, and `@import("thinkthen").linkNative(b, exe, tt.module("thinkthen"), native, .shared);` in the consumer build script. The build script import refers to the dependency's `build.zig`; the executable imports `tt.module("thinkthen")`. The gate's consumer shows the exact working form. Supply an absolute `-Dnative` path from the consumer's installation location. Shared mode stores that path in the executable's rpath. Move the archive only with a rebuilt executable. Static mode embeds the C library but still depends on glibc and system libraries.

`Engine.init(allocator)` returns `Result(Engine)` or Zig `error.OutOfMemory`. On failed construction free `Failure.message` with the same allocator. For a built engine call `deinit` once after all calls join. Each method returns either `.ok` or `.failed`, or a Zig allocation/validation error. Free every `.failed.message` with `engine.freeFailure`; free successful bulk answers and JSON byte slices with the engine allocator. These Zig-owned results and failures can outlive `Engine.deinit()` only while the allocator that created them remains alive. After engine teardown, call `allocator.free` directly on each owned slice or message; never call a method on the deinitialized engine. Calls block. Pass a thread-safe allocator when sharing an engine among threads. An error is copied from C thread-local state on the calling native thread immediately after the failing call, before handing control to another thread. Success does not clear the native last error. Evidence is byte-counted and may contain UTF-8 and NUL; the C engine decides whether it accepts that evidence. Questions, JSON requests and specs are NUL-terminated and reject an interior NUL before sending. Do not copy engine/token owner structs; keep token alive until all calls carrying it have joined. `CancelToken.cancel()` is thread safe, one-shot and repeatable; create a fresh token for recovery. **Native cancellation contract:** once a token fires, a call starts no new request or retry. Already-sent requests drain and may update cache and counters; the call then returns cancellation without results. The post-ticket-0166 verification must establish this behavior against the rebuilt matching native archive. The Zig wrapper reports the C result without fabricating cancellation. The C archive and this module do not provide a throttle constructor or a live backend guarantee.

`Engine.call` passes any of the ten JSON verbs and returns a copied JSON byte slice. `decide`, `decideMany`, `recognize`, and `relate` have typed entry points. `Options` carries `deadline_ms` and optional raw token pointer. The default deadline is unlimited, zero is already expired, any other negative or more than 4294967295000 is invalid. See `examples/decide.zig` and the C header for grammar and error codes.
