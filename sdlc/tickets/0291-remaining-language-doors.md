# 0291 — Remaining language doors (T9)

Status: COMPLETE.

Opened as: 2026-10-11. with ticket 0314's port pass: the C bridge in slice 4a, then Go and C++ (4b), C#, JVM and Dart (4c), Swift, Objective-C, COBOL, Ada and Zig (4d), and Ruby, PHP and TypeScript (4e). The score and tag probability refusal is vacuous in all fourteen doors, because none offers a probability option. Built in the port pass of [ADR 0112](../planning/adr/0112-rust-owns-the-result-schema.md) section 5. Draft from Codex branch `ticket/0283-sql-frame-redesign-preparation`.

## Outcome

Each of ada, c, cobol, cpp, csharp, dart, go, jvm, objective-c, php, ruby,
swift, typescript, zig: `deadline_ms` in the door's idiom, call settings
through the one core parser, `max_requests_total`, the score/tag probability
refusal **and a public plan method** over 0283's pure summary and 0289's
`Engine::plan`. No door is exempt from ADR 0105 section 6.

Twelve of these are C door bindings. Ruby and TypeScript are native Rust bindings: they call the shared core directly, not `thinkthen_plan_json`. Ruby's result layer moved in 0314 slice 3, so Ruby needs only this ticket's plan, deadline, cap and refusal methods. TypeScript also moves its hand-built detail JSON to the crate's `Serialize`, as 0314 slice 3 deferred.

Schedule bounded disjoint cohorts after rechecking live 0281 Swift/Zig and 0282 Ada/Objective-C/COBOL claims, including nested sources and installed member lists. ADR 0112 section 4 deletes the ports' typed facts structs, and it is later and accepted, so it wins over this draft. Each family deletes them in the same pass. The C door's typed facts exports stay, because its ABI is frozen.

**First bounded family: C bridge and independent author guide.** Add a distinct `thinkthen_plan_json(engine, plan_json, out, out_len)` export returning the existing six-kind status code and an owned UTF-8 JSON string freed by `thinkthen_free_string`. `plan_json` is a closed `thinkthen.plan-input/1` object with `verb`, `question`, `input`, optional `members` and optional `settings`, in the accepted common argument order; `input` is one text, an ordered collection or a keyed object as that verb requires. Apply the one core parser and the public `Engine::plan` wrapper, with no key requirement, cache read or send. Success is schema `thinkthen.plan/1` with `requests`, `records`, `estimated_bytes`, `estimated_input_tokens:{lower,upper}`, `upper_bound` and `first_request_body_utf8`; the byte/token rules and staged upper-bound mark come unchanged from 0283. Invalid/duplicate fields, wrong verb/input shape and null pointers fail as usage with out pointers untouched. Preserve the C door's existing `thinkthen_call[_opts]` ten-verb grammar in `libraries/c/src/call.rs`, its `{value,facts}` result, every existing symbol/signature and typed facts route: do **not** add `plan` to that closed JSON verb list or recast a call response as a preview. The additive symbol needs explicit header/export/source/member review because the existing ABI is frozen in shape.

Proposed first-family files are new `libraries/c/src/plan.rs` and a cohesive `libraries/c/src/ffi/plan.rs` extraction (current `libraries/c/src/ffi.rs` measures **499/500 nonblank**), plus `libraries/c/src/{lib.rs,ffi.rs}`, `libraries/c/include/thinkthen.h`, `libraries/c/{DESIGN.md,README.md}`, `libraries/c/tests/c/plan.c`, existing `libraries/c/tests/door/main.rs`, `sdlc/scripts/check-c-exports.py` only if its header-derived parser needs adjustment, and source/archive member inventories. Swift and Objective-C hold no tracked header copy: ticket 0332 made their checks and `release-pack` copy the C header from its source, and ticket 0337 stopped shipping it in the Objective-C package. Reconcile each consumer's direct declarations (JVM `door/thinkthen/Door.java`, Zig `src/thinkthen.zig`, C++ `include/thinkthen/door.hpp`, and the remaining hosts) when that family is claimed. `sdlc/scripts/release-go-cpp-pair` has an exact C archive member list; check it and the other package validators rather than assuming the new symbol is shipped. Add a concise `libraries/BINDING-AUTHOR.md` guide in this first family: core parser and C conversion rule, one shared corpus with one host conversion table, zero-send listener proof, minimal child environment, matched source versus installed/archive receipts, and exact member/export checks. This is a builder-facing guide, not site marketing.

