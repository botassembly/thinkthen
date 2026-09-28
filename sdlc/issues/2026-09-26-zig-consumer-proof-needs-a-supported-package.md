# A Zig consumer works through C, but no supported package exists

Status: the local package experiment is complete. Ticket 0166 repaired the native cancellation blocker, and a post-fix rerun at main `22f36d00` passed the strict contract. The queue owner decides the product integration ticket and release. Ian authorized the expanded Linux engineering and testing, not publication. He can overturn the thin wrapper and explicit native-archive recommendation.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 273 now supplies the Zig implementation, package metadata, examples, documentation, tests and local archive rehearsal. The repository still needs reviewed integration, release packaging and final-release verification before offering supported Zig use.

## Final evidence

Beelink experiment `experiments/273-thinkthen-zig-c-interface/stage2/` uses Zig 0.15.2 on Linux x86_64 with glibc. Its immutable source is landed main `e0a6150a0325d92b93ac8f4e504ffbfc3792fba9`; the parent verified all 4,761 exported blobs. No ThinkThen implementation, C header or compiler upgrade was needed. The local native archive uses a release build with remapped builder paths, not the first stage's debug library.

The six-file `package/` exports a real Zig module. It wraps typed scalar/bulk decisions, recognize, relate and the generic JSON door for all ten functions. Rust retains judgment, grammar and scheduling. Zig owns copies of results and error metadata. The package rejects interior NUL in C strings and preserves counted evidence bytes. Its documentation covers blocking calls, thread-safe allocators, engine/token lifetime, one-shot cancellation and the known native limitation.

The worker and parent separately completed the final gate. Parent evidence is `logs/gate-20260926T212430Z/`: 47 outer subprocess receipts and nine child receipts per isolated consumer matched expected outcomes. Two shared and two static-C consumers each accepted 48 independently counted requests across example, verb/ownership matrix, allocation failures, simultaneous callers, held deadlines, cancellation and fresh-token recovery. Actual shared loading used the supplied archive. Static mode had no ThinkThen shared dependency; it is not a fully static executable. Each consumer had a fresh cache, an unrelated path containing spaces, a real archive dependency, and a private network/filesystem namespace without Rust, Cargo, repository source or previous build outputs.

The gate verifies hashes, archive members, source/member byte equality, C layout, version macros and native identity evidence. Planted stale archives, bad header versions, private bytes, source-tree fallback and sanitizer misuse failed at their intended checks. Planted timeout/interruption descendants stopped while an unrelated control survived. Two fresh read-only reviews ran. The second found no additional experiment-side blocker after the approved corrections. The parent accepts the bounded handoff, not release readiness.

## Sealed native blocker evidence

This section records the pre-fix failure. Ticket 0166 repaired it; the post-fix verification below records the passing strict proof.

At the sealed stage-two pin, a scalar C call could return success after its cancellation token fired during an accepted, held HTTP request. All four parent consumers reproduced it. Three independent Python ctypes trials also reproduced it without Zig. Held bulk cancellation, held deadlines and fresh-token recovery passed.

[The closed cancellation issue](closed/2026-09-26-cancelled-c-scalar-call-can-return-success.md) records the repair. The stage-two wrapper did not hide the fault. The sealed stage-two gate reported `FINDING` and exited 1. Matching expected subprocess exits does not make this an all-pass contract.

## Handoff and remaining work

Historical stage-two artifacts remain local and unchanged:

- `stage2/package/`: the six source-package files.
- `stage2/fixtures/` and `stage2/gate.sh`: the implemented Linux tests and offline gate.
- `stage2/artifacts/manifest.json`: source pin, compiler versions, archive/header/library hashes, contents, exports and dependencies.
- `stage2/HANDOFF.md`: exact proposed product destinations and path adaptations.
- `stage2/CASE-LEDGER.md`, `FINDINGS.md`, `REVIEW.md`, and `reviews/`: coverage, parent verification, limits and independent reviews.
- `stage2/parent-verification/`: parent source checks, gate launch evidence and unchanged-source hashes.

