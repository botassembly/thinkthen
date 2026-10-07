# 0426: Expose typed C calls and complete result carriers

Status: complete. All ten typed C functions and owned readers pass 247 applicable shared cases, installed sanitizer consumers and fresh review. Full integration tests, lint and executable documentation passed; final platform qualification belongs to 0425.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

C consumers can construct all ten inputs and inspect every stable output field without assembling whole request JSON or decoding whole result JSON.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 2, 3 and 5.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Every admitted function/input combination is required; function-specific refusals remain explicit. Named calls are decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Known result fields must have typed accessors, including details, probabilities, final facts, annotation states, spans, relation endpoints and located file/first-line/last-line coordinates. Arbitrary caller payloads may remain JSON; known engine fields may not. Explicit question/file variants avoid guessing paths from text. Use additive opaque owned result/question/source handles, counted strings/arrays and function-specific accessors. Do not expose Rust collection layouts or borrow result memory beyond documented ownership.
- Proof: Use the existing canonical behavior/settings/types/files fixtures through actual named public consumers. Check admitted text, records, files, question files/sets, descriptions, contexts and options; six errors; complete typed results; cache, record and changed-reading replay with counted zero sends. Compile consumers where the language is static; inspect documented carrier accessors where dynamic. Retain duplicate identities, Unicode/CRLF locations, empty/null states and started-failure facts. No paid calls or new evidence system.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

Settle shared schema with 0408, 0300 and 0442 first. This ticket owns the additive C ABI and ownership contract; host families adopt it. Semantic owners 0406/0407/0413/0414/0418 implement shared behavior, not duplicate host parsers.

## Design notes

The coordinator accepted the reviewed complete signatures retained in [the private integration contract](../../libraries/c/src/complete-signatures.h) with the ruling that unpublished `record_v1` can include context and candidates. Preserve released symbols/layouts; do not freeze a temporary `*_current` ABI or invent `record_v2`. The branch reuses 22f9f787b constructors, storage and native regression behavior. [The C contract](../../libraries/c/TYPED.md) describes the unpublished target and its remaining integration. The coherent checkpoint now exports the complete signatures against actual native support. Publication and the landing record await actual final parity, root's fresh High whole-change review and landing checks.

Current implementation executes all ten native complete routes and clones ordered context/candidates/images. It uses native pixel validation, shared text/image readers and owned observations, and stores immutable canonical question/value/location/image/result/failure backing allocations. Complete identities, provenance, facts/attempts and stops come from native getters. Only coherent committed native checkpoints are merged; lane0 remains read-only. Final full tests/lint on the landing candidate remain with the coordinator.

Review exact exported signatures before code. Existing ABI symbols remain valid. The borrowed/owned lifetime contract and invalid-handle behavior must be explicit; memory safety needs High review.

## Vision and answer identity in 0.2

Second PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, asks 1–4, supersedes the image deferral. Adopt 0447 typed single/multiple-image and image-source inputs for decide/choose/score, 0448 route admission, and 0450 stable full-result identity/proxy reservations. Preserve image order/duplicates and absent text lines; ordinary strings/bytes do not imply images. Execute text-only refusals for the other seven functions, with zero sends. This ticket owns its public typed carriers/consumers; shared native image behavior stays in 0447 and SQL adaptation in 0452. Respect 0449’s single endpoint/key/API-type boundary. Known image/result/identity fields cannot remain raw JSON.

## Coherent native integration (2026-10-06)

The coherent C implementation uses the committed native complete routes, authored settings/declarations, lossless loaded-member composition, rank sets and located annotation admission. Remaining qualification is actual shared public parity, root's fresh High review and landing checks.

Additive C details preserve every canonical v1 layout. The implemented additive
`thinkthen_details_v1` exposes a native question/threshold/raw selection, separate
optional input/output token counts, ordered source details with optional batch
sizes, and ordered original input views with physical locations/images. New
`thinkthen_result_observation_details` and `thinkthen_result_details` accessors
borrow immutable owned storage. These additions are recorded here before any
family adoption; the exact installed declarations are the ABI authority. 0456 metadata/declaration additions below use the supplied native interfaces. This is integration work, not a landing/parity record.

Exact additive detail declarations (installed header is the layout authority):

