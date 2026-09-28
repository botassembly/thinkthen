---
flow: build
priority: 243
opens: sdlc/tickets/0243-rust-choice-descriptions.md sdlc/records/0243-rust-choice-descriptions-preflight.md sdlc/records/0243-design-review.md
---

# 0243: Describe typed Rust choices

Status: **Implementation candidate `79fba65d` pushed; fresh code/API review pending.** Design was accepted at `7d866431`, and the coordinator approved the exact runtime claims in the main Lanes table. This is the existing J4 row, not another type-contract outcome. ADR 0082 and [the type-contract issue](../issues/2026-09-27-one-type-contract-for-every-surface.md) already approve descriptions on `choices!` variants. The [fresh Sol Medium design review](../records/0243-design-review.md) accepted the spelling; the coordinator owns J4 closure.

## Starts from

At main `681a1d59`, `Choice` supplies a label, ordered labels and reverse lookup; `choices!` writes those methods and rejects duplicate labels. The typed `Question::choose::<C>` and `Question::tag::<C>` builders already accept an optional `Description` per member. `Description::text` and `Description::builder` already validate strings and structured objects. The question-file parser already accepts described `options`/`labels` maps, while `Question::into_choose` and `into_tag` bind an existing question by exact label order. [Specification types](../../specification/types.md) and [ADR 0082](../planning/adr/0082-one-type-contract-for-every-surface.md) require ordered labels, string or structured descriptions, an explicit map winning per label and unknown labels returning Usage. [Preflight](../records/0243-rust-choice-descriptions-preflight.md) traces the code and consumers.

## Keeps

- Existing `choices! { enum Team { Billing => "billing", Outage => "outage" } }` syntax, label order, duplicate-label compile refusal, typed `Option<Team>`/`Vec<Team>` results and existing manual `Choice` implementations. No new parser, scheduler, result type or dependency.
- Existing typed builder `Some(description)` is the explicit per-label override. `None` means use variant metadata when present. For an old enum with no metadata, `None` still means no description and preserves its request bytes/digest. The shared `Listing` order check runs before evaluating metadata.
- A loaded JSON question's complete ordered map remains authoritative when bound with `into_choose::<C>` or `into_tag::<C>`: each entry, including explicit null, is used as parsed and never retrofitted with enum metadata. An unknown or reordered mapped label fails with the existing Usage binding error. This preserves question-file identity and the existing list/map/null grammar.

## Changes

Extend each `choices!` variant with an optional colon after its label. Text uses `Billing => "billing": "Charges and refunds"`, as proposed in the type-contract issue. Structured metadata uses a Rust block yielding the existing `Result<Description, Error>`, for example `Outage => "outage": { Description::builder().what("A service outage")?.not_for("A billing question")?.example("The service is down")?.build() }`. An old bare variant can appear beside either form. The macro uses `$crate` paths for its generated code, so downstream crates need no private import or new macro helper. It emits a `Choice::description(&self) -> Result<Option<Description>, Error>` implementation; the trait supplies a default `Ok(None)` for downstream manual implementations. The structured block is ordinary Rust using the existing builder, not a new object syntax or parser.

In `public/builders.rs::Listing::next`, choose the supplied `Some(Description)` first, otherwise call the choice's description method, then pass the selected value through the existing `push`/core validation. A malformed default is a Usage error at construction when selected; a valid explicit override bypasses that malformed default. This keeps each label's metadata independent and preserves the existing `Description` JSON grammar and writer. The public builder method signatures remain unchanged. `Question::from_json` and `into_choose`/`into_tag` need no change.

## Proof

First make one external-crate compile fixture red for the new mixed bare/text/structured spelling and `Choice::description`; then pass it. Use the existing `tests/compile_contract.rs` outside-crate harness to exercise public and nested macro expansion, `$crate` hygiene, manual trait default compatibility and unchanged typed choose/tag output. Keep its duplicate-label compile failure. In one `tests/public_members.rs` loopback case, build typed choose and tag questions with text and structured metadata, capture the actual request bodies and returned question digests, and compare them with equivalent ordered question-file maps. Pin an old bare-enum request body and digest from prechange main `681a1d59`, rather than computing the expected identity from the new code. The same bounded case checks a per-label explicit builder override, a selected blank metadata Usage refusal with zero sends and a valid override of that default. Bind an existing loaded map with a null entry and verify its captured identity is unchanged; a map with an unknown label fails Usage before a send. These are distinct prompt-identity and precedence boundaries, not duplicate parser tests. Run focused fixture/listener selectors, Rust format, strict relevant lint, file caps and exact ratchet; no full surface matrix, stress or provider call.

After design acceptance, request exact runtime claims for `crates/thinkthen/src/public/{choice,builders}.rs`, `crates/thinkthen/tests/{compile_contract,public_members}.rs`, `libraries/rust/examples/{choose,tag}.rs`, and `libraries/rust/README.md` if its shown syntax changes, plus ticket build/code-review records and the derived `sdlc/ratchet.json`. `public/question.rs` remains read-only unless implementation establishes a concrete need and the coordinator grants it. The Rust examples are library-owned; site cleanup belongs to ticket 0241 under Ian’s explicit delegation. Measure growth against the existing file caps and shared ratchet, seek duplication in current builder/compile fixtures, and obtain fresh code/API review for the public trait and macro. The coordinator owns plan updates and J4 closure.

## Defers

Other language J rows, any new map-merge API, score/recognize metadata, site examples, a second description representation and broad platform gates. This ticket changes typed Rust choice authoring only.

## Evidence

- **Starts from:** Main `681a1d59`; `specification/types.md:20`; ADR 0082's J4 mapping; the type-contract issue's Rust spelling; existing `public/choice.rs`, `public/builders.rs`, `public/question.rs` and `core/question_file/fields.rs`.
- **Keeps:** Old macro syntax and exact bare-label identity, manual `Choice` source compatibility, typed outputs, question-file map/null authority, and existing Usage validation.
- **Changes:** Optional variant text or structured metadata, a default trait accessor, and selection at the existing typed listing step.
- **Proof:** Outside-crate compile and loopback request/digest equivalence, per-label override, malformed selected metadata and unknown map refusal, then focused strict checks and counters.
- **Defers:** New parsers or map-merge APIs, other verbs/ports, site edits, broad gates and provider work.

## What the build taught us

- The prechange bare `Team` choice sent the exact body and digest now pinned in the listener child; `Choice` and the listing path were unchanged between design base `681a1d59` and runtime base `e1bc785e`. The new selection stays in `Listing::next`, so loaded JSON questions are only bound, not rewritten. [Build evidence](../records/0243-build.md) names the captured bytes, commands and counters.
- The outside-crate compile fixture exposed inherited bare-result declarations after the already landed `Call<T>` change. Correcting those declarations restored its existing public contract proof while adding mixed macro syntax, downstream privacy/hygiene and manual `Choice` compatibility. The selected blank metadata, explicit override and unknown loaded label now have one bounded zero-send listener proof; no test-only hook or duplicate parser was needed.
- The new listener case would have exceeded the parent file's 500-line cap. A coherent private child keeps the parent at 336 nonblank lines and the child at 196. The root Rust total rose to 94,818 and the Rust examples total to 267; the build record names the reused code and footprint. No existing test was deleted or consolidated. Fresh code/API review and coordinator landing remain before J4 closure; site work stays with its separate owner.
