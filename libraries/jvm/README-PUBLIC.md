# ThinkThen for Java, Kotlin and Scala

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```kotlin
implementation("io.github.botassembly:thinkthen-jvm:0.2.0")
```

## Example

Configure a model backend before running this example.

```java
try (var engine = new thinkthen.Engine(new thinkthen.Inputs.EngineSettings())) {
    var call = engine.decide(
        new thinkthen.Inputs.RequestQuestionText().text("Does this ask for a refund?"),
        new thinkthen.Inputs.RequestInputText().text("Please refund my order."));
    System.out.println(call.join().decide());
}
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
