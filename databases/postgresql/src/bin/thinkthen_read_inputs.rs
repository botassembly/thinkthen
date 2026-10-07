//! Client-side native evidence reader for PostgreSQL's explicit descriptor workaround.
use std::io::{self, Read as _, Write as _};
use thinkthen::{InputReaderOptions, SourceItem};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;
    let mut value: serde_json::Value = serde_json::from_str(&source)?;
    let files = value
        .get("files")
        .ok_or("client reader requires explicit files")?;
    let paths: Vec<String> =
        serde_json::from_value(files.get("paths").ok_or("missing paths")?.clone())?;
    let options: InputReaderOptions = serde_json::from_value(
        files
            .get("options")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({})),
    )?;
    let jsonl = files.get("format").and_then(serde_json::Value::as_str) == Some("jsonl");
    let records=thinkthen::read_inputs(paths,options)?.map(|item|item.map(|item|match item {
        SourceItem::Text(s)=>serde_json::json!({if jsonl {"json_text"}else{"text"}:s.record,"source":{"file":s.file,"first_line":s.first_line,"last_line":s.last_line}}),
        SourceItem::Image(s)=>serde_json::json!({"images":[{"media":match s.record.media(){thinkthen::ImageMedia::Png=>"image/png",thinkthen::ImageMedia::Jpeg=>"image/jpeg"},"bytes":s.record.bytes()}],"source":{"file":s.file}}),
    })).collect::<Result<Vec<_>,_>>()?;
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
