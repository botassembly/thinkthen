//! Full records retain typed child projections beside each ordinary value.
use super::{Verb, hidden};
use crate::Failure;
use crate::receipt::{Cells, Receipt};
use crate::tables::Table;
use rusqlite::ffi::sqlite3;
use rusqlite::types::Value;
use std::borrow::Cow;
use std::ffi::{CStr, CString};

macro_rules! details {
    ($name:ident, $base:ty, $sql:literal) => {
        #[derive(Debug)]
        pub(crate) struct $name;
        impl Table for $name {
            const NAME: &'static str = $sql;
            const SCHEMA: &'static CStr = c"CREATE TABLE x()";
            const FIRST_HIDDEN: usize = 7;
            const COLUMNS: usize = 7 + <$base>::COLUMNS - <$base>::FIRST_HIDDEN;
            const REQUIRED: usize = 0;
            fn schema() -> Result<Cow<'static, CStr>, Failure> {
                let fields = hidden::<$base>()?.into_iter()
                    .map(|name| format!("{name} HIDDEN"))
                    .collect::<Vec<_>>().join(",");
                Ok(Cow::Owned(CString::new(format!(
                    "CREATE TABLE x(ordinal INTEGER,value,probability,result INTEGER,failure INTEGER,facts INTEGER,observations INTEGER,{fields})"
                )).map_err(|_| Failure::defect("a details schema contains a zero byte"))?))
            }
            fn rows(_: *mut sqlite3, _: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
                Err(Failure::defect("a details table lost its native scan"))
            }
        }
        impl Verb for $name {
            const VERB: &'static str = <$base as Verb>::VERB;
            const DETAILS: bool = true;
            const FRESH_ANSWER: bool = true;
        }
    };
}
details!(Decide, super::Decide, "thinkthen_decide_details");
details!(Choose, super::Choose, "thinkthen_choose_details");
details!(Tag, super::Tag, "thinkthen_tag_details");
details!(Score, super::Score, "thinkthen_score_details");
details!(Find, super::Find, "thinkthen_find_details");
details!(Annotate, super::Annotate, "thinkthen_annotate_details");
details!(
    Filter,
    crate::source_filter::Filter,
    "thinkthen_filter_details"
);
details!(Rank, crate::many::Rank, "thinkthen_rank_details");
details!(
    Recognize,
    crate::tables::Recognizer,
    "thinkthen_recognize_details"
);
details!(Relate, crate::tables::Relater, "thinkthen_relate_details");

pub(super) fn rows(receipt: &Receipt) -> Result<Vec<Cells>, Failure> {
    let rows = receipt
        .carrier
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| Failure::defect("a details receipt lost its rows"))?;
    rows.iter()
        .map(|row| {
            let mut cells = Cells::new();
            for (at, field) in [
                "ordinal",
                "value",
                "probability",
                "result",
                "failure",
                "facts",
                "observations",
            ]
            .into_iter()
            .enumerate()
            {
                let value = row.get(field).unwrap_or(&serde_json::Value::Null);
                cells.insert(
                    at,
                    match value {
                        serde_json::Value::Null => Value::Null,
                        serde_json::Value::Bool(value) => Value::Integer(i64::from(*value)),
                        serde_json::Value::String(value) => Value::Text(value.clone()),
                        serde_json::Value::Number(value) => value
                            .as_i64()
                            .map(Value::Integer)
                            .or_else(|| value.as_f64().map(Value::Real))
                            .unwrap_or(Value::Null),
                        _ => Value::Text(value.to_string()),
                    },
                );
            }
            Ok(cells)
        })
        .collect()
}
