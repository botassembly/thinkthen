//! Additive located values preserve physical occurrences without changing old layouts.
use super::{ContentV1, EndpointV1, EntityV1, OptionalLocationV1, StringV1};
/// Additive located values come from native span/occurrence mapping. An
/// unlocated call returns present=0. Every nested view borrows result ownership.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEntityV1 {
    /// C field `entity`.
    pub entity: EntityV1,
    /// C field `position`.
    pub position: OptionalLocationV1,
}
/// C descriptor or borrowed view `SourceEntitiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEntitiesV1 {
    /// C field `data`.
    pub data: *const SourceEntityV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `SourceEntityEdgeV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEntityEdgeV1 {
    /// C field `relation`.
    pub relation: StringV1,
    /// C field `source`.
    pub source: SourceEntityV1,
    /// C field `target`.
    pub target: SourceEntityV1,
    /// C field `probability`.
    pub probability: f64,
    /// C field `either`.
    pub either: std::ffi::c_int,
}
/// C descriptor or borrowed view `SourceEntityEdgesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEntityEdgesV1 {
    /// C field `data`.
    pub data: *const SourceEntityEdgeV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalSourceEntityEdgesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalSourceEntityEdgesV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: SourceEntityEdgesV1,
}
/// C descriptor or borrowed view `SourceRecognitionV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceRecognitionV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `entities`.
    pub entities: SourceEntitiesV1,
    /// C field `relations`.
    pub relations: OptionalSourceEntityEdgesV1,
}
/// C descriptor or borrowed view `SourceEndpointV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEndpointV1 {
    /// C field `ordinal`.
    pub ordinal: usize,
    /// C field `endpoint`.
    pub endpoint: EndpointV1,
    /// C field `record`.
    pub record: ContentV1,
    /// C field `position`.
    pub position: OptionalLocationV1,
}
/// C descriptor or borrowed view `SourceEdgeV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEdgeV1 {
    /// C field `relation`.
    pub relation: StringV1,
    /// C field `source`.
    pub source: SourceEndpointV1,
    /// C field `target`.
    pub target: SourceEndpointV1,
    /// C field `probability`.
    pub probability: f64,
    /// C field `either`.
    pub either: std::ffi::c_int,
}
/// C descriptor or borrowed view `SourceEdgesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceEdgesV1 {
    /// C field `data`.
    pub data: *const SourceEdgeV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `SourceRelationsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceRelationsV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `edges`.
    pub edges: SourceEdgesV1,
}
