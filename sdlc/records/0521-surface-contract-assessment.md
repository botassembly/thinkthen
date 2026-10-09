# 0521: Assess and strengthen the surface contracts

## Verdict and scope

The strategy is sound. Central Rust rules and generated structural conversions reduce duplicated work while each host retains the calls, types, errors and resource handling its users expect. The binding ruling and author guide already capture both goals. The remaining weaknesses are handoffs: incomplete source representations, ownership gaps, late package decisions and acceptance criteria that can pass without proving the user experience.

This assessment uses clean main `c0bcf97911404a5ba3d98a4464a0cacd70ba116f`, dated 2026-10-09, with a separate preparation checkout. Three independent read-only reviewers examined shared contracts, language experience, and SQL/packaging. The scope includes 0494–0505, 0508, 0510–0520, and related 0383–0385, 0467, 0470 and 0484. Earlier rulings, the guide, ADRs 0101/0112/0125/0129, existing source, package scripts and retained reviews supply the evidence. This is a ticket and strategy assessment, not another implementation audit or a qualification run. No product build, paid call, platform workflow or release action was used.

## Findings and remedies

1. **High, design gap: generation can preserve an incomplete result.** The [0513 record](0513-generated-results.md) identifies native fields omitted by serialization. The [annotation serializer](../../crates/thinkthen/src/core/result/complete/annotation.rs) and [relation serializer](../../crates/thinkthen/src/core/result/complete/relate.rs) omit observations that their native values retain. The 0513 amendment requires those Rust presentation repairs and native-to-generated cases before complete adoption. Structural cause: testing a projection without comparing it to its source.

2. **High, design gap: constrained hosts need new complete views before declaration generation.** [FactsV1](../../libraries/c/src/ffi/carriers/metadata.rs) lacks current facts, and [legacy usage conversion](../../libraries/c/src/complete/metadata.rs) intentionally discards partial dimensions. The header generator faithfully emits those declarations. The 0505 amendment owns additive complete session views, independent presence, safe retained ownership and unknown-member preservation before Ada/COBOL generation. Compatibility layouts remain intact. Structural cause: fixing declarations while leaving the lossy source unchanged.

3. **High, design gap: native cancellation does not settle host cancellation.** [ADR 0129](../planning/adr/0129-owned-json-sessions.md) allows Pending after cancellation and eventual cleanup of a blocked worker. The 0516 amendment requires a stated wait strategy, prompt caller Task cancellation, safe disposal and honest pending facts, proved against a held provider at the installed host boundary. Other migrations apply that contract in their own scheduling model. Structural cause: testing the native handle while claiming the behavior of its wrapper.

4. **High, inconsistency: Windows instructions contradict bundled installation.** The original 0384 and 0385 outcomes require manual DLL placement or an absolute library path. Their amendments supersede those requirements, retain the earlier prose as history and consume the final packages without user path setup. 0383–0385 retain native candidate obligations. Structural cause: an earlier plan remains authoritative beside its replacement.

5. **High, design gap: final distribution can discard newly packaged assets.** [Registry assembly](../scripts/release-registry.py) selects the old JVM artifact set; [workflow checks](../scripts/release-workflow) assume Linux Objective-C; [artifact collection](../../.github/workflows/release.yml) has target-specific inputs. 0517 now owns the complete local artifact path and 0501 its common inventory. The final consumer artifact must install and run independently of the checkout. Structural cause: testing a family archive instead of the product users receive.

