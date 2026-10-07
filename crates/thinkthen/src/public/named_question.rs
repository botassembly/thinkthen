//! Explicit named/reference loading through the ordinary capped reader.
use super::{Error, LoadedQuestion, Question, QuestionName};
use crate::core::QuestionRole;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn unavailable() -> Error {
    Error::local("the named question is unavailable")
}

pub(crate) fn named_text(name: &str) -> Result<String, Error> {
    let name = QuestionName::new(name)?;
    let config = crate::config::path()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .ok_or_else(unavailable)?;
    let config = config.canonicalize().map_err(|_| unavailable())?;
    let directory = config
        .join("questions")
        .canonicalize()
        .map_err(|_| unavailable())?;
    if !directory.starts_with(&config) || !directory.is_dir() {
        return Err(unavailable());
    }
    let path = directory
        .join(format!("{}.json", name.as_str()))
        .canonicalize()
        .map_err(|_| unavailable())?;
    if !path.starts_with(&directory) || !path.is_file() {
        return Err(unavailable());
    }
    let text =
        super::question_file::load_text(&path, "named question").map_err(|_| unavailable())?;
    // The ordinary grammar still owns validation. This comparison never guesses
    // a name when the file omitted it and never prints authored values or paths.
    let value = crate::core::Json::parse(&text).map_err(|error| Error::local(error.to_string()))?;
    if let Some(declared) = value.member("name")
        && declared.as_str().is_some_and(|held| held != name.as_str())
    {
        return Err(Error::local(
            "the named question name does not match the file",
        ));
    }
    Ok(text)
}

pub(crate) enum Reference {
    Path(PathBuf),
    Name(String),
}
impl Reference {
    pub(crate) fn resolve(value: &str) -> Result<Self, Error> {
        let value = value
            .strip_prefix('@')
            .ok_or_else(|| Error::usage("a question reference begins with @"))?;
        Self::of_path(value)
    }
    pub(crate) fn of_path(value: &str) -> Result<Self, Error> {
        if QuestionName::new(value).is_err() {
            return Ok(Self::Path(PathBuf::from(value)));
        }
        match fs::symlink_metadata(value) {
            Ok(_) => Ok(Self::Path(PathBuf::from(value))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(Self::Name(value.to_owned()))
            }
            Err(_) => Err(Error::local("the question file could not be read")),
        }
    }
}

fn role_text(text: String, role: QuestionRole) -> Result<String, Error> {
    if role.differs(&text) {
        return Err(Error::usage("the question file uses another function"));
    }
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
