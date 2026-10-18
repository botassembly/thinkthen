//! Typed convenience calls adopt the same settlement used by Request packets.
use super::settled::{AnswerResolution, AnswerResolutions, Settled, SettledAnswer};
use crate::{CompleteRecord, Error, InputEvidence, RequestValue};

pub(crate) trait SettledCarrier {
    fn settled_answer(&self) -> Result<SettledAnswer, Error>;
}
impl SettledCarrier for RequestValue {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        SettledAnswer::of(self).ok_or_else(|| Error::defect("a native call has no settled answer"))
    }
}
macro_rules! scalar {
    ($result:ident, $variant:ident, $project:path) => {
        impl SettledCarrier for Vec<crate::$result> {
            fn settled_answer(&self) -> Result<SettledAnswer, Error> {
                let mut answer = SettledAnswer::$variant(Settled::rows(self.iter().map($project)));
                answer.scalar();
                Ok(answer)
            }
        }
    };
}
scalar!(
    CompleteDecision,
    Decide,
    super::settled_rows::decision_answer
);
scalar!(CompleteChoice, Choose, super::settled_rows::choice_answer);
scalar!(CompleteTags, Tag, super::settled_rows::tag_answer);
scalar!(CompleteScore, Score, super::settled_rows::score_answer);

macro_rules! records {
    ($result:ident, $variant:ident) => {
        impl<T: InputEvidence> SettledCarrier for Vec<CompleteRecord<T, crate::$result>> {
            fn settled_answer(&self) -> Result<SettledAnswer, Error> {
                RequestValue::$variant(
                    self.iter()
                        .map(|row| CompleteRecord {
                            original: row.original().question_input(),
                            ordinal: row.ordinal(),
                            result: row.result().clone(),
                        })
                        .collect(),
                )
                .settled_answer()
            }
        }
    };
}
records!(CompleteFilter, Filtered);
records!(CompleteRank, Ranked);
records!(CompleteSetRank, SetRanked);
records!(CompleteAnnotated, Annotations);

impl<T: InputEvidence + Clone> SettledCarrier for crate::CompleteFound<T> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        RequestValue::Found(
            self.clone()
                .map_original(|original| original.question_input()),
        )
        .settled_answer()
    }
}
impl SettledCarrier for Vec<crate::CompleteRecognized> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        let mut answer = SettledAnswer::Recognize(Settled::rows(self.iter().map(|result| {
            Settled::one(
                super::SettledRecognition(result.clone()),
                result.probability(),
                None,
                None,
                AnswerResolutions::State(AnswerResolution::Resolved),
            )
        })));
        answer.scalar();
        Ok(answer)
    }
}
impl SettledCarrier for Vec<crate::CompleteRelated> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        let [result] = self.as_slice() else {
            return Err(Error::defect("a relation call has no single aggregate"));
        };
        Ok(SettledAnswer::Relate(Settled::one(
            super::SettledRelations(result.clone()),
            result.probability(),
            None,
            None,
            AnswerResolutions::State(AnswerResolution::Resolved),
        )))
    }
}