**Then small host families.** Each wrapper exposes an idiomatic public `plan` returning its owned native representation of the common summary through `thinkthen_plan_json`, or an equivalent direct shared-core call where the door is Rust-native. C/Go/C++ and direct native consumers can follow the C bridge; managed C#/JVM/Dart and Swift/Objective-C/COBOL, then scripting Ruby/PHP/TypeScript and compiler-backed Ada/Zig are candidate disjoint cohorts, subject to a fresh exact file and package-source inventory. Keep consumer signatures in step with the header. Plan is pure preview, never a second native send and never a mutable last-result slot. Ian's independent value assessment and developer-experience follow-up point back to this guide and these public methods; they create no extra issue or feature here.

## Prerequisites and proposed files

Prerequisite: T1 and T7; C bridge and guide first, then bounded family claims. Proposed file families: first-family exact list above, then `libraries/{ada,cobol,cpp,csharp,dart,go,jvm,objective-c,php,ruby,swift,typescript,zig}/` public wrappers, selected tests, source/archive member lists and copied consumers, inventoried per family. This is a future claim proposal, not permission to edit those files in this preparation. Preserve all fourteen public-plan obligations through final completion.

## Smallest meaningful proof

First C bridge: pass corpus P1 through the new export, assert owned `thinkthen.plan/1` bytes decode to `records=1`, `requests=1`, `estimated_bytes=182`, the 93–166 token band and the exact quoted first body that `databases/duckdb/tools/plan_suite.py:9` pins (checked on main at `acebf3044`; slice 1 of 0304 removed the unquoted 120-byte body this draft named) while the listener accepts zero requests; `thinkthen_free_string` releases the result. An invalid settings object and a duplicate question/settings member each return usage, leave sentinel out pointers untouched, and send zero. Header-derived export and installed native archive checks must see the new symbol and unchanged old symbols. Each later door's public `plan` repeats **one** valid/invalid/no-send conversion of P1, with its native return shape, plus deadline rename, active cap and score/tag refusal in its own bounded fixture. Keep source and installed receipts separate. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name later package/release qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, the C door's facts exports, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Additive C plan bridge and binding-author guide, then public plan/settings/deadline/cap methods on all fourteen doors in bounded families.
- Proof: First C P1 plan/failure/free/export/installed-member oracle, then one public valid/invalid/no-send P1 conversion plus cap and refusal per door.
- Defers: a `planInput` definition in `specification/question-file.schema.json`; the header, the C README and DESIGN describe `thinkthen.plan-input/1` until a port family needs the schema. Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## C bridge build (0314 slice 4a)

Branch `ticket/0314-s4-remaining-ports`. `thinkthen_plan_json` and `libraries/BINDING-AUTHOR.md` landed; ticket 0314's slice 4a section holds the evidence. The P1 proof uses the corrected figures: 182 bytes and the 93 to 166 band, with zero listener accepts and no key.

## Go and C++ build (0314 slice 4b)

Branch `ticket/0314-s4-remaining-ports`. Go gains `Engine.Plan`, and C++ gains `tt::plan`, both over `thinkthen_plan_json`. C++ `call`, `recognize` and `relate` now take a deadline and cancel token through their `_opts` twins; Go's context deadline already reached every `_opts` call. Both constructors already passed `max_requests_total` through unchanged, and a zero cap refuses before sending. Neither port offers a probability option on score or tag, so the refusal is vacuous for both. Ticket 0314's slice 4b section holds the evidence.

## C#, JVM and Dart build (0314 slice 4c)

