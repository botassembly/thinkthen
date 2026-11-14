//! Rows own one typed result and only the views C needs to borrow from it.
#![allow(
    unsafe_code,
    reason = "rows own an opaque allocation behind their supported prefix"
)]
use crate::{
    HostRow,
    values::{NameView, Owned, RelationView, ValueHandle},
};
use std::{collections::BTreeMap, ffi::CString, sync::Arc};
use thinkthen::{Error, Question, native, results::AnyRow};

#[derive(Debug, Default)]
pub(crate) struct Domain {
    pub(crate) words: Vec<String>,
    pub(crate) field: thinkthen::Pointer,
    pub(crate) spatial: bool,
    pub(crate) entries: BTreeMap<String, Arc<Domain>>,
}
impl Domain {
    fn from_entry(
        question: &thinkthen::questions::AnnotatableQuestion,
        fallback: &thinkthen::Pointer,
    ) -> Self {
        let field = question.field_override().unwrap_or(fallback).clone();
        use thinkthen::questions::AnnotatableQuestion as Q;
        let question: Question = match question {
            Q::Decide(q) => q.clone().into(),
            Q::Choose(q) => q.clone().into(),
            Q::Tag(q) => q.clone().into(),
            Q::Score(q) => q.clone().into(),
            Q::Find(q) => q.clone().into(),
            Q::Recognize(q) => q.clone().into(),
            Q::Relate(q) => q.clone().into(),
            _ => return Self::default(),
        };
        let mut domain = Self::from_question(&question);
        domain.field = field;
        domain
    }
    pub(crate) fn from_question(question: &Question) -> Self {
        let entries = if let Question::Annotate(q) = question {
            q.questions()
                .iter()
                .map(|(name, q)| {
                    (
                        name.clone(),
                        Arc::new(Self::from_entry(q, question.field_pointer())),
                    )
                })
                .collect()
        } else {
            BTreeMap::new()
        };
        Self {
            words: native::question_words(question),
            field: question.field_pointer().clone(),
            spatial: matches!(
                question.function(),
                thinkthen::FunctionName::Find
                    | thinkthen::FunctionName::Recognize
                    | thinkthen::FunctionName::Relate
            ),
            entries,
        }
    }
}
/// Private row allocation. Its fields and ownership are not part of the C interface.
#[derive(Debug)]
pub struct RowData {
    pub(crate) value: ValueHandle,
    pub(crate) selected: BTreeMap<String, crate::selected::SelectedText>,
    pub(crate) domain: Arc<Domain>,
    pub(crate) display: CString,
    pub(crate) names: Vec<NameView>,
    pub(crate) relations: Vec<RelationView>,
    pub(crate) labels: Vec<crate::values::ProbabilityView>,
    pub(crate) levels: Vec<f64>,
    pub(crate) pieces: Vec<crate::PieceDetailView>,
    pub(crate) kinds: Vec<crate::KindDetailView>,
    piece_tags: Vec<Vec<crate::values::ProbabilityView>>,
    kind_probabilities: Vec<Vec<crate::values::ProbabilityView>>,
    pub(crate) pairs: Vec<RelationView>,
    pub(crate) id_override: Option<Box<serde_json::value::RawValue>>,
}
impl RowData {
    pub(crate) fn new(value: ValueHandle, domain: Arc<Domain>) -> Result<Self, Error> {
        let mut result = Self {
            value,
            domain,
            selected: BTreeMap::new(),
            display: CString::default(),
            names: Vec::new(),
            relations: Vec::new(),
            id_override: None,
            labels: Vec::new(),
            levels: Vec::new(),
            pieces: Vec::new(),
            kinds: Vec::new(),
            piece_tags: Vec::new(),
            kind_probabilities: Vec::new(),
            pairs: Vec::new(),
        };
        #[cfg(test)]
        crate::memory::LIVE.with(|live| live.set(live.get() + 1));
        let display = match &result.value.0 {
            Owned::Row(row) => {
                macro_rules! display {($($variant:ident),*)=>{match row {$(AnyRow::$variant(row)=>row.to_string(),)* _=>return Err(native::abi_error())}};}
                display!(
                    Decide, Choose, Tag, Score, Filter, Rank, Find, Recognize, Relate, Annotate
                )
            }
            Owned::Entry(entry) => entry.to_string(),
        };
        // C strings cannot carry NUL. The display projection escapes it; JSON and answers stay exact.
        result.display =
            CString::new(display.replace('\0', "\\0")).map_err(|_| native::internal_error())?;
        use thinkthen::{FunctionName, questions};
        match result.value.function()? {
            FunctionName::Recognize if result.value.status()? == thinkthen::Status::Answered => {
                result.names = result
                    .value
                    .answer::<questions::Recognize>()?
                    .iter()
                    .map(crate::values::views::name)
                    .collect();
            }
            FunctionName::Relate => {
                if result.value.status()? == thinkthen::Status::Answered {
                    result.relations = result
                        .value
                        .answer::<questions::Relate>()?
                        .iter()
                        .map(crate::values::views::relation)
                        .collect();
                }
                result.names = result
                    .value
                    .detail::<questions::Relate>()?
                    .map_or_else(Vec::new, |v| {
                        v.names().iter().map(crate::values::views::name).collect()
                    });
            }
            _ => {}
        }
        match result.value.function()? {
            FunctionName::Choose => {
                if let Some(detail) = result.value.detail::<questions::Choose>()? {
                    result.labels = probabilities(detail.labels());
                }
            }
            FunctionName::Tag => {
                if let Some(detail) = result.value.detail::<questions::Tag>()? {
                    result.labels = probabilities(detail.labels());
                }
            }
            FunctionName::Score => {
                if let Some(detail) = result.value.detail::<questions::Score>()? {
                    result.levels = detail.levels().iter().map(|p| p.get()).collect();
                }
            }
            FunctionName::Rank => {
                if let Some(detail) = result.value.detail::<questions::Rank>()? {
                    result.levels = detail.levels().iter().map(|p| p.get()).collect();
                }
            }
            FunctionName::Recognize => {
                if let Some(detail) = result.value.detail::<questions::Recognize>()? {
                    result.piece_tags = detail
                        .pieces()
                        .iter()
                        .map(|p| probabilities(p.tags()))
                        .collect();
                    result.kind_probabilities = detail
                        .names()
                        .iter()
                        .map(|n| probabilities(n.kinds()))
                        .collect();
                    result.pieces = detail
                        .pieces()
                        .iter()
                        .zip(&result.piece_tags)
                        .map(|(p, tags)| crate::PieceDetailView {
                            span: crate::values::SpanView {
                                present: 1,
                                text: crate::values::views::text(p.text()),
                                start: p.start() as u64,
                                end: p.end() as u64,
                            },
                            tags: tags.as_ptr(),
                            tag_count: tags.len(),
                        })
                        .collect();
                    result.kinds = detail
                        .names()
                        .iter()
                        .zip(&result.kind_probabilities)
                        .map(|(n, kinds)| crate::KindDetailView {
                            span: crate::values::SpanView {
                                present: 1,
                                text: crate::values::views::text(n.text()),
                                start: n.start() as u64,
                                end: n.end() as u64,
                            },
                            kinds: kinds.as_ptr(),
                            kind_count: kinds.len(),
                        })
                        .collect();
                }
            }
            FunctionName::Relate => {
                if let Some(detail) = result.value.detail::<questions::Relate>()? {
                    result.pairs = detail
                        .pairs()
                        .iter()
                        .map(crate::values::views::relation)
                        .collect();
                }
            }
            _ => {}
        }
        result.capture_selected();
        Ok(result)
    }
}
pub(crate) unsafe fn data<'a>(row: *const HostRow) -> Result<&'a RowData, Error> {
    // SAFETY: the caller lends an initialized live row and its payload.
    unsafe {
        crate::memory::row_size(row.cast_mut())?;
        let data = crate::memory::live((*row).private_data)?;
        if (*row).function != data.value.function()?.abi()
            || (*row).status != data.value.status()?.abi()
            || (*row).error_code != data.value.error()?.map_or(0, |e| e.code().abi())
        {
            return Err(native::abi_error());
        }
        let position = match &data.value.0 {
            Owned::Row(row) => row.position(),
            Owned::Entry(_) => 0,
        };
        let probability = data.value.probability()?;
        if (*row).position != position
            || !((*row).probability == probability
                || ((*row).probability.is_nan() && probability.is_nan()))
        {
            return Err(native::abi_error());
        }
        Ok(data)
    }
}
impl HostRow {
    pub(crate) fn from_value(
        size: u32,
        value: ValueHandle,
        domain: Arc<Domain>,
    ) -> Result<Self, Error> {
        let mut out = Self::empty(size);
        out.function = value.function()?.abi();
        out.status = value.status()?.abi();
        out.error_code = value.error()?.map_or(0, |e| e.code().abi());
        out.probability = value.probability()?;
        if let Owned::Row(row) = &value.0 {
            out.position = row.position();
        }
        out.private_data = Box::into_raw(Box::new(RowData::new(value, domain)?));
        Ok(out)
    }
}

fn probabilities(
    values: &BTreeMap<String, thinkthen::Probability>,
) -> Vec<crate::values::ProbabilityView> {
    values
        .iter()
        .map(|(label, p)| crate::values::ProbabilityView {
            label: crate::values::views::text(label),
            probability: p.get(),
        })
        .collect()
}

#[cfg(test)]
impl Drop for RowData {
    fn drop(&mut self) {
        crate::memory::LIVE.with(|live| live.set(live.get() - 1));
    }
}
