# ADR 0006: The first demos change four settled rules

- Status: Accepted
- Date: 2026-09-19

An agent decision under ADR 0005. Ian can overturn any line cheaply. No code depended on these rules yet, except `Meta` from ticket 0001, which ticket 0003 now updates.

## Context

Twelve demos were written against the specification before any verb existed. `demos/FINDINGS.md` gathers what they found. Four findings touch pages marked Settled, and changing a settled page takes a record.

## Decision

**Exit code 3 means unsure and nothing else.** `--status` needs a pass mark, so an unassessed answer cannot occur under it.

**A plan always shows six fields, and `key_env` may be `null`.** A script can now prove that no key would be sent. The page also says plainly that the plan carries the evidence.

**An ad-hoc backend is a URL, an adapter, and a model together.** The earlier rule let a new URL borrow the built-in profile's adapter and model. That posts one vendor's format and model name to a server that may speak another. A model alone may still replace a profile's model.

**A result names the URL that answered, and `backend` is `null` for an ad-hoc backend.** The earlier shape would have reported the name `jev` for a request that went elsewhere.

## Findings taken into drafts and plans

Recording and replay need their own page, and a replay miss is a local failure. Stream rows need a way to tell a judgment from a failed record. A segment needs line numbers. Judged columns for JSONL should arrive with `where`. Options that do nothing in the chosen mode should be usage errors. The saved question file needs a pinned grammar. No demo needed `how`, `match`, `--invert`, `--abstain-on`, or `--from`, and each is now a candidate to cut or hold.

## Findings refused for now

A `rule` field and a `reason` field in the assessment wait for `which`, where an unsure answer has more than one cause. For `if` there is one rule and one cause.

## Consequences

The demo that dropped file-list input confirmed the drop: `find`, `sort -z`, and `xargs -0 -P` already do the job. It also found that a loop over files sits outside every request limit, and the help text has to say so.
