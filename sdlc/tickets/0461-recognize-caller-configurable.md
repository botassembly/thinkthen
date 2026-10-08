# 0461: Let callers define recognition entities

Status: OPEN. Slice A is landed: callers define recognition wording and kind descriptions across the supported surfaces, with corrected help and a shared custom-wording case. Slice B remains authorized for size-based admission and follows confirmed bug fixes; the 20-kind cap remains until B lands.

Reviews: accept

Priority: high
Milestone: 0.2

Reviews: revision 8f3ec7cfb4241e9cce9dbb012cf15783aef797b7, accept

Reviews: revision 33631cc62, reject

Reviews: revision a106961a4, accept

Reviews: revision d5619d276, accept

Reviews: revision cbc0ba43466772e0b0b21988c8635a3c3351ea36, accept

## Slice B: Admit complete recognition menus by encoded request size

Ian approved replacing the inherited 20-kind cap with a size limit. The proposal was sent to the PM before implementation and TCGA round one has reported. These conditions are satisfied. Slice A retains its existing checked caller declaration and changes no cap. Slice B follows it and receives a fresh amendment review before code.

Remove the recognition-only built-in kind-count maximum. Keep kind names, descriptions, caller order, duplicate and reserved-name rules. Keep public choose's 255-option limit and tag's 20-label limit. Use an internal validated recognition choice menu in both live question construction and both provider stored-question reconstruction. A menu stays whole: splitting requests means splitting between complete questions, never dividing labels or combining probabilities across label groups.

Recognition uses a hard limit on the actual encoded request body. The default effective limit is 96,000 bytes; caller max-request-bytes and a selected profile can change or lower it according to the existing ceiling contract. An explicitly selected profile max_options remains a local caller constraint; absent profiles and built-in recognition impose no option-count cap. This is software admission, not a claim that any vendor accepts an arbitrary large menu.

Add a recognition-specific sized, strict-singleton bound through engine facade each Asks.requests. Preserve ordinary functions' existing oversized-singleton behavior. Engine.ask_each must preflight all encoded bodies in a stage before any stage send. Apply this bound to both recognition execution stages, contextual requests, step-one context probes and CLI plan. A single oversized complete question produces a safe refusal without authored text. Ordinary packers may split only complete questions. Include descriptions, task wording, context and evidence in the measured adapter-encoded body.

Plan can validate stage-one and probe bodies but cannot know spans derived from later answers. Keep its documented later-stage upper-bound treatment. Validate every actual stage-two request after spans are derived and before any stage-two sends. If stage one has already sent, a later refusal preserves those actual run, usage and recording facts while sending zero additional requests. Never claim zero total requests for that path.

Default wording and cache bytes remain unchanged for previously admitted inputs. Stored-answer/replay construction accepts the same recognition menu and refuses malformed values normally. No model routing, new planner framework, dependency or proof tooling is added.

Outside-in checks cover 255 and 256 caller kinds, including internal none, through both providers' live and stored/replay paths; exact byte limit and one-byte excess for both encoders; a derived stage-two long span and preserved earlier facts; selected profile option and byte constraints; unchanged default keys and wording; public choose 256 and tag over-limit refusals. Count requests for initial and later refusal rather than relying on plan. Use saved exchanges only. Run policy before one fresh code review, full tests/lint on landing source and applicable spec/surfaces. One existing 0461 record gains the slice outcome and lessons.

Stage-selective context, boundary-only execution, proposed-span output and trace documentation are separately owned by 0478, 0479, 0476 and 0477 in 0.2. Slice B also reports the largest single request's bytes and estimated input tokens in --plan and --facts. Guaranteed local model-window refusal remains an unresolved completion criterion: ADR 0032 and specification/backends.md require a tokenizer or verified byte ceiling before enforcing a token-only limit. A byte-to-token estimate supplies neither. No paid call or release management is authorized by this amendment.

### Reversible implementation assumption pending the model-window ruling

