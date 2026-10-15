# ThinkThen for Java, Kotlin and Scala

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```kotlin
implementation("io.github.botassembly:thinkthen-jvm:0.2.0")
```
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.
```java
import thinkthen.Engine;
import thinkthen.Inputs.*;
public class RegistryExample {
    public static void main(String[] args) throws Exception {
        var teams = new Ordered<AuthoredDescription>();
        teams.entry("billing", new AuthoredDescription("Invoices, fees, and refunds."));
        teams.entry("shipping", new AuthoredDescription("Parcels and delivery."));
        teams.entry("account", new AuthoredDescription("Logins and passwords."));
        var teamQuestion = new RequestQuestionDefinition().value(new RequestDefinitionAuthoredChoose()
            .choose(new AuthoredQuestionText("Which team owns this?")).options(new AuthoredOptions(teams)));
        try (var engine = new Engine(new EngineSettings())) {
            var refunds = new String[]{"Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back."};
            for (var index = 0; index < refunds.length; index++) {
                var refund = engine.decide(new RequestQuestionText().text("Does the customer ask for a refund?"), new RequestInputText().text(refunds[index])).join();
                var isRefund = refund.value(); var odds = refund.probability();
                assert Boolean.valueOf(new boolean[]{true, false, true}[index]).equals(isRefund) && odds.compareTo(new java.math.BigDecimal(new String[]{"0.99", "0.01", "0.54"}[index])) == 0;
            }
            var messages = new String[]{"Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password."}; for (var index = 0; index < messages.length; index++) {
                var team = engine.choose(teamQuestion, new RequestInputText().text(messages[index])).join();
                var owner = team.value(); var odds = team.probability();
                assert owner.equals(new String[]{"billing", "shipping", "account"}[index]) && odds.compareTo(java.math.BigDecimal.ONE) == 0;
            }
        }
    }
}
```

`value()` gives the answer; `probability()` gives nullable `Double` and `List<Double>` odds. Record inputs return ordered lists. `results()` keeps typed records and `facts()` keeps call observations. Kotlin reads properties; Scala uses `Option` for missing scalar answers. The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/java/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
