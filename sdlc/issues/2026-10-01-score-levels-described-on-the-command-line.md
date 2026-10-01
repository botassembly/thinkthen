# `score --level NAME=MEANING` describes a level on the command line

Status: open. Filed 2026-10-01 from release QA's message "Two ideas for 0.2", which Ian asked QA to send. Not approved. Owner: the queue owner, in 0.2.
Kind: idea
When: 0.2 work starts on main
Milestone: 0.2

A user could describe `score` levels without writing a question file.

Today a level description lives only in a question file. `choose` takes `--option LABEL=DESCRIPTION` and `tag` takes `--label LABEL=DESCRIPTION`, so `score` is the one verb without a command-line form. `score --level NAME=MEANING` would fill the same described-levels map that `specification/question-file.md` already defines, so it adds no grammar.

QA's compatibility notes, to keep 0.1 users whole:

- An unused new setting leaves the question digest unchanged, so cached answers and audit records still match.
- Exit codes, messages and existing output fields keep their meanings.
- The language packages gain a new call or an optional setting, never a new argument on an existing call. The C JSON door can take a new key.