Branch `ticket/0314-s4-remaining-ports`. C# gains `Engine.Plan`, the JVM gains `Door.plan` and Dart gains `plan`, each over `thinkthen_plan_json`. C# `RelateWithOptions`, JVM `recognize` and `relate` overloads and Dart's `call`, `recognize` and `relate` now take a deadline and cancel token; the other sending calls already did. All three constructors already passed `max_requests_total` through unchanged, and a zero cap refuses before sending. None offers a probability option on score or tag, so the refusal is vacuous for all three. Ticket 0314's slice 4c section holds the evidence.

## Swift, Objective-C and COBOL build (0314 slice 4d)

Branch `ticket/0314-s4-remaining-ports`. Swift gains `Engine.plan`, Objective-C gains `plan:question:texts:lengths:count:settings:failure:` and COBOL gains `TT-PLAN`, each over `thinkthen_plan_json`. Swift and Objective-C `recognize` and `relate` now take a deadline and token; COBOL `TT-DECIDE` and `TT-CALL` take `tt-deadline-ms`, and `TT-CALL` moves to `thinkthen_call_opts`. All three constructors already passed `max_requests_total` through unchanged, and a zero cap refuses before sending. None offers a probability option on score or tag, so the refusal is vacuous for all three. Ada then gained `Plan` and Zig `Engine.plan` over `thinkthen_plan_json` on the same branch. Every Ada and Zig sending call already took a deadline through its `_opts` twin, both constructors already passed `max_requests_total` through unchanged, and neither offers a probability option on score or tag. Ticket 0314's slice 4d section holds the evidence.

## Ruby, PHP and TypeScript build (0314 slice 4e)

Branch `ticket/0314-s4-remaining-ports`. PHP gains `plan` over `thinkthen_plan_json`; Ruby and TypeScript gain `plan` over `Engine::plan_with`, as native bindings. Ruby's sending calls take `deadline_ms:` in milliseconds in place of `deadline:` in seconds; PHP and TypeScript sending calls already took a deadline in milliseconds. Ruby gains `max_requests_total:` and TypeScript `maxRequestsTotal`; PHP's constructor already passed settings JSON through, and a zero cap refuses before sending in all three. None offers a probability option on score or tag, so the refusal is vacuous for all three. The PHP check no longer waits on a lock its caller holds. Ticket 0314's slice 4e section holds the evidence. Lesson: a native binding's plan fits the host's verbs best when it takes a built question and keyword controls, not a verb name and settings JSON.

## What the build taught us

- The C door already had most of this ticket's per-call surface. `deadline_ms` rides every `_opts` export, and `max_requests_total` is an engine key of `thinkthen_engine_new_with`. So the C bridge adds only the plan export. The port families still add deadlines where a wrapper hard-codes `THINKTHEN_NO_DEADLINE`, and pass the cap through their constructor settings.
- The plan input drops the draft's `members`. `Engine::plan` takes judgment questions only, so no member list has a meaning there, and a plan input naming `members` is a usage refusal. Keyed objects are dropped too: the SQL hosts drop the keys before planning, and the C door's record arrays carry none.
- A question object takes only call controls (`batch`, `context`, `deadline_ms`) from the settings. Merging question fields into a caller's object would reorder its keys and change the request body. A settings field the question repeats gets the core parser's "settings repeats" refusal first.
- The C door's `call.rs` reads its envelope into a map that keeps the last repeated member. The plan reader uses a closed serde struct instead, so a repeated member is a usage refusal as the draft asked.
- Slice 4b: a port that takes the question as one string needs a rule for text versus a question object. Go follows the C door's own rule for `thinkthen_decide`: text that starts with `{` is a question object. C++ takes the question as its `Json` value, so no rule is needed.
- Slice 4d: a port whose host has no JSON value still needs one way to reach a member. COBOL gains `TT-JSON-MEMBER`, the smallest tolerant accessor; every other result stays JSON text.
- Slice 4d: Ada is the second such host. It gains `Member` and `Element` and keeps its JSON syntax checker, because the plan splices a caller's question object and settings into the request.
