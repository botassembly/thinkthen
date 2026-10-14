# ThinkThen for PHP

The development Composer package carries the native library and targets PHP 8.3 CLI with `ext-ffi` enabled. Linux x86_64 glibc is checked locally. Platform qualification and registry installation run at the release candidate. The public release remains 0.1.2; this API describes development 0.2.

```php
require 'vendor/autoload.php';
$client = new ThinkThen\Client(['cache' => false]);
try {
    $answer = $client->decide('Does this ask for a refund?', 'Please refund my order.')->decideAnswer();
    if ($answer === true) echo "refund requested\n";
} finally { $client->close(); }
```

Run PHP with `php -d ffi.enable=1 app.php`. The client loads `native/libthinkthen.so` from its package. Its optional second constructor argument selects an explicit native library. It does not download libraries or search system paths.

`Client` provides `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. Each accepts literal question text or a typed question definition, input values and optional call settings. Generated `Inputs\Settings` and `Inputs\Options` support named arguments. The ten-call example uses `RecognizeQuestion`, `RecognitionSpec`, `RelateQuestion`, `RelationSpec` and `RelationRule` builders. `questionFile`, `questionNamed` and `questionReference` select native question loaders explicitly. No text is inferred to be a path.

A PHP list supplies several records. `Client::item($value, $fields)` wraps one JSON item and carries per-item context, replacement options or images. `Client::images($attachments)` supplies image-only evidence. Attachments use the shared native descriptors. `Client::files($paths, $reading, $framing, $media)` selects native file reading, including JSONL framing and whole-file image media. A fresh `Traversable` supplies a bounded feed. Native code owns admission, file reading, cache, replay and route selection.

Results use generated PHP classes. A field accessor returns `Presence` with separate `present` and `value` members; absent, null and false differ. Property access returns the field value. `has()` checks unknown extensions, and `field()` reads their PHP values. Whole-packet serialization remains private. Objects remain `stdClass` and lists remain arrays. Wide integers and decimals that cannot round-trip through a PHP number become exact decimal strings. `Completed::packets` retains every native packet. Rust decodes the packet once; generated PHP readers copy its native values. Use the named answer accessor to read a scalar or a plain filter list of original values, including null and false, or a rank list of zero-based input indexes. The recognize and relate example constructs generated question types and reads edge names directly. Owned results survive client and operation closure.

Failures throw `UsageFailure`, `BackendFailure`, `DeadlineFailure`, `LocalFailure`, `CancelledFailure` or `DefectFailure`. Started failures retain typed `failure`, `terminal`, completed `results`, packets and `facts()`. Immediate failures copy the native retryable flag and typed `FailureFacts`, including count-only facts without an invented call identity. `ErrorKind` identifies each native failure. Pass a `Cancellation` as the fourth argument, or use `start()`, `poll()`, `cancel()` and `close()`. PHP calls block the ordinary VM. Polling advances at most one producer item. A client destructor closes open operations before freeing its engine and does not wait for a provider reply.

`plan($function, $question, $input, $options)` previews native request admission without sending or consuming a producer. Its readonly `Plan` contains record, request, byte and token estimates. Its `first_body_utf8` preserves the required nullable native preview body, and its typed token band preserves lower and upper estimates.

`usagePersistence()` and `finishUsageStatus()` return readonly `UsageStatus` observations with generated `UsagePersistenceState` and copied advice. They refuse calls after close. Failed persistence leaves earlier successful answers and facts intact. See [the native engine contract](../c/DESIGN.md).

## Local checks and package assembly

`fixtures/package.py --library LIBRARY --out DIRECTORY` assembles a native-bearing local archive. `THINKTHEN_COMPOSER_BIN=/absolute/path/to/composer.phar sh libraries/php/check.sh 0` installs that archive through Composer with network, scripts and plugins disabled and runs the routine synthetic-backend checks. The gate builds the native library offline; `THINKTHEN_COMPLETE_LIBRARY` selects an explicitly retained library. A fixture archive carrying a debug native library does not qualify a production distribution.

Set `THINKTHEN_ARTIFACT` to check a selected archive. `THINKTHEN_TEST_PROFILE=full` selects the shared installed suite at the release candidate, using `conformance/session_projection.py`. Full parity, large-input cases and other platforms are release-only. No paid backend runs in these checks.

The development PHP API renames `usage_persistence()` to `plan($function, $question, $input, $options)` previews native request admission without sending or consuming a producer. Its readonly `Plan` contains record, request, byte and token estimates. Its `first_body_utf8` preserves the required nullable native preview body, and its typed token band preserves lower and upper estimates.

`usagePersistence()` and `finish_usage_status()` to `finishUsageStatus()`. An operation retains its client until close, so `(new Client)->start(...)` remains live.
