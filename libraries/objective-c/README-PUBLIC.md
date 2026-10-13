# ThinkThen for Objective-C

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-objective-c-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

Use Apple Foundation and ARC. This asynchronous call reports completion through its callback.

```objc
NSError *error = nil;
TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
TTTask *task = [client decide:@{@"kind":@"text", @"text":@"Does this ask for a refund?"}
    input:@{@"kind":@"text", @"text":@"Please refund my order."}
    options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
        if (failure) { NSLog(@"%@", failure.localizedDescription); return; }
        NSError *answerError = nil;
        TTPresence<NSNumber *> *answer = [call decideAnswer:&answerError];
        if (answerError) { NSLog(@"%@", answerError); return; }
        NSLog(@"%@", answer.value);
    } error:&error];
// Retain task while the request is running.
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
