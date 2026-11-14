# ThinkThen for Kotlin

ThinkThen answers bounded questions about text. Use Kotlin/JVM on Java 22 or later, set the JVM target to 22, and add `io.github.botassembly:thinkthen-kotlin` to Maven or Gradle. The package includes the engine. Set `THINKTHEN_API_KEY` and run with `--enable-native-access=ALL-UNNAMED`.

```kotlin
import dev.thinkthen.kotlin.*
import java.nio.file.Path
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

suspend fun main() {
    ThinkThen().use { client ->
        if (client.decide("Does the customer request a refund?", "Please refund my order.").answer()) {
            println("refund queue")
        } else {
            println("inbox")
        }
    }
}
```

A scalar judgement failure throws `FailedException` at the call. A successfully returned scalar result is answered or unsure; its `answer()` can throw `UnsureException`. Bulk results and form entries retain failed states, and their reads can also throw `FailedException`; catch `NoAnswerException` for either missing-answer state. State checks, detail, printing and storing a result are safe. `Row<A, T>` keeps the caller's original record as `item`, excluding it from equality and hashing. Many-record calls are cold flows, and each collection makes a new call.

The [Kotlin guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/kotlin.md) covers every function, enum choices, forms, fields, settings and cancellation.
