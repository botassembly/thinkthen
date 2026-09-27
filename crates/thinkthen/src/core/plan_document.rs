//! The document `--plan` prints: what would be sent, and never a key.

use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

use crate::core::adapters::built_in::{self, EncodeError};
use crate::core::backend::{Backend, KEY_VAR};
use crate::core::plan::Plan;
use crate::core::question_file::Sources;
use crate::core::records::{Reading, ReadingPlan};
use crate::core::text::{ModelName, Url, Withheld};

/// What the command would send, in the four fields `channels.md` fixes.
///
/// `key_env` names the variable a key would be read from, so a script can
/// prove that a key stays home. The document carries the request body, and the
/// request body carries the evidence.
#[derive(Serialize)]
pub(crate) struct PlanDocument<'a> {
    url: &'a Url,
    model: &'a ModelName,
    key_env: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<ReadingPlan<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<Sources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<QuestionPointers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_requests: Option<Vec<usize>>,
    request: Box<RawValue>,
}

/// The request carries the evidence and any labels a record gave, so `Debug`
/// shows its length in their place.
impl std::fmt::Debug for PlanDocument<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PlanDocument")
            .field("url", &self.url)
            .field("model", &self.model)
            .field("key_env", &self.key_env)
            .field("input", &self.input)
            .field("from", &self.from)
            .field("on", &self.on)
            .field("request_count", &self.request_count)
            .field("group_requests", &self.group_requests)
            .field("request", &Withheld(self.request.get().len()))
            .finish()
    }
}

impl<'a> PlanDocument<'a> {
    /// Show the request this backend would send.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError`] when the request cannot be written as JSON.
    pub(crate) fn of(backend: &'a Backend, plan: &Plan) -> Result<Self, EncodeError> {
        Ok(Self {
            url: backend.url(),
            model: backend.model(),
            key_env: KEY_VAR,
            input: None,
            from: None,
            on: None,
            request_count: None,
            group_requests: None,
            request: built_in::encode_raw(plan)?,
        })
    }

    /// Name the framing and the pointers, as a record-mode plan does.
    ///
    /// A plan over one document carries four fields. A plan over a record
    /// stream carries this fifth one, so a reader sees which record the
    /// request was built from.
    #[must_use]
    pub(crate) fn reading(mut self, reading: &'a Reading) -> Self {
        self.input = Some(reading.plan());
        self
    }

    /// Name the source of every setting, as a plan over a question file does.
    ///
    /// A run that named no question file carries no `from` object, because
    /// every setting came from the command line or from the default and the
    /// user is looking at the command line already.
    #[must_use]
    pub(crate) const fn from(mut self, sources: Sources) -> Self {
        self.from = Some(sources);
        self
    }

    /// Show the normalized evidence pointers for every question in a set.
    #[must_use]
    pub(crate) fn questions_on(
        mut self,
        pointers: Vec<(String, Vec<crate::core::Pointer>)>,
    ) -> Self {
        self.on = Some(QuestionPointers(
            pointers
                .into_iter()
                .map(|(name, pointers)| {
                    let shown = if pointers.is_empty() {
                        vec![String::new()]
                    } else {
                        pointers
                            .into_iter()
                            .map(|pointer| pointer.as_str().to_owned())
                            .collect()
                    };
                    (name, shown)
                })
                .collect(),
        ));
        self
    }

    /// Count the requests each `on` group makes, in group order, and their sum.
    #[must_use]
    pub(crate) fn requests(mut self, groups: Vec<usize>) -> Self {
        self.request_count = Some(groups.iter().sum());
        self.group_requests = Some(groups);
        self
    }
}

#[derive(Debug)]
struct QuestionPointers(Vec<(String, Vec<String>)>);

impl Serialize for QuestionPointers {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, pointers)| (name, pointers)))
    }
}

#[cfg(test)]
mod tests {
    use super::PlanDocument;
    use crate::core::adapters::built_in::DEFAULT_MODEL;
    use crate::core::backend::Backend;
    use crate::core::plan::Plan;
    use crate::core::question::Question;
    use crate::core::render::json_line;
    use crate::core::text::{Evidence, ModelName, QuestionText};

    fn plan() -> Plan {
        Plan::new(
            Evidence::new("Help!").expect("not blank"),
            ModelName::new(DEFAULT_MODEL).expect("not blank"),
            vec![Question::Decide {
                text: QuestionText::new("is urgent").expect("not blank"),
                yes: None,
                no: None,
            }],
        )
        .expect("a plan of one question")
    }

    #[test]
    fn a_plan_document_carries_four_fields_in_the_order_the_specification_fixes() {
        let backend =
            Backend::resolve(None, None, DEFAULT_MODEL).expect("the default base resolves");
        let document = PlanDocument::of(&backend, &plan()).expect("the plan encodes");

        assert_eq!(
            json_line(&document).expect("a plan document serializes"),
            format!(
                r#"{{"url":"https://api.typesafe.ai/v1/systemone","model":"{DEFAULT_MODEL}","key_env":"THINKTHEN_API_KEY","request":{{"state":"Help!","model":"{DEFAULT_MODEL}","questions":{{"q1":{{"type":"noul","instructions":"is urgent"}}}}}}}}"#
            )
        );
    }

    #[test]
    fn a_plan_names_the_key_variable_wherever_the_request_would_go() {
        let backend = Backend::resolve(Some("http://127.0.0.1:1/v1"), None, "local-1")
            .expect("a named address resolves");
        let rendered = json_line(&PlanDocument::of(&backend, &plan()).expect("the plan encodes"))
            .expect("a plan document serializes");

        assert!(
            rendered.starts_with(
                r#"{"url":"http://127.0.0.1:1/v1/systemone","model":"local-1","key_env":"THINKTHEN_API_KEY","#
            ),
            "{rendered}"
        );
    }
}
