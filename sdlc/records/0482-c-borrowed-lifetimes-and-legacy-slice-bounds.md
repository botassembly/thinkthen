# 0482: Guard legacy C extents and describe borrowed failures

The C header now states when borrowed failure messages and facts expire, including engine-free, thread-exit, and the distinct null-engine slot. The legacy text and pointer/length array readers use the existing typed-facts extent checks before creating slices. Representationally impossible extents return the established usage error. The C ABI and normal request behavior remain unchanged. A separate formatting correction keeps the assertions in `tests/door/current.rs` intact.

The public C door test passed all 73 cases under address and leak sanitizers. Its new cases pass one-byte owned storage with impossible text and array extents through decide, recognize, many, and relate. They check untouched outputs and zero sends without dereferencing invalid storage. C Clippy, formatting, policy, both C ratchets, and the header export self-test passed. Fresh read-only code review accepted `da9ce6e04` with no findings. On that reviewed source, the full test, lint, and spec each exited 0: 1,773 workspace tests, 339 library-only tests, 23 external-consumer tests, 19 binding smokes including installed C, and 24 green demos.

Invalid but representable caller storage remains the caller's responsibility. No C ABI change, generated binding work, or ownership redesign belongs to this ticket.

## What the build taught us

The typed-facts extent guard was the source to reuse for legacy input readers. One public C consumer can test the refusal boundary and request count without running undefined behavior on the unguarded version.
