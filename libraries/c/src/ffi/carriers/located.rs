//! Additive located values preserve physical occurrences without changing old layouts.
use super::{ContentV1, EndpointV1, EntityV1, OptionalLocationV1, StringV1};
layout!(SourceEntityV1 {
    entity: EntityV1,
    position: OptionalLocationV1
});
layout!(SourceEntitiesV1 { data: *const SourceEntityV1, len: usize });
layout!(SourceEntityEdgeV1 {
    relation: StringV1,
    source: SourceEntityV1,
    target: SourceEntityV1,
    probability: f64,
    either: i32
});
layout!(SourceEntityEdgesV1 { data: *const SourceEntityEdgeV1, len: usize });
layout!(OptionalSourceEntityEdgesV1 {
    present: i32,
    value: SourceEntityEdgesV1
});
layout!(SourceRecognitionV1 {
    present: i32,
    entities: SourceEntitiesV1,
    relations: OptionalSourceEntityEdgesV1
});
layout!(SourceEndpointV1 {
    ordinal: usize,
    endpoint: EndpointV1,
    record: ContentV1,
    position: OptionalLocationV1
});
layout!(SourceEdgeV1 {
    relation: StringV1,
    source: SourceEndpointV1,
    target: SourceEndpointV1,
    probability: f64,
    either: i32
});
layout!(SourceEdgesV1 { data: *const SourceEdgeV1, len: usize });
layout!(SourceRelationsV1 {
    present: i32,
    edges: SourceEdgesV1
});
