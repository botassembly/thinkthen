//! Relation details retain complete results and the caller query's row identities.
use super::Ids;
use crate::errors::RowError;
use thinkthen::{Entity, Relate};

fn mapped_details(
    rows: &[serde_json::Value],
    mapped: &Ids,
) -> Result<Vec<serde_json::Value>, String> {
    let mut expanded = Vec::new();
    for row in rows {
        let source = row
            .pointer("/value/source/name")
            .and_then(serde_json::Value::as_str);
        let source_kind = row
            .pointer("/value/source/kind")
            .and_then(serde_json::Value::as_str);
        let target = row
            .pointer("/value/target/name")
            .and_then(serde_json::Value::as_str);
        let target_kind = row
            .pointer("/value/target/kind")
            .and_then(serde_json::Value::as_str);
        let (Some(source), Some(source_kind), Some(target), Some(target_kind)) =
            (source, source_kind, target, target_kind)
        else {
            expanded.push(row.clone());
            continue;
        };
        for from in mapped
            .get(&(source.to_owned(), source_kind.to_owned()))
            .into_iter()
            .flatten()
        {
            for to in mapped
                .get(&(target.to_owned(), target_kind.to_owned()))
                .into_iter()
                .flatten()
            {
                let mut next = row.clone();
                let source = next
                    .pointer_mut("/value/source/name")
                    .ok_or_else(|| "thinkthen defect: relation source disappeared".to_owned())?;
                *source = serde_json::Value::String(from.clone());
                let target = next
                    .pointer_mut("/value/target/name")
                    .ok_or_else(|| "thinkthen defect: relation target disappeared".to_owned())?;
                *target = serde_json::Value::String(to.clone());
                crate::complete_native::put(
                    &mut next,
                    "ordinal",
                    serde_json::json!(expanded.len()),
                )
                .map_err(|error| RowError::from(error).text)?;
                expanded.push(next);
            }
        }
    }
    Ok(expanded)
}

pub(super) fn run(
    engine: &thinkthen::Engine,
    ask: Relate,
    entities: Vec<Entity>,
    mapped: Ids,
    controls: thinkthen::CallOptions<'_>,
) -> Result<Vec<u8>, String> {
    let prepared = crate::complete_native::Prepared::Relate(ask);
    let request = thinkthen::Request::for_definition(
        thinkthen_host::sql_request::definition(&prepared),
        thinkthen::RequestInput::Feed {
            name: "duckdb".to_owned(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        Default::default(),
    )
    .admit()
    .map_err(|error| RowError::from(error).text)?;
    let reading = request
        .record_reading()
        .map_err(|error| RowError::from(error).text)?;
    let records = entities.iter().map(|entity| {
        let item = thinkthen::RequestItem::from_record_descriptor(
            &serde_json::json!({"json":{"name":entity.name(),"kind":entity.kind()}}).to_string(),
        )?;
        item.compose_record(&reading)
    });
    let mut carrier = thinkthen_host::sql_request::run_records_recovering(
        engine,
        &prepared,
        &request,
        records,
        controls,
        thinkthen::Surface::Duckdb,
    )
    .map_err(|error| RowError::from(error).text)?;
    if let Some(rows) = carrier
        .get_mut("rows")
        .and_then(serde_json::Value::as_array_mut)
    {
        *rows = mapped_details(rows, &mapped)?;
    }
    serde_json::to_vec(&carrier)
        .map_err(|_| "thinkthen defect: relate details could not be encoded".to_owned())
}
