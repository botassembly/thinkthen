//! Canonical reviewed C layouts; see include/thinkthen.h.
use super::{
    ContentV1, OptionalDoubleV1, OptionalSizeV1, OptionalStringV1, QuestionViewV1, RuleV1,
    StringV1, StringsV1,
};
union_layout!(DecideValueDataV1 {
    boolean: i32,
    authored: ContentV1,
});
layout!(DecideValueV1 {
    kind: u32,
    data: DecideValueDataV1,
});
layout!(ProbabilityV1 {
    name: StringV1,
    probability: f64,
});
layout!(ProbabilitiesV1 {
    data: *const ProbabilityV1,
    len: usize,
});
layout!(OptionalProbabilitiesV1 {
    present: i32,
    value: ProbabilitiesV1,
});
layout!(NamedAnswerV1 {
    pick: StringV1,
    probabilities: ProbabilitiesV1,
    confidence: OptionalDoubleV1,
});
layout!(ScoreAnswerV1 {
    level: StringV1,
    probabilities: ProbabilitiesV1,
    confidence: OptionalDoubleV1,
});
union_layout!(AnswerDataV1 {
    probability: f64,
    choice: NamedAnswerV1,
    tag: ProbabilitiesV1,
    score: ScoreAnswerV1,
    find: NamedAnswerV1,
});
layout!(AnswerV1 {
    kind: u32,
    data: AnswerDataV1,
});
layout!(OptionalAnswerV1 {
    present: i32,
    value: AnswerV1,
});
layout!(LocationV1 {
    file: OptionalStringV1,
    first_line: OptionalSizeV1,
    last_line: OptionalSizeV1,
});
layout!(OptionalLocationV1 {
    present: i32,
    value: LocationV1,
});
union_layout!(MemberValueDataV1 {
    decide: DecideValueV1,
    choose: OptionalStringV1,
    tag: StringsV1,
    score: f64,
});
layout!(MemberValueV1 {
    kind: u32,
    data: MemberValueDataV1,
});
layout!(MemberFailureV1 {
    failure_id: StringV1,
    cause: u32,
});
layout!(MemberSuccessV1 {
    answer_id: StringV1,
    value: MemberValueV1,
    answer: AnswerV1,
    threshold: RuleV1,
});
union_layout!(MemberDataV1 {
    success: MemberSuccessV1,
    failure: MemberFailureV1,
});
layout!(MemberV1 {
    name: StringV1,
    request: StringV1,
    question: QuestionViewV1,
    state: u32,
    data: MemberDataV1,
});
layout!(MembersV1 {
    data: *const MemberV1,
    len: usize,
});
layout!(EntityV1 {
    text: StringV1,
    start: usize,
    end: usize,
    length: usize,
    kind: StringV1,
    strength: f64,
});
layout!(EntitiesV1 {
    data: *const EntityV1,
    len: usize,
});
layout!(EntityEdgeV1 {
    relation: StringV1,
    source: EntityV1,
    target: EntityV1,
    probability: f64,
    either: i32,
});
layout!(EntityEdgesV1 {
    data: *const EntityEdgeV1,
    len: usize,
});
layout!(OptionalEntityEdgesV1 {
    present: i32,
    value: EntityEdgesV1,
});
layout!(PlaceV1 {
    start: usize,
    end: usize,
});
layout!(PieceV1 {
    start: usize,
    end: usize,
    tags: ProbabilitiesV1,
});
layout!(PiecesV1 {
    data: *const PieceV1,
    len: usize,
});
layout!(NameV1 {
    start: usize,
    end: usize,
    kinds: OptionalProbabilitiesV1,
    edges: OptionalProbabilitiesV1,
});
layout!(NamesV1 {
    data: *const NameV1,
    len: usize,
});
layout!(PairV1 {
    relation: StringV1,
    source: PlaceV1,
    target: PlaceV1,
    probability: f64,
});
layout!(PairsV1 {
    data: *const PairV1,
    len: usize,
});
layout!(RecognizeValueV1 {
    entities: EntitiesV1,
    relations: OptionalEntityEdgesV1,
});
layout!(RecognizeAnswerV1 {
    pieces: PiecesV1,
    names: NamesV1,
    pairs: PairsV1,
});
layout!(EndpointV1 {
    name: StringV1,
    kind: StringV1,
});
layout!(OptionalEndpointV1 {
    present: i32,
    value: EndpointV1,
});
layout!(EdgeV1 {
    relation: StringV1,
    source: EndpointV1,
    target: EndpointV1,
    probability: f64,
    either: i32,
});
layout!(EdgesV1 {
    data: *const EdgeV1,
    len: usize,
});
layout!(RelationSuccessV1 {
    answer_id: StringV1,
    probability: f64,
    accepted: i32,
});
union_layout!(RelationAnswerDataV1 {
    success: RelationSuccessV1,
    failure: MemberFailureV1,
});
layout!(RelationAnswerV1 {
    relation: StringV1,
    reads: StringV1,
    method: u32,
    direction: u32,
    source: EndpointV1,
    target: OptionalEndpointV1,
    request: StringV1,
    state: u32,
    data: RelationAnswerDataV1,
});
layout!(RelationAnswersV1 {
    data: *const RelationAnswerV1,
    len: usize,
});
