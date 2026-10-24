//! Saved entry admission and bounded local lookup.

use std::collections::BTreeMap;
use std::fmt;
use std::io::Read;
use std::path::{Component, Path};

use crate::files::{FileError, FileJob, FileReceipt};
use crate::{Cancel, Client, Error, ErrorKind, SdkRequest, TOOLBOX_LIMIT};
use serde::de::{Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

/// A saved function request or a nonempty sequence of declared steps.
#[derive(Debug)]
pub struct RunEntry(pub(crate) Vec<Step>);

#[derive(Debug)]
pub(crate) enum Step {
    Function(Box<SdkRequest>),
    Shell(Vec<String>),
}

impl RunEntry {
    /// Admit one entry from JSON bytes, without reading files or settings.
    ///
    /// # Errors
    /// Returns the toolbox or existing function admission error.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > TOOLBOX_LIMIT {
            return Err(invalid("the toolbox exceeds its byte limit"));
        }
        let fields: Fields = serde_json::from_slice(bytes).map_err(parse_error)?;
        let steps = if let Some(run) = fields.0.get("run") {
            if fields.0.len() != 1 {
                return Err(invalid("a run accepts only its run field"));
            }
            let steps: Vec<Box<RawValue>> = serde_json::from_str(run.get()).map_err(parse_error)?;
            if steps.is_empty() {
                return Err(invalid("a run needs at least one step"));
            }
            steps
                .iter()
                .map(|raw| step(raw.get()))
                .collect::<Result<_, _>>()?
        } else {
            let text = std::str::from_utf8(bytes).map_err(|_| invalid("the entry is not UTF-8"))?;
            vec![function(text)?]
        };
        Ok(Self(steps))
    }

    /// Borrow the function request when this entry contains exactly one function.
    #[must_use]
    pub fn request(&self) -> Option<&SdkRequest> {
        match self.0.as_slice() {
            [Step::Function(request)] => Some(request),
            _ => None,
        }
    }

    /// Execute file framing through the shared table operation for one function.
    ///
    /// # Errors
    /// Rejects a mixed-step entry before file or backend work, and otherwise
    /// preserves the file operation's admission, execution and publication errors.
    pub fn execute_file(
        &self,
        job: &FileJob,
        client: &Client,
        cancel: &Cancel,
    ) -> Result<FileReceipt, FileError> {
        let request = self.request().ok_or_else(|| {
            FileError::Invalid("file framing requires a single-function entry; use JSON lines for a mixed-step run".into())
        })?;
        job.execute(request, client, cancel)
    }
}

fn function(text: &str) -> Result<Step, Error> {
    let request = SdkRequest::from_json(text)?;
    let fields: Fields = serde_json::from_str(text).map_err(parse_error)?;
    if fields.0.contains_key("records") {
        return Err(invalid("a saved function takes records from its input"));
    }
    Ok(Step::Function(Box::new(request)))
}

fn step(text: &str) -> Result<Step, Error> {
    let fields: Fields = serde_json::from_str(text).map_err(parse_error)?;
    if fields.0.contains_key("run") {
        return Err(invalid("a run cannot contain another run"));
    }
    if let Some(shell) = fields.0.get("shell") {
        if fields.0.len() != 1 {
            return Err(invalid("a shell step accepts only its shell field"));
        }
        let args: Vec<String> = serde_json::from_str(shell.get()).map_err(parse_error)?;
        if args.first().is_none_or(String::is_empty) || args.iter().any(|arg| arg.contains('\0')) {
            return Err(invalid(
                "a shell step needs an executable and arguments without NUL",
            ));
        }
        Ok(Step::Shell(args))
    } else {
        function(text)
    }
}

