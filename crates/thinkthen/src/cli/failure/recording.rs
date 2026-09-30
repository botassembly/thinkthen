//! Fixed local recording diagnostics.

use super::Failure;
use crate::core::RecordError;

/// Closed, command-owned labels for a request that strict replay could not find.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ReplayContext {
    Decide,
    Filter,
    Rank,
    Choose,
    Tag,
    Score,
    Document(&'static str),
    Annotate,
    FindSet(usize),
    Recognize,
    Relate,
}

impl ReplayContext {
    fn description(self) -> String {
        match self {
            Self::Decide => "the decide request".to_owned(),
            Self::Filter => "the filter request".to_owned(),
            Self::Rank => "the rank request".to_owned(),
            Self::Choose => "the choose request".to_owned(),
            Self::Tag => "the tag request".to_owned(),
            Self::Score => "the score request".to_owned(),
            Self::Document(verb) => format!("the {verb} request for one document"),
            Self::Annotate => "the annotate request".to_owned(),
            Self::FindSet(units) => format!("the complete find set of {units} units"),
            Self::Recognize => "the recognize request".to_owned(),
            Self::Relate => "the relate request".to_owned(),
        }
    }
}

impl Failure {
    /// Add only a command-proved label to an actual replay miss. Existing stop
    /// and batch wrappers keep their one record position or full request range.
    pub(crate) fn with_replay_context(mut self, source: ReplayContext) -> Self {
        self.attach_replay_context(source);
        self
    }

    fn attach_replay_context(&mut self, source: ReplayContext) {
        match self {
            Self::QuestionMiss { context, .. } => {
                context.get_or_insert(source);
            }
            Self::Stopped { cause, .. } | Self::BatchFailed { cause, .. } => {
                cause.attach_replay_context(source);
            }
            _ => {}
        }
    }

    /// Name invalid text by its framing while preserving every other record error.
    pub(crate) fn record(error: RecordError, streamed: bool) -> Self {
        match error {
            RecordError::NotUtf8 => Self::InvalidUtf8 { record: streamed },
            other => Self::Record(other),
        }
    }
}

pub(super) fn message(failure: &Failure) -> Option<(u8, String)> {
    Some(match failure {
        Failure::QuestionMiss { key, context } => (
            5,
            format!(
                "{}the replay folder holds no answer for question `{key}`; \
                 the key is the SHA-256 of the adapter, address, model, shared state and question as sent",
                context.map(|source| format!("{}: ", source.description())).unwrap_or_default()
            ),
        ),
        Failure::StoreAmbiguous => (
            5,
            "the replay folder holds both thinkthen.jsonl and thinkthen.sqlite; \
             run `thinkthen cache convert DIR` to merge them into thinkthen.jsonl"
                .to_owned(),
        ),
        Failure::StoreHotJournal => (
            5,
            "the replay folder's thinkthen.sqlite holds a write that did not finish; \
             open the folder once with write access, as `--cache DIR` does, then replay it"
                .to_owned(),
        ),
        Failure::Entry(name, why) => (5, format!("the entry `{name}` was refused: {why}")),
        Failure::RecordingStorage => (
            5,
            "the recording folder could not be read or written; check its permissions and free space"
                .to_owned(),
        ),
        Failure::RecordingPathIsFile => (
            5,
            "the recording directory is a file; choose another path or remove the file".to_owned(),
        ),
        _ => return None,
    })
}
