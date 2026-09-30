//! Reading a text column in place, and building answer columns as rows arrive.
//!
//! Each builder takes one row at a time, so a call holds its output columns
//! and the pipeline's window, never a list of every row (ticket 0304 slice 3c).

use crate::public::{
    Annotated, AnnotatedRecord, Answer, Batch, Error, Facts, NamedAnnotation, QuestionKind,
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

/// Pull one call's rows in input order and hand each to `push` as it
/// arrives. A null cell gets `None` without a question. The call's first
/// failed row ends it with the engine's error.
pub(crate) fn streamed<T>(
    mut batch: Batch<'_, T>,
    cells: &StringChunked,
    mut push: impl FnMut(Option<T>) -> Result<(), Error>,
) -> Result<Facts, Error> {
    for cell in cells.iter() {
        let row = match cell {
            None => None,
            Some(_) => Some(
                batch
                    .next()
                    .ok_or_else(|| Error::defect("a frame answer lost a non-null input row"))??,
            ),
        };
        push(row)?;
    }
    if let Some(extra) = batch.next() {
        extra?;
        return Err(Error::defect(
            "a frame call answered more rows than it read",
        ));
    }
    batch
        .facts()
        .cloned()
        .ok_or_else(|| Error::defect("a completed Polars call has no final facts"))
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

    /// Add one annotated record's member at `place`, or a null cell's null.
    pub(crate) fn push_record(
        &mut self,
        record: Option<&AnnotatedRecord<&str>>,
        place: usize,
    ) -> Result<(), Error> {
        let value = record
            .map(|record| {
                record
                    .values()
                    .get(place)
                    .map(NamedAnnotation::value)
                    .ok_or_else(|| Error::defect(&format!("a record has no member {}", self.name)))
            })
            .transpose()?;
        self.push(value)
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

    /// Add one record's failures, or a null cell's null.
    pub(crate) fn push(&mut self, record: Option<&AnnotatedRecord<&str>>) -> Result<(), Error> {
        let mut any = false;
        for (place, member) in self.members.iter_mut().enumerate() {
            let failed = record
                .map(|record| member.failure(record, place))
                .transpose()?
                .flatten();
            member.present.push(failed.is_some());
            any |= failed.is_some();
            let (kind, cause) = failed.unzip();
            member.kinds.append_option(kind);
            member.causes.append_option(cause);
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
    /// The member's failure marker in this record, if its question failed.
    fn failure(
        &self,
        record: &AnnotatedRecord<&str>,
        place: usize,
    ) -> Result<Option<(String, String)>, Error> {
        let value = record
            .values()
            .get(place)
            .ok_or_else(|| Error::defect(&format!("a record has no member {}", self.name)))?;
        matches!(value.value(), Annotated::Failed(_))
            .then(|| marker(record, &self.name))
            .transpose()
    }
}

fn marker(record: &AnnotatedRecord<&str>, name: &str) -> Result<(String, String), Error> {
    let json: serde_json::Value = serde_json::from_str(&record.value_json())
        .map_err(|error| Error::defect(&format!("the record's JSON did not parse: {error}")))?;
    let failed = json
        .get(name)
        .and_then(|value| value.get("failed"))
        .ok_or_else(|| Error::defect(&format!("the record has no failure marker for {name}")))?;
    let field = |key| {
        failed
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| Error::defect(&format!("the failure marker has no {key}")))
    };
    Ok((field("kind")?, field("cause")?))
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
