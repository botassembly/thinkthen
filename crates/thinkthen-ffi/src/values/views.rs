//! Borrowed typed fields stay valid until their ValueHandle is freed.
use crate::builders::TextView;
use thinkthen::results;
/// A part of a text, as thinkthen_find and the find, piece and kind accessors write it.
/// Its text borrows from the value, or from the caller's own text for thinkthen_find.
#[repr(C)]
#[derive(Debug, Default)]
pub struct SpanView {
    /// 1 when a part answers the question; 0 when the answer is that none does.
    pub present: u8,
    /// The part's text; not NUL-terminated.
    pub text: TextView,
    /// Where the part starts, in the call's offset unit (bytes for C).
    pub start: u64,
    /// Where the part ends, exclusive, in the same unit.
    pub end: u64,
}
/// A recognized name, as thinkthen_value_name_at writes it; its texts borrow from the value.
#[repr(C)]
#[derive(Debug, Default)]
pub struct NameView {
    /// The name as it appears in the record.
    pub text: TextView,
    /// Where the name starts, in the call's offset unit.
    pub start: u64,
    /// Where the name ends, exclusive.
    pub end: u64,
    /// The kind of name, one of the question's kinds.
    pub kind: TextView,
    /// How likely the name is right, from 0 to 1.
    pub probability: f64,
}
/// One name a relation joins; its texts borrow from the value.
#[repr(C)]
#[derive(Debug, Default)]
pub struct EndpointView {
    /// The name as it appears in the record.
    pub text: TextView,
    /// Where the name starts, in the call's offset unit.
    pub start: u64,
    /// Where the name ends, exclusive.
    pub end: u64,
    /// The kind of name.
    pub kind: TextView,
}
/// A relation between two names, as thinkthen_value_relation_at and thinkthen_value_pair_at
/// write it; its texts borrow from the value.
#[repr(C)]
#[derive(Debug, Default)]
pub struct RelationView {
    /// The name the relation starts from.
    pub source: EndpointView,
    /// The name the relation points to.
    pub target: EndpointView,
    /// The relation's name, from the question's rule.
    pub relation: TextView,
    /// How likely the relation holds, from 0 to 1.
    pub probability: f64,
    /// 1 when the rule runs both ways, 0 otherwise.
    pub either: u8,
}
/// A label and its probability, as the detail accessors write it; label borrows from the value.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ProbabilityView {
    /// The label.
    pub label: TextView,
    /// Its probability, from 0 to 1.
    pub probability: f64,
}
/// Which question, backend and model gave a row, failed rows included; texts borrow from the value.
#[repr(C)]
#[derive(Debug, Default)]
pub struct SourceView {
    /// A stable identity of the question.
    pub question_identity: TextView,
    /// The backend, a thinkthen_backend value.
    pub backend: u32,
    /// The model the backend used.
    pub model: TextView,
}
/// A record's id, exactly as given: integers never pass through a double.
#[repr(C)]
#[derive(Debug, Default)]
pub struct IdView {
    /// Which field holds the id: THINKTHEN_ID_ABSENT, _STRING, _SIGNED or _UNSIGNED.
    pub kind: u32,
    /// The id, for THINKTHEN_ID_STRING; borrows from the value.
    pub text: TextView,
    /// The id, for THINKTHEN_ID_SIGNED.
    pub signed_value: i64,
    /// The id, for THINKTHEN_ID_UNSIGNED.
    pub unsigned_value: u64,
}
impl Default for TextView {
    fn default() -> Self {
        Self {
            data: std::ptr::null(),
            len: 0,
        }
    }
}
pub(super) fn text(value: &str) -> TextView {
    TextView {
        data: value.as_ptr(),
        len: value.len(),
    }
}
pub(super) fn name(value: &results::Name) -> NameView {
    NameView {
        text: text(value.text()),
        start: value.start() as u64,
        end: value.end() as u64,
        kind: text(value.kind()),
        probability: value.probability().get(),
    }
}
fn endpoint(value: &results::RelationName) -> EndpointView {
    EndpointView {
        text: text(value.text()),
        start: value.start() as u64,
        end: value.end() as u64,
        kind: text(value.kind()),
    }
}
pub(super) fn relation(value: &results::Relation) -> RelationView {
    RelationView {
        source: endpoint(value.source()),
        target: endpoint(value.target()),
        relation: text(value.relation()),
        probability: value.probability().get(),
        either: u8::from(value.either()),
    }
}
