# ThinkThen for Java

Use Java 22 or later. Add `io.github.botassembly:thinkthen-java:0.2.0` to your Maven or Gradle dependencies. Run with `--enable-native-access=ALL-UNNAMED`. The JAR includes the native engine.

The example uses the stub backend and needs no key or service.

```java
import io.thinkthen.*;

try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
    Row<Boolean, NoDetail> row = client.decide(
        "Does the customer ask for money back?",
        "Please refund my order.");
    if (row.status() == Status.UNSURE) {
        System.out.println("Please review this message.");
    } else if (row instanceof Answered<Boolean, NoDetail> answered && answered.answer()) {
        System.out.println("Send this message to the refunds queue.");
    }
}
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/java/API.md).
