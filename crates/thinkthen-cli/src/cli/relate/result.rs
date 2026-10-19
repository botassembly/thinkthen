//! Complete relation details delegate to the shared native renderer.
use crate::core::{self, json_line};
use crate::failure::Failure;
use serde::{Serialize, Serializer};
use std::io::Write;
pub(super) fn write(
    writer: &mut dyn Write,
    details: bool,
    canonical: &core::CompleteRelation,
    originals: &[core::Record],
) -> Result<(), Failure> {
    let mut text = String::new();
    if details {
        text = json_line(&Complete {
            canonical,
            originals,
        })? + "\n";
    } else {
        for edge in &canonical.value {
            text += &(json_line(edge)? + "\n");
        }
    }
    match writer
        .write_all(text.as_bytes())
        .and_then(|()| writer.flush())
    {
        Err(error) if !Failure::closed_output(&error) => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

struct Complete<'a> {
    canonical: &'a core::CompleteRelation,
    originals: &'a [core::Record],
}
impl Serialize for Complete<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize_occurrence(
            Some(&self.originals),
            &self.canonical.value,
            Some(0),
            serializer,
        )
    }
}
