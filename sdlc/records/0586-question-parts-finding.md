# 0586: Separate semantic question parts while retaining exact dependencies

Ticket: 0586
Date: 2026-10-14

Review: accept. A fresh read-only reviewer checked the independent comparisons, all snapshot hashes and original exact keys. The counterexamples resolve the planning risk; the record explicitly does not claim all-recordings acceptance coverage. No findings required changes.

The ten functions can expose reusable authored wording separately from evidence and context. Those three semantic values alone cannot rebuild every wire question. Recognition needs a stage and prior span choices; relations need ordered entity IDs and the asked pair or menu source. A database that drops those dependencies cannot safely derive today's exact key from three digests.

Recommendation: retain current exact cache identity and the original canonical wire state/question bytes. Add semantic wording, evidence and context fields where the planner can supply them from typed inputs, with their own digests. Record a versioned renderer and ordered stage/candidate dependencies for reconstructed content. If a part cannot be identified before rendering, leave that part absent and allow only exact reuse for that record. Do not guess parts by parsing instructions or infer semantic reuse from a successful byte round trip. This supports the proposed unsplit exact-only fallback.

Base revision: 0e46dc2daf53bba7ad40165e67068948607f7bec. Machine: beelink. No product edits, builds, model calls, network requests, paid jobs or publication. The experiment owns only this folder. Other workers changed ticket/record files in the shared source checkout while this experiment ran; this experiment did not touch them.

# Inputs and independent comparisons

`probe.py` reads original input files and saved question definitions. Literal command arguments come from the respective demo README/record script. Recognition engine constants come from current rendering source. It renders compact ordered JSON independently in Python, then compares exact UTF-8 question and state strings against separately saved recording fields. It never derives parts by splitting a recorded instruction. `inputs/` retains source snapshots and recordings. `manifest.json` lists the consumed files, hashes and lengths. Repository inputs are MIT-licensed; LICENSE is included.

The bounded sample covers 81 generated questions, each paired with its recorded state. All 81 match both byte strings:

| Function | Matched cases | Independent input and coverage |
| --- | ---: | --- |
| decide | 2 | Demo 01 message and nonrefund input; authored command question |
| choose | 1 | Demo 02 ticket, command question and ordered four labels |
| tag | 3 | Demo 39 message and three authored label descriptions |
| score | 4 | Demo 17 request files and ordered difficulty levels |
| filter | 5 | Demo 03 CSV /body values and authored yes/no question |
| rank | 6 | Demo 06 query/passage selections and fixed question |
| annotate | 30 | Demo 14 six cases times five typed questions, including two field-selection groups |
| find | 2 | Demo 15 twelve policy lines, one ordinary question and one none-enabled question |
| recognize | 21 | Historical demo 44 ten boundary and three kind questions; current controlled caller-defined fixture six boundary, one kind and one edge question |
| relate | 7 | Demo 45 both ordered service pairs; Beatles relate original names/rules, all five song-to-album menus |

The recognition kind spans use known independent fixture outcomes: Maria Chen, Northwind Freight and Chicago in the historical demo; receipt amount 42.75 at scalar positions [8,13) in the controlled fixture README. They are explicit prior-stage dependencies, not a claimed independently rerun recognition pipeline. Controlled fixture answers establish wording/serialization behavior and no model accuracy.

An additional complete two-question context request matches `specification/fixtures/systemone/batch-context.request.json` byte for byte, excluding the fixture file's terminal newline. Its context, records and question come from `crates/thinkthen/tests/backend/batching/context.rs`. This catches context/state placement and double JSON escaping independently of the saved question-store rows.

All 137 question rows in the selected recording files pass their original v1/v2 key formula and recorded-state SHA-256 checks. This includes rows outside the generated sample. It validates their existing exact identity, not semantic decomposition of those extra rows. `key-checks.json` identifies them individually.

Prior findings reused: experiment 352 documents the older batching/disclosure structure; experiment 299 demonstrates historical whole-batch identity invalidation after insertion. Neither proves current behavior. Current `batch.rs`, request serializer and cache specification own the conclusions here; no engine or historical benchmark was rerun.

# Mutation and dependency checks

For every one of the 81 cases, changing a semantic evidence string changes only the evidence digest among wording/evidence/context. A rendered question or state also changes. These are prototype semantic digests, not replacements for the existing public digest or exact key. Wording/context digests stay fixed by construction, while comparison with independently recorded baseline bytes checks that the parts really render the sampled wire content. `mutations.json` retains each before/after digest and whether the rendered question/state changed.