Preserve existing route admission; enforce actual encoded-byte limits; report token estimates honestly; provider token refusal remains authoritative until the model-window contract is settled. Build the complete menus, strict byte admission and measurements now. Do not close the unresolved token-window criterion or refuse existing unknown-capacity routes. Do not introduce a tokenizer calibration project.

Use the existing versioned estimate method without lowering its coefficient from a few examples. Add largest_request_bytes, largest_request_estimated_input_tokens and token_estimate_method through existing plan and facts paths. Plan maxima cover prepared execution bodies, exclude admission probes and cannot promise an eventual derived stage-two maximum. Facts maxima cover actual sent bodies; a refused unsent body belongs in safe refusal details, not sent maxima. Preserve earlier actual sends, usage and recordings on later-stage refusal.

The PM ruling needed is: may B complete with strict encoded-byte limits and explicitly estimated largest-request tokens, with providers enforcing actual token fit and unknown routes remaining supported? Guaranteed local token-window refusal would then be deferred until an authoritative token count or verified byte ceiling exists. Alternatively, explicitly approve an estimated admission budget, including possible false refusals and provider overflow, as a revision to the existing contract. Until then, this assumption authorizes the independent byte/menu/facts work only.
Owner: lane 2
Signed: lane 2 ticket owner, 2026-10-07.

## Outcome

Callers supply recognition instructions, an entity definition and descriptions for their own labels. Every recognition stage follows that declaration. A caller can request literal values with units, dates, counts, codes, doses, amounts, ordinary words or web addresses. Recognition applies no fixed semantic label list or suppression to a caller-defined task. The existing question wording remains the compatibility default only when callers omit all custom instructions, entity definition and label descriptions.

Build started on lane 2 after fresh ticket acceptance. Ian promoted 0461 into 0.2 on 2026-10-08 and directed landing now alongside the after-sprint review. The shared custom-wording case and tested example accompany the change. The separate kind-limit proposal is authorized after A and stays below confirmed bugs in priority.

Slice A contains the implemented caller declaration, corrected help, shared case, prior installed qualification and model comparison. Slice B concerns a recognition-only menu with no built-in count cap, subject to hard actual encoded-byte admission in both stages, complete menus and preserved storage and replay. TCGA round one and the proposal condition are satisfied. Build B separately after A. Slice A changes no cap.

## Evidence

- Starts from: `origin/main` at `cba1e86dd1b97d211041ef08c87d86ebe8c97089`; `crates/thinkthen/src/core/recognize/questions.rs` hard-codes `MARKS_OUT`, the proper-name definition, ordinary-word exclusion, `KIND_WORDS`, `DECLINE_WORDS` and `EDGE_WORDS`. `engine/facade/recognize.rs::step_one` reduces kinds to names and loses descriptions. Shared inbox `2026-10-07-pm-thinkthen-make-recognize-caller-configurable-own-instructions-labels-and-definitions.md`, asks 1–4, and `2026-10-07-tcga-demo-let-recognize-find-values-and-caller-defined-entities.md`, answers 1–3, record Ian's outcome. ThinkThen experiment repository record `sdlc/records/0031-rules-confirm-versus-recognize.md` and `experiments/0031-rules-confirm-versus-recognize/results/final-report.md` at landed `787ada336c88023ac080b60f84f77355293e2162` provide the prior evaluation. Its frozen custom-role recognize arm returned 0 true positives, 80 false positives and 100 false negatives for receipt totals across 100 documents, with one native fault. The inbox describes tagging TOTAL rather than its amount. This evidence identifies a task-definition defect; it establishes no general recognition accuracy claim.
- Keeps: existing bare and described kinds, caller label order and exact spellings, omitted-customization request wording and replay identities, token splitting, BILOU decoding, windows, grouping, exact occurrence offsets, strength and threshold rules, text and relation limits, beta text-stated relations, storage controls, secrecy and pre-send validation. Protocol labels such as BILOU tags, ENTITY and none of these retain their existing roles and reservations; they do not define a semantic entity taxonomy.
- Changes: add optional `instructions` and `entity_definition` to the recognition declaration, pass those values and existing per-kind descriptions through token tagging, kind selection, decline and span-boundary questions, and expose the same declaration through native, CLI, saved-file, SDK, SQL, dataframe and MCP surfaces. Include custom semantics in authored question descriptions, normalized identity, cache and replay requests. Update the settled recognition contract through a reviewed ADR, settings reference, affected schemas, executable examples and parity cases together.
  Claim `crates/thinkthen/src/core/recognize/**`, `crates/thinkthen/src/engine/**`, `crates/thinkthen/src/public/**`, `crates/thinkthen/src/cli/recognize/**`, `specification/recognize.md` and `conformance/**`.
