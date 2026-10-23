# ThinkThen for Kotlin

Use `io.github.botassembly:thinkthen-kotlin` in a Kotlin/JVM application on Java 22 or later. It brings the Java library and its packaged native engine. Run the application with `--enable-native-access=ALL-UNNAMED`.

```kotlin
import io.thinkthen.kotlin.ThinkThen

suspend fun routeMessage() {
    ThinkThen(adapter = "stub").use { client ->
        val row = client.decide(
            question = "Does the customer ask for money back?",
            record = "Please refund my order.",
            threshold = "0.2:0.8",
        )
        println(row.answer())
    }
}
```

Remove `adapter = "stub"` to use the default backend with its key in the environment. `ThinkThen(mode, adapter, url, keySource, model, batch)` forwards the same optional client settings as Java. All default values, parsing, proxy availability and error messages belong to the engine. No Kotlin proxy transport exists.

Single-record methods suspend while native work runs on a worker thread. Stream methods return cold `Flow` values, opening one native call per collection. They pull input and rows without collecting the input into a list. Flow collection happens in the caller's coroutine context; only blocking Java/native operations run on workers.

```kotlin
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.take

suspend fun review() {
    ThinkThen().use { client ->
        val messages = listOf("Please refund my order.", "Where is my delivery?")
        client.decideStream("Does this need review?", messages.asFlow())
            .take(1)
            .collect { row -> println(row.answer()) }

        client.filter("Is a refund requested?", messages.asFlow())
            .collect { original -> println(original) }
    }
}
```

The facade exposes the same ten named functions as Java, with named arguments and optional per-call `on`, supported `threshold`, and rank `top` and `levels` parameters. Score, rank and annotate have no threshold parameter because the call contract does not allow it. `choose`, `tag` and `score` take lists of choices, labels and levels respectively. `recognize` takes kinds, `relate` takes rules, and `annotate` takes a Jackson toolbox tree. A `Question` overload supports a named toolbox or reference. Single-record methods take `Any?`; strings remain raw text and other values use Java's ordinary JSON serialization. Array and list records stay single records. Each of the eight single-record methods has a `…Stream` form accepting `Flow<Any?>`; filter is generic over the original input type, and rank returns a flow of ranked rows. All stream methods also accept `Iterable` through a small `asFlow` overload.

Kotlin reuses Java's immutable `Row<A>` and small answer records. A row's nullable answer is visible through the Java accessors; JSpecify preserves nullable types. There is no second result hierarchy or copied engine rule. JSON detail remains a Jackson tree and is decoded lazily by the Java row. Java accessors return defensive JSON copies, so Kotlin mutations cannot change retained rows. Span indices work with Kotlin/JVM `String.substring`.

Cancelling a coroutine, stopping flow collection, or an exception in an input flow requests cancellation of that native call and closes its owners. Cancellation preserves the coroutine's `CancellationException`; it never becomes an unsure row or a replacement exception. The bridge registers cancellation before starting blocking work, requests Java's native cancel handle when the coroutine is cancelled, and lets the active worker leave the native operation before freeing its call. It does not assume thread interruption cancels a foreign call. Parent cancellation never leaves a native call running without an owner. Worker tasks and any input producer belong to the calling coroutine scope, with no detached scope. Cleanup waits for active foreign operations in a cancellation-safe cleanup context before releasing their owners. The cancellation hook also covers cancellation before the native handle is attached. A result discarded by prompt coroutine cancellation releases its call ownership rather than leaking it. An exception from input collection propagates unchanged after cleanup. A demand bridge adapts Flow input to the same Java call driver, with bounded handoff and no second native protocol loop. It collects input in the caller context only when the driver requests a record, and cancels that producer when output collection stops. No Java blocking iterator runs on the collector thread.

Engine failures use Java's `ThinkThenException`, with `code()` and `message`. Single calls throw the native final error; row flows retain native failed rows and propagate native terminal failures. Filters return original passing objects selected by the engine and propagate the engine's completion result. Closing `ThinkThen` closes its Java client; active calls retain their engine independently. Use coroutine scopes inside the client lifetime when work should finish before closing it.

The [Java API](../java/API.md) defines shared rows, questions, options and call ownership. The [packaging guide](../java/PACKAGING.md) keeps coroutine dependencies out of Java-only applications. This facade adds coroutine scheduling, named argument conversion and cancellation wiring only.
