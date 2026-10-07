//! Concrete row views use native readings, raw selections and original occurrences.
use super::{inputs::Original, metadata, questions};
use crate::current::author::native_author;
use crate::current::{Content, Storage};
use crate::failures::Failure;
use crate::ffi::carriers::{
    AnswerDataV1, AnswerV1, ChooseViewV1, DecideViewV1, FilterViewV1, FindViewV1, MemberValueV1,
    NamedAnswerV1, OptionalAnswerV1, OptionalQuestionV1, OptionalStringV1, QuestionViewV1,
    RankViewV1, RowObservationDataV1, RowObservationV1, RowV1, ScoreAnswerV1, ScoreViewV1,
    TagViewV1,
};
use thinkthen::{
    Answer, Judgment, Probabilities, ResolvedQuestion, ResolvedThreshold, ResultMetadata,
};
impl Storage {
    pub(super) fn meaning(
        &mut self,
        answer: Answer,
        question: ResolvedQuestion<'_>,
    ) -> Result<Option<Content>, Failure> {
        let authored = match answer {
            Answer::Yes => question.yes(),
            Answer::No => question.no(),
            Answer::Unsure => None,
        };
        authored
            .map(|a| match a.text() {
                Some(text) => Ok(Content::Text(text.into())),
                None => Ok(Content::Json(serde_json::from_str(&a.to_json()?).map_err(
                    |_| Failure::defect("native authored content could not be retained"),
                )?)),
            })
            .transpose()
    }
    pub(super) fn answer(
        &mut self,
        kind: u32,
        probabilities: &Probabilities,
        raw: Option<&str>,
        confidence: Option<f64>,
    ) -> Result<AnswerV1, Failure> {
        let mut data = AnswerDataV1::default();
        match (kind, probabilities) {
            (1, Probabilities::YesNo { yes }) => data.probability = *yes,
            (2 | 7, Probabilities::Named(named)) => {
                let answer = NamedAnswerV1 {
                    pick: self.string(raw.ok_or_else(|| {
                        Failure::defect("native named answer lost its raw selection")
                    })?),
                    probabilities: self.named_probabilities(named),
                    confidence: metadata::double(confidence),
                };
                match kind {
                    2 => data.choice = answer,
                    _ => data.find = answer,
                }
            }
            (4, Probabilities::Named(named)) => {
                data.score = ScoreAnswerV1 {
                    level: self.string(raw.ok_or_else(|| {
                        Failure::defect("native score answer lost its raw selection")
                    })?),
                    probabilities: self.named_probabilities(named),
                    confidence: metadata::double(confidence),
                };
            }
            (3, Probabilities::Named(named)) => data.tag = self.named_probabilities(named),
            _ => {
                return Err(Failure::defect(
                    "native answer kind differs from its probability table",
                ));
            }
        }
        Ok(AnswerV1 { kind, data })
    }
    pub(super) fn original_row(
        &mut self,
        original: Option<&Original>,
        meta: ResultMetadata<'_>,
    ) -> RowV1 {
        let input = original.map(|v| &v.retained);
        RowV1 {
            answer_id: self.string(meta.identity().answer_id().as_str()),
            input: self.optional_content(input.and_then(|v| v.original.as_ref())),
            position: self.position(input.and_then(|v| v.position.as_ref())),
            input_file: self
                .optional_string(input.and_then(|v| v.position.as_ref().map(|p| p.file.as_str()))),
            images: input.map(|v| self.images(&v.images)).unwrap_or_default(),
            meta: self.metadata(meta),
            ..RowV1::default()
        }
    }
    pub(super) fn atomic_row(
        &mut self,
        original: &Original,
        meta: ResultMetadata<'_>,
        question: ResolvedQuestion<'_>,
        cut: Option<ResolvedThreshold>,
        answer: AnswerV1,
    ) -> Result<RowV1, Failure> {
        let mut row = self.original_row(Some(original), meta);
        row.question = self.primitive(question, cut)?;
        row.threshold = questions::threshold(cut);
        row.answer = OptionalAnswerV1 {
            present: 1,
            value: answer,
        };
        Ok(row)
    }
}
macro_rules! atomic {
    ($fn:ident, $ty:ident, $view:ident, $arm:ident, $kind:literal, $value:expr, $raw:expr) => {
        pub(super) fn $fn(
            s: &mut Storage,
            row: &thinkthen::CompleteRecord<Original, thinkthen::$ty>,
        ) -> Result<RowObservationV1, Failure> {
            let r = row.result();
            let question = r.question();
            s.row_author(&native_author!(question), Vec::new());
            let answer = s.answer(
                question_kind(question),
                &r.probabilities(),
                $raw(r).as_deref(),
                r.confidence(),
            )?;
            let common = s.atomic_row(row.original(), r.meta(), question, r.threshold(), answer)?;
            let value = $value(s, r)?;
            s.row_details(
                common,
                r.meta(),
                std::iter::once(&row.original().native),
                $raw(r).as_deref(),
            )?;
            let mut data = RowObservationDataV1::default();
            data.$arm = $view { common, value };
            Ok(RowObservationV1 {
                index: row.ordinal(),
                function: $kind,
                data,
            })
        }
    };
}
pub(super) fn question_kind(q: ResolvedQuestion<'_>) -> u32 {
    match q.kind() {
        thinkthen::QuestionKind::Decide => 1,
        thinkthen::QuestionKind::Choose => 2,
        thinkthen::QuestionKind::Tag => 3,
        thinkthen::QuestionKind::Score => 4,
        thinkthen::QuestionKind::Rank => 6,
        thinkthen::QuestionKind::Find => 7,
    }
}
atomic!(
    decide,
    CompleteDecision,
    DecideViewV1,
    decide,
    1,
    |s: &mut Storage, r: &thinkthen::CompleteDecision| -> Result<_, Failure> {
        let meaning = s.meaning(r.value(), r.question())?;
        Ok(s.decision(r.value(), meaning.as_ref()))
    },
    |_: &thinkthen::CompleteDecision| None::<String>
);
atomic!(
    choose,
    CompleteChoice,
    ChooseViewV1,
    choose,
    2,
    |s: &mut Storage, r: &thinkthen::CompleteChoice| -> Result<_, Failure> {
        Ok(s.optional_string(r.value()))
    },
    |r: &thinkthen::CompleteChoice| r.raw_pick().map(str::to_owned)
);
atomic!(
    tag,
    CompleteTags,
    TagViewV1,
    tag,
    3,
    |s: &mut Storage, r: &thinkthen::CompleteTags| -> Result<_, Failure> {
        Ok(s.strings(r.value()))
    },
    |_: &thinkthen::CompleteTags| None::<String>
);
atomic!(
    score,
    CompleteScore,
    ScoreViewV1,
    score,
    4,
    |_: &mut Storage, r: &thinkthen::CompleteScore| -> Result<_, Failure> { Ok(r.value()) },
    |r: &thinkthen::CompleteScore| r.raw_level().map(str::to_owned)
);
atomic!(
    filter,
    CompleteFilter,
    FilterViewV1,
    filter,
    5,
    |_: &mut Storage, r: &thinkthen::CompleteFilter| -> Result<_, Failure> {
        Ok(i32::from(r.value()))
    },
    |_: &thinkthen::CompleteFilter| None::<String>
);
pub(super) fn rank(
    s: &mut Storage,
    row: &thinkthen::CompleteRecord<Original, thinkthen::CompleteRank>,
    raw: Option<&str>,
) -> Result<RowObservationV1, Failure> {
    let r = row.result();
    s.row_author(&native_author!(r.question()), Vec::new());
    let answer = s.answer(
        question_kind(r.question()),
        &r.probabilities(),
        raw,
        r.confidence(),
    )?;
    let common = s.atomic_row(
        row.original(),
        r.meta(),
        r.question(),
        r.threshold(),
        answer,
    )?;
    s.row_details(
        common,
        r.meta(),
        std::iter::once(&row.original().native),
        raw,
    )?;
    let mut data = RowObservationDataV1::default();
    data.rank = RankViewV1 {
        common,
        value: metadata::size(Some(r.value())),
        question_name: OptionalStringV1::default(),
    };
    Ok(RowObservationV1 {
        index: row.ordinal(),
        function: 6,
        data,
    })
}
pub(super) fn find(
    s: &mut Storage,
    r: &thinkthen::CompleteFound<Original>,
    events: &[thinkthen::OwnedRecordObservation],
) -> Result<RowObservationV1, Failure> {
    s.row_author(&native_author!(r.question()), Vec::new());
    let mut common = s.original_row(None, r.meta());
    common.question = OptionalQuestionV1 {
        present: 1,
        value: QuestionViewV1 {
            kind: 7,
            text: s.native_content(r.question().text())?,
            none: i32::from(r.question().offers_none()),
            profile: s.optional_string(r.question().profile()),
            model: s.optional_string(r.question().model()),
            on: s.native_on(r.question().on()),
            ..QuestionViewV1::default()
        },
    };
    // Native owned observations retain the actual request labels, including none.
    let probabilities = events
        .iter()
        .find_map(|event| match event {
            thinkthen::OwnedRecordObservation::Question { detail, .. } => {
                detail.detail().probabilities().cloned()
            }
            _ => None,
        })
        .ok_or_else(|| Failure::defect("native find observation lost its candidate labels"))?;
    let Probabilities::Named(named) = probabilities else {
        return Err(Failure::defect("native find candidate table is not named"));
    };
    let probabilities = s.named_probabilities(&named);
    let mut answer = AnswerDataV1::default();
    answer.find = NamedAnswerV1 {
        pick: s.string(r.raw_pick()),
        probabilities,
        confidence: metadata::double(r.confidence()),
    };
    common.answer = OptionalAnswerV1 {
        present: 1,
        value: AnswerV1 {
            kind: 5,
            data: answer,
        },
    };
    common.threshold = questions::threshold(None);
    let value = s.optional_content(r.selected().and_then(|v| v.retained.original.as_ref()));
    let index = metadata::size(r.selected().map(|v| v.retained.index));
    s.row_details(
        common,
        r.meta(),
        r.candidates()
            .iter()
            .filter_map(|c| c.input().map(|v| &v.native)),
        Some(r.raw_pick()),
    )?;
    let mut data = RowObservationDataV1::default();
    data.find = FindViewV1 {
        common,
        value,
        index,
    };
    Ok(RowObservationV1 {
        index: 0,
        function: 7,
        data,
    })
}
pub(super) fn member_value(
    s: &mut Storage,
    value: &Judgment,
    q: ResolvedQuestion<'_>,
) -> Result<MemberValueV1, Failure> {
    let meaning = match value {
        Judgment::Decision(answer) => s.meaning(*answer, q)?,
        _ => None,
    };
    Ok(s.member_value(value, meaning.as_ref()))
}
