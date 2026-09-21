# Question file refusals name the wrong thing

Status: Open

Two refusal messages on the question-file path misdiagnose the failure. A single question file that holds the verb plus an unknown key is told it holds no verb. A question set without its wrapper is told it holds no key the user never wrote.

## Reproduction

A single question file with the verb present and an extra key, 2026-09-21:

    $ printf '{"verb":"decide","text":"Does this ask for a refund?","urgency":"high"}' > q.json
    $ thinkthen decide @q.json --url http://127.0.0.1:8802 < one.txt
    thinkthen: a question file holds one of `decide`, `choose`, `tag`, or `score`
    $ echo $?
    5

The file holds `decide`. The failure is the extra `urgency` key, and the message sends a stranger looking for a missing verb. The same sentence fires for a question file whose only fault is a top-level `version` key, so every shape failure under the single-file path shares one misdiagnosing message.

The question-set path names the offending key for the same class of mistake, so the two paths disagree:

    $ thinkthen annotate set-with-unknown-key.json ...
    thinkthen: the question set holds no key `backend`

A question set missing its `questions` wrapper names the user's first key instead of the missing structure:

    $ thinkthen annotate triage.json --jsonl --field /body < inbox.jsonl
    thinkthen: the question set holds no key `unresolved`
    exit 5

The file's first question key is `unresolved`. The grammar needs `{"version": 1, "questions": {...}}`, and nothing in the message points there.

## Expected

The exit 5 behavior is right; `specification/question-file.md` rule 1 says no key beyond the fixed order appears. The messages should name the offending key on the single-file path, and the missing wrapper on the set path.

## How bad it is for a user

Minor. The refusal is correct and the diagnosis points elsewhere.

Found by experiment 218, wave 1, areas 2 and 3.
