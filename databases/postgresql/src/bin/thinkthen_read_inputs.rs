//! Client-side native evidence reader for PostgreSQL's explicit descriptor workaround.
use std::io::{self, Read as _, Write as _};
use thinkthen::{InputReaderOptions, SourceItem};
#[path = "../../../sqlite/src/complete_native/file_format.rs"]
mod file_format;
type Descriptors = Box<dyn Iterator<Item = Result<serde_json::Value, thinkthen::Error>>>;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;
    let mut value: serde_json::Value = serde_json::from_str(&source)?;
    let files = value
        .get("files")
        .ok_or("client reader requires explicit files")?;
    let format = files.get("format").map(ToString::to_string);
    let framing = file_format::framing(format.as_deref())?;
    let paths: Vec<String> =
        serde_json::from_value(files.get("paths").ok_or("missing paths")?.clone())?;
    let options: InputReaderOptions = serde_json::from_value(
        files
            .get("options")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({})),
    )?;
    let descriptor = |item: SourceItem| match item {
        SourceItem::Text(s) => {
            serde_json::json!({"text":s.record,"source":{"file":s.file,"first_line":s.first_line,"last_line":s.last_line}})
        }
        SourceItem::Image(s) => {
            serde_json::json!({"images":[{"media":match s.record.media(){thinkthen::ImageMedia::Png=>"image/png",thinkthen::ImageMedia::Jpeg=>"image/jpeg"},"bytes":s.record.bytes()}],"source":{"file":s.file}})
        }
    };
    let mut records = Vec::new();
    let items: Result<Descriptors, thinkthen::Error> = if let Some(framing) = framing {
        thinkthen::RequestSource {
            paths: paths.into_iter().map(Into::into).collect(),
            reading: options.reading,
            media: options.media,
            framing: Some(framing),
        }
        .read_framed(Default::default())
        .map(|items| Box::new(items.map(|item| item.and_then(file_format::descriptor))) as _)
    } else {
        thinkthen::read_inputs(paths, options)
            .map(|items| Box::new(items.map(move |item| item.map(descriptor))) as _)
    };
    match items {
        Err(error) => records.push(serde_json::json!({"read_error":error.complete()})),
        Ok(items) => {
            for item in items {
                match item {
                    Ok(item) => records.push(item),
                    Err(error) => {
                        records.push(serde_json::json!({"read_error":error.complete()}));
                        break;
                    }
                }
            }
        }
    }
    value
        .as_object_mut()
        .ok_or("inputs must be one object")?
        .remove("files");
    value
        .as_object_mut()
        .ok_or("inputs must be one object")?
        .insert("records".to_owned(), serde_json::json!(records));
    writeln!(io::stdout().lock(), "{value}")?;
    Ok(())
}
