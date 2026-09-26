# A Zig consumer works through C, but no supported package exists

Status: the local package experiment is complete, with a verified native cancellation blocker. The queue owner decides the product integration ticket and release. Ian authorized the expanded Linux engineering and testing, not publication. He can overturn the thin wrapper and explicit native-archive recommendation.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 273 now supplies the Zig implementation, package metadata, examples, documentation, tests and local archive rehearsal. The repository still needs reviewed integration, a native cancellation repair and final-release verification before offering supported Zig use.

## Final evidence

Beelink experiment `experiments/273-thinkthen-zig-c-interface/stage2/` uses Zig 0.15.2 on Linux x86_64 with glibc. Its immutable source is landed main `e0a6150a0325d92b93ac8f4e504ffbfc3792fba9`; the parent verified all 4,761 exported blobs. No ThinkThen implementation, C header or compiler upgrade was needed. The local native archive uses a release build with remapped builder paths, not the first stage's debug library.

The six-file `package/` exports a real Zig module. It wraps typed scalar/bulk decisions, recognize, relate and the generic JSON door for all ten functions. Rust retains judgment, grammar and scheduling. Zig owns copies of results and error metadata. The package rejects interior NUL in C strings and preserves counted evidence bytes. Its documentation covers blocking calls, thread-safe allocators, engine/token lifetime, one-shot cancellation and the known native limitation.

The worker and parent separately completed the final gate. Parent evidence is `logs/gate-20260926T212430Z/`: 47 outer subprocess receipts and nine child receipts per isolated consumer matched expected outcomes. Two shared and two static-C consumers each accepted 48 independently counted requests across example, verb/ownership matrix, allocation failures, simultaneous callers, held deadlines, cancellation and fresh-token recovery. Actual shared loading used the supplied archive. Static mode had no ThinkThen shared dependency; it is not a fully static executable. Each consumer had a fresh cache, an unrelated path containing spaces, a real archive dependency, and a private network/filesystem namespace without Rust, Cargo, repository source or previous build outputs.

The gate verifies hashes, archive members, source/member byte equality, C layout, version macros and native identity evidence. Planted stale archives, bad header versions, private bytes, source-tree fallback and sanitizer misuse failed at their intended checks. Planted timeout/interruption descendants stopped while an unrelated control survived. Two fresh read-only reviews ran. The second found no additional experiment-side blocker after the approved corrections. The parent accepts the bounded handoff, not release readiness.

## Native blocker

A scalar C call can return success after its cancellation token fires during an accepted, held HTTP request. All four parent consumers reproduced it. Three independent Python ctypes trials also reproduced it without Zig. Held bulk cancellation, held deadlines and fresh-token recovery passed.

[The existing cancellation issue](2026-09-26-cancelled-c-scalar-call-can-return-success.md) owns repair. The wrapper does not hide the fault. The gate explicitly reports `FINDING` and exits 1. Matching expected subprocess exits does not make this an all-pass contract.

## Handoff and remaining work

Local, unpushed deliverables:

- `stage2/package/`: the six source-package files.
- `stage2/fixtures/` and `stage2/gate.sh`: the implemented Linux tests and offline gate.
- `stage2/artifacts/manifest.json`: source pin, compiler versions, archive/header/library hashes, contents, exports and dependencies.
- `stage2/HANDOFF.md`: exact proposed product destinations and path adaptations.
- `stage2/CASE-LEDGER.md`, `FINDINGS.md`, `REVIEW.md`, and `reviews/`: coverage, parent verification, limits and independent reviews.
- `stage2/parent-verification/`: parent source checks, gate launch evidence and unchanged-source hashes.

The integration ticket should copy `package/` to `libraries/zig/`, place the named tests beside it, adapt the gate to the repository's artifact/tool/scratch paths and attach it to the offline surface rung. It should add source-archive release packaging, public API/install documentation and specification registry entries. The release owner must decide distribution and compatibility policy, rebuild from the final release commit and rerun the gate. Do not publish the experiment's rehearsal binaries. After repairing native cancellation, replace the rehearsal's exact known-failure expectation with the strict contract and a genuinely passing gate.

Keep Zig 0.15.2 for this measured target. Start with a thin optional source package and a separately supplied matching native archive. The current shared mode embeds the supplied library path and requires rebuilding if it moves. A relocatable installation policy or automatic download is a separate product choice. The C constructor remains environment-based and has no public throttle setter.

## Limits and earlier evidence

An earlier intermediate static run accepted 33 rather than 34 requests without identifying the missing request. The final gate checks the input multiset; all eight final worker/parent consumers passed their count checks. The original shortfall remains unexplained. Zig allocation checks and C-caller AddressSanitizer checks passed, including a deliberate misuse. Native Rust allocations were not instrumented, and Valgrind was unavailable. The C API exposes no runtime binary ABI identity. Exhaustive diagnostic secrecy, other targets, other Zig versions, performance and live-model quality remain unproved. Synthetic loopback replies do not measure model accuracy. No paid inference ran.

The first-stage proof remains intact at the experiment root. It tested source `873b04abdeca56bcfc5fcc15b99665b7c32ee116` with a 14-line direct example, an 81-line wrapper, ten independently counted requests and deliberately faulty ordering/message-lifetime copies. Its `FINDINGS.md`, `REVIEW.md` and `HARVEST.md` describe that narrower result. Experiment 205 supplied earlier stand-in lessons; ADR 0037 and ticket 0094 establish the landed C interface.

JVM and Go remain future experiments. Both must account for same-native-thread C error retrieval when their runtimes schedule calls. No experiment number was claimed for either.
