# ThinkThen for Java

ThinkThen answers bounded questions about text. Use Java 22 or later and add `io.github.botassembly:thinkthen-java` to Maven or Gradle. The package includes the engine. Set `THINKTHEN_API_KEY` and run with `--enable-native-access=ALL-UNNAMED`.

```java
import dev.thinkthen.*;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;

public class Example {
    public static void main(String[] args) throws java.io.IOException {
        try (var client = ThinkThen.builder().build()) {
            if (client.decide("Does the customer request a refund?", "Please refund my order.").answer()) {
                System.out.println("refund queue");
            } else {
                System.out.println("inbox");
            }
        }
    }
}
```

A scalar judgement failure throws `FailedException` at the call. A successfully returned scalar result is answered or unsure; its `answer()` can throw `UnsureException`. Bulk results and form entries retain their failed states, and their reads can also throw `FailedException`; catch `NoAnswerException` for either missing-answer state. State checks, detail, printing and storing a result are safe. A result keeps the original record as `item()`, excluding it from equality and hashing.

The [Java guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/java.md) covers every function and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/java/API.md) describes settings and ownership.
