//! Ordinary find file preparation shares the capped reader and native field validators.
use crate::args::{Common, FindArguments};
use crate::core::{ProfileName, QuestionFile, QuestionText, Source, Sources};
use crate::failure::Failure;
use std::path::Path;

pub(super) struct Prepared {
    pub(super) common: Common,
    pub(super) question: QuestionText,
    pub(super) profile: Option<ProfileName>,
    pub(super) sources: Option<Sources>,
}
impl Prepared {
    pub(super) fn new(arguments: &FindArguments) -> Result<Self, Failure> {
        let mut common = arguments.common.as_common();
        common.check_plan_name()?;
        let Some(path) = arguments.question.strip_prefix('@') else {
            let question = QuestionText::new(&arguments.question).map_err(|_| {
                Failure::Usage("`find` takes a question that is text, not white space")
            })?;
            return Ok(Self {
                common,
                question,
                profile: None,
                sources: None,
            });
        };
        let text =
            crate::cli::question_text::reference(Path::new(path), Failure::OpenQuestionFile)?;
        let file = QuestionFile::parse_find(&text)?;
        let sources = Sources::for_find(
            if !common.field.is_empty() {
                Source::CommandLine
            } else if !file.on.is_empty() {
                Source::File
            } else {
                Source::Default
            },
            if common.model.is_some() {
                Source::CommandLine
            } else if file.model.is_some() {
                Source::File
            } else {
                Source::Default
            },
        );
        if common.field.is_empty() {
            common.field = file
                .on
                .iter()
                .map(|pointer| pointer.as_str().to_owned())
                .collect();
        }
        if common.model.is_none() {
            common.model = file.model.map(|model| model.as_str().to_owned());
        }
        Ok(Self {
            common,
            question: file.text,
            profile: file.profile,
            sources: Some(sources),
        })
    }
}
