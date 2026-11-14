# Java API

Use Java 22 or later. `ThinkThen` owns the native client; close it with try-with-resources. Classpath applications enable native access with `--enable-native-access=ALL-UNNAMED`, and module-path applications use `--enable-native-access=dev.thinkthen`.

`Row<A,T>` is the sealed result interface: `Answered`, `Unsure` or `Failed`, where `A` is the answer and `T` is the caller's record. Each result keeps `item()`, `position()`, `id()` and `source()`. Scalar calls throw `FailedException` for a failed parent and return answered or unsure rows. `answer()` throws `UnsureException` on a returned scalar row; bulk rows and form entries can also throw `FailedException` when there is no answer. `probability()` is an `OptionalDouble`; `detail()` is a sealed `Detail`, with `NoDetail` when absent. Both are always safe. Only `Failed` has `error()`.

`throwIfFailed()` returns the same result unchanged, or throws for the result's failure or any failed form entry. Unsure entries do not throw. `toString()`, printing, formatting and concatenation show the command's words. `toJson()` writes the shared saved row without `item`. Value equality and hashing use function, status, answer, probability, error, position, id, source and detail only. The caller's mutable item never participates.

| Call | Result |
| --- | --- |
| `decide(question, T)` | `Row<Boolean,T>` |
| `choose(question, T, Topic.class)` | `Row<Topic,T>` |
| `tag(question, T, Topic.class)` | `Row<List<Topic>,T>` |
| `score(question, T, Urgency.class)` | `Row<Double,T>` |
| `find(question, T)` | `Row<Optional<Span>,T>` |
| `recognize(T, kinds)` | `Row<List<Entity>,T>` |
| `relate(T, rules)` | `Row<List<Relation>,T>` |
| `annotate(questions, T)` | `Row<Annotations,T>` |
| `filter(question, records)` | `Stream<T>` of kept original records |
| `rank(question, records, Urgency.class)` | `Stream<Row<Long,T>>` |

Each single-record call has a corresponding `...Stream` method over `Iterable<T>` or `Stream<T>`. A returned stream must be closed, including when traversal stops early. Closing it closes its input stream. Rank retains original records until ranking completes, including when `top` is set. One failed record does not stop ordinary result streams. Filter emits every kept record before throwing one `FailedException`; its `rows()` holds all failed results, including their original objects.

Choices, labels and levels also accept `List<String>`. An enum's `toString()` supplies its model-facing words; its declaration order defines a scale. Enum constants key descriptions. Choose and tag question types retain the member type. All question types are reusable definitions, and `Toolbox` loads and saves them. Typed toolbox and annotation selectors throw `UsageException` for missing or wrong-function names.

A form's `answer()` is `Annotations`. Its function-specific selectors return ordinary results, including `choose(name, Topic.class)` and `tag(name, Topic.class)`. Check every consumed entry's state. Scalar annotate returns an answered parent even when one, several or all entries failed; only an explicit `throwIfFailed()` scans children. Filter and rank cannot be form entries.

Options are immutable builder values passed last. All accept `field(Pointer)` and a reference-only run `name(String)`. Decide accepts `Threshold.cut` and `Threshold.band`; filter, choose, tag, find, recognize and relate accept `Threshold.Cut` only. Score and rank have no threshold. `RankOptions.top(int)` sets the number returned. Decide/filter accept `yesMeans` and `noMeans`; choose/tag/score/rank accept `descriptions`, keyed by strings or enum constants. The shared engine admits semantic values before making requests.

Records are strings, JSON-like objects, immutable `JsonValue`s, scalars or null. Other classes use a `RecordCodec` on the client builder. Collections given to a single-record call are refused with `IllegalArgumentException` naming the stream form. Blank scalar records throw `FailedException` with `record.empty`; null scalar records throw it with `record.missing`. Bulk calls retain those failed rows, even for one input. Neither makes a request. A codec or input iterator exception propagates unchanged.

Span `start()` and `end()` are Java UTF-16 indexes as `int`, usable by `String.substring`. Saved JSON and printed name coordinates count characters. Recognized names expose `kind`, `text`, `start`, `end` and `probability`; relation endpoints expose the same text/slice fields. `RelationRule.of(source,target,relation)` and its builder accept optional `reads` and `either`.

`client.async()` provides cancellable `CompletableFuture`s for the same single-record calls. Scalar judgement failures complete exceptionally with their original `FailedException` cause; `join()` exposes it through `CompletionException`. Unsure completes successfully, with the same shared row type. Interrupting the waiting thread, cancelling a future or closing a stream stops the native call with `CancellationException`. An interrupted thread retains its interrupt flag. A client is thread-safe; closing it stops its running calls.

`ThinkThen.builder()` accepts `backend`, `model`, `url`, `apiKey`, `batch`, `retry` and `recordCodec`. An explicit backend takes precedence over `THINKTHEN_BACKEND`. `RetryPolicy` accepts `maxRetries`, `initialBackoff(Duration)` and `attemptTimeout(Duration)`. `maxRequests` remains a reference-only client setting for remote send allowance; omitted means unlimited. Credentials have no getters and are excluded from diagnostics. `usage()` returns immutable cumulative accounting, also after close; tokens are reported subtotals, not prices or invoices.

`NoAnswerException` is the parent of `UnsureException` and `FailedException`. The former carries `row` and `probability`; the latter carries `code`, `httpStatus` and failed `rows`. Call failures are `UsageException`, `SetupException` and `InternalException`, under `ThinkThenException`. Catch `FailedException | ThinkThenException` for failures without catching uncertainty. All are unchecked. `Toolbox.load` additionally throws the JDK's checked `IOException` for file access. `status()` remains available for reference use; the guide teaches the exhaustive result switch.
