# Find accepts one unit with a none option

Native Find admits one to 254 units when none is available and two to 255 without it. The CLI derives its count rule from the native owner. Diagnostics, specification and site guidance agree.

Saved-response checks cover choosing the original unit or none, source positions, probabilities and cache/replay reuse without another send. Empty input, one unit without none and overflow retain zero-send refusals. Twelve Rust and 33 CLI focused cases pass; release-only cases remain outside routine checks.

Review and integration found stale count expectations in the Rust request contract, C consumer, borrowed admission fixture and external Rust consumer. Each existing test now matches the admitted range while preserving blank, overflow, unread-tail, reader-failure and zero-send assertions. The external singleton keeps its original unit and none candidate and sends once. Fresh reviews accept the production change and each repaired consumer, ending at `0f5c33f9381eaf2385aa7b7744fab5d92ccb2df9`. The accepted coverage adds 174 nonblank Rust lines in total. Combined with 0515's independent deletion, the root ceiling is 189310.

Closure evidence covers all 1996 routine Rust cases, one doctest and 21 external consumer cases. The interrupted run resumed unrun cases instead of repeating unchanged passes. Routine schema, child-process, transform, pipeline and live-script self-tests pass. Remaining lint, formatting, strict workspace Clippy, documentation and public API inventory pass; earlier unchanged lint checks are reused. Ticket 0515 removes the obsolete private Python-reader helper that first interrupted the run, and the README named-answer correction is landed.

Evidence is in the Find lane's `target/0533-*.log` and `target/0533-external-*.log`, with integration logs `/tmp/thinkthen-closure-{unrun,final}.log` and `/tmp/thinkthen-final-closure-{tail,rest}.log`. These are ordinary check output, not new verification machinery. No paid, load, full-parity or platform checks ran.

## What the build taught us

An input-count rule belongs to the native function. Search both in-repo and external installed consumers when its contract changes; otherwise their old negative cases can contradict the new behavior. Saved answers establish the new reading without new provider calls.
