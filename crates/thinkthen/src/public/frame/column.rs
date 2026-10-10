//! Reading a text column in place, and building answer columns as rows arrive.
//!
//! Builders project nullable rows from native Request results or pull batches.

use crate::public::{
    Annotated, Answer, CompleteAnnotated, Error, ErrorKind, Judgment, QuestionKind,
};
use polars::prelude::{
    BooleanChunked, BooleanChunkedBuilder, ChunkedBuilder, DataType, Float64Type, IntoSeries,
    ListBuilderTrait, ListStringChunkedBuilder, NewChunkedArray, PrimitiveChunkedBuilder, Series,
    StringChunked, StringChunkedBuilder, StructChunked,
};

/// The column's text cells, read in place. A null cell asks no question.
pub(crate) fn text(column: &Series) -> Result<&StringChunked, Error> {
    let refused = || {
        Error::usage(format!(
            "the column {} is {}, not text",
            column.name(),
            column.dtype()
        ))
    };
    if column.dtype() != &DataType::String {
        return Err(refused());
    }
    column.str().map_err(|_| refused())
}

/// The lowercase word for a question kind.
pub(crate) const fn kind_word(kind: QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Decide => "decide",
        QuestionKind::Choose => "choose",
        QuestionKind::Tag => "tag",
        QuestionKind::Score => "score",
        QuestionKind::Rank => "rank",
        QuestionKind::Find => "find",
    }
}

/// A decide answer as a nullable Boolean: not sure is null.
pub(crate) const fn decided(answer: Answer) -> Option<bool> {
    match answer {
        Answer::Yes => Some(true),
        Answer::No => Some(false),
        Answer::Unsure => None,
    }
}

/// One question's answers as a column of its kind, one row at a time.
pub(crate) struct Answers {
    name: String,
    column: Typed,
}

enum Typed {
    Decide(BooleanChunkedBuilder),
    Choose(StringChunkedBuilder),
    Score(PrimitiveChunkedBuilder<Float64Type>),
    Tag(ListStringChunkedBuilder),
}

impl Answers {
    pub(crate) fn new(name: &str, kind: QuestionKind, rows: usize) -> Result<Self, Error> {
        let named = name.into();
        let column = match kind {
            QuestionKind::Decide => Typed::Decide(BooleanChunkedBuilder::new(named, rows)),
            QuestionKind::Choose => Typed::Choose(StringChunkedBuilder::new(named, rows)),
            QuestionKind::Score => Typed::Score(PrimitiveChunkedBuilder::new(named, rows)),
            QuestionKind::Tag => Typed::Tag(ListStringChunkedBuilder::new(named, rows, rows)),
            QuestionKind::Rank | QuestionKind::Find => {
                return Err(Error::defect(&format!(
                    "the member {name} has no column kind"
                )));
            }
        };
        Ok(Self {
            name: name.to_owned(),
            column,
        })
    }

    /// Add one row: `None` for a null cell, a failed question is null too.
    pub(crate) fn push(&mut self, value: Option<&Annotated>) -> Result<(), Error> {
        let mismatch = || Error::defect(&format!("the member {} answered another kind", self.name));
        match (&mut self.column, value) {
            (Typed::Decide(column), None | Some(Annotated::Failed(_))) => column.append_null(),
            (Typed::Choose(column), None | Some(Annotated::Failed(_))) => column.append_null(),
            (Typed::Score(column), None | Some(Annotated::Failed(_))) => column.append_null(),
            (Typed::Tag(column), None | Some(Annotated::Failed(_))) => column.append_null(),
            (Typed::Decide(column), Some(Annotated::Decision(answer))) => {
                column.append_option(decided(*answer));
            }
            (Typed::Choose(column), Some(Annotated::Choice(label))) => {
                column.append_option(label.as_deref());
            }
            (Typed::Score(column), Some(Annotated::Score(position))) => {
                column.append_value(*position);
            }
            (Typed::Tag(column), Some(Annotated::Tags(labels))) => {
                column.append_values_iter(labels.iter().map(String::as_str));
            }
            _ => return Err(mismatch()),
        }
        Ok(())
    }

    /// Project a successful native judgment; absent and failed members leave null cells.
    pub(super) fn push_judgment(&mut self, value: Option<Judgment>) -> Result<(), Error> {
        let value = value.map(|value| match value {
            Judgment::Decision(value) => Annotated::Decision(value),
            Judgment::Choice(value) => Annotated::Choice(value),
            Judgment::Score(value) => Annotated::Score(value),
            Judgment::Tags(value) => Annotated::Tags(value),
        });
        self.push(value.as_ref())
    }

