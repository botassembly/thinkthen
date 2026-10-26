# ThinkThen for Kotlin

Use Kotlin/JVM with Java 22 or later. Add `io.github.botassembly:thinkthen-kotlin:0.2.0` to your Maven or Gradle dependencies. Run with `--enable-native-access=ALL-UNNAMED`. The dependency includes the native engine.

The example uses the stub backend and needs no key or service.

```kotlin
import io.thinkthen.kotlin.*

suspend fun routeMessage() {
    ThinkThen { backend = Backend.Stub }.use { client ->
        val row = client.decide(
            question = "Does the customer ask for money back?",
            record = "Please refund my order.",
            threshold = Threshold.band(0.2, 0.8),
        )
        println(row)
    }
}
```

[API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/kotlin/src/main/kotlin/io/thinkthen/kotlin).
