# Area 16: Backend wire dialects

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

The `systemone` adapter turns one plan into the request body bytes and one response body into an answer per question, and the built-in backend table picks how descriptions travel (as authored, or as text for `ollama`).

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/` or `sdlc/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code | `core/adapters.rs` (15), `core/adapters/systemone.rs` (282), `core/adapters/systemone/request.rs` (379), `response.rs` (323), `response/observed.rs` (47), `backends.rs` (42), `recorded.rs` (50), `core/reply.rs` (107), `core/backend_profile.rs` (252), `core/render.rs` (18): 1,515 together. `core/backend.rs` (316) and `core/batch.rs` (142) are read here but counted under areas 17 and 1 |
| Tests | About 68 tests. Unit: `request_tests.rs` (11, four of them property tests), `response_tests.rs` (13), `response_partial_tests.rs` (5), `response_distribution_tests.rs` (3), `core/backend_profile.rs` (2). Integration: `crates/thinkthen/tests/backend/wire.rs` (1), `exchange.rs` (23), `json_syntax.rs` (6), `named_backends/ollama.rs` (4). 17 fixture files in `specification/fixtures/systemone/` |
| Contract | `specification/backends.md` "The adapter contract" (:145), "The `systemone` adapter" (:162), "What the adapter keeps" (:198), the Ollama paragraph (:59); ADRs 0004, 0010, 0039, 0040, 0110, 0111 section 1, 0115 |

## Complexity: 3 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | 1,515 nonblank lines across 10 files, plus 316 shared with area 17 |
| States and concurrency | 1 | Two pure translations. The adapter touches no network, file or clock (`core/adapters/systemone.rs:1-9`) |
| Rules and refusals | 3 | About 20. Nine decode refusals (`DecodeError`, `systemone.rs:102-125`), one repeated-name refusal, one distribution tolerance, the null and empty description rules, four `Text` form rules, the tag sentence rule, the score null rule, profile name and four limits |
| Surfaces touched | 5 | All 22, through the one encode and decode path every send uses. Only the `ollama` form is reachable on two surfaces (command and Rust builder) |
| Settings | 3 | Model, Backend (description form), Backend profile |
| Contract weight | 4 | Two spec pages with sections (`backends.md`, `result.md`), plus seven ADRs: 0004, 0010, 0039, 0040, 0110, 0111, 0115 |
| Churn and debt | 4 | 37 commits on the owned core paths in 7 days, most of them moves under other tickets. One open issue, which is tracked debt with a named removal condition |

Mean 3.3, rounded to 3.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The encoder matches the table in `backends.md:170-183`. A null noul description sends no member, and a question with neither side sends no `criteria` (`request.rs:88-95`). A null score description sends an empty object in the authored form and the level name in the text form (`request.rs:393-400`). The decoder accepts a distribution within `0.01 + n * f64::EPSILON` (`response.rs:314`), as the contract says. Choice and score answers must carry exactly the keys sent (`response.rs:294-313`). Drift: `backends.md:149-152` says an adapter is "two pure functions", but the module exports 13 items (`systemone.rs:20-29`). One rule is unconfirmed: a `usage` object missing one token count makes the whole reply malformed, because `ResponseUsage` requires both fields (`response.rs:103-107`), and the contract does not say so |
| Reliability | B | Failures are pinned by counted sends or exit codes. A reply the adapter refuses is exit 4 (`crates/thinkthen/tests/backend/exchange.rs:407`), a cut reply is exit 4 (`:427`), a repeated member is refused (`json_syntax.rs`, 6 tests), and four fixtures hold refused replies (`specification/fixtures/systemone/refused-*.response.json`). Two wire defects were found only by live runs after the code landed: null criteria refused by Liquid d1 (`8286b99d7`) and object descriptions refused by Ollama (open issue, workaround `5e21063bf`). Both are fixed or tracked. The area saw 37 commits in 7 days, which caps the grade at B |
| Maintainability | B | One owner for the wire shape, and the vendor words sit behind `adapters.rs`. Files are well under the cap (largest `request.rs` at 379). No lint suppression in the area. Two small duplicates: `decode_questions` and `decode_answers` repeat the parse and usage read (`response/observed.rs:13-47`), and `read` has two arms that both return `WrongKind` (`response.rs:277-278`). `core/check.rs:52` spells `ollama` inside a sentence that any future `Text` backend would also print |

