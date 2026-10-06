# 0426: Expose typed C calls and complete result carriers

Status: in progress. Lane2 implements the canonical complete C integration against pushed native 76e925432; remaining native dependencies and root whole-change qualification are open.

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

The coordinator accepted the reviewed complete signatures retained in [the private integration contract](../../libraries/c/src/complete-signatures.h) with the ruling that unpublished `record_v1` can include context and candidates. Preserve released symbols/layouts; do not freeze a temporary `*_current` ABI or invent `record_v2`. The branch reuses 22f9f787b constructors, storage and native regression behavior. [The C contract](../../libraries/c/TYPED.md) describes the unpublished target and its remaining integration. The coherent checkpoint now exports the complete signatures against actual native support. Publication, parity claims and the landing record await remaining dependencies and root's fresh High whole-change review.

Current implementation executes all ten native complete routes and clones ordered context/candidates/images. It uses native pixel validation, shared text/image readers and owned observations, and stores immutable canonical question/value/location/image/result/failure backing allocations. Complete identities, provenance, facts/attempts and stops come from native getters. Lane0 is read-only to this builder; its active WIP is not merged. Final full tests/lint on the landing candidate remain with the coordinator.

Review exact exported signatures before code. Existing ABI symbols remain valid. The borrowed/owned lifetime contract and invalid-handle behavior must be explicit; memory safety needs High review.

## Vision and answer identity in 0.2

Second PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, asks 1–4, supersedes the image deferral. Adopt 0447 typed single/multiple-image and image-source inputs for decide/choose/score, 0448 route admission, and 0450 stable full-result identity/proxy reservations. Preserve image order/duplicates and absent text lines; ordinary strings/bytes do not imply images. Execute text-only refusals for the other seven functions, with zero sends. This ticket owns its public typed carriers/consumers; shared native image behavior stays in 0447 and SQL adaptation in 0452. Respect 0449’s single endpoint/key/API-type boundary. Known image/result/identity fields cannot remain raw JSON.

## Coherent native integration (2026-10-06)

Lane2 integrates pushed native checkpoint 76e925432 against the preserved canonical
complete-signatures.h layouts. Native getters now supply all atomic/record,
find, annotate, recognize and relate complete routes, owned question events,
raw selections, normalized questions/thresholds, aligned provenance/identities,
actual call/send IDs, final facts and structured stops. No result JSON is decoded.

Exact remaining native dependency: there is no public complete `RankSet` route
returning native final ranked complete rows, their `question_name`, original
ordinal and final answer identity. Existing `rank_set_with` returns the released
legacy carrier and cannot supply the final complete identity. Complete C refuses
that variant before sends rather than building a second merge/scheduler. The
0456 coherent native update is separately pending: name/wording_version,
item_schema/context_schema getters and setters on authored/resolved questions,
validated typed declarations, named/reference loaders, and declared object
per-record context admission. Lane2 does not edit native sources to supply them.

Additional missing native interfaces: typed authored model/profile/batch/on getters
for `ResolvedQuestion`, authored model/on for `FindReading`/`RecognitionReading`,
authored model for `RelationReading`, atomic/rank non-root reading admission,
custom relate pointer admission, and a
located annotation-text composition entry point that applies the native
annotation parser (valid JSON structurally; syntax-invalid JSON as literal text)
while retaining `SourceLocation`. C does not supply substitute parsers or infer
authored setup from response metadata. Unlocated annotation text already uses
the existing native parser. Native saved find/recognize selected readers are
integrated.

Additive C details preserve every canonical v1 layout. The implemented additive
`thinkthen_details_v1` exposes a native question/threshold/raw selection, separate
optional input/output token counts, ordered source details with optional batch
sizes, and ordered original input views with physical locations/images. New
`thinkthen_result_observation_details` and `thinkthen_result_details` accessors
borrow immutable owned storage. These additions are recorded here before any
family adoption; the exact installed declarations are the ABI authority. Optional
0456 metadata/declaration additions will be recorded here when its coherent native
interfaces are supplied. This is integration work, not a landing/parity record.

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

