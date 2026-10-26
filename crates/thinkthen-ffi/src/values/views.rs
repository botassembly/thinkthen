//! Borrowed typed fields stay valid until their ValueHandle is freed.
use crate::builders::TextView;
use thinkthen::results;
/// A typed borrowed text span; present distinguishes answered null from an empty span.
#[repr(C)]
#[derive(Debug, Default)]
pub struct SpanView {
    /// Zero means find answered no match.
    pub present: u8,
    /// Decoded UTF-8 text owned by the value handle.
    pub text: TextView,
    /// Start in the call's selected offset unit.
    pub start: u64,
    /// End in the selected offset unit.
    pub end: u64,
}
/// A recognized name and its support probability.
#[repr(C)]
#[derive(Debug, Default)]
pub struct NameView {
    /// Original decoded UTF-8 text.
    pub text: TextView,
    /// Span start.
    pub start: u64,
    /// Span end.
    pub end: u64,
    /// Decoded kind name.
    pub kind: TextView,
    /// Full precision support probability.
    pub probability: f64,
}
/// A relation endpoint has no invented aggregate probability.
#[repr(C)]
#[derive(Debug, Default)]
pub struct EndpointView {
    /// Original endpoint text.
    pub text: TextView,
    /// Span start.
    pub start: u64,
    /// Span end.
    pub end: u64,
    /// Endpoint kind.
    pub kind: TextView,
}
/// A typed relation, usable for answers and candidate-pair detail alike.
#[repr(C)]
#[derive(Debug, Default)]
pub struct RelationView {
    /// Typed source endpoint.
    pub source: EndpointView,
    /// Typed target endpoint.
    pub target: EndpointView,
    /// Decoded relation name.
    pub relation: TextView,
    /// Full precision relation probability.
    pub probability: f64,
    /// One for an unordered rule, zero otherwise.
    pub either: u8,
}
/// One caller-domain or boundary label and its probability.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ProbabilityView {
    /// Decoded label.
    pub label: TextView,
    /// Full precision probability.
    pub probability: f64,
}
/// The attempted route, including on failed rows.
#[repr(C)]
#[derive(Debug, Default)]
pub struct SourceView {
    /// Stable question identity.
    pub question_identity: TextView,
    /// Generated backend discriminator.
    pub backend: u32,
    /// Resolved model name.
    pub model: TextView,
}
/// Exact identifier values never pass through a floating-point representation.
#[repr(C)]
#[derive(Debug, Default)]
pub struct IdView {
    /// Generated identifier-kind discriminator.
    pub kind: u32,
    /// String value, usable only for the string kind.
    pub text: TextView,
    /// Integer value, usable only for the signed kind.
    pub signed_value: i64,
    /// Integer value, usable only for the unsigned kind.
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
        start: *value.start() as u64,
        end: *value.end() as u64,
        kind: text(value.kind()),
        probability: value.probability().get(),
    }
}
fn endpoint(value: &results::RelationName) -> EndpointView {
    EndpointView {
        text: text(value.text()),
        start: *value.start() as u64,
        end: *value.end() as u64,
        kind: text(value.kind()),
    }
}
pub(super) fn relation(value: &results::Relation) -> RelationView {
    RelationView {
        source: endpoint(value.source()),
        target: endpoint(value.target()),
        relation: text(value.relation()),
        probability: value.probability().get(),
        either: u8::from(*value.either()),
    }
}
