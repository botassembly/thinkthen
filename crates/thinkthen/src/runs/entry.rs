//! Saved entry admission and bounded local lookup.

use std::collections::BTreeMap;
use std::fmt;
use std::io::Read;
use std::path::Path;

use crate::core::diagnostic;
use crate::files::{FileError, FileJob, FileReceipt};
use crate::{Cancel, Engine, Error, ErrorKind, Request, TOOLBOX_LIMIT};
use serde::de::{Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

/// A saved function request or a nonempty sequence of declared steps.
#[derive(Debug)]
pub struct RunEntry(pub(crate) Vec<Step>);

#[derive(Debug)]
pub(crate) enum Step {
    Function(Box<Request>),
    Shell {
        arguments: Vec<String>,
        env: Vec<String>,
    },
}

impl RunEntry {
    /// Admit one entry from JSON bytes, without reading files or settings.
    ///
    /// # Errors
    /// Returns the toolbox or existing function admission error.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > TOOLBOX_LIMIT {
            return Err(invalid(&diagnostic::requirement(
                "toolbox",
                "a toolbox within its byte limit",
                "toolbox exceeds its byte limit",
            )));
        }
        let fields: Fields = serde_json::from_slice(bytes).map_err(parse_error)?;
        let steps = if let Some(run) = fields.0.get("run") {
            if fields.0.len() != 1 {
                return Err(invalid(&diagnostic::unknown(
                    "run",
                    fields
                        .0
                        .keys()
                        .find(|name| name.as_str() != "run")
                        .map_or("", String::as_str),
                    &["run"],
                )));
            }
            let steps: Vec<Box<RawValue>> =
                serde_json::from_str(run.get()).map_err(|error| parse_error_at("run", error))?;
            if steps.is_empty() {
                return Err(invalid(&diagnostic::requirement(
                    "run",
                    "at least one step",
                    "an empty array",
                )));
            }
            steps
                .iter()
                .enumerate()
                .map(|(index, raw)| {
                    step(raw.get()).map_err(|error| {
                        Error::from_contract(
                            error.code(),
                            diagnostic::context(&format!("run[{index}]"), &error.to_string()),
                        )
                    })
                })
                .collect::<Result<_, _>>()?
        } else {
            let text = std::str::from_utf8(bytes).map_err(|_| {
                invalid(&diagnostic::requirement(
                    "entry",
                    "UTF-8 JSON",
                    "invalid UTF-8",
                ))
            })?;
            vec![function(text)?]
        };
        Ok(Self(steps))
    }

    pub(crate) fn map_requests(
        self,
        mut map: impl FnMut(Request) -> Result<Request, Error>,
    ) -> Result<Self, Error> {
        self.0
            .into_iter()
            .map(|step| match step {
                Step::Function(request) => {
                    map(*request).map(|request| Step::Function(Box::new(request)))
                }
                Step::Shell { arguments, env } => Ok(Step::Shell { arguments, env }),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }

    pub(crate) fn shell_steps(&self) -> impl Iterator<Item = (&[String], &[String])> {
        self.0.iter().filter_map(|step| match step {
            Step::Shell { arguments, env } => Some((arguments.as_slice(), env.as_slice())),
            Step::Function(_) => None,
        })
    }

    /// Borrow the function request when this entry contains exactly one function.
    #[must_use]
    pub fn request(&self) -> Option<&Request> {
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
        client: &Engine,
        cancel: &Cancel,
    ) -> Result<FileReceipt, FileError> {
        let request = self.request().ok_or_else(|| {
            FileError::Invalid(diagnostic::requirement(
                "entry",
                "a single-function entry for file framing",
                "a mixed-step run",
            ))
        })?;
        job.execute(request, client, cancel)
    }
}

fn function(text: &str) -> Result<Step, Error> {
    let request = Request::from_json(text)?;
    let fields: Fields = serde_json::from_str(text).map_err(parse_error)?;
    if fields.0.contains_key("records") {
        return Err(invalid(&diagnostic::requirement(
            "records",
            "records from command input",
            "saved records",
        )));
    }
    Ok(Step::Function(Box::new(request)))
}

fn step(text: &str) -> Result<Step, Error> {
    let fields: Fields = serde_json::from_str(text).map_err(parse_error)?;
    if fields.0.contains_key("run") {
        return Err(invalid(&diagnostic::requirement(
            "run",
            "function or shell steps",
            "a nested run",
        )));
    }
    if !fields.0.contains_key("shell") {
        if !fields.0.contains_key("question") {
            // Only malformed function steps need the schema. Its property keys
            // keep the accepted request names owned by request admission.
            let schema = schemars::schema_for!(Request);
            let mut accepted: Vec<&str> = schema
                .as_value()
                .get("properties")
                .and_then(serde_json::Value::as_object)
                .into_iter()
                .flat_map(serde_json::Map::keys)
                .map(String::as_str)
                .collect();
            accepted.push("shell");
            if let Some(name) = fields
                .0
                .keys()
                .find(|name| !accepted.contains(&name.as_str()))
            {
                return Err(Error::new(
                    ErrorKind::Question,
                    diagnostic::unknown("step", name, &accepted),
                ));
            }
        }
        return function(text);
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ShellStep {
        #[serde(deserialize_with = "shell_arguments")]
        shell: Vec<String>,
        #[serde(default)]
        env: Vec<String>,
    }
    let shell: ShellStep = serde_json::from_str(text).map_err(|error| {
        Error::new(
            ErrorKind::Toolbox,
            diagnostic::context("step", &error.to_string()),
        )
    })?;
    let args = shell.shell;
    if args.first().is_none_or(String::is_empty) || args.iter().any(|arg| arg.contains('\0')) {
        return Err(invalid(&diagnostic::requirement(
            "shell",
            "an executable and arguments without NUL",
            "empty or NUL-containing arguments",
        )));
    }
    if shell
        .env
        .iter()
        .any(|name| name.is_empty() || name.contains(['=', '\0']))
    {
        return Err(invalid(
            "shell env requires variable names without NUL or equals",
        ));
    }
    Ok(Step::Shell {
        arguments: args,
        env: shell.env,
    })
}

fn shell_arguments<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<String>, D::Error> {
    Vec::<String>::deserialize(deserializer)
        .map_err(|error| serde::de::Error::custom(diagnostic::context("shell", &error.to_string())))
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
        let name = name.ok_or_else(|| {
            invalid(&diagnostic::requirement(
                "entry",
                "an entry name",
                "absent name",
            ))
        })?;
        return RunEntry::from_bytes(&read(&path.join(format!("{name}.json")))?);
    }
    let bytes = read(path)?;
    let probe: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| {
        Error::new(
            ErrorKind::Question,
            diagnostic::context("toolbox", &error.to_string()),
        )
    })?;
    // Probe only whether this is a richer local run. Strict shared admission
    // handles every question-file field, duplicate and domain.
    if let Some(entries) = probe.get("entries")
        && !entries
            .as_object()
            .is_some_and(|entries| entries.values().any(rich_entry))
    {
        let json = std::str::from_utf8(&bytes).map_err(|_| {
            Error::new(
                ErrorKind::Question,
                diagnostic::requirement("toolbox", "UTF-8 JSON", "invalid UTF-8"),
            )
        })?;
        let toolbox = crate::Toolbox::from_json(json)?;
        let selected = match name {
            Some(name) => toolbox.get(name),
            None if toolbox.iter().count() == 1 => {
                toolbox.iter().next().map(|(_, question)| question)
            }
            None => {
                return Err(invalid(&diagnostic::requirement(
                    "entry",
                    "an entry name for multiple entries",
                    "absent name",
                )));
            }
        }
        .ok_or_else(|| {
            invalid(&diagnostic::unknown(
                "entry",
                name.unwrap_or_default(),
                &toolbox.iter().map(|(name, _)| name).collect::<Vec<_>>(),
            ))
        })?;
        return Ok(RunEntry(vec![Step::Function(Box::new(Request::new(
            crate::Question::from(selected.clone()),
        )))]));
    }
    let fields: Fields = serde_json::from_slice(&bytes).map_err(parse_error)?;
    if let Some(entries) = fields.0.get("entries") {
        if fields.0.len() != 1 {
            return Err(Error::new(
                ErrorKind::Question,
                diagnostic::unknown(
                    "toolbox",
                    fields
                        .0
                        .keys()
                        .find(|name| name.as_str() != "entries")
                        .map_or("", String::as_str),
                    &["entries"],
                ),
            ));
        }
        let entries: Fields = serde_json::from_str(entries.get())
            .map_err(|error| parse_error_at("entries", error))?;
        for key in entries.0.keys() {
            validate_name(key)?;
        }
        let entry = match name {
            Some(name) => entries.0.get(name).ok_or_else(|| {
                invalid(&diagnostic::unknown(
                    "entry",
                    name,
                    &entries.0.keys().map(String::as_str).collect::<Vec<_>>(),
                ))
            })?,
            None if entries.0.len() == 1 => entries.0.values().next().ok_or_else(|| {
                invalid(&diagnostic::requirement(
                    "entries",
                    "at least one entry",
                    "an empty object",
                ))
            })?,
            None => {
                return Err(invalid(&diagnostic::requirement(
                    "entry",
                    "an entry name for multiple entries",
                    "absent name",
                )));
            }
        };
        RunEntry::from_bytes(entry.get().as_bytes())
    } else if name.is_some() {
        Err(invalid(&diagnostic::requirement(
            "entries",
            "an entries object for a named toolbox",
            "absent field",
        )))
    } else {
        RunEntry::from_bytes(&bytes)
    }
}

