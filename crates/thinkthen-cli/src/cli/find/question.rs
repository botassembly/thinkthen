//! Capture saved host settings once; native admission owns the complete find grammar.
use crate::args::{Common, FindArguments};
use crate::failure::Failure;
use crate::plan_document::Sources;
use crate::saved_json::{Json, Kind};

pub(super) struct Prepared {
    pub(super) authored_model: Option<String>,
    pub(super) common: Common,
    pub(super) sources: Option<Sources>,
}
impl Prepared {
    pub(super) fn new(arguments: &FindArguments, admitted: &mut crate::AdmittedRequest) -> Result<Self, Failure> {
        let authored = if arguments.question.starts_with('@') {
            let reference = crate::QuestionFileReference::reference(&arguments.question)?;
            let text = crate::cli::question_text::selected_text(&reference, Failure::OpenQuestionFile)?;
            let definition = reference.parse(&text, crate::QuestionFileRole::Find, |text| {
                crate::FindQuestionFile::from_json(text).map(crate::RequestDefinition::Find)
            })?;
            *admitted = admitted.clone().with_resolved_definition(definition)?;
            Some(Json::parse(&text)?)
        } else { None };
        let definition = admitted.resolve_question()?;
        match &definition {
            crate::RequestDefinition::Find(_)
            | crate::RequestDefinition::Atomic(crate::LoadedQuestion::Question(_)) => {},
            _ => return Err(Failure::Defect("find admission retained another question kind")),
        };
        let saved_fields = match authored.as_ref().and_then(|value| value.member("on")).map(Json::kind) {
            Some(Kind::String(pointer)) => vec![pointer.clone()],
            Some(Kind::Array(pointers)) => pointers.iter().filter_map(Json::as_str).map(str::to_owned).collect(),
            _ => Vec::new(),
        };
        let authored_model = authored.as_ref().and_then(|value| value.member("model"))
            .and_then(Json::as_str).map(str::to_owned);
        let mut common = arguments.common.as_common();
        let sources = authored.is_some().then(|| Sources::default().setting("question", "file")
            .setting("on", if !common.field.is_empty() { "command line" } else if !saved_fields.is_empty() { "file" } else { "default" })
            .setting("model", if common.model.is_some() { "command line" } else if authored_model.is_some() { "file" } else { "default" }));
        if common.field.is_empty() { common.field = saved_fields; }
        if common.model.is_none() { common.model.clone_from(&authored_model); }
        Ok(Self { authored_model, common, sources })
    }
}
