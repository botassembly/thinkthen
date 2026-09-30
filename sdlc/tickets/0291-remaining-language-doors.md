# 0291 — Remaining language doors (T9)

Status: deferred. Build with ticket 0314 in the port pass of [ADR 0112](../planning/adr/0112-rust-owns-the-result-schema.md) section 5, so the fourteen bindings change once. Draft from Codex branch `ticket/0283-sql-frame-redesign-preparation`.

## Outcome

Each of ada, c, cobol, cpp, csharp, dart, go, jvm, objective-c, php, ruby,
swift, typescript, zig: `deadline_ms` in the door's idiom, call settings
through the one core parser, `max_requests_total`, the score/tag probability
refusal **and a public plan method** over 0283's pure summary and 0289's
`Engine::plan`. No door is exempt from ADR 0105 section 6.

Schedule bounded disjoint cohorts after rechecking live 0281 Swift/Zig and 0282 Ada/Objective-C/COBOL claims, including nested sources and installed member lists. Preserve typed owned-facts routes.

**First bounded family: C bridge and independent author guide.** Add a distinct `thinkthen_plan_json(engine, plan_json, out, out_len)` export returning the existing six-kind status code and an owned UTF-8 JSON string freed by `thinkthen_free_string`. `plan_json` is a closed `thinkthen.plan-input/1` object with `verb`, `question`, `input`, optional `members` and optional `settings`, in the accepted common argument order; `input` is one text, an ordered collection or a keyed object as that verb requires. Apply the one core parser and the public `Engine::plan` wrapper, with no key requirement, cache read or send. Success is schema `thinkthen.plan/1` with `requests`, `records`, `estimated_bytes`, `estimated_input_tokens:{lower,upper}`, `upper_bound` and `first_request_body_utf8`; the byte/token rules and staged upper-bound mark come unchanged from 0283. Invalid/duplicate fields, wrong verb/input shape and null pointers fail as usage with out pointers untouched. Preserve the C door's existing `thinkthen_call[_opts]` ten-verb grammar in `libraries/c/src/call.rs`, its `{value,facts}` result, every existing symbol/signature and typed facts route: do **not** add `plan` to that closed JSON verb list or recast a call response as a preview. The additive symbol needs explicit header/export/source/member review because the existing ABI is frozen in shape.

Proposed first-family files are new `libraries/c/src/plan.rs` and a cohesive `libraries/c/src/ffi/plan.rs` extraction (current `libraries/c/src/ffi.rs` measures **499/500 nonblank**), plus `libraries/c/src/{lib.rs,ffi.rs}`, `libraries/c/include/thinkthen.h`, `libraries/c/{DESIGN.md,README.md}`, `libraries/c/tests/c/plan.c`, existing `libraries/c/tests/door/main.rs`, `sdlc/scripts/check-c-exports.py` only if its header-derived parser needs adjustment, and source/archive member inventories. Reconcile copied headers at `libraries/swift/Sources/CThinkThen/include/thinkthen.h` and `libraries/objective-c/Sources/thinkthen.h`, plus each consumer's direct declarations (JVM `door/thinkthen/Door.java`, Zig `src/thinkthen.zig`, C++ `include/thinkthen/door.hpp`, and the remaining hosts) when that family is claimed. `sdlc/scripts/release-go-cpp-pair` has an exact C archive member list; check it and the other package validators rather than assuming the new symbol is shipped. Add a concise `libraries/BINDING-AUTHOR.md` guide in this first family: core parser and C conversion rule, one shared corpus with one host conversion table, zero-send listener proof, minimal child environment, matched source versus installed/archive receipts, and exact member/export checks. This is a builder-facing guide, not site marketing.

**Then small host families.** Each wrapper exposes an idiomatic public `plan` returning its owned native representation of the common summary through `thinkthen_plan_json`, or an equivalent direct shared-core call where the door is Rust-native. C/Go/C++ and direct native consumers can follow the C bridge; managed C#/JVM/Dart and Swift/Objective-C/COBOL, then scripting Ruby/PHP/TypeScript and compiler-backed Ada/Zig are candidate disjoint cohorts, subject to a fresh exact file and package-source inventory. Keep 0281/0282 typed-facts methods and copied consumer signatures. Plan is pure preview, never a second native send and never a mutable last-result slot. Ian's independent value assessment and developer-experience follow-up point back to this guide and these public methods; they create no extra issue or feature here.

## Prerequisites and proposed files

Prerequisite: T1 and T7; C bridge and guide first, then bounded family claims. Proposed file families: first-family exact list above, then `libraries/{ada,cobol,cpp,csharp,dart,go,jvm,objective-c,php,ruby,swift,typescript,zig}/` public wrappers, selected tests, source/archive member lists and copied consumers, inventoried per family. This is a future claim proposal, not permission to edit those files in this preparation. Preserve all fourteen public-plan obligations through final completion.

## Smallest meaningful proof

First C bridge: pass corpus P1 through the new export, assert owned `thinkthen.plan/1` bytes decode to `records=1`, `requests=1`, `estimated_bytes=120`, the 61–109 token band and the exact first body while the listener accepts zero requests; `thinkthen_free_string` releases the result. An invalid settings object and a duplicate question/settings member each return usage, leave sentinel out pointers untouched, and send zero. Header-derived export and installed native archive checks must see the new symbol and unchanged old symbols. Each later door's public `plan` repeats **one** valid/invalid/no-send conversion of P1, with its native return shape, plus deadline rename, active cap and score/tag refusal in its own bounded fixture. Keep source and installed receipts separate. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name later package/release qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Additive C plan bridge and binding-author guide, then public plan/settings/deadline/cap methods on all fourteen doors in bounded families.
- Proof: First C P1 plan/failure/free/export/installed-member oracle, then one public valid/invalid/no-send P1 conversion plus cap and refusal per door.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
