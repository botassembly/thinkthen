//! Find host settings come from the retained native admitted question.
use crate::args::{Common, FindArguments};
use crate::core::{Source, Sources};
use crate::failure::Failure;

pub(super) struct Prepared {
    pub(super) metadata: crate::core::declaration::QuestionMetadata,
    pub(super) common: Common,
    pub(super) sources: Option<Sources>,
}
impl Prepared {
    pub(super) fn new(
        arguments: &FindArguments,
        admitted: &mut crate::AdmittedRequest,
    ) -> Result<Self, Failure> {
        let definition = admitted.resolve_once().map_err(Failure::from)?;
        let saved_fields = match &definition {
            crate::RequestDefinition::Find(file) => file
                .reading()
                .fields()
                .iter()
                .map(|p| p.as_str().to_owned())
                .collect(),
            _ => Vec::new(),
        };
        let question = match &definition {
            crate::RequestDefinition::Find(file) => file.question(),
            crate::RequestDefinition::Atomic(crate::LoadedQuestion::Question(question)) => question,
            _ => {
                return Err(Failure::Defect(
                    "find admission retained another question kind",
                ));
            }
        };
        let crate::core::Question::Decide { .. } = &question.core else {
            return Err(Failure::Defect(
                "find admission retained another question shape",
            ));
        };
        let mut common = arguments.common.as_common();
        let fields = &saved_fields;
        let sources = arguments.question.starts_with('@').then(|| {
            Sources::for_find(
                if !common.field.is_empty() {
                    Source::CommandLine
                } else if !fields.is_empty() {
                    Source::File
                } else {
                    Source::Default
                },
                if common.model.is_some() {
                    Source::CommandLine
                } else if question.model.is_some() {
                    Source::File
                } else {
                    Source::Default
                },
            )
        });
        if common.field.is_empty() {
            common.field = fields.clone();
        }
        if common.model.is_none() {
            common.model = question.model.as_ref().map(|m| m.as_str().to_owned());
        }
        Ok(Self {
            metadata: question.metadata.clone(),
            common,
            sources,
        })
    }
}
