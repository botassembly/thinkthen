# A Zig consumer works through C, but no supported package exists

Status: experiment complete. The queue owner decides whether to open a product ticket. Ian authorized the feasibility experiment and routed its work to Sol Medium. He did not authorize a release commitment. He can overturn the recommendation below.

## Gap

The ideal state makes the C interface the route to additional languages. A Zig caller can already use that interface, but the repository supplies no supported Zig module, installation contract or caller gate. A user must supply the linking and ownership code.

## Evidence

Beelink experiment `experiments/273-thinkthen-zig-c-interface/` tested Zig 0.15.2 on Linux x86_64 against an unchanged export of landed main `873b04abdeca56bcfc5fcc15b99665b7c32ee116`. It built the real C library offline under the heavy lock. No ThinkThen source changes or tool upgrades were required. Experiment 205 supplied earlier stand-in lessons. The new result uses the landed interface from ticket 0094 and ADR 0037.

The direct example is 14 formatted Zig lines. The thin wrapper is 81. It imports `thinkthen.h`, calls typed scalar and bulk decisions, copies JSON and errors into Zig-owned memory, frees C strings through C, and rejects interior NUL in C-string arguments. Counted evidence retains UTF-8 and NUL bytes.

The parent repeated the checks. The independent loopback fixture received exactly ten POST requests with the expected input multiset. Distinct bulk answers stayed in input order while barriers forced reverse reply completion. Exact saved error bytes survived a subsequent same-thread failure. Empty bulk, uncertainty, a non-retryable backend failure, malformed input, zero and invalid deadlines, a pre-fired cancellation token and Zig allocator cleanup also passed. Faulty wrapper copies that reversed results or changed an earlier error message failed at the intended assertions. A fresh read-only review accepted the corrected bounded prototype.

Evidence remains local and unpushed:

- `FINDINGS.md`: verified result, limits, code example and work packages.
- `REVIEW.md`: independent acceptance and its scope.
- `logs/check-20260926T190427480588Z/`: positive checks and exact commands.
- `attempts/plants-20260926T190520418374Z/`: faulty copies, failed checks and receipts.
- `inputs/manifest.json`: source and header identity.

The fixture used synthetic inputs, an explicit numeric loopback destination and a canary key in an allow-listed environment. No paid inference ran. Packet-level egress capture was not performed.

## Recommendation

Keep Zig 0.15.2 for the next step. If Zig support belongs in the product, open a small additive package ticket:

1. Define allocator, error, question and cancellation ownership in a consumer-facing API. Keep judgment rules and scheduling in Rust.
2. Add Zig build and package metadata, a consumer example, licensing, header/library compatibility checks and release-artifact discovery.
3. Prove clean installation without Rust, then add the selected platform and compiler combinations to an offline caller gate.
4. Test in-flight cancellation, simultaneous callers, thread-local error capture, failed bulk, allocation and JSON failures, and C allocation cleanup.

The C interface has an environment-based constructor and no public throttle setter. Any additional constructor or setting requires its own C-interface decision.

Only Linux x86_64 shared linkage with Zig 0.15.2 was measured. Static linkage, macOS, Windows, all nineteen C symbols, all ten functions through JSON, performance and live-model quality remain unproved. The prototype establishes interoperability. It does not establish production readiness.

JVM and Go remain future experiments. Both must account for the C interface's same-native-thread error retrieval when their runtimes schedule calls. No experiment number was claimed for either.