    pub(crate) fn finish(self) -> Series {
        match self.column {
            Typed::Decide(column) => column.finish().into_series(),
            Typed::Choose(column) => column.finish().into_series(),
            Typed::Score(column) => column.finish().into_series(),
            Typed::Tag(mut column) => column.finish().into_series(),
        }
    }
}

/// The fixed-field failure map, one row at a time. Explicit parent validity
/// distinguishes a row with no failures from a row whose other question
/// fields are null.
pub(crate) struct Failures {
    members: Vec<Member>,
    any: Vec<bool>,
}

struct Member {
    name: String,
    kinds: StringChunkedBuilder,
    causes: StringChunkedBuilder,
    present: Vec<bool>,
}

impl Failures {
    pub(crate) fn new(names: &[&str], rows: usize) -> Self {
        let members = names
            .iter()
            .map(|name| Member {
                name: (*name).to_owned(),
                kinds: StringChunkedBuilder::new("kind".into(), rows),
                causes: StringChunkedBuilder::new("cause".into(), rows),
                present: Vec::with_capacity(rows),
            })
            .collect();
        Self {
            members,
            any: Vec::with_capacity(rows),
        }
    }

    /// Preserve native failed members without reading a result JSON document.
    pub(super) fn push_complete(
        &mut self,
        record: Option<&CompleteAnnotated>,
    ) -> Result<(), Error> {
        let mut any = false;
        for (place, member) in self.members.iter_mut().enumerate() {
            let failed = record
                .map(|record| member.failure(record, place))
                .transpose()?
                .flatten();
            member.present.push(failed.is_some());
            any |= failed.is_some();
            member
                .kinds
                .append_option(failed.as_ref().map(|_| ErrorKind::Backend.name()));
            member.causes.append_option(failed);
        }
        self.any.push(any);
        Ok(())
    }

    pub(crate) fn finish(self) -> Result<Series, Error> {
        let fields = self
            .members
            .into_iter()
            .map(|member| {
                let marker = struct_with_validity(
                    "failed",
                    &[
                        member.kinds.finish().into_series(),
                        member.causes.finish().into_series(),
                    ],
                    &member.present,
                )?;
                struct_with_validity(&member.name, &[marker], &member.present)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        struct_with_validity("failed", &fields, &self.any)
    }
}

impl Member {
    fn failure(&self, record: &CompleteAnnotated, place: usize) -> Result<Option<String>, Error> {
        let (name, value) = record
            .canonical
            .members
            .get(place)
            .ok_or_else(|| Error::defect("an annotation lost its failure member"))?;
        if name != &self.name {
            return Err(Error::defect("an annotation changed its member order"));
        }
        let Some((_, failure, _)) = value.legacy.failed() else {
            return Ok(None);
        };
        // Serialize only the native enum's declared spelling for the String column.
        let cause = serde_json::to_value(crate::core::FailedValue::new(failure).cause())
            .map_err(|_| Error::defect("a native failure cause could not be projected"))?;
        cause
            .as_str()
            .map(|cause| Some(cause.to_owned()))
            .ok_or_else(|| Error::defect("a native failure cause has no spelling"))
    }
}

pub(super) fn annotated(
    columns: &mut [Answers],
    failures: &mut Failures,
    record: Option<&CompleteAnnotated>,
) -> Result<(), Error> {
    for (place, column) in columns.iter_mut().enumerate() {
        let value = record
            .map(|record| {
                record
                    .members()
                    .nth(place)
                    .ok_or_else(|| Error::defect("an annotation lost its named member"))
            })
            .transpose()?
            .and_then(|member| member.value());
        column.push_judgment(value)?;
    }
    failures.push_complete(record)
}

fn struct_with_validity(name: &str, fields: &[Series], present: &[bool]) -> Result<Series, Error> {
    let mask = BooleanChunked::from_slice("valid".into(), present);
    let bitmap = mask
        .downcast_iter()
        .next()
        .map(|array| array.values().clone());
    StructChunked::from_series(name.into(), present.len(), fields.iter())
        .map(|column| column.with_outer_validity(bitmap).into_series())
        .map_err(|error| Error::defect(&format!("the failed column could not be built: {error}")))
}
