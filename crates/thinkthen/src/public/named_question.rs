//! Native file selection and ordinary named/reference loading.
use super::{Error, LoadedQuestion, Question, QuestionName};
use crate::core::QuestionRole;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn unavailable() -> Error {
    Error::local("the named question is unavailable")
}

/// A selected question file whose contents must be read by an authorized caller.
///
/// Resolution inspects filesystem metadata but opens no content. This value
/// grants no permission to read and does not prevent replacement of a path.
/// Hosts must authorize the selected spelling and check an opened descriptor
/// against [`Self::named_root`] where present before reading its contents.
pub struct QuestionFileReference {
    path: PathBuf,
    name: Option<QuestionName>,
    named_root: Option<PathBuf>,
    canonical_target: Option<PathBuf>,
}
impl std::fmt::Debug for QuestionFileReference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("QuestionFileReference")
            .finish_non_exhaustive()
    }
}
impl QuestionFileReference {
    /// Select NAME.json beneath the native config questions folder without reading it.
    /// Config.json need not exist. The name is validated before filesystem lookup.
    /// # Errors
    /// Invalid names are Usage; unavailable or unconfined named files are Local.
    pub fn named(name: &str) -> Result<Self, Error> {
        let name = QuestionName::new(name)?;
        let config_path = crate::config::path()
            .and_then(|path| path.parent().map(Path::to_path_buf))
            .ok_or_else(unavailable)?;
        let config = config_path.canonicalize().map_err(|_| unavailable())?;
        let directory = config_path.join("questions");
        let named_root = directory.canonicalize().map_err(|_| unavailable())?;
        if !named_root.starts_with(&config) || !named_root.is_dir() {
            return Err(unavailable());
        }
        let path = directory.join(format!("{}.json", name.as_str()));
        let target = path.canonicalize().map_err(|_| unavailable())?;
        if !target.starts_with(&named_root) || !target.is_file() {
            return Err(unavailable());
        }
        Ok(Self {
            path,
            name: Some(name),
            named_root: Some(named_root),
            canonical_target: Some(target),
        })
    }
    /// Select an explicit @ reference relative to the process working directory.
    /// Any existing local entry wins over a named file, including dangling links
    /// and directories. Only a missing valid bare name permits named lookup.
    /// # Errors
    /// Missing @ is Usage; metadata or named-file failures are Local.
    pub fn reference(reference: &str) -> Result<Self, Error> {
        let value = reference_value(reference)?;
        Self::in_directory(value, &working_directory()?)
    }
    /// Select an @ reference using an explicit relative-path lookup directory.
    /// A relative directory is based on the process working directory once.
    /// Selection changes no process directory and preserves parent components.
    /// # Errors
    /// As [`Self::reference`], including an unavailable working directory.
    pub fn reference_in(reference: &str, directory: &Path) -> Result<Self, Error> {
        let value = reference_value(reference)?;
        let directory = if directory.is_absolute() {
            directory.to_path_buf()
        } else {
            working_directory()?.join(directory)
        };
        Self::in_directory(value, &directory)
    }
    fn in_directory(value: &str, directory: &Path) -> Result<Self, Error> {
        match Reference::select(value, directory.join(value))? {
            Reference::Name(name) => Self::named(&name),
            Reference::Path(path) => Ok(Self {
                path,
                name: None,
                named_root: None,
                canonical_target: None,
            }),
        }
    }
    /// The selected absolute path spelling, including symlinks and parent components.
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// The checked canonical questions directory captured by named selection.
    /// Explicit local paths have no named root. Hosts must compare this captured
    /// root to the actual opened file; recanonicalizing a replaced root is unsafe.
    pub fn named_root(&self) -> Option<&Path> {
        self.named_root.as_deref()
    }
    /// Admit original caller-read text through an existing native content parser.
    ///
    /// This performs no filesystem or environment operations. It checks the byte
    /// cap, original authored name and file role before calling `parser` once
    /// with the unchanged text. Supply a content-only grammar such as
    /// [`Question::from_json`]; a loader could read another unauthorized file.
    /// Apply caller settings after this saved-file boundary.
    /// # Errors
    /// Name, syntax, size and parser failures are Local; an unambiguous different
    /// role is Usage. An omitted name stays absent in the supplied grammar.
    pub fn parse<T>(
        &self,
        original_json: &str,
        role: QuestionRole,
        parser: impl FnOnce(&str) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if original_json.len() as u64 > super::question_file::LIMIT {
            return Err(Error::local("the question file is too large"));
        }
        self.validate_name(original_json)?;
        check_role(original_json, role)?;
        parser(original_json).map_err(file_error)
    }
    fn validate_name(&self, text: &str) -> Result<(), Error> {
        let Some(name) = &self.name else {
            return Ok(());
        };
        // Ordinary grammar owns declaration validation and never infers a name.
        let value =
            crate::core::Json::parse(text).map_err(|error| Error::local(error.to_string()))?;
        if let Some(declared) = value.member("name")
            && declared.as_str().is_some_and(|held| held != name.as_str())
        {
            return Err(Error::local(
                "the named question name does not match the file",
            ));
        }
        Ok(())
    }
}
fn working_directory() -> Result<PathBuf, Error> {
    std::env::current_dir().map_err(|_| Error::local("the question file could not be read"))
}
fn reference_value(value: &str) -> Result<&str, Error> {
    value
        .strip_prefix('@')
        .ok_or_else(|| Error::usage("a question reference begins with @"))
}
pub(crate) fn named_text(name: &str) -> Result<String, Error> {
    let reference = QuestionFileReference::named(name)?;
    let path = reference
        .canonical_target
        .as_deref()
        .unwrap_or(reference.path());
    let text =
        super::question_file::load_text(path, "named question").map_err(|_| unavailable())?;
    reference.validate_name(&text)?;
    Ok(text)
}

