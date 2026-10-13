# ThinkThen for cobol

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. Failed calls remain separate from answers.

## Install

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-cobol-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Configure a model backend before running a judgment.

## Example

This excerpt starts a decision after you fill the question and input records. The packaged `examples/session.cob` includes record construction, result reading and cleanup.

```cobol
call "TT_SESSION_DECIDE" using by value engine
   by reference tt-n-cobol-RequestCall session-owner
   returning status-code
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
