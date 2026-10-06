//! Full annotation, recognition and relation tables from native getters.
use super::{inputs::Original, observations, questions, rows};
use crate::current::Storage;
use crate::failures::Failure;
use crate::ffi::carriers::{
    AnnotateViewV1, ContentV1, EndpointV1, MemberDataV1, MemberFailureV1, MemberSuccessV1,
    MemberV1, MembersV1, NameV1, NamesV1, OptionalContentV1, OptionalEndpointV1,
    OptionalProbabilitiesV1, OptionalQuestionV1, PairV1, PairsV1, PieceV1, PiecesV1, PlaceV1,
    QuestionMemberV1, QuestionMembersV1, QuestionViewV1, RecognizeAnswerV1, RecognizeViewV1,
    RelateViewV1, RelationAnswerDataV1, RelationAnswerV1, RelationAnswersV1, RelationSuccessV1,
    RowObservationDataV1, RowObservationV1,
};
use thinkthen::{
    CompleteAnnotated, CompleteRecognized, CompleteRecord, CompleteRelated, OwnedRecordObservation,
};
pub(super) fn annotate(
    s: &mut Storage,
    row: &CompleteRecord<Original, CompleteAnnotated>,
    events: &[OwnedRecordObservation],
) -> Result<RowObservationV1, Failure> {
    let r = row.result();
    let mut common = s.original_row(Some(row.original()), r.meta());
    let mut questions = Vec::new();
    let members = r
        .members()
        .map(|m| {
            let question = s.resolved_question(m.question(), m.threshold())?;
            let (ptr, _) = s.array(vec![question]);
            questions.push(QuestionMemberV1 {
                name: s.string(m.name()),
                question: ptr,
            });
            let mut data = MemberDataV1::default();
            let state = if let Some(cause) = m.failure() {
                data.failure = MemberFailureV1 {
                    failure_id: s.string(
                        m.failure_id()
                            .ok_or_else(|| {
                                Failure::defect("native annotation failure lost its identity")
                            })?
                            .as_str(),
                    ),
                    cause: observations::cause(cause),
                };
                2
            } else {
                let probabilities = m.probabilities().ok_or_else(|| {
                    Failure::defect("native annotation success lost its probabilities")
                })?;
                let answer = s.answer(
                    rows::question_kind(m.question()),
                    &probabilities,
                    observations::raw(events, row.ordinal(), Some(m.name())).as_deref(),
                    m.confidence(),
                )?;
                data.success = MemberSuccessV1 {
                    answer_id: s.string(
                        m.answer_id()
                            .ok_or_else(|| {
                                Failure::defect("native annotation success lost its identity")
                            })?
                            .as_str(),
                    ),
                    value: rows::member_value(
                        s,
                        &m.value().ok_or_else(|| {
                            Failure::defect("native annotation success lost its value")
                        })?,
                        m.question(),
                    )?,
                    answer,
                    threshold: questions::rule(m.threshold()),
                };
                1
            };
            Ok(MemberV1 {
                name: s.string(m.name()),
                request: s.string(m.request()),
                question,
                state,
                data,
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let (data, len) = s.array(questions);
    common.question = OptionalQuestionV1 {
        present: 1,
        value: QuestionViewV1 {
            kind: 8,
            members: QuestionMembersV1 { data, len },
            ..QuestionViewV1::default()
        },
    };
    let (data, len) = s.array(members);
    s.row_details(
        common,
        r.meta(),
        std::iter::once(&row.original().native),
        None,
    )?;
    let mut output = RowObservationDataV1::default();
    output.annotate = AnnotateViewV1 {
        common,
        answers: MembersV1 { data, len },
    };
    Ok(RowObservationV1 {
        index: row.ordinal(),
        function: 8,
        data: output,
    })
}
pub(super) fn recognize(
    s: &mut Storage,
    row: &CompleteRecord<Original, CompleteRecognized>,
) -> Result<RowObservationV1, Failure> {
    let r = row.result();
    let mut common = s.original_row(Some(row.original()), r.meta());
    let q = r.question();
    common.question = OptionalQuestionV1 {
        present: 1,
        value: QuestionViewV1 {
            kind: 9,
            kinds: s.native_choices(q.kinds())?,
            relations: s.native_relations(q.relations()),
            threshold: questions::rule(Some(q.threshold())),
            relation_threshold: questions::rule(Some(q.relation_threshold())),
            profile: s.optional_string(q.profile()),
            ..QuestionViewV1::default()
        },
    };
    common.threshold = questions::threshold(Some(q.threshold()));
    let p = r.probabilities();
    let pieces = p
        .pieces()
        .map(|p| PieceV1 {
            start: p.range().start,
            end: p.range().end,
            tags: s.named_probabilities(&p.tags()),
        })
        .collect();
    let (data, len) = s.array(pieces);
    let pieces = PiecesV1 { data, len };
    let names = p
        .names()
        .map(|p| NameV1 {
            start: p.range().start,
            end: p.range().end,
            kinds: p
                .kinds()
                .map(|p| OptionalProbabilitiesV1 {
                    present: 1,
                    value: s.named_probabilities(&p),
                })
                .unwrap_or_default(),
            edges: p
                .edges()
                .map(|p| OptionalProbabilitiesV1 {
                    present: 1,
                    value: s.named_probabilities(&p),
                })
                .unwrap_or_default(),
        })
        .collect();
    let (data, len) = s.array(names);
    let names = NamesV1 { data, len };
    let pairs = p
        .pairs()
        .map(|p| PairV1 {
            relation: s.string(p.relation()),
            source: PlaceV1 {
                start: p.source().start,
                end: p.source().end,
            },
            target: PlaceV1 {
                start: p.target().start,
                end: p.target().end,
            },
            probability: p.probability(),
        })
        .collect();
    let (data, len) = s.array(pairs);
    let answer = RecognizeAnswerV1 {
        pieces,
        names,
        pairs: PairsV1 { data, len },
    };
    s.row_details(
        common,
        r.meta(),
        std::iter::once(&row.original().native),
        None,
    )?;
    let mut data = RowObservationDataV1::default();
    data.recognize = RecognizeViewV1 {
        common,
        value: s.recognition_value(r.value()),
        answer,
    };
    Ok(RowObservationV1 {
        index: row.ordinal(),
        function: 9,
        data,
    })
}
fn endpoint(s: &mut Storage, entity: &thinkthen::Entity) -> EndpointV1 {
    EndpointV1 {
        name: s.string(entity.name()),
        kind: s.string(entity.kind()),
    }
}
pub(super) fn relate(
    s: &mut Storage,
    row: &CompleteRecord<Vec<Original>, CompleteRelated>,
) -> Result<RowObservationV1, Failure> {
    let r = row.result();
    let mut common = s.original_row(None, r.meta());
    let originals = row
        .original()
        .iter()
        .map(|o| &o.retained)
        .collect::<Vec<_>>();
    let input = serde_json::to_string(&originals)
        .map_err(|_| Failure::defect("original relation records could not be retained"))?;
    common.input = OptionalContentV1 {
        present: 1,
        value: ContentV1 {
            kind: 2,
            data: s.string(&input),
        },
    };
    let q = r.question();
    common.question = OptionalQuestionV1 {
        present: 1,
        value: QuestionViewV1 {
            kind: 10,
            relations: s.native_relations(q.relations()),
            threshold: questions::rule(Some(q.threshold())),
            profile: s.optional_string(q.profile()),
            name_pointer: s.optional_string(q.fields().map(|(name, _)| name)),
            kind_pointer: s.optional_string(q.fields().map(|(_, kind)| kind)),
            ..QuestionViewV1::default()
        },
    };
    common.threshold = questions::threshold(Some(q.threshold()));
    let members = relation_members(s, r)?;
    let (data, len) = s.array(members);
    s.row_details(
        common,
        r.meta(),
        row.original().iter().map(|v| &v.native),
        None,
    )?;
    let mut output = RowObservationDataV1::default();
    output.relate = RelateViewV1 {
        common,
        value: s.edges(r.value()),
        questions: RelationAnswersV1 { data, len },
    };
    Ok(RowObservationV1 {
        index: 0,
        function: 10,
        data: output,
    })
}

fn relation_members(
    s: &mut Storage,
    r: &CompleteRelated,
) -> Result<Vec<RelationAnswerV1>, Failure> {
    r.members()
        .map(|m| {
            let mut data = RelationAnswerDataV1::default();
            let state = if let Some(cause) = m.failure() {
                data.failure = MemberFailureV1 {
                    failure_id: s.string(
                        m.failure_id()
                            .ok_or_else(|| {
                                Failure::defect("native relation failure lost its identity")
                            })?
                            .as_str(),
                    ),
                    cause: observations::cause(cause),
                };
                2
            } else {
                data.success = RelationSuccessV1 {
                    answer_id: s.string(
                        m.answer_id()
                            .ok_or_else(|| {
                                Failure::defect("native relation success lost its identity")
                            })?
                            .as_str(),
                    ),
                    probability: m.probability().ok_or_else(|| {
                        Failure::defect("native relation success lost its probability")
                    })?,
                    accepted: i32::from(m.accepted().ok_or_else(|| {
                        Failure::defect("native relation success lost its reading")
                    })?),
                };
                1
            };
            Ok(RelationAnswerV1 {
                relation: s.string(m.relation()),
                reads: s.string(m.reads()),
                method: match m.method() {
                    thinkthen::RelationMethod::YesNo => 1,
                    thinkthen::RelationMethod::Choice => 2,
                },
                direction: match m.direction() {
                    thinkthen::RelationDirection::SourceToTarget => 1,
                    thinkthen::RelationDirection::Either => 2,
                },
                source: endpoint(s, &m.source()),
                target: m
                    .target()
                    .map(|v| OptionalEndpointV1 {
                        present: 1,
                        value: endpoint(s, &v),
                    })
                    .unwrap_or_default(),
                request: s.string(m.request()),
                state,
                data,
            })
        })
        .collect()
}
