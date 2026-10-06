//! Explicit pure adapter dispatch; one selected API type per engine.
pub(crate) mod openai;
pub(crate) mod systemone;
use built_in::{DecodeError, EncodeError};
pub(crate) use systemone as built_in;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ApiType {
    #[default]
    Primary,
    Decisions,
}
pub(crate) fn built_ins() -> impl Iterator<Item = (&'static systemone::backends::BuiltIn, ApiType)>
{
    systemone::backends::BUILT_INS
        .iter()
        .map(|entry| (entry, ApiType::Primary))
        .chain(std::iter::once((&openai::BUILT_IN, ApiType::Decisions)))
}
impl ApiType {
    pub(crate) fn reported_model(self, body: &[u8]) -> Option<crate::core::ModelName> {
        match self {
            Self::Primary => None,
            Self::Decisions => openai::reported_model(body),
        }
    }
    pub(crate) fn is_mutable_alias(self, model: &crate::core::ModelName) -> bool {
        self == Self::Primary && systemone::is_mutable_alias(model)
    }
    pub(crate) fn split(
        self,
        questions: &[crate::core::Question],
        body: &[u8],
    ) -> Result<crate::core::pack::Split, (DecodeError, Option<crate::core::ReportedUsage>)> {
        match self {
            Self::Primary => crate::core::pack::split(questions, body),
            Self::Decisions => openai::split(questions, body),
        }
    }
    pub(crate) fn recorded_type(request: &[u8]) -> Option<Self> {
        let text = std::str::from_utf8(request).ok()?;
        let json = crate::core::Json::parse(text).ok()?;
        match json.member("questions")? {
            crate::core::Json::Array(_) => Some(Self::Decisions),
            crate::core::Json::Object(_) => Some(Self::Primary),
            _ => None,
        }
    }
    pub(crate) fn validate_exchange(
        self,
        request: &[u8],
        response: &[u8],
    ) -> Result<(), DecodeError> {
        match self {
            Self::Primary => Ok(()),
            Self::Decisions => openai::validate_exchange(request, response),
        }
    }
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Primary => systemone::NAME,
            Self::Decisions => openai::NAME,
        }
    }
    pub(crate) fn from_name(name: &str) -> Option<Self> {
        [Self::Primary, Self::Decisions]
            .into_iter()
            .find(|api| api.name() == name)
    }
    pub(crate) fn parts(
        self,
        plan: &crate::core::Plan,
    ) -> Result<systemone::request::Parts, EncodeError> {
        match self {
            Self::Primary => systemone::parts(plan),
            Self::Decisions => openai::parts(plan),
        }
    }
    pub(crate) fn join<'a>(
        self,
        state: &str,
        model: &str,
        questions: impl Iterator<Item = &'a str>,
    ) -> Vec<u8> {
        match self {
            Self::Primary => systemone::join(state, model, questions),
            Self::Decisions => openai::join(state, model, questions),
        }
    }
    pub(crate) fn encode(self, plan: &crate::core::Plan) -> Result<Vec<u8>, EncodeError> {
        match self {
            Self::Primary => systemone::encode(plan),
            Self::Decisions => {
                let parts = self.parts(plan)?;
                Ok(self.join(
                    &parts.state,
                    &parts.model,
                    parts.questions.iter().map(String::as_str),
                ))
            }
        }
    }
    pub(crate) fn decode_observed(
        self,
        plan: &crate::core::Plan,
        body: &[u8],
    ) -> systemone::response::Decoded {
        match self {
            Self::Primary => systemone::decode_observed(plan, body),
            Self::Decisions => openai::decode_observed(plan, body),
        }
    }
    pub(crate) fn canonical_question(self, text: &str) -> Option<(String, crate::core::Question)> {
        match self {
            Self::Primary => systemone::canonical_question(text),
            Self::Decisions => openai::canonical_question(text),
        }
    }
    pub(crate) const fn request_id_header(self) -> &'static str {
        match self {
            Self::Primary => systemone::ATTEMPT_REQUEST_ID_HEADER,
            Self::Decisions => "x-request-id",
        }
    }
    pub(crate) fn input_bytes(self, state: &str) -> usize {
        match self {
            Self::Primary => state.len(),
            Self::Decisions => openai::input(state).len(),
        }
    }
    pub(crate) fn added_bytes(self, count: usize, length: usize) -> usize {
        length
            + usize::from(count > 0)
            + match self {
                Self::Primary => 4 + (count + 1).to_string().len(),
                Self::Decisions => 11 + (count + 1).to_string().len(),
            }
    }
    pub(crate) fn stored(
        self,
        question: &crate::core::Question,
        text: &str,
    ) -> Result<crate::core::Answer, DecodeError> {
        match self {
            Self::Decisions => openai::stored(question, text),
            Self::Primary => Err(DecodeError::UnexpectedAnswer),
        }
    }
}