pub(crate) enum Reference {
    Path(PathBuf),
    Name(String),
}
impl Reference {
    pub(crate) fn resolve(value: &str) -> Result<Self, Error> {
        Self::of_path(reference_value(value)?)
    }
    pub(crate) fn of_path(value: &str) -> Result<Self, Error> {
        Self::select(value, PathBuf::from(value))
    }
    fn select(value: &str, path: PathBuf) -> Result<Self, Error> {
        if QuestionName::new(value).is_err() {
            return Ok(Self::Path(path));
        }
        match fs::symlink_metadata(&path) {
            Ok(_) => Ok(Self::Path(path)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(Self::Name(value.to_owned()))
            }
            Err(_) => Err(Error::local("the question file could not be read")),
        }
    }
}

fn check_role(text: &str, role: QuestionRole) -> Result<(), Error> {
    if role.differs(text) {
        return Err(Error::usage("the question file uses another function"));
    }
    Ok(())
}
fn role_text(text: String, role: QuestionRole) -> Result<String, Error> {
    check_role(&text, role)?;
    Ok(text)
}
fn named_role(name: &str, role: QuestionRole) -> Result<String, Error> {
    role_text(named_text(name)?, role)
}
fn reference_role(value: &str, role: QuestionRole) -> Result<String, Error> {
    match Reference::resolve(value)? {
        Reference::Path(path) => role_text(
            super::question_file::load_text(&path, "question file")?,
            role,
        ),
        Reference::Name(name) => named_role(&name, role),
    }
}
fn file_error(error: Error) -> Error {
    Error::local(error.detail().message())
}
impl Question {
    /// Load exactly NAME.json from the native config questions folder.
    /// # Errors
    /// Invalid names or another file role are Usage; unavailable/invalid files are Local.
    pub fn load_named(name: &str) -> Result<LoadedQuestion, Error> {
        Self::from_json(&named_role(name, QuestionRole::Atomic)?).map_err(file_error)
    }
    /// Load an explicit @ reference, preserving working-directory path precedence.
    /// # Errors
    /// Refuses missing @, wrong roles and file/name failures at their native boundaries.
    pub fn load_reference(value: &str) -> Result<LoadedQuestion, Error> {
        Self::from_json(&reference_role(value, QuestionRole::Atomic)?).map_err(file_error)
    }
}
macro_rules! role_loaders {
    ($($role:ident => $kind:ident),+ $(,)?) => {$(
        impl super::$role {
            /// Load a named file through the role's existing grammar and capped reader.
            /// # Errors
            /// Invalid names or another role are Usage; unavailable/invalid files are Local.
            pub fn load_named(name: &str) -> Result<Self, Error> {
                Self::from_json(&named_role(name,QuestionRole::$kind)?).map_err(file_error)
            }
            /// Load an explicit @ reference with ordinary local-path precedence.
            /// # Errors
            /// Uses the same role/name and file/content boundaries as load_named.
            pub fn load_reference(value: &str) -> Result<Self, Error> {
                Self::from_json(&reference_role(value,QuestionRole::$kind)?).map_err(file_error)
            }
        }
    )+};
}
role_loaders!(QuestionSet=>Set, Recognize=>Recognize, RecognizeQuestionFile=>Recognize,
    Relate=>Relate, RecordChooseQuestion=>Choose, FindQuestionFile=>Find, RankSet=>Set);
macro_rules! selected_loaders {
    ($owner:ident, $named:ident, $reference:ident, $parser:ident, $role:ident) => {
        impl super::$owner {
            /// Load this explicit saved reading by validated native name.
            /// # Errors
            /// Wrong role/name is Usage; unavailable or invalid saved content is Local.
            pub fn $named(name: &str) -> Result<Self, Error> {
                Self::$parser(&named_role(name, QuestionRole::$role)?).map_err(file_error)
            }
            /// Load this explicit saved reading by native @ reference.
            /// # Errors
            /// Preserves local-path precedence and the named role/content boundaries.
            pub fn $reference(value: &str) -> Result<Self, Error> {
                Self::$parser(&reference_role(value, QuestionRole::$role)?).map_err(file_error)
            }
        }
    };
}
selected_loaders!(
    Question,
    load_rank_named,
    load_rank_reference,
    rank_from_json,
    Rank
);
selected_loaders!(
    Question,
    load_find_named,
    load_find_reference,
    find_from_json,
    Find
);
selected_loaders!(
    Relate,
    load_records_named,
    load_records_reference,
    from_records_json,
    Relate
);
