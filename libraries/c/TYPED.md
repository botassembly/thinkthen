# Counted C carriers (0426, unpublished work)

Ticket 0426 is in progress. The reviewed target is ten `thinkthen_FUNCTION_complete` calls with result/2 views, observations and owned failure snapshots. Their exact canonical signatures from `src/complete-signatures.h` are now declared in `include/thinkthen.h` and exported by this integration checkpoint. No temporary `*_current` ABI is exported. Every released symbol, signature, layout, bare call and JSON compatibility door is preserved.

`include/thinkthen.h` contains the canonical reviewed descriptors and views. They remain unpublished until the whole ticket is integrated and reviewed. The unpublished `record_v1` includes optional original content and context, ordered candidate options and ordered images; no invented `record_v2` is needed.

## Constructors and ownership

`thinkthen_question_new` clones counted descriptors and child questions, then uses the native question grammars for admission. Options and members retain their authored order. `thinkthen_question_load` uses the bounded native question-file reader and parsers. Native refusals remain explicit, including unsupported authored weights. Atomic/rank non-root `on`, custom relate pointers and complete saved rank sets use their actual native routes. Dynamic choose uses actual per-record candidates. Saved find and recognize readings use native `FindQuestionFile`/`RecognizeQuestionFile` and `RecordReading`, including their selected file evidence.

`thinkthen_image_clone` validates JPEG/PNG media, dimensions and pixels through native `ImageInput`, clones the original compressed bytes and optional filename, and never resizes or re-encodes. `thinkthen_image_view` borrows those bytes, dimensions, media and filename until `thinkthen_image_free`. The owner survives engine destruction. Media is explicit; ordinary text/bytes never imply an image.

`thinkthen_source_records` clones original content, context, candidate names/descriptions/weights and every referenced immutable image. It preserves order and duplicates. Text records require original content; image records may omit it. Native image-evidence count/byte/text bounds apply during construction. Candidate cloning does not claim function-specific execution admission.

`thinkthen_source_files` clones an explicit LINE/WINDOW/FILE/IMAGE_FILE/JSONL selection and validates it with native reader options. Construction selects paths without opening evidence. The private integration adapter uses `read_inputs`; it adds no traversal or parser. Image files retain their filename with absent text line coordinates. Text sources retain native Unicode/CRLF and physical line coordinates. Filenames are provenance, never model evidence.

Counted storage must be initialized, readable, UTF-8 where specified, and have its declared extent. NULL data is valid at zero length only. Extent multiplication is checked before slices are formed. Flags/presence are exactly 0/1. Invalid required pointers/descriptors return Usage without writing output. The boundary cannot validate arbitrary addresses.

Constructors own their cloned values. Image and result accessors borrow immutable owner memory; their nested pointers expire when that owner is freed. Reading immutable handles/views concurrently is permitted while owners remain live. Rust alone allocates/frees handles. Free each nonnull owner once after all borrowers finish; NULL frees are harmless. Forged, stale and concurrently freed handles violate the lifetime contract; no registry is added.

## Complete execution and views

All ten canonical complete calls execute native complete routes through one existing engine, parser, reader, cache and scheduler. Each input carries its own context and whole candidate replacement plus ordered native images. The bridge captures owned native question/row observations in their actual event order and keeps final facts through conversion failures. It does not decode result JSON, derive raw picks from probabilities, infer token counts/cost or manufacture identities/provenance.

The complete views retain native resolved questions and readings, full probabilities, raw selections, annotation success/failure states, recognition spans/tables, relation rules/endpoints and original file coordinates. Result storage deep owns every nested byte/array until `thinkthen_result_free`; it survives destruction of engines, questions, sources and caller buffers. Row events join by native original ordinal after rank reordering. A singular summary carries the actual result ID/meta; a multirow summary leaves aggregate ID/meta absent and exposes each row's native values.

