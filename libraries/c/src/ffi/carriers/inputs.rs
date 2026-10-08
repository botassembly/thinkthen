//! Canonical reviewed C layouts; see include/thinkthen.h.
/// C descriptor or borrowed view `StringV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct StringV1 {
    /// C field `data`.
    pub data: *const std::ffi::c_char,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `StringsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct StringsV1 {
    /// C field `data`.
    pub data: *const StringV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalStringV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalStringV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: StringV1,
}
/// C descriptor or borrowed view `OptionalSizeV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalSizeV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: usize,
}
/// C descriptor or borrowed view `OptionalU64V1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalU64V1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: u64,
}
/// C descriptor or borrowed view `OptionalU16V1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalU16V1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: u16,
}
/// C descriptor or borrowed view `OptionalDoubleV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalDoubleV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: f64,
}
/// C descriptor or borrowed view `OptionalDiscriminatorV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalDiscriminatorV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: u32,
}
/// C descriptor or borrowed view `ContentV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ContentV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: StringV1,
}
/// C descriptor or borrowed view `OptionalContentV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalContentV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: ContentV1,
}
/// CUT uses low; BAND uses low/high; NULL/DEFAULT use neither.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RuleV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `low`.
    pub low: f64,
    /// C field `high`.
    pub high: f64,
}
/// C descriptor or borrowed view `OptionalRuleV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalRuleV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: RuleV1,
}
/// C descriptor or borrowed view `ChoiceV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ChoiceV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `description`.
    pub description: OptionalContentV1,
    /// C field `weight`.
    pub weight: OptionalDoubleV1,
}
/// C descriptor or borrowed view `ChoicesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ChoicesV1 {
    /// C field `data`.
    pub data: *const ChoiceV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `RelationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RelationV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `source`.
    pub source: StringV1,
    /// C field `target`.
    pub target: StringV1,
    /// C field `reads`.
    pub reads: OptionalStringV1,
    /// C field `either`.
    pub either: std::ffi::c_int,
    /// C field `single`.
    pub single: std::ffi::c_int,
}
/// C descriptor or borrowed view `RelationsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RelationsV1 {
    /// C field `data`.
    pub data: *const RelationV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `MemberSpecV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemberSpecV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `question`.
    pub question: *const crate::current::QuestionHandle,
}
/// C descriptor or borrowed view `MemberSpecsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemberSpecsV1 {
    /// C field `data`.
    pub data: *const MemberSpecV1,
    /// C field `len`.
    pub len: usize,
}
/// Additive 0.3 recognition task; existing V1 question layouts remain unchanged.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RecognitionTaskV1 {
    /// C field `instructions`.
    pub instructions: OptionalStringV1,
    /// C field `entity_definition`.
    pub entity_definition: OptionalStringV1,
}
/// C descriptor or borrowed view `QuestionSpecV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionSpecV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `text`.
    pub text: ContentV1,
    /// C field `yes`.
    pub yes: OptionalContentV1,
    /// C field `no`.
    pub no: OptionalContentV1,
    /// C field `choices`.
    pub choices: ChoicesV1,
    /// C field `threshold`.
    pub threshold: RuleV1,
    /// C field `relation_threshold`.
    pub relation_threshold: RuleV1,
    /// C field `model`.
    pub model: OptionalStringV1,
    /// C field `profile`.
    pub profile: OptionalStringV1,
    /// C field `batch`.
    pub batch: OptionalSizeV1,
    /// C field `batch_max`.
    pub batch_max: std::ffi::c_int,
    /// C field `none`.
    pub none: std::ffi::c_int,
    /// C field `on`.
    pub on: StringsV1,
    /// C field `members`.
    pub members: MemberSpecsV1,
    /// C field `kinds`.
    pub kinds: ChoicesV1,
    /// C field `relations`.
    pub relations: RelationsV1,
    /// C field `name_pointer`.
    pub name_pointer: OptionalStringV1,
    /// C field `kind_pointer`.
    pub kind_pointer: OptionalStringV1,
}
/// C descriptor or borrowed view `QuestionMemberV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionMemberV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `question`.
    pub question: *const QuestionViewV1,
}
/// C descriptor or borrowed view `QuestionMembersV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionMembersV1 {
    /// C field `data`.
    pub data: *const QuestionMemberV1,
    /// C field `len`.
    pub len: usize,
}
/// Result questions expose nested questions directly, with no engine handle.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionViewV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `text`.
    pub text: ContentV1,
    /// C field `yes`.
    pub yes: OptionalContentV1,
    /// C field `no`.
    pub no: OptionalContentV1,
    /// C field `choices`.
    pub choices: ChoicesV1,
    /// C field `threshold`.
    pub threshold: RuleV1,
    /// C field `relation_threshold`.
    pub relation_threshold: RuleV1,
    /// C field `model`.
    pub model: OptionalStringV1,
    /// C field `profile`.
    pub profile: OptionalStringV1,
    /// C field `batch`.
    pub batch: OptionalSizeV1,
    /// C field `batch_max`.
    pub batch_max: std::ffi::c_int,
    /// C field `none`.
    pub none: std::ffi::c_int,
    /// C field `on`.
    pub on: StringsV1,
    /// C field `members`.
    pub members: QuestionMembersV1,
    /// C field `kinds`.
    pub kinds: ChoicesV1,
    /// C field `relations`.
    pub relations: RelationsV1,
    /// C field `name_pointer`.
    pub name_pointer: OptionalStringV1,
    /// C field `kind_pointer`.
    pub kind_pointer: OptionalStringV1,
}
/// C descriptor or borrowed view `OptionalQuestionV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalQuestionV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: QuestionViewV1,
}
/// C descriptor or borrowed view `ImagesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ImagesV1 {
    /// C field `data`.
    pub data: *const *const crate::current::ImageHandle,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `ImageViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ImageViewV1 {
    /// C field `media`.
    pub media: u32,
    /// C field `bytes`.
    pub bytes: *const u8,
    /// C field `bytes_len`.
    pub bytes_len: usize,
    /// C field `width`.
    pub width: u32,
    /// C field `height`.
    pub height: u32,
    /// C field `filename`.
    pub filename: OptionalStringV1,
}
/// C descriptor or borrowed view `ImageViewsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ImageViewsV1 {
    /// C field `data`.
    pub data: *const ImageViewV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalImageViewsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalImageViewsV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: ImageViewsV1,
}
/// C descriptor or borrowed view `RecordV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RecordV1 {
    /// C field `original`.
    pub original: OptionalContentV1,
    /// C field `context`.
    pub context: OptionalContentV1,
    /// C field `options`.
    pub options: ChoicesV1,
    /// C field `images`.
    pub images: ImagesV1,
}
/// C descriptor or borrowed view `SourceSpecV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceSpecV1 {
    /// C field `paths`.
    pub paths: StringsV1,
    /// C field `unit`.
    pub unit: u32,
    /// C field `window`.
    pub window: usize,
}
/// C descriptor or borrowed view `ControlsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ControlsV1 {
    /// C field `deadline_ms`.
    pub deadline_ms: i64,
    /// C field `cancel`.
    pub cancel: *mut thinkthen::CancelToken,
    /// C field `context`.
    pub context: OptionalContentV1,
    /// C field `batch`.
    pub batch: OptionalSizeV1,
    /// C field `batch_max`.
    pub batch_max: std::ffi::c_int,
    /// C field `attempts`.
    pub attempts: std::ffi::c_int,
    /// C field `surface`.
    pub surface: StringV1,
}
