//! Explicit named/reference loading through the ordinary capped reader.
use super::{Error, LoadedQuestion, Question, QuestionName};
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

impl Question {
    /// Load exactly NAME.json from the native config questions folder.
    /// # Errors
    /// Invalid names are Usage; unavailable, escaping or malformed files are Local.
    pub fn load_named(name: &str) -> Result<LoadedQuestion, Error> {
        Self::from_json(&named_text(name)?).map_err(|error| Error::local(error.detail().message()))
    }
    /// Load an explicit @ reference, preserving existing working-directory paths.
    /// # Errors
    /// Refuses missing @ and file/name errors using the ordinary error boundaries.
    pub fn load_reference(value: &str) -> Result<LoadedQuestion, Error> {
        match Reference::resolve(value)? {
            Reference::Path(path) => Self::load(path),
            Reference::Name(name) => Self::load_named(&name),
        }
    }
}

impl super::QuestionSet {
    /// Load a named question set with the ordinary closed set grammar.
    /// # Errors
    /// Invalid names are Usage; unavailable or malformed files are Local.
    pub fn load_named(name: &str) -> Result<Self, Error> {
        Self::from_json(&named_text(name)?).map_err(|error| Error::local(error.detail().message()))
    }
    /// Load an explicit @ reference as a question set.
    /// # Errors
    /// Uses the same path precedence and failures as Question::load_reference.
    pub fn load_reference(value: &str) -> Result<Self, Error> {
        match Reference::resolve(value)? {
            Reference::Path(path) => Self::load(path),
            Reference::Name(name) => Self::load_named(&name),
        }
    }
}

macro_rules! aggregate_loaders {
    ($($role:ident),+) => {$(
        impl super::$role {
            /// Load a named file through this role's existing grammar and capped reader.
            /// # Errors
            /// Invalid names are Usage; unavailable or malformed files are Local.
            pub fn load_named(name: &str) -> Result<Self, Error> {
                Self::from_json(&named_text(name)?).map_err(|error| Error::local(error.detail().message()))
            }
            /// Load an explicit @ reference with ordinary local-path precedence.
            /// # Errors
            /// Uses this role's existing path loader and the native named loader.
            pub fn load_reference(value: &str) -> Result<Self, Error> {
                match Reference::resolve(value)? { Reference::Path(path) => Self::load(path), Reference::Name(name) => Self::load_named(&name) }
            }
        }
    )+};
}
aggregate_loaders!(
    Recognize,
    RecognizeQuestionFile,
    Relate,
    RecordChooseQuestion
);
