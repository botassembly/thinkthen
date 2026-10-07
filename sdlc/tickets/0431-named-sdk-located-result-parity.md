# 0431: Complete typed located results in existing named SDKs

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Rust, Python, TypeScript/JavaScript, Ruby and R expose all ten file routes with complete typed located results and final facts.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 2, 3 and 5.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Every admitted function/input combination is required; function-specific refusals remain explicit. Named calls are decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Known result fields must have typed accessors, including details, probabilities, final facts, annotation states, spans, relation endpoints and located file/first-line/last-line coordinates. Arbitrary caller payloads may remain JSON; known engine fields may not. Explicit question/file variants avoid guessing paths from text. Reuse Rust SourceRecord/read_files and Python FileSelection. Replace known object/raw-JSON holes with validated result carriers. Retain original source occurrences and both relation endpoints.
- Proof: Use the existing canonical behavior/settings/types/files fixtures through actual named public consumers. Check admitted text, records, files, question files/sets, descriptions, contexts and options; six errors; complete typed results; cache, record and changed-reading replay with counted zero sends. Compile consumers where the language is static; inspect documented carrier accessors where dynamic. Retain duplicate identities, Unicode/CRLF locations, empty/null states and started-failure facts. No paid calls or new evidence system.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0409 owns ordinary TypeScript rank/find declarations; 0412 owns R position conventions; 0410 owns dataframe adaptation. This ticket owns ordinary SDK/source carrier gaps and their local docs. Adopt 0442 metadata, 0300 prices and 0445 attempts.

## Design notes

JavaScript runtime and TypeScript compile consumers are distinct checks. A file relocation changes coordinates while identical evidence preserves provider/cache identity. Decoded field projection cannot fabricate a physical source map.

## Existing API issue

Own item 7 of the public-library-api-gaps issue for these SDKs: string and structured description objects, including recognize kinds, and typed named annotate carriers preserve the native grammar and request identity. 0426–0430 adopt the same contract; 0432 compares shared bytes/digests. No Python reflection/derive framework is required.

## Vision and answer identity in 0.2

Second PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, asks 1–4, supersedes the image deferral. Adopt 0447 typed single/multiple-image and image-source inputs for decide/choose/score, 0448 route admission, and 0450 stable full-result identity/proxy reservations. Preserve image order/duplicates and absent text lines; ordinary strings/bytes do not imply images. Execute text-only refusals for the other seven functions, with zero sends. This ticket owns its public typed carriers/consumers; shared native image behavior stays in 0447 and SQL adaptation in 0452. Respect 0449’s single endpoint/key/API-type boundary. Known image/result/identity fields cannot remain raw JSON.

### Added public declarations

```text
fn Engine::try_choose_dynamic_records_complete_with<'a, I, T>(&'a self, &'a RecordChooseQuestion, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteChoice>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn QuestionInput::annotation_document(&str) -> Result<QuestionInput, Error>
```

The location-free annotation document constructor reuses the existing native document reading and record composition. Valid JSON stays structural, syntax-invalid text stays literal, and duplicate/depth/size/blank refusals stay native. `annotation_text(text, location)` shares the constructor’s native admission helper and retains its physical source separately. No location is invented for caller documents.

The lazy dynamic choose prerequisite reuses native record admission and scheduling. It requires candidates on every original and yields the completed prefix, one terminal error with joined final facts, then exhaustion. Empty input validates call controls and completes without a question or request. The eager dynamic choose method retains whole-set admission before sending.

This family adopts the supported 0409 rank/find declarations, 0411 native changed-reading cache/record/replay and answer identities, and 0418 typed complete rank sets. The R docs retain 0412’s separate complete/native and ordinary R position conventions. Host carriers admit the pending shared serializer’s typed occurrence index, physical source and ordered find candidates; native serialization remains with its owner.

Ordered rank-set children adopt the accepted native `members` serialization as typed `RankMember`/`RankMemberResult` values, preserving saved keys separately from authored question names, positive member positions, identities, sources and independently optional usage dimensions. No adapter reconstructs judgments or adds overlapping member/parent usage. One shared `rank-set-ordered-members` case under the existing 0432 named-input descriptor uses the independent six-answer capture; all ordinary SDK consumers execute their actual public rank-set calls against it.

The existing native Python `_complete(request, deadline, token, surface=None)` and `_complete_batch` doors admit only `python`, `pandas` and `python-polars` using native `Surface` admission; absence defaults to Python. Both eager and lazy native options carry that surface. This is a wrapper identity argument, with no configuration or endpoint policy. Frame-specific consumer files remain owned by the frame family.
