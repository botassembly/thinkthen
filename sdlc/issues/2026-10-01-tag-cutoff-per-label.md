# `tag` takes a separate cutoff for each label

Status: open. Filed 2026-10-01 from release QA's message "Two ideas for 0.2", which Ian asked QA to send. Not approved. Owner: the queue owner.
Kind: idea
When: a user asks for it
Milestone: later

A label that is rare or costly to miss could get its own cut.

Today one `--threshold` applies to every `tag` label. QA asked to record this idea only and wait for a user who asks.

QA's compatibility notes, to keep 0.1 users whole:

- An unused new setting leaves the question digest unchanged, so cached answers and audit records still match.
- Exit codes, messages and existing output fields keep their meanings.
- The language packages gain a new call or an optional setting, never a new argument on an existing call. The C JSON door can take a new key.
