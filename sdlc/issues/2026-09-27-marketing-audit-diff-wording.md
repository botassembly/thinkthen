# Update copied audit and diff wording in marketing examples

Status: open. Owner: coordinator for the marketing follow-up. Ticket 0152 Part B changes the command's machine word to `unsure` and its prose count to "not sure"; this repository does not own the marketing copies.

Claimed by the marketing lead on 2026-09-29 for 0.1. Marketing fixes, replays and lands it, and closes this issue with the commit. SQL examples wait for ADR 0105 (workspace experiment 2038) so each page is rewritten once.

- The marketing copy at `functions/audit/README.md` shows `0 unresolved`. The current `audit --table` count line reads `0 not sure`.
- The marketing copy at `functions/diff/README.md` line 44 shows `unresolved -> no 44`. The current `diff --table` token reads `unsure -> no 44`.
- The product vocabulary can list `unsure` as the machine word for a not sure answer. Its prose word remains "not sure".

Done when the marketing owner updates those two copied examples and the vocabulary note. No external message is authorized by this handoff.