6. **Medium, ownership gaps: Python async and Rust Polars need actual owners.** [Python's worker](../../libraries/python/src/worker.rs) waits on its caller while releasing the interpreter. That does not establish asyncio progress. 0496 now owns async execution, cancellation and object truth behavior. The actual Rust Polars implementation lives under [public/frame.rs](../../crates/thinkthen/src/public/frame.rs), outside 0504's original implementation claims. The guide and 0504 distinguish it from Python Polars and name the real paths. Structural cause: ownership follows package folders rather than executing code.

7. **Medium, ordering risk: broad parent tickets can delay or depend on their own consumers.** 0513 originally required every installed language while 0516 depended on it. The pilot also promised all ten guide items while packaging came after family rollout. Shared generation now ends at the common graph and C# reference; host migrations own other targets. The pilot owns its first local NuGet package. Package design precedes JVM, Swift and Flutter implementation, and 0504 owns stable JVM foreign-function adoption. Actual hard prerequisites are recorded in ticket metadata; slice prerequisites use pm's accepted progress evidence. Structural cause: unclear completion boundaries.

8. **Medium, acceptance gap: SQL tests can pass against existing helpers.** Existing binary image functions do not prove that complete-result calls accept native bytes. The 0519 amendment requires a named complete-result binary call per database, native return-type assertions for DuckDB JSON, actual description discovery, and NULL partner cases with zero extension reads/sends. Structural cause: testing a nearby path instead of the promised path.

9. **Medium, scope risk: the CLI import ban can create unrelated work.** [CLI Request admission](../../crates/thinkthen/src/cli/request.rs) excludes maintenance commands, while [cache](../../crates/thinkthen/src/cli/cache.rs) and [status](../../crates/thinkthen/src/cli/status.rs) legitimately use private maintenance APIs. 0512 scopes its check to judgment execution/admission. 0508 is explicitly the single final after-sprint review. Structural cause: a broad structural check or repeated review expands the outcome it was meant to verify.

## Coverage and retained strengths

The 0511 admission amendment protects the pure-core boundary, semantic ownership and legitimate host checks. 0510 carries its cutoff through Request and counts cache reuse. 0514 provides the ten-item author guide. 0499 and 0500 preserve SQL authority and native composition; 0519 owns their later SQL user-experience changes. 0520 is a narrow compatibility bridge rather than a substitute for generation. 0484 retains distinct behavior checks rather than promising a deletion quota. Those directions need no replacement design.

The amended owners are 0494 for R; 0496 for Python/pandas/Python Polars; 0497 for Ruby; 0498 for TypeScript; 0504 for the remaining families and native Rust/Polars; 0505 for complete C views and constrained hosts; 0518 for Objective-C; 0470/0495 for the bounded DuckDB path; 0512 for CLI/MCP; and 0519 for SQL conventions. 0515 owns removal only after replacement parity. 0467 consolidates the migration guidance. The [guide](../../libraries/BINDING-AUTHOR.md) is the shared user-experience contract, and the [rollout](../planning/0-2-sdk-rollout.md) defines order without storing work status.

## Remaining strategy risks and stopping rule

The generator must not become another independently maintained schema or a general binding framework. Count its handwritten code and templates when judging reduction. Preserve a little explicit host glue when it makes the API safer and more natural. Do not force direct Rust bindings through JSON to make every language look alike.

A prompt cancel cannot force a permanently blocked provider to release native resources. The accepted session design states that limit. The pilot must make it visible through honest caller semantics and bounded active ownership, without promising final counts early.

Packaging remains the broadest external constraint. Supported platform pairs, runtime floors and native dependencies must be settled before the affected host work; actual macOS and Windows execution cannot be replaced by Linux checks. The release hold remains in force. Local preparation does not claim those checks passed.

Use one fresh review of these amendments and the existing focused documentation checks. Implementation owners retain their existing gates and installed consumers. Reuse applicable evidence, fix consequential failures, and avoid adding a new audit, checker or per-language report. None of these amendments authorizes a feature beyond the approved first-class surface outcome.

## Validation

A fresh independent review accepted the complete amendment diff at `4d59e7c6a` against the ruling, ADRs and cited source. It found no lost approved outcome, dependency deadlock or new proof system. Each amended ticket and 0521 carries that verdict through `pm ticket review`; it is a ticket/document review, not acceptance of later implementation.

`pm lint records`, `pm lint links`, `pm lint coherence`, `python3 sdlc/scripts/tickets` and `git diff --check` pass. A read-only check across the ticket directory found unique IDs, existing explicit prerequisites and no dependency cycle. Product tests were not run for this documentation-only change. The preparation stays on its ticket branch for the coordinator's ordinary landing, preserving active code checkouts.
