# Java SDK

The Java SDK asks questions about text from your Java code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Add `io.github.botassembly:thinkthen-java:0.2.0` to Maven or Gradle. It needs Java 22 or later and carries its own engine. Run your program with `--enable-native-access=ALL-UNNAMED`. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```java
import dev.thinkthen.*;

public class Example {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            boolean isRefund = client.decide(
                "Does the customer request a refund?",
                "Please refund my order.").answer();
            System.out.println(isRefund);
        }
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

```java
import dev.thinkthen.*;

public class Unsure {
    public static void main(String[] args) {
        var bars = DecideOptions.builder()
            .threshold(Threshold.band(0.2, 0.8)).build();
        try (var client = ThinkThen.builder().build()) {
            var isRefund = client.decide(
                "Does the customer request a refund?",
                "I was charged twice for one invoice.", bars);
            try {
                if (isRefund.answer()) {
                    System.out.println("Send to refunds.");
                } else {
                    System.out.println("Leave in the inbox.");
                }
            } catch (UnsureException notSure) {
                System.out.println("Not sure. Ask a person.");
            }
        }
    }
}
```

Output:

```text
Not sure. Ask a person.
```

## Next

The [Java SDK page](https://thinkthen.dev/sdks/java/) shows all ten functions. The [Java guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/java.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/java/API.md) cover every option.

## Usage accounting

```java
import dev.thinkthen.*;

public class UsageExample {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            client.decide("Ready?", "A message");
            Usage usage = client.usage();
            System.out.println(usage.attempts());
            System.out.println(usage.inputTokens().reported());
        }
    }
}
```

Snapshots are immutable cumulative totals for one explicit client and remain available after close. Counters include retries; token totals are provider-reported subtotals with separate unknown-attempt counts. Null token totals indicate overflow. Use an independent client for an isolated run. No prices or invoice totals are inferred.

## Request allowance

```java
import dev.thinkthen.*;

public class AllowanceExample {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().maxRequests(100).build()) {
            client.decide("Ready?", "A message");
        }
    }
}
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.
