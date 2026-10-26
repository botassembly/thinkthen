//! Copy canonical typed rows into the stable native output prefix.
use crate::{Buffer, HostRow};
use thinkthen::{
    Error, native,
    results::{AnyRow, Outcome},
};
impl HostRow {
    pub(crate) fn from_row(size: u32, row: AnyRow) -> Result<Self, Error> {
        let value = serde_json::to_value(&row).map_err(|_| native::internal_error())?;
        let mut out = Self::empty(size);
        out.function = row.function().abi();
        out.status = row.status().abi();
        out.position = row.position();
        out.probability = row
            .probability()
            .map_or(f64::NAN, thinkthen::Probability::get);
        out.error_code = row.error().map_or(0, |error| error.code().abi());
        macro_rules! scalar {
            ($row:ident,$target:ident,$convert:expr) => {
                if let Outcome::Answered { answer, .. } = $row.outcome() {
                    out.$target = ($convert)(answer);
                }
            };
        }
        match &row {
            AnyRow::Decide(row) => scalar!(row, answer_bool, |value: &bool| u8::from(*value)),
            AnyRow::Filter(row) => scalar!(row, answer_bool, |value: &bool| u8::from(*value)),
            AnyRow::Score(row) => scalar!(row, number, |value: &f64| *value),
            AnyRow::Rank(row) => scalar!(row, rank, |value: &std::num::NonZeroUsize| value.get()
                as u64),
            _ => {}
        }
        fn field(value: &serde_json::Value, name: &str) -> Buffer {
            value
                .get(name)
                .map(|value| Buffer::new(value.to_string()))
                .unwrap_or_default()
        }
        out.id = field(&value, "id");
        out.answer = field(&value, "answer");
        out.detail = field(&value, "detail");
        out.source = field(&value, "source");
        out.error = field(&value, "error");
        Ok(out)
    }
}
