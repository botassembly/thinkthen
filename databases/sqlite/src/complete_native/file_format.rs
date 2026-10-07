//! The complete file descriptor admits only the explicit JSON-line format.
use thinkthen::{Error, ErrorKind};
pub(crate) fn jsonl(format: Option<&str>) -> Result<bool, Error> {
    match format {
        None => Ok(false),
        Some(source) if serde_json::from_str::<String>(source).ok().as_deref() == Some("jsonl") => {
            Ok(true)
        }
        _ => Err(Error::new(ErrorKind::Usage, "file format is jsonl")),
    }
}
