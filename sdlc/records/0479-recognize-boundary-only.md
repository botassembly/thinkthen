# 0479: Return boundary proposals without later recognition stages

Accepted [ADR 0126](../planning/adr/0126-recognition-stage-context-and-boundary-proposals.md) defines this contract. Shared Request and native Rust types select Whole or BoundaryOnly. Saved declarations and CLI options consume the same mode. Whole remains the default and retains existing complete output, question bodies, cache keys and aggregate identities. BoundaryOnly reuses boundary questions and their existing cache/replay identities, returns decoded model spans, and has a separate aggregate answer identity even for empty results.

## Implementation

The shared mode lives in `core/recognize.rs`. Saved admission lives in `core/recognize_file.rs`; Request options, admission, reconstruction and execution live in `public/request/`. The recognition scheduler returns before kinds, edges, relations and seed augmentation. Supplied seeds still pass existing validation and feed classification in Whole mode; they do not add BoundaryOnly model proposals.

`core/recognition_result.rs` carries untagged Whole and BoundaryOnly values and answer details. `public/recognize/proposals.rs` exposes typed proposals and a borrowed typed value view; complete reading retains precut proposal probabilities. Source reading maps actual proposal spans to locations without manufacturing entity kinds. Proposals preserve decoded text and scalar offsets, including punctuation. Their P(span) comes from existing valid-path boundary odds, rounds to four places, and uses the existing single cut with default 0.5.

BoundaryOnly refuses authored relations, including an empty saved declaration, any explicit relation threshold, and supplied kind_edge or relation context, including empty strings, before evidence. Existing call overlays resolve before that refusal. `RequestDefinition::recognition_json` has the narrow authored-wire exception: it omits an implicit relation threshold and preserves an authored cut and authored empty relations. Native Request can round-trip through its canonical wire without manufacturing a forbidden explicit control. Whole complete serialization, saved canonical descriptions and reading digests retain their existing behavior.

The existing schema generator produces Request, result and complete-reading schema changes. Native Rust and generated schemas settle the shared contract here. Every SDK and SQL surface consumes it in 0.2 through the existing adoption tickets 0494–0505.

Offline audit and diff use proposal probability and match offsets without assigning a classified kind. Raised cuts retain the existing run-cut floor. Audit case rows retain BoundaryOnly shape even when a cut leaves no proposals. Diff JSON retains raw proposals; tables print text, offsets and probability. Whole audit parsing and fixtures retain their behavior.

## Evidence and lessons

All backend responses came from existing saved fixtures or owned loopback listeners. No paid calls or release operations ran. Cargo checks used the existing warm outputs with offline mode, jobs two, the mold link flag, and a scope capped at 8 GiB memory and 1 GiB swap.

The initial shared Request test failed on the unknown mode field. Canonical Request tests then passed mode resolution, safe refusal, single-cut admission, authored-wire reconstruction and the retained default identity. Six schema tests passed after regeneration; the schema writer first failed as its write contract requires. The saved canonical-description tests passed their existing byte/digest oracles.

The native recognition selection passed 25 cases. It checks exact existing boundary bodies across stage contexts, shared cache observations, precut P(span), printed rounding and local cuts, empty-mode identity separation, skipped-stage zero sends, seed behavior, Unicode/source locations, punctuation retention and saved admission before missing evidence. The backend recognition selection passed 73 cases before offline support. Its added command cases check Whole/default byte equality, saved/call mode precedence, Whole-to-Boundary replay with zero new sends, reverse replay miss, zero later planning bounds and safe pre-evidence refusals.

The initial offline command case failed because audit could not grade the BoundaryOnly value. After the existing parser adopted proposals, the fixture key failed because a plain-text complete row uses audit's implicit line id, not a top-level id. Correcting the key to that existing convention made the case pass. It checks probability rescoring, the run-cut floor, key offsets, empty filtered shape and JSON/table diff with one total model send.

Earlier fixture failures taught specific lessons. JSON object construction through `serde_json::Value` reordered authored kinds and changed existing question wording; fixture reconstruction now preserves the original kind order. The existing KestrelLabs boundary fixture yields 0.9972 rather than 1.0. The listener's `requests()` accessor drains captured requests, so the test captures once before comparing all bodies. Local replay misses use exit 5; the first oracle incorrectly expected 4. Native fixtures use the actual constructors and source accessors rather than inferred APIs.

Affected Clippy first rejected increased parser/scheduler complexity; small same-file helpers kept behavior at the existing boundaries. The complete command cases retain explained test-only length expectations because each drives one counted user behavior. No new proof harness, cache domain or business routing was introduced. Policy passed with existing source-size warnings; earlier implementation commits explain growth in the owning public, saved, scheduler and CLI files.

The existing public API inventory found three literal ticket omissions: the lifetime on `RecognitionValue<'a>`, its Whole variant, and `Serialize` for `SourceBoundaryProposal`. Those belong in the owning ticket's declaration block, not another inventory mechanism.

The final affected Clippy command passed with `--lib --test native_complete --test backend -- -D warnings`. Existing audit selections passed 51 cases and diff selections passed 19 cases, including literal Whole JSON and table goldens. All three new BoundaryOnly command cases passed after the offline additions. `cargo fmt --all` and `git diff --check` passed.