Context-present evidence mutations leave the context bytes fixed. Context-only mutations leave wording and evidence digests fixed, change state, and retain quoted question bytes.

Three explicit dependency checks pass in `dependency-checks.json`:

- A find candidate addition changes generated choice criteria while authored wording stays fixed.
- Holding all three semantic values fixed and changing a recognition prior span changes the rendered question.
- Holding all three semantic values fixed and reversing a relation pair changes the rendered question.

Those last two are counterexamples to rebuilding exact wire content from only a wording/evidence/context triple. Adding dependency fields solves the prototype reconstruction problem; it does not prove those dependencies can be ignored for reuse.

# Per-function storage implications

Decide, choose, tag, score and filter quote evidence into instructions even when state contains only a shared context or fixed quotation notice. Hashing instructions as wording falsely makes the rule evidence-specific. Authored readings and ordered label/level descriptions belong with wording. Structured descriptions and absent/null distinctions must retain typed JSON and order, not flatten into strings.

Rank uses the same per-record question pattern in this sample. Packed batches share state and hold ordered questions. Existing exact cache identity is per canonical wire question and state, with request-local qN omitted from persistent identity. Preserve original exchange/group information when reconstructing a whole body; the sample recordings do not recover their original network grouping. No claim of model independence from packed neighbors follows.

Annotate has one authored rule per named question and an explicit selected-field group. Store the selection/group mapping alongside rule identity. A source field named context in a selected record is still evidence; field spelling does not establish shared context.

Find has one authored question and a none policy, but ordered candidate evidence generates u001 onward and choice criteria. Its answer depends on the whole candidate set, not an isolated selected row. Keep candidate order, IDs and none policy. A public question digest intentionally excludes generated IDs/count; that digest alone is insufficient exact scope.

Recognition boundary instructions include a marked snippet. Kind and edge stages depend on prior spans; edge criteria contain literal evidence excerpts and candidate labels. Later optional relation stages inherit the ordered recognized entities. A semantic wording digest can represent the caller declaration and engine instruction version, but it must not absorb evidence-derived snippets/options. Preserve stage, source ranges, candidate stretches, stage-effective context, examples and renderer version. When those are unavailable from old rows, keep absent parts and exact-only reuse.

Relate pair questions contain generated IDs. Menus embed the source's name/kind and ordered candidate names/descriptions directly in instructions/criteria. A rule's wording alone cannot describe the rendered menu. Preserve rule, ordered admitted entity set, source/pair and candidates; entity ordering determines IDs. No semantic equivalence or safe cross-candidate reuse was measured.

# Checks, attempts and limits

Final command: `python3 experiments/482-thinkthen-parts-0586/probe.py`. Exit 0; `attempt-5.log` has 81/81 exact question/state comparisons, 81 evidence mutation checks, exact context-body/context-only checks, three dependency counterexamples and 137/137 original key/state validations.

Attempt 1 exposed erroneous trimming of raw text inputs; the independent recording comparison caught the lost final newline in ten cases. Attempt 2 exposed the prototype's incorrect one-token entity description in six custom boundary questions. Attempts 3 through 5 correct those prototype errors and add menu/dependency/key checks. Logs retain each attempt's standard output; attempt 2's assertion traceback appeared on the tool stream because stderr was not redirected then. No product defect is inferred from these failures.

This is deliberately a bounded source-level experiment. It does not meet a literal reading of the ticket's request to decompose every recording in the entire repository. Only the above input-generated 81 cases have decomposition proof; selected recording files' other rows have only key validation. No full engine run, complete release suite, full request grouping replay, OpenAI adapter, images, arbitrary structured wording, every context/example/stage override, recognition relation stage, Unicode tokenization matrix or backend description mode was exercised. The prototype splitter suffices for these fixtures and is not a production Unicode tokenizer. Source review identifies those cases, but does not give them byte-round-trip proof. Unsplit historical or unproven cases should retain absent semantic parts and exact-only reuse until the real database ticket supplies typed planner parts and its own acceptance coverage.

The named schema risk is resolved by a concrete counterexample and a safe fallback. Broadening this prototype into a framework would not improve that decision. The numbered experiment folder holds the report, prototype, input snapshots and raw evidence. It can be removed after review and this finding are recorded. The schema conclusions and limits above remain authoritative; this prototype is not a product acceptance suite.
