//! Owned native causes retained privately for safe typed projections.
//! Stays: Private typed diagnostic causes must retain native error kinds and exact command projections without parsing message text.
use std::any::Any;

pub(crate) enum Diagnostic {
    Configuration(crate::config::ConfigError),
    RequestDecode(crate::public::request::RequestDecodeCause),
    Engine(crate::engine::error::Error),
    EngineRange {
        cause: crate::engine::error::Error,
        first: usize,
        last: usize,
    },
    Context {
        initial: bool,
        kind: crate::core::LimitKind,
        limit: usize,
        actual: usize,
        profile: Option<crate::core::ProfileName>,
    },
    PartialReply {
        cause: Option<crate::core::adapters::built_in::DecodeError>,
        first: usize,
        last: usize,
    },
    Refusal(Box<dyn Any + Send + Sync>),
    RelationEntityCount(usize),
    Model(crate::core::BlankTextError),
    Pointer(&'static str, String, crate::core::PointerError),
    Batch,
    ThresholdFunction(crate::RequestFunction),
    QuestionRead(crate::QuestionFileError),
}

#[cfg(test)]
mod tests {
    #[test]
    fn hidden_native_causes_do_not_reach_error_debug_or_standard_sources() {
        let error = crate::Error::usage("a safe refusal").with_diagnostic(
            super::Diagnostic::Refusal(Box::new("private-cause-marker".to_owned())),
        );
        assert!(!format!("{error:?}").contains("private-cause-marker"));
        assert!(std::error::Error::source(&error).is_none());
    }
}
