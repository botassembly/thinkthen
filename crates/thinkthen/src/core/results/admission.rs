//! Row decoding preserves absent versus explicit-null answers and checks status fields.
use super::*;
#[derive(Debug, Default)]
enum Field<T> {
    #[default]
    Missing,
    Present(T),
}
impl<T> Field<T> {
    fn optional(self) -> Option<T> {
        match self {
            Self::Missing => None,
            Self::Present(value) => Some(value),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Field<T> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        T::deserialize(d).map(Self::Present)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = ""))]
pub(super) struct RowWire<F: Function> {
    #[serde(default)]
    function: Option<FunctionName>,
    position: NonZeroU64,
    id: Option<RecordId>,
    source: Source,
    status: Status,
    #[serde(default)]
    answer: Field<F::Answer>,
    #[serde(default)]
    probability: Field<Probability>,
    #[serde(default)]
    detail: Field<F::Detail>,
    #[serde(default)]
    error: Field<Error>,
}
impl<F: Function> RowWire<F> {
    pub(super) fn admit(self, required_function: bool) -> Result<Row<F>, &'static str> {
        if self.function.is_some_and(|name| name != F::NAME)
            || (required_function && self.function.is_none())
        {
            return Err("the row function does not match its type");
        }
        let outcome = match (
            self.status,
            self.answer,
            self.probability,
            self.detail,
            self.error,
        ) {
            (Status::Answered, Field::Present(answer), probability, detail, Field::Missing) => {
                Outcome::Answered {
                    answer,
                    probability: probability.optional(),
                    detail: detail.optional(),
                }
            }
            (
                Status::Unsure,
                Field::Missing,
                Field::Present(probability),
                detail,
                Field::Missing,
            ) => Outcome::Unsure {
                probability,
                detail: detail.optional(),
            },
            (
                Status::Failed,
                Field::Missing,
                Field::Missing,
                Field::Missing,
                Field::Present(error),
            ) if error.code().is_record() => Outcome::Failed { error },
            _ => return Err("the row fields do not match its status"),
        };
        Ok(Row::new(self.position, self.id, self.source, outcome))
    }
}
impl<'de, F: Function> Deserialize<'de> for Row<F> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        RowWire::<F>::deserialize(d)?
            .admit(true)
            .map_err(serde::de::Error::custom)
    }
}
