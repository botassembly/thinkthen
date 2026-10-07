//! Canonical reviewed C layouts; see include/thinkthen.h.
layout!(StringV1 {
    data: *const std::ffi::c_char,
    len: usize,
});
layout!(StringsV1 {
    data: *const StringV1,
    len: usize,
});
layout!(OptionalStringV1 {
    present: i32,
    value: StringV1,
});
layout!(OptionalSizeV1 {
    present: i32,
    value: usize,
});
layout!(OptionalU64V1 {
    present: i32,
    value: u64,
});
layout!(OptionalU16V1 {
    present: i32,
    value: u16,
});
layout!(OptionalDoubleV1 {
    present: i32,
    value: f64,
});
layout!(OptionalDiscriminatorV1 {
    present: i32,
    value: u32,
});
layout!(ContentV1 {
    kind: u32,
    data: StringV1,
});
layout!(OptionalContentV1 {
    present: i32,
    value: ContentV1,
});
layout!(RuleV1 {
    kind: u32,
    low: f64,
    high: f64,
});
layout!(OptionalRuleV1 {
    present: i32,
    value: RuleV1,
});
layout!(ChoiceV1 {
    name: StringV1,
    description: OptionalContentV1,
    weight: OptionalDoubleV1,
});
layout!(ChoicesV1 {
    data: *const ChoiceV1,
    len: usize,
});
layout!(RelationV1 {
    name: StringV1,
    source: StringV1,
    target: StringV1,
    reads: OptionalStringV1,
    either: i32,
    single: i32,
});
layout!(RelationsV1 {
    data: *const RelationV1,
    len: usize,
});
layout!(MemberSpecV1 {
    name: StringV1,
    question: *const crate::current::QuestionHandle,
});
layout!(MemberSpecsV1 {
    data: *const MemberSpecV1,
    len: usize,
});
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
    kind_pointer: OptionalStringV1,
});
layout!(QuestionMemberV1 {
    name: StringV1,
    question: *const QuestionViewV1,
});
layout!(QuestionMembersV1 {
    data: *const QuestionMemberV1,
    len: usize,
});
layout!(QuestionViewV1 {
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
    members: QuestionMembersV1,
    kinds: ChoicesV1,
    relations: RelationsV1,
    name_pointer: OptionalStringV1,
    kind_pointer: OptionalStringV1,
});
layout!(OptionalQuestionV1 {
    present: i32,
    value: QuestionViewV1,
});
layout!(ImagesV1 {
    data: *const *const crate::current::ImageHandle,
    len: usize,
});
layout!(ImageViewV1 {
    media: u32,
    bytes: *const u8,
    bytes_len: usize,
    width: u32,
    height: u32,
    filename: OptionalStringV1,
});
layout!(ImageViewsV1 {
    data: *const ImageViewV1,
    len: usize,
});
layout!(OptionalImageViewsV1 {
    present: i32,
    value: ImageViewsV1,
});
layout!(RecordV1 {
    original: OptionalContentV1,
    context: OptionalContentV1,
    options: ChoicesV1,
    images: ImagesV1,
});
layout!(SourceSpecV1 {
    paths: StringsV1,
    unit: u32,
    window: usize,
});
layout!(ControlsV1 {
    deadline_ms: i64,
    cancel: *mut thinkthen::CancelToken,
    context: OptionalContentV1,
    batch: OptionalSizeV1,
    batch_max: i32,
    attempts: i32,
    surface: StringV1,
});
