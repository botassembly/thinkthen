# ThinkThen for PHP

This source package targets PHP 8.3 CLI with FFI enabled (`php -d ffi.enable=1`). Its package gate runs on Linux x86_64 glibc. Packagist publishes it as `botassembly/thinkthen`. On Ubuntu 24.04 install `php8.3-cli php8.3-common`; the latter contains ffi.so. `php -m` alone can omit FFI; test `php -d ffi.enable=1 -r 'var_dump(class_exists("FFI"));'`. Composer reads `composer.json` and checks `php >=8.3`, `ext-ffi`.

Run the local product gate with `sh libraries/php/check.sh 0` from the repository root. It starts counted loopback backends. By default it uses `/usr/bin/php8.3`, `/usr/bin/python3`, `/usr/bin/bwrap`, `/usr/bin/flock`, and `/usr/bin/git`; set `THINKTHEN_PHP_BIN`, `THINKTHEN_PYTHON_BIN`, `THINKTHEN_BWRAP_BIN`, `THINKTHEN_FLOCK_BIN`, or `THINKTHEN_GIT_BIN` to executable absolute paths when those tools live elsewhere. It returns 77 for a missing host tool or PHP FFI extension. The native build uses the named heavy lock and offline Cargo cache.

## From a repository clone into a PHP project

These commands build the native library and copy the PHP wrapper into a local project on Ubuntu 24.04 x86_64. A released installation will use a separately installed, checksum-verified native archive. `cargo` must already have its dependencies in its cache for `--offline`; PHP 8.3 with FFI must be installed. Run the commands from a fresh working directory:

```sh
git clone https://github.com/botassembly/thinkthen.git
cd thinkthen
cargo build --locked --offline --release --manifest-path libraries/c/Cargo.toml -j2
cd ..
mkdir -p my-php-project/native my-php-project/vendor-local
cp thinkthen/libraries/c/target/release/libthinkthen_c.so my-php-project/native/libthinkthen.so
ln -s libthinkthen.so my-php-project/native/libthinkthen.so.0
cp thinkthen/libraries/c/include/thinkthen.h my-php-project/native/thinkthen.h
cp -a thinkthen/libraries/php my-php-project/vendor-local/thinkthen-ffi
```

In your application use `require __DIR__ . '/vendor-local/thinkthen-ffi/autoload.php';`, then `new ThinkThen(realpath(__DIR__ . '/native/libthinkthen.so'))` and call `decide('Is it?', 'example')` against a configured backend. Run with `php -d ffi.enable=1 app.php`. For a Composer project, add a local `path` repository pointing at `vendor-local/thinkthen-ffi` and require `botassembly/thinkthen`; it reads this package's `composer.json` and autoloads `autoload.php`. The binary must still be installed and passed by absolute path; Composer does not provide it.

The native `libthinkthen.so.0` SONAME is supplied by `libthinkthen_c.so`; the local install must include a matching `libthinkthen.so.0` symlink. This source checkout recipe is not a release channel.

The local Linux package pilot uses `release-pack x86_64-unknown-linux-gnu OUT c php dart` from one clean commit. It writes separate versioned PHP, Dart and C archives with adjacent checksums. The PHP archive carries Composer metadata and source, not the C library. After verifying the archive pair, `THINKTHEN_ARTIFACT=/absolute/path/to/thinkthen-php-...tar.gz THINKTHEN_C_ARTIFACT=/absolute/path/to/thinkthen-c-...tar.gz sh libraries/php/check.sh 0` runs a fresh installed-file consumer against the selected C archive. This direct-file proof does not run Composer or publish to Packagist.

## Installation contract

The PHP source package is separate from the native library archive. Verify each release's SHA-256 and manifest before extraction. Install the archive's header and library together. Header version must match the installed library; the library SONAME and exports must match the manifest. Resolve the library to an **absolute** path and pass it to `new ThinkThen($absoluteLibrary)`; no fallback lookup or network download exists. Pass an optional JSON settings object as the second constructor argument. Run `require 'vendor/autoload.php'` after Composer installation (or `require 'autoload.php'` for a direct source install). `examples/direct.php` exercises scalar and JSON calls with numeric loopback.

A PHP FFI call blocks the ordinary single-threaded VM. This wrapper supports pre-fired refusal and native deadline expiry, **not PHP-driven in-flight cancellation or PHP concurrency**. The Python ctypes held-call proof exercises the engine C ABI only; it does not change this limitation. Do not use FFI CData engine/token escape hatches across engine close or after token free. Close the engine after all calls and tokens settle; free every token only after the last native call using it returns. `call` returns the JSON `{value,facts}` envelope. The direct `decide`, `decideMany`, `recognize` and `relate` methods return `['value' => former value, 'facts' => final call facts]`; for example, read `$door->decide('Is it?', 'example')['value']['outcome']`. Facts are host arrays decoded from the engine's JSON, and a member the wrapper does not know reads like any other. Optional usage keys are absent when unreported. `ThinkThen::YES`, `NO` and `UNSURE` name the outcome codes 1, 0 and 2. `ThinkThen::failed($member)` returns an annotate answer's failure array (`kind`, `cause`), or null for any value; a JSON null is unresolved, never a failure. `plan($verb, $question, $texts, $settingsJson)` previews a decide, choose, score or tag call through `thinkthen_plan_json` and returns the result schema's `plan` array; it needs no key and sends nothing. A question starting with `{` is a question object. A `ThinkThenFailure` exposes a named `kind`, its `nativeCode` 1 to 6, retryable flag, message and copied `factsJson` for a started failure. Pass `max_requests_total` in the constructor's settings JSON to cap live sends. Returned JSON strings are copied and freed with `thinkthen_free_string`. C-string input rejects embedded NUL and invalid UTF-8; counted evidence accepts embedded NUL. Calls that send work can drain to cache/counters on deadline or cancellation without returning results.

## Release work

Each GitHub release ships `thinkthen-php-VERSION-x86_64-unknown-linux-gnu.tar.gz` beside the matching C archive. Other hosts remain separate work. Local build outputs are not release assets.

`new ThinkThen($absoluteLibrary, $settingsJson)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.
