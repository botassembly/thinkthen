# 0569: Make annotate take shared context once, name score levels, and fix status

Status: OPEN.

Milestone: 0.2

## Outcome

`annotate` accepts `--context` as its help says. It sends a shared document once per request instead of once per question. A score answer reports the level the question names. `thinkthen status` works on a machine with a fresh cache.

## Evidence

- Starts from: a 2026-10-12 report from a team that scored three 2,000-word documents against 21 questions over 25 live requests, with no failures.
  - `annotate --help` lists `--context`, but `annotate` refuses it. The team pasted the document into each record instead.
  - `annotate` repeats the whole record text inside every question. `--plan` estimated 1.65 MB and 21 requests for three records and 21 questions.
  - Score answers came back numbered 0 to 4 when the question named a 1 to 5 scale.
  - `thinkthen status` failed with "could not read the local cache or usage state" on the Beelink.
- Keeps: annotate's output shape and every other command's handling of `--context`.
- Changes:
  - Make `annotate` accept `--context`, or remove it from `annotate --help`. Prefer acceptance when the request shape already carries shared context.
  - Send the record once per request, with the questions referring to it. Show the smaller size in `--plan`.
  - Return the score level the question names. If a scale has no names, document the numbering in the score help and the result schema.
  - Find why `status` cannot read the cache or usage state on a fresh machine, and fix it. A missing cache or usage file reads as empty.
- Proof:
  - An outside-in CLI test shows `annotate --context FILE` runs, and that `--plan` for three records and 21 questions is smaller than before.
  - An outside-in test shows a 1 to 5 score question returns levels 1 to 5.
  - An outside-in test shows `status` succeeds with an empty cache directory and no usage file.
  - Routine `test` and `lint` pass.
- Defers: nothing.

## Progress