## Strengths

- The Ollama form is isolated. Only the `ollama` table row sets `Descriptions::Text` (`core/adapters/systemone/backends.rs:36`). Configured entries always use `Authored` (`core/backend/named.rs:51`), and so does the unnamed path (`core/backend.rs:133`). The form is chosen only when a named backend is chosen (`named.rs:205-215`).
- Every `Text` rule sits in one function, `sent`, with its debt issue named in the comment (`request.rs:254-285`). The removal condition is in `sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md`.
- Tag expansion keeps the old sentence only when every description is a string, so no JSON is ever pasted into a sentence (`request.rs:226-230`, `:327-338`).
- Decode fails one logical question and keeps the rest, and it refuses the whole reply when every question fails (`response.rs:148-189`).
- The body is written once from joined parts, so the bytes a key hashes equal the bytes sent (`request.rs:185-208`).

## Cleanup

1. **Say what a half-filled `usage` does.** Where: `core/adapters/systemone/response.rs:103-107`. Why: a reply with valid answers and a `usage` holding only one count is refused whole (unconfirmed, read from the derive, not run). Either keep the rule and add it to `backends.md:162-185`, or accept the answers and drop the usage. Add one table case to `response_tests.rs`. Size: S. Blocks 0.1: no.
2. **Fix "two pure functions".** Where: `specification/backends.md:149-152`, `core/adapters/systemone.rs:20-29`. Why: the contract names `encode` and `decode`, and the adapter exports `parts`, `join`, `encode_raw`, `decode_questions`, `decode_answers`, `decode_observed`, `drops_any` and more, because ADR 0111 builds bodies from parts. Size: S. Blocks 0.1: no.
3. **Name the description form from the backend, not from a constant.** Where: `core/check.rs:52`, `:228`. Why: the sentence hard-codes `ollama`. Pass the backend name in, so a second `Text` backend prints the right name. Size: S. Blocks 0.1: no.
4. **Merge the two decode entry points.** Where: `core/adapters/systemone/response/observed.rs:13-47`. Why: both parse the body and read usage in the same way. Size: S. Blocks 0.1: no.
5. **Remove the redundant `read` arm.** Where: `core/adapters/systemone/response.rs:277-278`. Why: `(Tag, _)` and `_` return the same error. Size: S. Blocks 0.1: no.
6. **Let `status` print the description form, and check Ollama with an object `state`.** Where: `cli/status.rs`, the gaps list in the Ollama issue. Why: the issue records that `status` does not show the form and that nobody has tried an object `state` or structured question text against Ollama. Size: S for status, unknown for the live check. Blocks 0.1: no.

## Confidence: medium

What was read: `request.rs`, `response.rs`, `observed.rs`, `backends.rs`, `recorded.rs`, `systemone.rs`, `adapters.rs`, `reply.rs` (first 60 lines), `render.rs`, the profile type's first 40 lines, the `backends.md` sections named in the brief, the Ollama issue in full, and the names of every integration test in `exchange.rs` and `named_backends/ollama.rs`. `wire.rs` was skimmed.

Not checked: no test or build was run, so cleanup item 1 is inferred. The fixture files were listed, not read. `core/reply.rs` past line 60 and `core/backend_profile.rs` past line 40 were sampled only. The brief lists `core/reply.rs` and `core/render.rs` here, but they hold little dialect logic. The live Ollama behavior rests on the issue text and the loopback mimic, because no paid or local Ollama call was made.