Additive `thinkthen_result_details(result, row, out)` and `thinkthen_result_observation_details(result, observation, out)` expose `thinkthen_details_v1`: native question/threshold/raw pick, independently optional reported input/output tokens, all ordered source origins/answerers/batch sizes, all observation/failure identities, and ordered original input views with locations/images. Existing canonical layouts stay unchanged. Canonical `usage` is present only with both known dimensions; additive usage preserves partial reporting. A logical observation with several identities has NULL/zero-length in its canonical singular `observation_id`; the details array retains every identity. Wrong-kind/out-of-range access returns Usage without changing output.

`thinkthen_error_complete` snapshots the existing calling-thread error slot without clearing/replacing it. It returns no owner when no error exists, an owned prestart failure without facts/attempts, or a started failure with actual final facts and opt-in attempts (including empty). Failure summaries have absent schema/function/answer/meta; the code, safe message, retryability and available native structured stop remain typed. The six existing safe errors and every bare/JSON ABI door remain.

## Native integration

All eight native question-loader roles execute directly: atomic, set, per-record choose, recognize, record relate, rank, rank set and find. Counted `thinkthen_question_parse` admits a caller's saved question grammar through an explicit native role. Named/reference loaders use the same roles; loaded members retain native authored grammar when composed into a set. Complete rows preserve authored model/profile/batch/on, name/version and item/context declarations. Original context, literal text and parsed JSON retain their native distinctions.

Complete saved rank sets retain question names, original ordinals, final identities and facts. Additive rank-member accessors expose each selecting judgment and author. Located recognition/relation accessors preserve original source records, spans and endpoint occurrences. The remaining work is whole-change review and landing checks.

## Owned native file batches

The six native streaming record routes expose `thinkthen_decide_batch_start`, `choose_batch_start`, `tag_batch_start`, `score_batch_start`, `filter_batch_start` and `annotate_batch_start`. `thinkthen_batch_next` yields independently owned typed row results. `thinkthen_batch_facts` returns final facts after termination; `thinkthen_batch_free` drops and joins the native batch. Rank, find, recognize and relate keep their aggregate complete calls.

The engine must remain live through batch_free. Start, next, facts and free stay on the creating thread. The batch clones its question/source/context and retains the cancellation flag, so those original handles may be freed after start. Yielded results outlive the batch, source, question and engine. Wrong-thread operations refuse before pulling or freeing. The installed header defines exhaustion, terminal errors and unchanged-output behavior.

`THINKTHEN_SOURCE_JSONL_V1` reads explicit JSONL through the native line reader and `RawRecord::json`. Ordinary text file selections remain literal text. `thinkthen_source_image_files` accepts an explicit image-reader selection; image line/window contradictions refuse before opening a missing path or sending a request. No caller stage markers, host callbacks or second scheduler are introduced.

## Verification and qualification

The existing ASan/LSan public consumer harness now executes all ten calls against owned loopback replies, for both records and native files (exactly 13 requests per run), strict saved-answer rereading and cache hits, per-record contexts/candidates, native partial-member failures, authored nulls, selected find files, ordered duplicate images on decide/choose/score, native image files and seven zero-send image refusals. It retains existing constructor, layout, secrecy, cancellation and private projection regressions. The legacy C key checks now share an independent documented v2 oracle with the Rust public consumer. Original fixture question bytes and v1 validation/replay remain distinct. The synthetic cache-hit fixture reports its literal requested model, preserving its original request and cache counts. The portable C consumer additionally exercises a read-only original v1 snapshot and verifies zero sends and unchanged bytes/mtime.

Root owns the fresh High whole-change review, full landing gates, release rehearsal, Ian's approvals and the one landing record. This pushed integration checkpoint is not a complete parity or source landing claim.

## Coherent 0456 integration API

The following additive API preserves every existing layout and symbol. Native declarations validate both constructors and execution. Explicit object context uses native `ObjectContext` and `RecordContext`; literal JSON-looking text remains text.

