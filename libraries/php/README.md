# ThinkThen for PHP

The development Composer package carries the native library and targets PHP 8.3 CLI with `ext-ffi` enabled. Linux x86_64 glibc is checked locally. Platform qualification and registry installation run at the release candidate. The public release remains 0.1.2; this API describes development 0.2.

```php
require 'vendor/autoload.php';
$client = new ThinkThen\Client(['cache' => false]);
try {
    $call = $client->decide('Does this ask for a refund?', 'Please refund my order.');
    $answer = $call->results[0]->value();
    if ($answer->present && $answer->value === true) echo "refund requested\n";
} finally { $client->close(); }
```

Run PHP with `php -d ffi.enable=1 app.php`. The client loads `native/libthinkthen.so` from its package. Its optional second constructor argument selects an explicit native library. It does not download libraries or search system paths.

`Client` provides `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. Each accepts literal question text or a PHP question-definition array/object, input values and optional native call settings. `questionJson` retains authored JSON bytes, including duplicate members and numeric forms, for native validation. `questionFile`, `questionNamed` and `questionReference` select native question loaders explicitly. No text is inferred to be a path.

A PHP list supplies several records. `Client::item($value, $fields)` wraps one JSON item and carries per-item context, replacement options or images. `Client::images($attachments)` supplies image-only evidence. Attachments use the shared native descriptors. `Client::files($paths, $reading, $framing, $media)` selects native file reading, including JSONL framing and whole-file image media. A fresh `Traversable` supplies a bounded feed. Native code owns admission, file reading, cache, replay and route selection.

Results use generated PHP classes. A field accessor returns `Presence` with separate `present` and `value` members; absent, null and false differ. Property access returns the field value. `has()` checks unknown extensions, and `toObject()` retains their PHP representation. Objects remain `stdClass` and lists remain arrays. Wide integers become exact decimal strings. `Completed::packets` retains every native packet, and `nativeJson()` retains its original numeric types. Read answer values explicitly: PHP treats result objects as true. Owned results survive client and operation closure.

Failures throw `UsageFailure`, `BackendFailure`, `DeadlineFailure`, `LocalFailure`, `CancelledFailure` or `DefectFailure`. Started failures retain typed `failure`, `terminal`, completed `results`, packets and `facts()`. Admission failures have no invented facts. Pass a `Cancellation` as the fourth argument, or use `start()`, `poll()`, `cancel()` and `close()`. PHP calls block the ordinary VM. Polling advances at most one producer item. A client destructor closes open operations before freeing its engine and does not wait for a provider reply.

`usage_persistence()` and `finish_usage_status()` return readonly `UsageStatus` observations with generated `UsagePersistenceState` and copied advice. They refuse calls after close. Failed persistence leaves earlier successful answers and facts intact. See [the native engine contract](../c/DESIGN.md).

## Upgrade from the old APIs

| Old call | Development call |
| --- | --- |
| `new ThinkThen($library, $settingsJson)` | `new ThinkThen\Client($settings, $library)` |
| `ThinkThen` scalar and generic `call` methods | The named `Client` method and `Completed::results` |
| `ThinkThen\Native\Engine` complete calls and batches | The named `Client` method or `start()` and `poll()` |
| Handwritten native views and readers | Generated result classes and native file/record inputs |
| Legacy failure facts JSON | Typed failure `facts()` and retained packets |

The old PHP public APIs and handwritten native view readers are removed. The frozen C 0.1 exports remain in the native library.

## Local checks and package assembly

`fixtures/package.py --library LIBRARY --out DIRECTORY` assembles a native-bearing local archive. `THINKTHEN_COMPOSER_BIN=/absolute/path/to/composer.phar sh libraries/php/check.sh 0` installs that archive through Composer with network, scripts and plugins disabled and runs the routine synthetic-backend checks. The gate builds the native library offline; `THINKTHEN_COMPLETE_LIBRARY` selects an explicitly retained library. A fixture archive carrying a debug native library does not qualify a production distribution.

Set `THINKTHEN_ARTIFACT` to check a selected archive. `THINKTHEN_TEST_PROFILE=full` selects the shared installed suite at the release candidate, using `conformance/session_projection.py`. Full parity, large-input cases and other platforms are release-only. No paid backend runs in these checks.
