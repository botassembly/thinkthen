# Counted C carriers (0426, unpublished work)

Ticket 0426 is in progress. The reviewed target is ten `thinkthen_FUNCTION_complete` calls with result/2 views, observations and owned failure snapshots. Their exact canonical signatures from `src/complete-signatures.h` are now declared in `include/thinkthen.h` and exported by this integration checkpoint. No temporary `*_current` ABI is exported. Every released symbol, signature, layout, bare call and JSON compatibility door is preserved.

`include/thinkthen.h` contains the canonical reviewed descriptors and views. They remain unpublished until the whole ticket is integrated and reviewed. The unpublished `record_v1` includes optional original content and context, ordered candidate options and ordered images; no invented `record_v2` is needed.

## Constructors and ownership

`thinkthen_question_new` clones counted descriptors and child questions, then uses the native question grammars for admission. Options and members retain their authored order. `thinkthen_question_load` uses the bounded native question-file reader and parsers. Native refusals remain explicit, including authored weights, atomic/rank non-root `on`, custom relate pointers and complete saved rank sets. Dynamic choose uses actual per-record candidates. Saved find and recognize readings use native `FindQuestionFile`/`RecognizeQuestionFile` and `RecordReading`, including their selected file evidence.

`thinkthen_image_clone` validates JPEG/PNG media, dimensions and pixels through native `ImageInput`, clones the original compressed bytes and optional filename, and never resizes or re-encodes. `thinkthen_image_view` borrows those bytes, dimensions, media and filename until `thinkthen_image_free`. The owner survives engine destruction. Media is explicit; ordinary text/bytes never imply an image.

`thinkthen_source_records` clones original content, context, candidate names/descriptions/weights and every referenced immutable image. It preserves order and duplicates. Text records require original content; image records may omit it. Native image-evidence count/byte/text bounds apply during construction. Candidate cloning does not claim function-specific execution admission.

`thinkthen_source_files` clones an explicit LINE/WINDOW/FILE/IMAGE_FILE selection and validates it with native reader options. Construction selects paths without opening evidence. The private integration adapter uses `read_inputs`; it adds no traversal or parser. Image files retain their filename with absent text line coordinates. Text sources retain native Unicode/CRLF and physical line coordinates. Filenames are provenance, never model evidence.

Counted storage must be initialized, readable, UTF-8 where specified, and have its declared extent. NULL data is valid at zero length only. Extent multiplication is checked before slices are formed. Flags/presence are exactly 0/1. Invalid required pointers/descriptors return Usage without writing output. The boundary cannot validate arbitrary addresses.

Constructors own their cloned values. Image and result accessors borrow immutable owner memory; their nested pointers expire when that owner is freed. Reading immutable handles/views concurrently is permitted while owners remain live. Rust alone allocates/frees handles. Free each nonnull owner once after all borrowers finish; NULL frees are harmless. Forged, stale and concurrently freed handles violate the lifetime contract; no registry is added.

## Complete execution and views

All ten canonical complete calls execute native complete routes through one existing engine, parser, reader, cache and scheduler. Each input carries its own context and whole candidate replacement plus ordered native images. The bridge captures owned native question/row observations in their actual event order and keeps final facts through conversion failures. It does not decode result JSON, derive raw picks from probabilities, infer token counts/cost or manufacture identities/provenance.

The complete views retain native resolved questions and readings, full probabilities, raw selections, annotation success/failure states, recognition spans/tables, relation rules/endpoints and original file coordinates. Result storage deep owns every nested byte/array until `thinkthen_result_free`; it survives destruction of engines, questions, sources and caller buffers. Row events join by native original ordinal after rank reordering. A singular summary carries the actual result ID/meta; a multirow summary leaves aggregate ID/meta absent and exposes each row's native values.

Additive `thinkthen_result_details(result, row, out)` and `thinkthen_result_observation_details(result, observation, out)` expose `thinkthen_details_v1`: native question/threshold/raw pick, independently optional reported input/output tokens, all ordered source origins/answerers/batch sizes, all observation/failure identities, and ordered original input views with locations/images. Existing canonical layouts stay unchanged. Canonical `usage` is present only with both known dimensions; additive usage preserves partial reporting. A logical observation with several identities has NULL/zero-length in its canonical singular `observation_id`; the details array retains every identity. Wrong-kind/out-of-range access returns Usage without changing output.

`thinkthen_error_complete` snapshots the existing calling-thread error slot without clearing/replacing it. It returns no owner when no error exists, an owned prestart failure without facts/attempts, or a started failure with actual final facts and opt-in attempts (including empty). Failure summaries have absent schema/function/answer/meta; the code, safe message, retryability and available native structured stop remain typed. The six existing safe errors and every bare/JSON ABI door remain.

## Native dependencies still open

This checkpoint uses pushed native 76e925432 and merged main only. It does not merge active lane0 files. Complete `RankSet` execution is missing: legacy `rank_set_with` cannot supply final complete rows, question_name, original ordinal and final identity. The complete C route refuses that variant before sends.

Native resolved primitive questions expose content/options/yes/no and readings, but no typed authored model/profile/batch/on getters. `FindReading` lacks authored model/on; `RecognitionReading` lacks authored model/on; `RelationReading` lacks authored model. These authored slots stay absent rather than being guessed from effective response metadata. Atomic/rank non-root selection and custom relate pointer admission remain native gaps. Located annotation text has no public native route through the annotation text parser while retaining its location; unlocated annotation text uses the native parser, including valid JSON and invalid JSON literals. Native annotation record composition currently keeps located literal text verbatim. This distinction requires a native correction before qualification.

0456's coherent native update is pending: optional name/wording_version/item_schema/context_schema getters/setters, typed declarations, named/reference loaders and declared object per-record contexts. The C bridge currently accepts text context and refuses object context. Exact additive C declaration/metadata APIs will be recorded in 0426 before family adoption after those native interfaces are supplied.

## Verification and qualification

The existing ASan/LSan public consumer harness now executes all ten calls against owned loopback replies, for both records and native files (exactly 13 requests per run), strict saved-answer rereading and cache hits, per-record contexts/candidates, native partial-member failures, authored nulls, selected find files, ordered duplicate images on decide/choose/score, native image files and seven zero-send image refusals. It retains existing constructor, layout, secrecy, cancellation and private projection regressions. Existing JSON request-key checks currently expose two failures against this native checkpoint; their expectations remain intact.

Root owns the fresh High whole-change review, native integration, full landing gates, release rehearsal, Ian's approvals and the one landing record. This pushed integration checkpoint is not a complete parity or source landing claim.
