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

The integration includes the committed 8140d01dc foundation and 8463aa30c correction with current main; no active lane0 files are merged. 0456 author metadata/declarations and explicit object context are now adopted through native APIs. Native named/reference loaders execute directly for atomic questions, sets, dynamic choose, recognize and relate. Additive constructor and borrowed question/row/member/observation author views are specified below. Native declaration admission stages invalid later records before any sends.

The remaining native interface requirements are:

- A complete saved rank-set record route returning final facts, native row identity, original ordinal and question_name. The existing legacy `rank_set_with` cannot provide those complete values.
- Authored-only `ResolvedQuestion` model/profile/batch/on getters; `FindReading` model/on; `RecognitionReading` model/on; `RelationReading` model. Existing effective response metadata cannot substitute for authored settings. 0456 name/version/declaration getters are available and mapped.
- Native complete admission for custom relate field pointers and a located annotation text composition path that preserves native JSON-versus-literal parsing and original coordinates. Existing atomic/rank loader non-root `on` refusals remain unchanged; selected evidence requires a coherent native route.
- Native named/reference rank admission, plus native authored serialization or set composition retaining all saved member settings. Named atomic handles can execute atomic judgments, but C cannot reparse them for rank or embed them in the existing descriptor-built set without that native support. No host file/name resolver or reconstructed authored JSON is added. `FindQuestionFile` has no native named/reference loader; find literal/builder metadata works through the existing native grammar.

These are integration dependencies, not manufactured absent data or a claim that the native whole ticket is complete. Their concrete remaining needs are recorded in 0426.

## Verification and qualification

The existing ASan/LSan public consumer harness now executes all ten calls against owned loopback replies, for both records and native files (exactly 13 requests per run), strict saved-answer rereading and cache hits, per-record contexts/candidates, native partial-member failures, authored nulls, selected find files, ordered duplicate images on decide/choose/score, native image files and seven zero-send image refusals. It retains existing constructor, layout, secrecy, cancellation and private projection regressions. The legacy C key checks now share an independent documented v2 oracle with the Rust public consumer. Original fixture question bytes and v1 validation/replay remain distinct. The synthetic cache-hit fixture reports its literal requested model, preserving its original request and cache counts. The portable C consumer additionally exercises a read-only original v1 snapshot and verifies zero sends and unchanged bytes/mtime.

Root owns the fresh High whole-change review, native integration, full landing gates, release rehearsal, Ian's approvals and the one landing record. This pushed integration checkpoint is not a complete parity or source landing claim.

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
    THINKTHEN_LOAD_RELATE_V1=5
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
 * RECOGNIZE=4, RELATE=5. No host search or fallback after file refusal.
 * These native-loaded handles execute directly. Embedding them as typed set
 * members awaits a native authored serialization/set composition API.
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

Current integration checks pass: 19 Rust tests and all 69 C public-consumer tests,
C all-targets Clippy, shared Rust corpus and its Clippy, C11/C++17 layouts and
signatures, exact static/dynamic header exports, offline policy and measured
root/C ratchets. Actual loopback counts remain 13 for each ten-call records/files
run, 7 for native named roles, 3 for typed context, 0 for declaration refusals.
Read-only original-v1 replay preserves bytes/mtime and sends nothing. Root's
whole High review and landing gates remain pending native completion.
