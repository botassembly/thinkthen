//! Typed convenience calls adopt the same settlement used by Request packets.
use super::settled::{AnswerResolution, AnswerResolutions, Settled, SettledAnswer};
use crate::{CompleteRecord, Error, RequestValue};

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

impl<T> SettledCarrier for Vec<CompleteRecord<T, crate::CompleteFilter>> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        let selected = self.iter().filter(|row| row.result().value()).map(|row| {
            let input = row.result().input.as_ref().ok_or_else(|| Error::defect("a filter result lost its original snapshot"))?;
            Ok((super::SettledOriginal(input.clone()), row.result().probability()))
        }).collect::<Result<Vec<_>, Error>>()?;
        Ok(SettledAnswer::Filter(super::settled_rows::filter_answer(selected.into_iter())))
    }
}
macro_rules! ranked {
    ($result:ident) => {
        impl<T> SettledCarrier for Vec<CompleteRecord<T, crate::$result>> {
            fn settled_answer(&self) -> Result<SettledAnswer, Error> {
                Ok(SettledAnswer::Rank(super::settled_aggregates::rank_answer(self.iter().map(|row| (row.ordinal(), row.result().probability())))))
            }
        }
    };
}
ranked!(CompleteRank);
ranked!(CompleteSetRank);
impl<T> SettledCarrier for Vec<CompleteRecord<T, crate::CompleteAnnotated>> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        Ok(SettledAnswer::Annotate(Settled::rows(self.iter().map(|row| super::settled_rows::annotation(row.result())))))
    }
}
impl<T> SettledCarrier for crate::CompleteFound<T> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        Ok(SettledAnswer::Find(super::settled_aggregates::find_answer(self)))
    }
}
impl SettledCarrier for Vec<crate::CompleteRecognized> {
    fn settled_answer(&self) -> Result<SettledAnswer, Error> {
        let mut answer = SettledAnswer::Recognize(Settled::rows(self.iter().map(|result| {
            Settled::one(
                super::SettledRecognition(Box::new(result.clone())),
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
            super::SettledRelations(Box::new(result.clone())),
            result.probability(),
            None,
            None,
            AnswerResolutions::State(AnswerResolution::Resolved),
        )))
    }
}
