---
flow: build
priority: 249
opens: libraries sdlc/issues sdlc/tickets
---

# 0249: Merge the language bindings

Status: **blocked** (not one of the usual three; deliberate). Ian holds this ticket and hand-delivers it to the queue owner. Work starts only on his explicit go-ahead. Filed 2026-09-28 by the consumer-language program (local experiments 273-301). No merge step has been performed: `libraries/` is untouched by the program, nothing is published, no CI is configured.

Number corrected from 0247 during queue intake because the existing DuckDB lane holds 0247 and the usage-persistence design holds 0248. Scope and the explicit start hold are retained. This handoff consolidates existing J8 rows; it is not another independent issue.

## Evidence

- Starts from: the sealed consumer experiments 273–301, their re-pin reports and the exact package-copy table in the language merge runbook.
- Keeps: accepted C result envelopes, packed batches, entities-only relation state, default retries, subset cache identity and exact per-gate arrival counts. The drift issue records their confirmation.
- Changes: integrates reviewed package copies into libraries, adds each actual binding to the root README and supplies its existing J8 contract and package proof.
- Proof: rebuild the selected package at the final source pin, retain exact request multisets and planted negatives, exercise the J1 corpus through the public binding and record each actual host and artifact hash.
- Defers: publication and registry/account changes, unproved host targets, unrelated website work and any integration before the explicit start hold is released.

## Everything the program has done

Twelve consumer languages proven through the landed C door, each with two gated stages, parent-verified runs, fresh reviews, fix cycles where findings appeared, exact-evidence gates with planted negatives, and strict post-0166 cancellation: Zig (273), Go (274), Java/Kotlin/Scala (289), C# (290), PHP (291), COBOL (292), Ada (293), Swift (294), Objective-C (295), Dart plus a proven Flutter surface (300), C++ (301). Every port was then re-pinned and re-proven on current main `71f25087` (2026-09-28): eight adapted to the documented contract changes, Dart unchanged-pass, C++ review-verified as already matching. Each re-pin preserved its unchanged-gate FAIL as drift evidence, asserted exact new arrival multisets, kept every planted negative, verified 6,839/6,839 source provenance, and derived the 21-export ABI list from the sealed header. Reviews during the re-pin caught three would-have-shipped defects: stale packaged examples (JVM, Swift), a CMake multilib prefix assumption (C++), and silent 2^53 integer rounding (C++).

## Where everything is

- Experiments: `/home/ian/workspace/experiments/{273,274,289,290,291,292,293,294,295,300,301}-thinkthen-*-c-interface/` — per stage: `CLAIM.json`, README, PLAN/FINDINGS/REVIEW, BUILD-REPORTs, reviews/, FIX-REPORTs, gate logs; re-pin reports at `repin-71f25087-REPORT.md` with adapted packages under `repin/package/` (merge input except Dart/C++: see below). Toolchain versions and hashes in each `inputs/toolchain.json`.
- Issues in this repo: eleven per-language handoff issues (each now carries its re-pin disposition), `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md` (the documented contract changes and the coordinator’s confirmation against accepted decisions), `2026-09-28-language-port-integration-priority-brief.md` (priority order and rationale), `2026-09-28-language-merge-runbook.md` (the copy table, toolchain pins, registry metadata, CI guidance, M5 first commands), `2026-09-25-release-and-install-for-0-1.md` (distribution decision, pub.dev/Dart addition, `io.github.botassembly` ruling), `2026-09-27-one-type-contract-for-every-surface.md` (the type contract the last four ports meet fully and the earlier six carry named deltas for).
- Ian's to-dos (`notes/todos/`): registry account setup (NuGet, Packagist, Maven Central, pub.dev — pub.dev and the JVM namespace already decided), the Beatles deck icon update (Marketing's lane; house marks already drawn in mktg `qf-language-icons`), and the launch checklist.
- mktg: icon library complete (Simple Icons plus four house marks) in `decks/2026-09-24-thinkthen-beatles/brand/icons/`; slide inclusion remains Marketing's Quick Fix.

## The merger process

1. Read the confirmed contract decisions in the drift issue. Preserve exact per-gate counts. Reuse the existing batch-one output-order proof; add a small functional case only for an uncovered port boundary.
2. For each binding, in the priority-brief order (C#, JVM, PHP, Dart first for registry value; then Swift, Zig, Ada, Objective-C, C++ before COBOL): copy the runbook's merge-input folder into `libraries/<lang>/`, adopt its repin README as the per-language page raw material, rebuild at the final pin, and run the port's gate as the CI job (keep planted negatives).
3. Wire registries per the runbook's metadata section as Ian's accounts come online; publish only product-built archives; rehearsal archives never ship.
4. Run the J8 residuals each issue names: full J1 corpus through each public binding, `engine_new_with`/`error_facts_json` where unbound, typed deltas for the six pre-contract ports, per-platform packaging (XCFramework on the M5 Mac, Windows/macOS, Flutter targets), model-produced non-BMP case.
5. Documentation pages per the agreed skeleton; site content belongs to Marketing.

Blocked on: Ian's go-ahead, delivered with this ticket. Everything above is ready; nothing in this ticket requires the program's author to execute.
