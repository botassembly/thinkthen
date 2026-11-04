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