- Proof: use the existing loopback request helper, CLI/API tests, saved responses and shared conformance runner. Prove custom instructions and descriptions reach every applicable stage; exact output text, labels, offsets, row counts and exit codes; omitted compatibility; input refusal before sending; and distinct cache/replay identity when custom semantics change. Freeze generic examples and exact gold before evaluation. Report default versus caller-defined recognition on identical texts, including a retained 0031 receipt input, with misses, false positives, abstentions, faults and actual cost. Offline transport and saved-output checks prove behavior, not model accuracy. A model evaluation remains an explicit completion requirement and needs the coordinator's priced allocation before any paid call.
- Defers: GLiNER or another dedicated recognizer comparison to the PM's separate 0.3 experiment; new entity extraction algorithms, clinical interpretation, new provider routing, new proof tooling, dataset expansion and release publication. No paid call, credential access or implementation belongs to this ticket-preparation step.

## Proposed contract for review

### Added public declarations

```text
fn RecognitionReading::entity_definition(&self) -> Option<&str>
fn RecognitionReading::instructions(&self) -> Option<&str>
fn Recognize::reading(&self) -> RecognitionReading<'_>
fn RecognizeBuilder::entity_definition(self, &str) -> Result<RecognizeBuilder, Error>
fn RecognizeBuilder::instructions(self, &str) -> Result<RecognizeBuilder, Error>
```

Use the existing `recognize.kinds` map for caller labels and descriptions. Add optional string members `recognize.instructions` and `recognize.entity_definition` to the closed version-one saved grammar. Rust adds `RecognizeBuilder::instructions` and `RecognizeBuilder::entity_definition`; native question descriptors preserve both authored values. The CLI adds `--instructions TEXT` and `--entity-definition TEXT`. Existing `--kind KIND=DESCRIPTION` remains the per-label input. Explicit CLI values override the corresponding saved members; omission preserves the saved value. Retain the existing refusal for mixing inline kinds or relations with `@FILE`.

Reject an explicitly blank string, null or another JSON type for either new member before sending. Absence means omitted; it never means an explicitly empty override. Accept ordinary multiline instructions through the existing validated content boundary. Treat instructions as model task wording; ThinkThen executes no caller commands. Keep error and Debug output free of authored text and credentials.

When all three custom inputs are absent, preserve the current default questions exactly, including bare caller kind names. When any custom input is present, use neutral entity language and include all supplied semantics in every stage that decides membership, kind or boundary. Omitted fields in that custom mode receive neutral structural wording rather than the old proper-name restrictions. Descriptions alone must permit a requested value without requiring a second switch. An instructions-only declaration and a definition-only declaration must work without kinds; the result uses ENTITY. Kind selection declines only when the span fails the caller's declaration or no caller kind applies. Boundary selection retains literal punctuation that belongs to the requested entity, including decimal points, slashes and web-address punctuation. Preserve structural token and option rules without adding semantic filters.