fn validate_name(name: &str) -> Result<(), Error> {
    crate::core::questions::validate_saved_name(name)
}

fn read(path: &Path) -> Result<Vec<u8>, Error> {
    let file = std::fs::File::open(path).map_err(|_| invalid("the toolbox could not be opened"))?;
    let mut bytes = Vec::new();
    file.take((TOOLBOX_LIMIT + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid("the toolbox could not be read"))?;
    if bytes.len() > TOOLBOX_LIMIT {
        return Err(invalid(&diagnostic::requirement(
            "toolbox",
            "a toolbox within its byte limit",
            "toolbox exceeds its byte limit",
        )));
    }
    Ok(bytes)
}

fn invalid(message: &str) -> Error {
    Error::new(ErrorKind::Toolbox, message)
}
fn parse_error(error: serde_json::Error) -> Error {
    parse_error_at("toolbox", error)
}
fn parse_error_at(field: &str, error: serde_json::Error) -> Error {
    Error::new(
        ErrorKind::Toolbox,
        diagnostic::context(field, &error.to_string()),
    )
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
                    match fields.entry(key) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(value);
                        }
                        std::collections::btree_map::Entry::Occupied(entry) => {
                            return Err(M::Error::custom(format!(
                                "duplicate field `{}`",
                                entry.key()
                            )));
                        }
                    }
                }
                Ok(Fields(fields))
            }
        }
        deserializer.deserialize_map(Distinct)
    }
}

fn rich_entry(entry: &serde_json::Value) -> bool {
    entry.get("run").is_some()
        || entry.get("name").is_some()
        || entry
            .get("question")
            .and_then(|question| question.get("function"))
            .and_then(serde_json::Value::as_str)
            == Some("annotate")
}
