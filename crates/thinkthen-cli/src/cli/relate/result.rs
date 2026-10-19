//! Command occurrence presentation retains every native judgment field and its order.
use crate::failure::Failure;
use crate::render::json_line;
use crate::saved_json::{Json, Kind};
use crate::{CompleteRelated, RawRecord};
use serde::{Serialize, Serializer};
use std::io::Write;

pub(super) fn write(
    writer: &mut dyn Write,
    detailed: bool,
    result: &CompleteRelated,
    originals: &[RawRecord],
) -> Result<(), Failure> {
    if detailed {
        return details(writer, result, originals, None::<&()>);
    }
    let mut text = String::new();
    for edge in result.value() {
        text += &(edge.to_json() + "\n");
    }
    match writer
        .write_all(text.as_bytes())
        .and_then(|()| writer.flush())
    {
        Err(error) if !Failure::closed_output(&error) => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

pub(super) fn details<I: Serialize + ?Sized, V: Serialize + ?Sized>(
    writer: &mut dyn Write,
    result: &CompleteRelated,
    originals: &I,
    value: Option<&V>,
) -> Result<(), Failure> {
    let document = Json::parse(&result.to_json()?)?;
    let Kind::Object(fields) = document.kind() else {
        return Err(Failure::Defect(
            "a complete relation result is not an object",
        ));
    };
    crate::edge::write_line(
        writer,
        &json_line(&Complete {
            fields,
            originals,
            value,
        })?,
    )?;
    Ok(())
}
struct Complete<'a, I: ?Sized, V: ?Sized> {
    fields: &'a [(String, Json)],
    originals: &'a I,
    value: Option<&'a V>,
}
impl<I: Serialize + ?Sized, V: Serialize + ?Sized> Serialize for Complete<'_, I, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(None)?;
        for (name, held) in self.fields {
            if name == "input" || name == "index" || name == "input_sources" {
                continue;
            }
            if name == "value" {
                if let Some(value) = self.value {
                    map.serialize_entry(name, value)?;
                } else {
                    map.serialize_entry(name, held)?;
                }
                map.serialize_entry("input", self.originals)?;
                map.serialize_entry("index", &0)?;
            } else {
                map.serialize_entry(name, held)?;
            }
        }
        map.end()
    }
}
