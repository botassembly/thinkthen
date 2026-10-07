//! Located output snapshots use native physical span and occurrence getters.
use crate::current::{Position, Storage};
use crate::failures::Failure;
use crate::ffi::carriers::{
    ContentV1, EndpointV1, OptionalLocationV1, OptionalSourceEntityEdgesV1, SourceEdgeV1,
    SourceEdgesV1, SourceEndpointV1, SourceEntitiesV1, SourceEntityEdgeV1, SourceEntityEdgesV1,
    SourceEntityV1, SourceRecognitionV1, SourceRelationsV1,
};
fn position(s: &mut Storage, value: &thinkthen::SourceLocation) -> OptionalLocationV1 {
    s.position(Some(&Position {
        file: value.file().to_owned(),
        first_line: value.first_line(),
        last_line: value.last_line(),
    }))
}
fn entity(s: &mut Storage, value: &thinkthen::SourceRecognizedEntity) -> SourceEntityV1 {
    SourceEntityV1 {
        entity: s.entity(value.entity()),
        position: position(s, value.location()),
    }
}
pub(super) fn recognition(
    s: &mut Storage,
    value: Option<&thinkthen::SourceRecognition>,
) -> SourceRecognitionV1 {
    let Some(value) = value else {
        return SourceRecognitionV1::default();
    };
    let values = value.entities().iter().map(|v| entity(s, v)).collect();
    let (data, len) = s.array(values);
    let entities = SourceEntitiesV1 { data, len };
    let relations = value
        .relations()
        .map(|edges| {
            let values = edges
                .iter()
                .map(|v| SourceEntityEdgeV1 {
                    relation: s.string(v.relation()),
                    source: entity(s, v.source()),
                    target: entity(s, v.target()),
                    probability: v.probability(),
                    either: i32::from(v.either()),
                })
                .collect();
            let (data, len) = s.array(values);
            OptionalSourceEntityEdgesV1 {
                present: 1,
                value: SourceEntityEdgesV1 { data, len },
            }
        })
        .unwrap_or_default();
    SourceRecognitionV1 {
        present: 1,
        entities,
        relations,
    }
}
fn endpoint(
    s: &mut Storage,
    value: &thinkthen::SourceRelationEndpoint,
) -> Result<SourceEndpointV1, Failure> {
    let raw = value.record();
    let record = match raw.literal() {
        Some(text) => ContentV1 {
            kind: 1,
            data: s.string(text),
        },
        None => ContentV1 {
            kind: 2,
            data: s.string(
                &serde_json::to_string(raw)
                    .map_err(|_| Failure::defect("native source record could not be retained"))?,
            ),
        },
    };
    Ok(SourceEndpointV1 {
        ordinal: value.ordinal(),
        endpoint: EndpointV1 {
            name: s.string(value.entity().name()),
            kind: s.string(value.entity().kind()),
        },
        record,
        position: position(s, value.location()),
    })
}
pub(super) fn relations(
    s: &mut Storage,
    value: Option<&[thinkthen::SourceRelationEdge]>,
) -> Result<SourceRelationsV1, Failure> {
    let Some(value) = value else {
        return Ok(SourceRelationsV1::default());
    };
    let values = value
        .iter()
        .map(|v| {
            Ok(SourceEdgeV1 {
                relation: s.string(v.edge().relation()),
                source: endpoint(s, v.source())?,
                target: endpoint(s, v.target())?,
                probability: v.edge().probability(),
                either: i32::from(v.edge().either()),
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let (data, len) = s.array(values);
    Ok(SourceRelationsV1 {
        present: 1,
        edges: SourceEdgesV1 { data, len },
    })
}
