# ThinkThen for objective-c

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. Failed calls remain separate from answers.

## Install

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-objective-c-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Configure a model backend before running a judgment.

## Example

Use Apple Foundation and ARC. This asynchronous call reports completion through its callback.

```objc
NSError *error = nil;
TTFoundationClient *client = [[TTFoundationClient alloc] initWithSettings:nil error:&error];
TTTask *task = [client decide:@{@"kind":@"text", @"text":@"Does this ask for a refund?"}
    input:@{@"kind":@"text", @"text":@"Please refund my order."}
    options:nil feed:nil completion:^(TTCall *call, NSError *failure) {
        if (failure) { NSLog(@"%@", failure.localizedDescription); return; }
        NSLog(@"%@", call.terminal.facts.value.callId.value);
    } error:&error];
// Retain task while the request is running.
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
