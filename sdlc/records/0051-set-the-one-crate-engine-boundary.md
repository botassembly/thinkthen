# 0051: Set the one-crate engine boundary

Date: 2026-09-21

Status: landed

## Result

ADR 0017 now describes an implementable first step for the one-crate move. The command keeps arguments, credentials, input framing, its deliberately detached standard-input reader, output, diagnostics, and exit codes. A private bounded bridge sends framed rows to the engine and returns typed row events. The engine owns scoped request workers, scheduling, transport, recording, and cache locks. It never collects the whole run, and no engine worker survives a call.

The private engine failure carries a future public kind plus exact structured causes and ordered-stop metadata. The command remains the only layer that builds `Failure` and chooses an exit code. The move freezes requests, retries, order, width, memory bounds, recordings, output bytes, diagnostics, and exit codes. Fast refusal of a dead address stays in section 8 step 3.

The amended proof preserves core purity after the crate boundary disappears. `core` keeps its accepted API bans, cannot refer to `engine` or `cli`, and may use only the four runtime dependencies its old manifest allowed: `serde`, `serde_json`, `sha2`, and `thiserror`. Three planted failures prove those checks. The package proof also requires every CLI-only dependency to be optional, a library-only dependency graph and package, retained doctests, unwind for release libraries, and an abort-only command build path.

Job 2's DuckDB interrupt proof is complete. The remaining order is Job 3's shared conformance cases, then the one-crate merge.

## Review and proof

Independent design review rejected the first proposal for joining the CLI reader, listing completed Job 2 as future work, omitting package panic and dependency proofs, and leaving purity enforcement vague. The rewrite preserved the reader, scoped the engine workers, corrected the order, and made the checks concrete. Re-review accepted it.

Independent implementation review then found that the old core manifest's dependency boundary was still missing and that the original ADR paragraph kept a stale Job 2 order. The repair added the exact allowlist and an outer-dependency self-test, and marked the old order superseded. The same reviewer accepted the result.

Focused documentation and link checks and `git diff --check` pass. No Rust, manifest, gate, specification, how-to, package, or live-call behavior changed.
