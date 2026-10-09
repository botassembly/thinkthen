//! Owned native causes for host diagnostics, absent from every public view.
use std::any::Any;

pub(crate) enum Diagnostic {
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
    #[cfg(feature = "cli")]
    CliInput(Box<crate::cli::failure::Failure>),
    Refusal(Box<dyn Any + Send + Sync>),
    Model(crate::core::BlankTextError),
    Pointer(&'static str, String, crate::core::PointerError),
    Batch,
    ThresholdFunction(crate::RequestFunction),
    QuestionRead(crate::QuestionFileError),
}

#[cfg(all(test, feature = "cli"))]
mod tests {
    use crate::{Error, ErrorKind};

    #[test]
    fn native_backend_causes_keep_cli_status_and_sentence() {
        let error = Error::from(crate::engine::error::Error::Status(302));
        assert_eq!(error.kind(), ErrorKind::Backend);
        let failure = crate::cli::failure::Failure::from(error);
        assert!(matches!(failure, crate::cli::failure::Failure::Status(302)));
        let mut output = Vec::new();
        assert_eq!(
            crate::cli::failure::report(&failure, &mut output),
            std::process::ExitCode::from(4)
        );
        assert!(
            String::from_utf8(output)
                .expect("diagnostic UTF-8")
                .contains("302")
        );
    }

    #[test]
    fn hidden_causes_do_not_reach_debug_or_error_sources() {
        let error = Error::usage("a safe refusal").with_diagnostic(super::Diagnostic::Refusal(
            Box::new("private-cause-marker".to_owned()),
        ));
        assert!(!format!("{error:?}").contains("private-cause-marker"));
        assert!(std::error::Error::source(&error).is_none());
        let mut output = Vec::new();
        assert_eq!(
            crate::cli::failure::report(&Error::cancelled().into(), &mut output),
            std::process::ExitCode::from(130)
        );
        assert!(output.is_empty());
    }
}
