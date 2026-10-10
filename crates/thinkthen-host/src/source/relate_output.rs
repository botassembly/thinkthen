//! Count the actual escaped endpoint expansion before cloning source evidence.
use serde::Serialize;
use serde_json::Value;
use std::io::Write;
use thinkthen::{Edge, Error, SourceRecord};

#[derive(Serialize)]
struct Name<'a> {
    name: &'a str,
    kind: &'a str,
}

#[derive(Serialize)]
struct Endpoint<'a> {
    #[serde(flatten)]
    source: &'a SourceRecord<String>,
    value: Name<'a>,
}

#[derive(Serialize)]
struct LocatedEdge<'a> {
    relation: &'a str,
    source: Endpoint<'a>,
    target: Endpoint<'a>,
    probability: f64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    either: bool,
}

pub(super) struct Budget(usize);

impl Budget {
    pub(super) fn new() -> Self {
        Self(thinkthen::Relate::max_source_output_bytes() - 12) // {"edges":[]}; separators are counted between edges.
    }
}

impl Write for Budget {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 = self
            .0
            .checked_sub(bytes.len())
            .ok_or_else(|| std::io::Error::other("source relate output exceeds 16 MiB"))?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(super) fn edge(
    edge: &Edge,
    source: &SourceRecord<String>,
    target: &SourceRecord<String>,
    budget: &mut Budget,
    separator: bool,
) -> Result<Value, Error> {
    let located = LocatedEdge {
        relation: edge.relation(),
        source: Endpoint {
            source,
            value: Name {
                name: edge.source().name(),
                kind: edge.source().kind(),
            },
        },
        target: Endpoint {
            source: target,
            value: Name {
                name: edge.target().name(),
                kind: edge.target().kind(),
            },
        },
        probability: edge.probability(),
        either: edge.either(),
    };
    serde_json::to_writer(&mut *budget, &located)
        .map_err(|_| super::usage("source relate output exceeds 16 MiB"))?;
    if separator {
        budget
            .write_all(b",")
            .map_err(|_| super::usage("source relate output exceeds 16 MiB"))?;
    }
    serde_json::to_value(located).map_err(|_| super::defect("located edges could not be written"))
}