Actual checkpoint checks: offline `policy.py` passed; C all-targets Clippy with
`-D warnings` passed; C/C++ canonical/additive layout checks and exact static/dynamic
header export inventories passed. C `cargo test --locked --offline` finished with
19/19 Rust tests and 65/67 public-consumer tests passing, including all eight
`current::` consumer tests. The two unchanged failures are
`cases::portable::c_json_records_keep_fixture_questions_and_keys_in_one_request`
and `cases::every_applicable_shared_case_passes_through_the_door` (15 shared
request-key expectations differ). Both C ratchets equal measured totals. These
are C-local integration checks, not root landing gates.

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

## Current coherent native integration and remaining interfaces

Merged committed native 8140d01dc and 8463aa30c (including current main) at the
clean pushed checkpoint; no uncommitted lane0 input is used. The source-count
merge conflict was resolved by measuring 143695 nonblank Rust lines from the
configured directories. The additions above now map actual native 0456
name/version/declaration getters on questions, complete rows, annotation members
and native question/row observations. Typed object context is constructed with
native RawRecord/ObjectContext/RecordContext. All ten real judgments run with
metadata after their original handles/buffers are freed; named/reference native
roles also execute real judgments. Invalid later declared records/context send
nothing. No production hash, parser, cache, scheduler, IDs or authored settings
are reconstructed at the host.

Exact remaining native getter needs: authored `ResolvedQuestion::{model,
profile,batch,on}`, `FindReading::{model,on}`, `RecognitionReading::{model,on}`,
and `RelationReading::model`. Existing author name/wording_version/item_schema/
context_schema getters are adopted; effective metadata.model is not used to
fill authored model. Complete saved rank-set execution still needs a public
record route and complete row values with question_name/original ordinal/native
identity/final facts. Custom relate pointers and located annotation text need
native admission/composition while keeping original coordinates and native
JSON-versus-literal parsing. Existing atomic/rank non-root on refusal is retained
until a coherent native record-selection route is supplied.

Additional native loader needs identified while adopting the actual available
interfaces: rank-specific named/reference admission (ordinary LoadedQuestion
cannot preserve whether a cut was authored during host conversion), and native
authored serialization or complete set composition for native-loaded members.
The existing C member constructor serializes its already-owned original
question grammar; native named handles expose no equivalent original grammar.
They are refused as new typed members rather than reconstructed from incomplete
getters. FindQuestionFile has no named/reference loader; native literal/builder
find metadata is adopted without adding a new grammar. The additive named C
roles are precisely the five native-supported roles documented above.

The controlled shared/C v2 oracle adoption is coordinated in existing 0444.
Rust and C consumers now reuse one independent framing/fixture-mapping helper.
Only synthetic case 40's reported model and corresponding detailed-model
expectation change, aligning the fixture with its literal requested model so
its two-calls/one-send/one-cache-answer behavior remains valid under v2 freshness.
Every request/question byte and distinct case remains; all committed v1 saved
record fixtures stay unchanged. The existing portable C test additionally pins
original fixture question bytes and read-only v1 replay bytes/mtime with zero
sends through the actual JSON compatibility door.

Current checkpoint checks: C `cargo test --locked --offline` passes 19/19 Rust
and 69/69 public-consumer tests, including all ten sanitizer judgments for
records/files (13 sends each), five native named roles (7 sends), declared
object/absent/empty context (3 sends) and seven staged declaration refusals
(0 sends). Full C shared corpus: 55 cases, 53 applicable pass, 0 fail, the
same 2 established inapplicable cases. C/C++ layout/signature checks and exact
static/dynamic installed-header export checks pass. C all-targets Clippy with
-D warnings, shared Rust public consumer Clippy, formatting and offline
policy.py pass. The shared Rust public corpus passes the same 53 applicable
cases with no failures. The original-v1 portable C replay check passes with
unchanged request/question bytes, no new sends and unchanged saved bytes/mtime.

Measured ceilings now equal actual: root 143731 Rust lines, C 13899 Rust lines,
C 2177 C lines. Root growth from the measured merge total 143695 is the shared
independent fixture/key oracle; C growth from 13236/1983 is typed author storage,
validated additive FFI, native loader/context adoption and actual consumer
coverage. Duplication was checked in the existing descriptor constructor,
complete question/row/observation storage and both legacy key helpers: the
constructor is shared, backing conversions are reused, and duplicate key/
fixture mapping algorithms are replaced with the one shared independent oracle.
No new dependency or runner is introduced. These are integration checks;
root retains fresh whole High C+shared-oracle review and full landing gates
once native dependencies are main. No full parity or source landing is claimed.
