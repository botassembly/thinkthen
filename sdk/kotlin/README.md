# Kotlin SDK

The Kotlin SDK asks questions about text from your Kotlin code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Add `io.github.botassembly:thinkthen-kotlin:0.2.0` to Gradle or Maven. It needs Kotlin/JVM on Java 22 or later and carries its own engine. Run your program with `--enable-native-access=ALL-UNNAMED`. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```kotlin
import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val isRefund = client.decide(
            "Does the customer request a refund?",
            "Please refund my order.",
        ).answer()
        println(isRefund)
    }
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. `answer()` throws `UnsureException` when decide is not sure, and `ThinkThenException` when ThinkThen could not judge the message, so neither one passes for `false`. Send both to a person.

```kotlin
import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val isRefund = client.decide(
            "Does the customer request a refund?",
            "I was charged twice for one invoice.",
            threshold = Threshold.band(0.2, 0.8),
        )
        try {
            if (isRefund.answer()) println("Send to refunds.")
            else println("Leave in the inbox.")
        } catch (notSure: UnsureException) {
            println("Not sure. Ask a person.")
        }
    }
}
```

Output:

```text
Not sure. Ask a person.
```

## Next

The [Kotlin SDK page](https://thinkthen.dev/sdks/kotlin/) shows all ten functions. The [Kotlin guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/kotlin.md) and the [API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/kotlin/src/main/kotlin/dev/thinkthen/kotlin) cover every option.

## Usage accounting

```kotlin
import dev.thinkthen.kotlin.*

fun main() {
    ThinkThen().use { client ->
        val usage = client.usage
        println(usage.attempts)
        println(usage.inputTokens.reported)
    }
}
```

Snapshots are immutable cumulative totals for one explicit client and remain available after close. Counters include retries; token totals are provider-reported subtotals with separate unknown-attempt counts. Null token totals indicate overflow. Use an independent client for an isolated run. No prices or invoice totals are inferred.

## Request allowance

```kotlin
import dev.thinkthen.kotlin.*

fun main() {
    ThinkThen { maxRequests = 100uL }.use { client ->
        println(client.usage.attempts)
    }
}
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

## Local response storage

Explicitly select `cache`, `record`, or `replay` storage; leaving it unset retains no text. For an existing recording:

Set `responseStore = ResponseStore("./answers", ResponseMode.REPLAY, "example", "model-v1")` inside the `ThinkThen` settings block. Read `client.usage.replayHits` for the copied hit count. This assumes that `./answers` already contains a matching recording.


Store paths must name private local directories. This opt-in feature retains input and output text without encryption. Namespace and model generation are explicit nonsecret labels; change the generation when the provider's model binding changes. Replay opens existing recorded answers without credentials or network fallback, and a missing answer fails with `local.replay-miss`. Cache/replay hit counters count materialized asks, not output rows, and do not add historical token usage to current spending. Optional limits use the engine defaults: 64 MiB logical payload, 4096 eligible asks, 86400 seconds cache age, and 5000 ms busy timeout. Recordings do not expire or get evicted to continue a recording. Filesystem allocation may exceed the logical payload limit. No environment variable enables retention.
