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
            const REQUIRED: usize = <$base>::REQUIRED;
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

pub(super) fn rows(receipt: &Receipt, kind: &str, columns: usize) -> Result<Vec<Cells>, Failure> {
    let rows = receipt
        .carrier
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| Failure::defect("a details receipt lost its rows"))?;
    let projected = receipt.projected(kind, columns)?;
    let metadata = [
        ("failure", receipt.projected("error", columns)?),
        ("facts", receipt.projected("facts", columns)?),
    ];
    let details = receipt.projected("question_detail", columns)?;
    let events = receipt
        .carrier
        .get("observations")
        .and_then(serde_json::Value::as_array);

    rows.iter()
        .map(|row| {
            let mut cells = row
                .get("result")
                .and_then(serde_json::Value::as_u64)
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| projected.get(index))
                .cloned()
                .unwrap_or_else(Cells::new);
            for (field, values) in &metadata {
                if let Some(held) = row
                    .get(field)
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|index| usize::try_from(index).ok())
                    .and_then(|index| values.get(index))
                {
                    cells.extend(held.clone());
                }
            }
            let detail = events
                .into_iter()
                .flatten()
                .filter(|event| {
                    event
                        .get("detail")
                        .is_some_and(serde_json::Value::is_object)
                })
                .position(|event| event.get("index") == row.get("ordinal"));
            if let Some(held) = detail.and_then(|index| details.get(index)) {
                cells.extend(held.clone());
            }
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
                cells.insert(
                    at,
                    crate::receipt::scalar(crate::receipt::Scalar::Data, row.get(field))?,
                );
            }
            Ok(cells)
        })
        .collect()
}
