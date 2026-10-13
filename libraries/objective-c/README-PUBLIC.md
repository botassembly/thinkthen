# ThinkThen for Objective-C

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

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
