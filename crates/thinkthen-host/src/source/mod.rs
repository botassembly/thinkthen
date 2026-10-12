//! Located readers retained by the frozen C request and plan transports.

use serde_json::Value;
use thinkthen::{Error, ReaderOptions};

pub struct Selection {
    paths: Vec<String>,
    options: ReaderOptions,
}

impl Selection {
    pub fn request_source(&self) -> thinkthen::RequestSource {
        thinkthen::RequestSource {
            framing: None,
            paths: self.paths.iter().map(std::path::PathBuf::from).collect(),
            reading: self.options,
            media: thinkthen::ReaderMedia::Text,
        }
    }
    pub fn read(&self) -> Result<thinkthen::SourceRecords, Error> {
        thinkthen::read_files(&self.paths, self.options)
    }
}

pub fn parse(text: &str) -> Result<Selection, Error> {
    let mut body: serde_json::Map<String, Value> = serde_json::from_str(text)
        .map_err(|_| usage("source takes paths, unit, and optional window"))?;
    let paths = serde_json::from_value(
        body.remove("paths")
            .ok_or_else(|| usage("source requires paths"))?,
    )
    .map_err(|_| usage("source paths are an array of strings"))?;
    let options: ReaderOptions = serde_json::from_value(Value::Object(body))
        .map_err(|_| usage("source takes paths, unit, and optional window"))?;
    Ok(Selection {
        paths,
        options: options.validate()?,
    })
}

fn usage(message: &str) -> Error {
    Error::new(thinkthen::ErrorKind::Usage, message)
}

#[doc(hidden)]
pub fn defect(message: &str) -> Error {
    Error::new(thinkthen::ErrorKind::Defect, message)
}