```c
/* Additive 0456 declarations. kind: ABSENT=0, STRING=1, OBJECT=2.
 * Object properties preserve authored order; property kinds STRING=1,
 * NUMBER=2, BOOLEAN=3, STRING_LIST=4. Required names preserve their order.
 * Non-object declarations require empty property/required arrays.
 * name/version absence is explicit; no inferred name or default version.
 */
enum thinkthen_input_declaration_kind_v1 {
    THINKTHEN_DECLARATION_ABSENT_V1=0, THINKTHEN_DECLARATION_STRING_V1=1,
    THINKTHEN_DECLARATION_OBJECT_V1=2
};
enum thinkthen_input_property_kind_v1 {
    THINKTHEN_PROPERTY_STRING_V1=1, THINKTHEN_PROPERTY_NUMBER_V1=2,
    THINKTHEN_PROPERTY_BOOLEAN_V1=3, THINKTHEN_PROPERTY_STRING_LIST_V1=4
};
enum thinkthen_question_loader_role_v1 {
    THINKTHEN_LOAD_ATOMIC_V1=1, THINKTHEN_LOAD_SET_V1=2,
    THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1=3, THINKTHEN_LOAD_RECOGNIZE_V1=4,
    THINKTHEN_LOAD_RELATE_V1=5, THINKTHEN_LOAD_RANK_V1=6,
    THINKTHEN_LOAD_RANK_SET_V1=7, THINKTHEN_LOAD_FIND_V1=8
};
typedef struct thinkthen_input_property_v1 {
    thinkthen_string_v1 name; uint32_t kind;
} thinkthen_input_property_v1;
typedef struct thinkthen_input_properties_v1 {
    const thinkthen_input_property_v1 *data; size_t len;
} thinkthen_input_properties_v1;
typedef struct thinkthen_input_declaration_v1 {
    uint32_t kind;
    thinkthen_input_properties_v1 properties;
    thinkthen_strings_v1 required;
} thinkthen_input_declaration_v1;
typedef struct thinkthen_question_author_v1 {
    thinkthen_optional_string_v1 name;
    thinkthen_optional_u64_v1 wording_version;
    thinkthen_input_declaration_v1 item_schema, context_schema;
} thinkthen_question_author_v1;
/* Construct through the same native grammar, with separately counted author
 * metadata. author=NULL means no author metadata. All inputs are cloned.
 */
int thinkthen_question_new_authored(const thinkthen_engine *, const thinkthen_question_spec_v1 *, const thinkthen_question_author_v1 *, thinkthen_question **);
/* Borrow metadata owned by this immutable question until question_free. */
int thinkthen_question_author(const thinkthen_question *, thinkthen_question_author_v1 *);
/* Explicit native loader role: ATOMIC=1, SET=2, DYNAMIC_CHOOSE=3,
 * RECOGNIZE=4, RELATE=5, RANK=6, RANK_SET=7, FIND=8.
 * No host search or fallback after file refusal. Loaded members use native
 * lossless authored grammar for composition; known fields remain typed.
 */
int thinkthen_question_load_named(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 name, thinkthen_question **);
int thinkthen_question_load_reference(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 reference, thinkthen_question **);
/* Borrow native author snapshots until result_free. Row/member ordinals
 * follow the existing named result accessors and annotation member order.
 * Set envelopes and generated internal questions carry absent author fields.
 * Invalid owner/output/ordinal or a member on a non-annotation row is Usage
 * and leaves output unchanged. Observation includes both question/row events.
 */
int thinkthen_result_question_author(const thinkthen_result *, size_t row, thinkthen_question_author_v1 *);
int thinkthen_result_member_author(const thinkthen_result *, size_t row, size_t member, thinkthen_question_author_v1 *);
int thinkthen_result_observation_author(const thinkthen_result *, size_t observation, thinkthen_question_author_v1 *);
```

Shared public parity runs actual typed calls and known result accessors against the canonical fixture inputs. Internal fault injection remains native/C safety coverage; schema-only envelopes run at their decoder boundary. Runtime gaps fail visibly. Root owns the final whole-change review and landing gates.

Named `thinkthen_recognize_complete` accepts each source record's existing context carrier, including an explicit empty text that clears call context. The existing native admission checks every record before dispatch. See [the recognition contract](../../specification/recognize.md#per-record-context) for stage batching and context identity.