The integration ticket must copy `post-fix/package/` to `libraries/zig/`, place the tests from `post-fix/fixtures/` beside it, and adapt `post-fix/gate.sh` to the repository's artifact/tool/scratch paths and offline surface rung. The sealed stage-two artifacts above retain the earlier evidence; the post-fix copy is the integration source. It should add source-archive release packaging, public API/install documentation and specification registry entries. The release owner must decide distribution and compatibility policy, rebuild from the final release commit and rerun the gate. Do not publish the experiment's rehearsal binaries. The post-fix experiment copy already requires strict success and passed. Product integration must carry that contract and rerun its gate at the final release pin.

Keep Zig 0.15.2 for this measured target. Start with a thin optional source package and a separately supplied matching native archive. The current shared mode embeds the supplied library path and requires rebuilding if it moves. A relocatable installation policy or automatic download is a separate product choice. The C constructor remains environment-based and has no public throttle setter.

## Limits and earlier evidence

An earlier intermediate static run accepted 33 rather than 34 requests without identifying the missing request. The final gate checks the input multiset; all eight final worker/parent consumers passed their count checks. The original shortfall remains unexplained. Zig allocation checks and C-caller AddressSanitizer checks passed, including a deliberate misuse. Native Rust allocations were not instrumented, and Valgrind was unavailable. The C API exposes no runtime binary ABI identity. Exhaustive diagnostic secrecy, other targets, other Zig versions, performance and live-model quality remain unproved. Synthetic loopback replies do not measure model accuracy. No paid inference ran.

The first-stage proof remains intact at the experiment root. It tested source `873b04abdeca56bcfc5fcc15b99665b7c32ee116` with a 14-line direct example, an 81-line wrapper, ten independently counted requests and deliberately faulty ordering/message-lifetime copies. Its `FINDINGS.md`, `REVIEW.md` and `HARVEST.md` describe that narrower result. Experiment 205 supplied earlier stand-in lessons; ADR 0037 and ticket 0094 establish the landed C interface.

Go ran as experiment 274 and the JVM as experiment 289, renumbered from a collided 275 claim. Their wrappers account for thread-local C error retrieval; each has its own issue and still needs product integration.

## Post-fix verification (2026-09-27)

Ticket 0166 landed the engine fix. A parent-verified rerun at pin `22f36d0006fd34e7390a71d15c0458e493f3e844` (header `7fdca29a...`) rebuilt the native library offline and passed the complete copied gate with exit 0 across two shared and two static-C isolated consumers: held scalar returns cancellation code 5 with untouched outputs, held bulk 5, held deadline 3, fresh-token recovery passes, and a direct-C ctypes proof (`STRICT_C_CANCEL_PASS`) shows code 5, drained accepted request, spent-token refusal, and exact counted states. Evidence: `post-fix/POST-FIX-REPORT.md`, worker gate `post-fix/logs/gate-20260927T140739Z`, parent rerun `post-fix/logs/gate-20260927T142411Z`. The recognize schedule now sends two requests per name on main; consumers must re-derive counts per pin. Integrators carry the strict-pass expectations into the product ticket and rerun on its final release pin.

## Dependencies and installation (Beelink, recorded 2026-09-27)

Host: Ubuntu 24.04.3 LTS, kernel 6.17.0-35-generic, glibc 2.39, x86_64. Zig 0.15.2 at `~/.local/opt/zig-0.15.2` (upstream tarball, pre-existing). Native rebuild needs Rust 1.95.0 with an offline Cargo registry copy. Consumers linking C need clang 18.1.3 (ASan checks) and bwrap for namespace-isolated installs; fixtures are Python 3.12. The package ships as a Zig source archive plus a separate matching native C archive with an explicit absolute path option; no automatic artifact download.

## CI and release direction (Ian, 2026-09-27)

Each port must state its dependencies and how they were installed, and the product must also build and release on GitHub Actions `ubuntu-24.04` runners (x86_64 matches this rehearsal; Zig and the pinned Go/JDK/Kotlin/Scala versions must be fetched or pinned in the workflow), publish native archives through GitHub Releases, and let consumers install directly or through the proper language repository. The queue owner owns the workflow, release, and publishing tickets; nothing is published from the experiments.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, multiset 50->42, retry 3->4 source-proven). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.