```c
typedef struct thinkthen_reported_usage_v1 {
    int present;
    thinkthen_optional_u64_v1 input_tokens, output_tokens;
} thinkthen_reported_usage_v1;
typedef struct thinkthen_source_detail_v1 {
    uint32_t origin;
    thinkthen_string_v1 answered_by;
    thinkthen_optional_size_v1 batch_size;
} thinkthen_source_detail_v1;
typedef struct thinkthen_source_details_v1 {
    const thinkthen_source_detail_v1 *data; size_t len;
} thinkthen_source_details_v1;
typedef struct thinkthen_input_view_v1 {
    thinkthen_optional_content_v1 original;
    thinkthen_optional_location_v1 position;
    thinkthen_optional_image_views_v1 images;
} thinkthen_input_view_v1;
typedef struct thinkthen_input_views_v1 {
    const thinkthen_input_view_v1 *data; size_t len;
} thinkthen_input_views_v1;
typedef struct thinkthen_details_v1 {
    thinkthen_optional_question_v1 question;
    thinkthen_optional_rule_v1 threshold;
    thinkthen_optional_string_v1 raw_pick;
    thinkthen_reported_usage_v1 usage;
    thinkthen_source_details_v1 question_sources;
    thinkthen_observation_identities_v1 observations;
    thinkthen_input_views_v1 inputs;
} thinkthen_details_v1;
int thinkthen_result_details(const thinkthen_result *, size_t, thinkthen_details_v1 *);
int thinkthen_result_observation_details(const thinkthen_result *, size_t, thinkthen_details_v1 *);
```

Both getters borrow result-owned immutable memory; row ordinals match named
result accessors and observation ordinals match `thinkthen_result_observation`.
Invalid owner/output/ordinal returns Usage unchanged. Partial usage dimensions
retain separate presence flags. Logical observations with several native IDs
leave their canonical singular observation_id NULL/zero-length; details retains
all actual IDs. Multirow summaries leave aggregate ID/meta absent.

Checkpoint verification uses the existing sanitizer/public consumers and saved
image replies. All ten calls run on records and native files with exactly 13
loopback requests each. Per-record context/dynamic candidates, record/replay,
changed readings, cache hits, failure snapshots, image order/duplicates/files
and seven zero-send refusals execute actual native judgments. Existing JSON-door
request-key tests report native/fixture hash mismatches; they are not rewritten
or bypassed here. Root retains whole-change review, final gates and landing.

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

## Current native adoption

All eight named/reference roles, authored setting getters, selected-record admission, loaded-member composition and complete rank sets use native APIs. C performs no substitute parser, cache or scheduler work; original v1 saved replay and generic ABI compatibility remain covered separately.

C adoption checkpoint after main cache fixes and native e714bb018: complete
saved rank sets now execute through the native record route and retain their
selecting member, original ordinals, every member judgment, separate metadata
and authors through additive rank-member accessors. Eight explicit named roles
include rank, rank set and find. Custom relation record plans use the native
record grammar. Additive located recognition/relation accessors preserve the
native physical spans and expanded endpoint occurrences, with typed originals;
old layouts and symbols remain unchanged. Actual ASan consumers retain these
views after engine, question and source handles are freed. Focused checks pass
14 Rust checks and 11 public consumers; all-target Clippy passes. Shared applicable public
C parity adoption remains in progress; no whole-surface parity claim is made.

Native consumer adoption preserves loaded atomic/rank questions through the native file grammar for set composition. Complete C views now retain explicit model/profile/batch/pointer settings and located annotation uses native structural/literal admission. Bare score levels remain absent descriptions; fully described levels preserve authored null; mixed levels refuse with the same Usage as the public Rust ScoreBuilder. Fourteen private Rust tests and eleven actual ASan C consumers pass; a focused public regression also proves authored null versus absence and mixed refusal. Shared public parity remains open and currently reports actual gaps, without compatibility JSON substituting for typed known fields.

The additive lazy C batch starts cover the six native streaming record routes. The batch owns stable question/source/context/cancellation backing, uses the existing fallible native scheduler and explicit JSONL input, and drops the native batch before backing. The engine stays live through batch_free and all batch operations stay on the creating thread. Independently owned typed row results outlive the batch. Rank, find, recognize and relate keep their native aggregate complete APIs. The public sanitizer consumer checks a completed two-row prefix, rejected valid sibling, first-stage zero-send refusal, literal-text distinction, cancellation after start, and drop before/after a yielded row.

Named-provider fixture isolation was corrected after six fake-key Liquid calls received 401 responses from its default endpoint: the generic URL environment variable does not override a named provider. Every fixture now supplies its owned loopback URL explicitly in settings, and the C consumer validates that setting before engine creation. Only the literal fixture key was used; no real credentials or paid probes were read or sent. Whole-change review must inspect this isolation across new projections.

## What the build taught us

The C boundary must preserve native distinctions between literal files, explicit JSONL, absent descriptions and authored nulls. Native batching chooses its own admitted prefix; caller stage markers were not a public capability. Named providers require an explicit owned address in fixture settings, and refreshing a stored answer creates a new observation and answer ID while replaying the preserved original keeps its original identities.

The shared C executor now passes all 247 applicable public cases through typed complete calls and known-field accessors, including all 24 image admission cells. Schema-only envelopes and private invariant injection remain at their actual decoder and safety boundaries. Whole-change review and landing checks remain open.