The canonical declaration includes supplied customization and omits absent customization, preserving historical identity for an unchanged default declaration. Request generation and result provenance must retain supplied descriptions rather than reducing them to label names. Changing instructions, entity definition or a label description must not reuse a question answered under different semantics. Existing default replay recordings must remain usable.

## Build surfaces and coordination

The native path starts in `core/recognize_file.rs`, `core/recognize/questions.rs`, `core/recognize.rs`, `engine/facade/recognize.rs`, `public/recognize.rs` and `public/recognize_question.rs`; CLI admission starts in `cli/args.rs` and `cli/recognize/config.rs`. The saved serializer and normalized description path must change with the parser. Read the landing 0.2 descriptors before coding; this ticket's baseline predates final installed qualification.

The C JSON question loaders already converge on `RecognizeQuestionFile`. Extend native typed construction and authored description readers through an additive, versioned C entry point or carrier; do not append fields to an existing public V1 struct or silently discard them. Record and review the exact additive ABI in the implementation ADR before host adoption. Carry the same fields through typed host recognition builders, question-file loaders, SQL declaration inputs, pandas and Polars adapters, and the MCP recognition schema. Reuse existing common declaration readers. Update the affected installed-package conformance cases; generic JSON acceptance alone does not establish typed parity.

Keep all work on `ticket/0461-recognize-caller-configurable` in lane 2. Existing installed qualification covers the shared case. Land this ticket in 0.2 after integrated review and checks. Add no dependency or verification runner feature.

## Distinguishing examples and checks

Freeze exact occurrence spans for examples such as `Length: 12.5 mm.`, `Date: 2026-10-07.`, `Count: 3 items.`, `Code: AB-12.`, `Dose label: 5 mg.`, `TOTAL 42.75`, and `Address: https://example.org/a.`. The dose example tests literal text and units only. It performs no clinical interpretation. Include a requested ordinary word, an absent amount, repeated amounts and a Unicode prefix to distinguish offsets and false positives. For the receipt regression, retain an actual 0031 text and its independently frozen gold; a synthetic TOTAL line supplements that evidence.

Run a new request-admission regression against the baseline and show its failure before implementation. Through the existing loopback helper, assert the actual questions carry customization and label descriptions in token, kind, decline and boundary stages, with no legacy semantic suppression. Then drive controlled responses through the complete CLI and native API and assert exact entities and source spans. Compare CLI, saved-file and typed-builder precedence with conflicting inputs. Count zero loopback requests for malformed declarations. Exercise the normal cache and replay path after changing each semantic field. Reuse existing default fixtures and compare unchanged request bytes.

Use focused recognition and question-file integration tests during implementation, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` before code review, `sdlc/scripts/tickets`, affected shared and installed surface cases, and `sdlc/scripts/spec` when the executable examples change. The coordinator runs full tests and lint on the landing commit. Keep one complete set of behavior tests and remove temporary scaffolding.

The later before-and-after model evaluation uses identical frozen generic texts, labels, model, route and threshold. Preserve the prior raw 0031 result as historical evidence rather than treating it as a new prediction under a different question. Report exact-span precision and recall with denominators, label errors, faults, requests and actual usage. Reuse existing evaluation facilities; do not build new proof tooling. Before any paid arm, cite current official pricing and obtain the coordinator's finite allocation inside the shared $20 ledger through `sdlc/scripts/live`. This ticket reserves no dollars. If the route is unavailable, retain that gap and do not claim the model evaluation passed. Send the bounded result to the PM after review through the owning coordinator.

## Progress

- 2026-10-08 started
- 2026-10-08 landed 09a07f102a636bd7407a674544f9e2a7c36b7895; next: Partial slice B admits complete recognition menus under strict encoded-byte limits and reports prepared/sent request maxima. Workspace, consumer, binding, lint, specification and affected integration checks pass; R accepts current facts and older snapshots. Guaranteed local model-window refusal remains unresolved under the recorded assumption. Keep the ticket open for the PM ruling; continue independent approved controls before binding migration.
