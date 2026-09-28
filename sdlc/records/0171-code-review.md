# 0171 code review and integration

A fresh independent Codex Sol High reviewer examined the warning, saved metadata and byte-preserving question-file edits at `03ace8b2`. High reasoning was used for the interacting warnings and writes to an existing user file. The review accepted the measured growth and found two corrections: invalid present batch metadata was treated as absent, and a detailed-row assertion could pass without observing any rows.

The same reviewer accepted `d67506f2`. Audit and diff now share a fallible reader that distinguishes missing legacy metadata from a malformed setting. Audit passes the validated setting set to its write path. An outside-in regression pins the line-2 refusal, exit 2, empty stdout and unchanged question bytes. The warning regression asserts exactly two result rows before checking their metadata.

The coordinator approves the reviewed increase of 592 nonblank Rust lines over `f7080025`: 246 product and 346 test lines. The review examined the shared typed reader, print-once warning state, byte-preserving splice and distinct command contracts. It found no duplicate implementation or test harness requiring removal. Existing equivalent helpers were reused. The original estimate was not an allowance to evade the measured ceiling.

Integration onto main `b9be8589` conflicts only in the derived ratchet. The measured merged total is 79,580: main's 78,988 plus 592. Product source is unchanged from the accepted candidate; the independent main change is the previously reviewed 0207 conformance backend. Formatting, source policy, ticket evidence, pages, ratchet equality and diff hygiene are checked on the merged tree. The candidate's focused tests, strict all-target/all-feature Clippy, settings check and audit specification checks remain valid; the build record gives their exact scope.

The interrupted ticket-only surfaces run remains partial and is not a passing gate. No repeat full suite is warranted by this merge. The next command batching integration checkpoint follows 0172's local implementation and correction review, covering 0170 facts, 0171 warnings and 0172 context together. The coordinator will name the exact selection at that point. Paid proofs remain separate and require authorization.

Register 14 is done. The builder keeps its context for 0172. Its preparation must inventory all shared constructors, distinguish absent from malformed metadata and assert that a proof observes the expected rows before comparing them.
