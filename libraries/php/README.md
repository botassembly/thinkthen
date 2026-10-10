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

## Complete typed calls

`autoload.php` also provides `ThinkThen\Native\Engine`. It exposes `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate` through the matching complete native functions. Existing `ThinkThen` methods retain their return types.

```php
use ThinkThen\Native\{Engine, Question, QuestionSpec, FunctionKind, Content, Records, Record};

$engine = new Engine($absoluteLibrary, $settingsJson);
try {
    $question = Question::spec(new QuestionSpec(
        FunctionKind::DECIDE, Content::text('Does this ask for a refund?')));
    $result = $engine->decide($question,
        new Records([new Record(Content::text('Refund me please.'))]));
    $row = $result->rows[0];
    $answerId = $row->common->answer_id->data;
    $probability = $row->common->answer->value->data->probability;
} finally { $engine->close(); }
```

`QuestionSpec` carries typed options/descriptions, meanings, thresholds, models, profiles, batch settings, pointers, question members, recognition kinds and relation rules. `Author`, `Declaration` and `Property` carry optional names, wording versions and item/context schemas. Native admission enforces their contract. `Question::file`, `named` and `reference` use native loaders; `Question::saved(LoaderRole::…, $json)` explicitly imports saved grammar through one native parser. It never infers a path from text.

`Records` retains each `Record`'s original `Content`, context, complete candidate replacement and ordered `Image` attachments. `Content::text` preserves literal text; `Content::json` carries arbitrary caller JSON, including explicit null. `Files` selects native line, window, whole-file, image-file or JSONL reading with `FileUnit`. Only decide/choose/score admit images on an admitted route. Other combinations fail explicitly before sending. No host reader, cache or scheduler is added.

Results contain copied function views, summary/facts/attempts, every observation and its details/author, row details/authors, annotation member authors, rank member views and located recognition/relation views. Nested known fields have typed properties; arbitrary authored/original JSON remains explicit `ContentView` bytes. An optional view's `present` distinguishes absence from zero, false and empty values. Decide's `kind` distinguishes uncertainty (0), ordinary Boolean (1) and authored meaning (2), including authored Boolean/null. Find answer views require the declared C answer tag 5 and copy its named-answer fields. Unsigned native counters are exact decimal strings. Wording versions remain subject to native bounds.

`CompleteFailure` exposes the six named kinds through `kind()`, safe message/code/retryability, and its copied `summary`, with final facts/attempts only when the native call started. `Controls` supplies deadline, native cancellation, context, packing and attempts. `decideBatch`, `chooseBatch`, `tagBatch`, `scoreBatch`, `filterBatch` and `annotateBatch` own native lazy batches: call `next()` until exhaustion, then `facts()`, and always `close()`. Close batches before the engine. Eager results and yielded rows remain readable after close. PHP blocks during FFI calls; a native cooperating thread can fire the borrowed cancellation token, while ordinary PHP code can pre-fire it or set a deadline.

Cache, record, refresh and strict replay use the engine settings and native storage, including changed-reading replay and native identities. Large admitted inputs need an adequate PHP `memory_limit`; the complete shared image suite uses 2 GiB, because copied views retain original evidence. No setting is changed automatically.

Run the existing canonical suite with `python3 libraries/php/fixtures/complete_parity.py php` from the checkout; it counts owned loopback arrivals and emits cells only for actual public calls. No paid backend is used.

Rank-set rows retain every member in saved declaration order. Each member exposes its native positive rank position, probability, answer identity, author declarations and complete details. Details preserve independently reported token dimensions and source batch sizes. Parent and member metadata overlap; read final call facts for invocation usage.

## Native session API in development 0.2

`ThinkThen\Client` adds the ten named calls over the shared native request contract. A local Composer archive can carry `native/libthinkthen.so`; the constructor uses that library by default. Final platform distribution assembly remains separate. PHP 8.3 with `ext-ffi` and FFI enabled is required.

```php
require 'vendor/autoload.php';
$client = new ThinkThen\Client(['cache' => false]);
try {
    $call = $client->decide('Does the message ask for a refund?', 'Please refund my order.');
    $value = $call->results[0]->value();
    if ($value->present && $value->value === true) {
        echo "refund requested\n";
    }
} finally {
    $client->close();
}
```

The result classes are generated from the Rust result graph. Each field accessor returns a `Presence` with separate `present` and `value` members, so absent, null and false differ. Property access returns the field value. `has()` also checks unknown extensions, and `toObject()` retains their PHP representation. Integers outside PHP's signed integer range become exact decimal strings, never rounded floats. `Completed::packets` retains every native packet, including observations; each packet's `nativeJson()` keeps the original JSON bytes and numeric types. Read an answer's value explicitly: PHP treats every result object as true. Results own PHP values and survive operation and client closure.

All ten methods accept a text question or a PHP question-definition array/object, PHP values, arrays of records, `Client::files($paths, $reading)`, or a fresh `Traversable`. A PHP list means several records; wrap a list with `Client::item()` to make it one JSON item. `Client::item($value, $fields)` carries explicit per-item context, options or images. PHP preserves JSON objects as `stdClass` and JSON arrays as arrays. Native code checks the request, selects the route, computes cache identity and owns reading rules.

Calls throw `UsageFailure`, `BackendFailure`, `DeadlineFailure`, `LocalFailure`, `CancelledFailure` or `DefectFailure`. An execution failure retains generated `failure`, `terminal`, completed `results` and `facts()`. Immediate conversion/admission refusals have no invented facts. Pass a `Cancellation` as the fourth argument or start an `Operation`, call `poll()`, then `cancel()` or `close()`. A client destructor closes its open operations before freeing the engine. It does not wait for a provider reply.

For the migration, released `ThinkThen` and `ThinkThen\Native\Engine` calls stay available. Their corresponding new calls use `Client` and return `Completed::results` plus terminal facts. The old readers are removed only after full installed parity. The C session currently reports its native default Rust surface; this API preserves that report and does not claim PHP-specific attribution.

Focused installed consumer checks build a local archive with `fixtures/package.py`, install it through Composer, and run `fixtures/session_installed.py`. They use a synthetic loopback backend. A fixture archive carrying a debug native library does not qualify a production distribution.