/// Load an explicit toolbox and name, a single-entry file, or a local entry name.
///
/// Relative paths are resolved against `directory`; this never changes working directory.
///
/// # Errors
/// Returns a local error for unreadable, ambiguous, missing or invalid entries.
pub fn load(
    directory: &Path,
    toolbox_or_name: &str,
    name: Option<&str>,
) -> Result<RunEntry, Error> {
    if let Some(name) = name {
        validate_name(name)?;
        return selected(&directory.join(toolbox_or_name), Some(name));
    }
    if toolbox_or_name.ends_with(".json") {
        return selected(&directory.join(toolbox_or_name), None);
    }
    validate_name(toolbox_or_name)?;
    let file = directory.join("thinkthen.json");
    let folder = directory.join("thinkthen");
    match (file.try_exists(), folder.try_exists()) {
        (Ok(true), Ok(true)) => Err(invalid(
            "both local toolboxes exist; name the toolbox explicitly",
        )),
        (Ok(true), Ok(false)) => selected(&file, Some(toolbox_or_name)),
        (Ok(false), Ok(true)) => selected(&folder, Some(toolbox_or_name)),
        (Ok(false), Ok(false)) => Err(invalid("no local toolbox exists")),
        _ => Err(invalid("the local toolbox could not be inspected")),
    }
}

fn selected(path: &Path, name: Option<&str>) -> Result<RunEntry, Error> {
    if path.is_dir() {
        let name = name.ok_or_else(|| invalid("a toolbox folder needs an entry name"))?;
        return RunEntry::from_bytes(&read(&path.join(format!("{name}.json")))?);
    }
    let bytes = read(path)?;
    let fields: Fields = serde_json::from_slice(&bytes).map_err(parse_error)?;
    if let Some(entries) = fields.0.get("entries") {
        if fields.0.len() != 1 {
            return Err(invalid("a toolbox accepts only its entries field"));
        }
        let entries: Fields = serde_json::from_str(entries.get()).map_err(parse_error)?;
        for key in entries.0.keys() {
            validate_name(key)?;
        }
        let entry = match name {
            Some(name) => entries
                .0
                .get(name)
                .ok_or_else(|| invalid("the toolbox has no such entry"))?,
            None if entries.0.len() == 1 => entries
                .0
                .values()
                .next()
                .ok_or_else(|| invalid("the toolbox is empty"))?,
            None => return Err(invalid("the toolbox needs an entry name")),
        };
        RunEntry::from_bytes(entry.get().as_bytes())
    } else if name.is_some() {
        Err(invalid("a named toolbox needs an entries object"))
    } else {
        RunEntry::from_bytes(&bytes)
    }
}

fn validate_name(name: &str) -> Result<(), Error> {
    let mut parts = Path::new(name).components();
    if name.is_empty()
        || name.contains(['/', '\\', '\0'])
        || !matches!(parts.next(), Some(Component::Normal(_)))
        || parts.next().is_some()
    {
        return Err(invalid(
            "an entry name must be one filename without path separators",
        ));
    }
    Ok(())
}

fn read(path: &Path) -> Result<Vec<u8>, Error> {
    let file = std::fs::File::open(path).map_err(|_| invalid("the toolbox could not be opened"))?;
    let mut bytes = Vec::new();
    file.take((TOOLBOX_LIMIT + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid("the toolbox could not be read"))?;
    if bytes.len() > TOOLBOX_LIMIT {
        return Err(invalid("the toolbox exceeds its byte limit"));
    }
    Ok(bytes)
}

fn invalid(message: &str) -> Error {
    Error::new(ErrorKind::Toolbox, message)
}
fn parse_error(error: serde_json::Error) -> Error {
    Error::new(ErrorKind::Toolbox, format!("invalid toolbox: {error}"))
}

struct Fields(BTreeMap<String, Box<RawValue>>);
impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Distinct;
        impl<'de> Visitor<'de> for Distinct {
            type Value = Fields;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an object with distinct fields")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Fields, M::Error> {
                let mut fields = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, Box<RawValue>>()? {
                    if fields.insert(key, value).is_some() {
                        return Err(M::Error::custom("duplicate toolbox field"));
                    }
                }
                Ok(Fields(fields))
            }
        }
        deserializer.deserialize_map(Distinct)
    }
}
