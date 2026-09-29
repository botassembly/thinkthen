# Update copied audit and diff wording in marketing examples

Status: closed 2026-09-29 by the marketing lead. Beatles Bench 7240b86c keeps the pinned build's audit and diff blocks, which its byte check holds, and adds notes that ThinkThen main at ce04682c prints `0 not sure` and `unsure -> no 44`. The words come from a real run of that build over the bench's saved answers. The marketing vocabulary names `unsure` as the machine word at mktg 3fa82c0. No site page carried the old words. Fresh read-only review accepted the wording.

Claimed by the marketing lead on 2026-09-29 for 0.1. Marketing fixes, replays and lands it, and closes this issue with the commit. SQL examples wait for ADR 0105 (workspace experiment 2038) so each page is rewritten once.

- The marketing copy at `functions/audit/README.md` shows `0 unresolved`. The current `audit --table` count line reads `0 not sure`.
- The marketing copy at `functions/diff/README.md` line 44 shows `unresolved -> no 44`. The current `diff --table` token reads `unsure -> no 44`.
- The product vocabulary can list `unsure` as the machine word for a not sure answer. Its prose word remains "not sure".

Done when the marketing owner updates those two copied examples and the vocabulary note. No external message is authorized by this handoff.
