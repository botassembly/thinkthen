# The empty evidence refusal names the rule backwards

Status: Closed on 2026-09-22. Merged into 2026-09-22-command-wording-and-help-fixes-for-0-1.md.

Empty input to any command that reads one document prints "evidence is text, not white space." A stranger reads the first half as a claim that there is text, and goes looking for it.

## Reproduction

    $ thinkthen decide 'Is this here?' --url http://127.0.0.1:8804 < /dev/null
    thinkthen: evidence is text, not white space
    $ echo $?
    2

The message appears in a pipe at its worst, because an upstream stage that printed nothing hands the next stage an empty stream. The candidate-6 use-case flow hits exactly this, and its issue cross-references this one.

## Expected

The condition is that the evidence is empty or blank, and the message should name that condition. The exit code 2 is correct. `specification/decide.md` line 15 says an empty document is a usage error, because a judgment about nothing is a mistake in the pipeline. The message shape is shared across question, model name, URL, and meaning refusals in `crates/thinkthen-core/src/text.rs`, so the fix belongs to the evidence case alone.

## How bad it is for a user

Minor. The rule is right and the sentence points the wrong way.

Found by experiment 218, wave 1, areas 3 and 4.
