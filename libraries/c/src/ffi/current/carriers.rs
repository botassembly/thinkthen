//! Frozen C descriptor layouts; field order matches include/thinkthen.h.
use std::ffi::c_char;
/// Reserved image handle; no exports exist until native image validation lands.
#[derive(Debug)]
pub struct ImageHandle {
    _private: (),
}
macro_rules! layout {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[doc = concat!("Frozen C layout `", stringify!($name), "`; see include/thinkthen.h.")]
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name {
            $(#[doc = concat!("Header field `", stringify!($field), "`.")] pub $field: $ty,)*
        }
    };
}
layout!(StringV1 { data: *const c_char, len: usize });
layout!(StringsV1 { data: *const StringV1, len: usize });
layout!(OptionalStringV1 {
    present: i32,
    value: StringV1
});
layout!(OptionalSizeV1 {
    present: i32,
    value: usize
});
layout!(OptionalU64V1 {
    present: i32,
    value: u64
});
layout!(OptionalU16V1 {
    present: i32,
    value: u16
});
layout!(OptionalDoubleV1 {
    present: i32,
    value: f64
});
layout!(OptionalDiscriminatorV1 {
    present: i32,
    value: u32
});
layout!(ContentV1 {
    kind: u32,
    data: StringV1
});
layout!(OptionalContentV1 {
    present: i32,
    value: ContentV1
});
layout!(RuleV1 {
    kind: u32,
    low: f64,
    high: f64
});
layout!(OptionalRuleV1 {
    present: i32,
    value: RuleV1
});
layout!(ChoiceV1 {
    name: StringV1,
    description: OptionalContentV1,
    weight: OptionalDoubleV1
});
layout!(ChoicesV1 { data: *const ChoiceV1, len: usize });
layout!(RelationV1 {
    name: StringV1,
    source: StringV1,
    target: StringV1,
    reads: OptionalStringV1,
    either: i32,
    single: i32
});
layout!(RelationsV1 { data: *const RelationV1, len: usize });
layout!(MemberSpecV1 { name: StringV1, question: *const crate::current::QuestionHandle });
layout!(MemberSpecsV1 { data: *const MemberSpecV1, len: usize });
layout!(QuestionSpecV1 {
    kind: u32,
    text: ContentV1,
    yes: OptionalContentV1,
    no: OptionalContentV1,
    choices: ChoicesV1,
    threshold: RuleV1,
    relation_threshold: RuleV1,
    model: OptionalStringV1,
    profile: OptionalStringV1,
    batch: OptionalSizeV1,
    batch_max: i32,
    none: i32,
    on: StringsV1,
    members: MemberSpecsV1,
    kinds: ChoicesV1,
    relations: RelationsV1,
    name_pointer: OptionalStringV1,
    kind_pointer: OptionalStringV1
});
layout!(ImagesV1 { data: *const *const ImageHandle, len: usize });
layout!(ImageViewV1 { media: u32, bytes: *const u8, bytes_len: usize, width: u32, height: u32, filename: OptionalStringV1 });
layout!(ImageViewsV1 { data: *const ImageViewV1, len: usize });
layout!(OptionalImageViewsV1 {
    present: i32,
    value: ImageViewsV1
});
layout!(RecordV1 {
    original: OptionalContentV1,
    context: OptionalContentV1,
    options: ChoicesV1,
    images: ImagesV1
});
layout!(SourceSpecV1 {
    paths: StringsV1,
    unit: u32,
    window: usize
});
layout!(ControlsV1 { deadline_ms: i64, cancel: *mut thinkthen::CancelToken, context: OptionalContentV1, batch: OptionalSizeV1, batch_max: i32, attempts: i32, surface: StringV1 });
layout!(CurrentValueV1 {
    kind: u32,
    decide_kind: u32,
    boolean: i32,
    authored: OptionalContentV1,
    choice: OptionalStringV1,
    tags: StringsV1,
    score: f64
});
layout!(ProbabilityV1 {
    name: StringV1,
    probability: f64
});
layout!(ProbabilitiesV1 { data: *const ProbabilityV1, len: usize });
layout!(LocationV1 {
    file: OptionalStringV1,
    first_line: OptionalSizeV1,
    last_line: OptionalSizeV1
});
layout!(OptionalLocationV1 {
    present: i32,
    value: LocationV1
});
layout!(CurrentFactsV1 {
    cache_answers: u64,
    estimated_cost_usd: OptionalStringV1,
    input_tokens: OptionalU64V1,
    model: OptionalStringV1,
    output_tokens: OptionalU64V1,
    records: u64,
    requests_sent: u64,
    seconds: f64
});
layout!(CurrentMetaV1 {
    question_sha256: StringV1,
    model: StringV1,
    url: StringV1,
    requests: StringsV1,
    requests_sent: u64,
    cached: i32,
    usage_present: i32,
    input_tokens: u64,
    output_tokens: u64,
    tuned_for: OptionalStringV1,
    running: OptionalStringV1
});
layout!(CurrentRowV1 {
    input: OptionalContentV1,
    position: OptionalLocationV1,
    index: OptionalSizeV1,
    probability: OptionalDoubleV1,
    question_name: OptionalStringV1
});
layout!(CurrentAtomicV1 {
    common: CurrentRowV1,
    value: CurrentValueV1,
    yes_probability: OptionalDoubleV1,
    probabilities: ProbabilitiesV1,
    nearest: OptionalStringV1,
    confidence: OptionalDoubleV1,
    meta: CurrentMetaV1
});
layout!(CurrentMemberV1 {
    name: StringV1,
    state: u32,
    value: CurrentValueV1,
    failure: u32
});
layout!(CurrentMembersV1 { data: *const CurrentMemberV1, len: usize });
layout!(CurrentAnnotationV1 {
    common: CurrentRowV1,
    members: CurrentMembersV1
});
layout!(EntityV1 {
    text: StringV1,
    start: usize,
    end: usize,
    length: usize,
    kind: StringV1,
    strength: f64
});
layout!(EntitiesV1 { data: *const EntityV1, len: usize });
layout!(EntityEdgeV1 {
    relation: StringV1,
    source: EntityV1,
    target: EntityV1,
    probability: f64,
    either: i32
});
layout!(EntityEdgesV1 { data: *const EntityEdgeV1, len: usize });
layout!(CurrentRecognitionV1 {
    common: CurrentRowV1,
    entities: EntitiesV1,
    relations_present: i32,
    relations: EntityEdgesV1
});
layout!(EndpointV1 {
    name: StringV1,
    kind: StringV1
});
layout!(EdgeV1 {
    relation: StringV1,
    source: EndpointV1,
    target: EndpointV1,
    probability: f64,
    either: i32
});
layout!(EdgesV1 { data: *const EdgeV1, len: usize });
layout!(CurrentRowsV1 { data: *const CurrentRowV1, len: usize });
layout!(CurrentRelationsV1 {
    edges: EdgesV1,
    inputs: CurrentRowsV1
});
layout!(CurrentQuestionV1 {
    index: usize,
    member: OptionalStringV1,
    stage: OptionalStringV1,
    position: usize,
    meta: CurrentMetaV1,
    state: u32,
    value: CurrentValueV1,
    failure: u32,
    yes_probability: OptionalDoubleV1,
    probabilities: ProbabilitiesV1,
    confidence: OptionalDoubleV1,
    failed_questions: usize
});
layout!(CurrentAttemptV1 {
    ordinal: u64,
    request_sha256: StringV1,
    wall_ms: u64,
    outcome: u32,
    status: OptionalU16V1,
    server_ms: OptionalU64V1,
    request_id: OptionalStringV1
});
layout!(CurrentSummaryV1 {
    function: u32,
    count: usize,
    question_count: usize,
    facts: CurrentFactsV1,
    attempts_present: i32,
    attempt_count: usize
});

layout!(CurrentCandidateV1 {
    common: CurrentRowV1,
    probability: f64
});
layout!(CurrentCandidatesV1 { data: *const CurrentCandidateV1, len: usize });
layout!(CurrentFindV1 {
    selected: CurrentRowV1,
    candidates: CurrentCandidatesV1
});
