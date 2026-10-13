# ThinkThen for Java, Kotlin and Scala

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```kotlin
implementation("io.github.botassembly:thinkthen-jvm:0.2.0")
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```java
try (var engine = new thinkthen.Engine(new thinkthen.Inputs.EngineSettings())) {
    var call = engine.decide(
        new thinkthen.Inputs.RequestQuestionText().text("Does this ask for a refund?"),
        new thinkthen.Inputs.RequestInputText().text("Please refund my order."));
    System.out.println(call.join().decide());
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
