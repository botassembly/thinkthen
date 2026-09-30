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
    /// The exact first encoded request this document discloses.
    pub(crate) fn request_body(&self) -> &[u8] {
        self.request.get().as_bytes()
    }

    /// Show the request this backend would send.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError`] when the request cannot be written as JSON.
    pub(crate) fn of(backend: &'a Backend, plan: &Plan) -> Result<Self, EncodeError> {
        Ok(Self {
            request: built_in::encode_raw(plan)?,
            ..Self::of_nothing(backend)
        })
    }

    /// Show this exact request body, as the packer closed it.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError`] when the body is not JSON text.
    pub(crate) fn of_body(backend: &'a Backend, body: Vec<u8>) -> Result<Self, EncodeError> {
        let text = String::from_utf8(body).map_err(|error| EncodeError::of(&error))?;
        let request = RawValue::from_string(text).map_err(|error| EncodeError::of(&error))?;
        Ok(Self {
            request,
            ..Self::of_nothing(backend)
        })
    }

    fn of_nothing(backend: &'a Backend) -> Self {
        Self {
            url: backend.url(),
            model: backend.model(),
            key_env: KEY_VAR,
            input: None,
            from: None,
            on: None,
            request_count: None,
            group_requests: None,
            request: RawValue::NULL.to_owned(),
        }
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

    /// The plan's request count, and the requests each `on` group joins, in
    /// group order. Groups may share a request, so the counts need not sum.
    #[must_use]
    pub(crate) fn requests(mut self, count: usize, groups: Vec<usize>) -> Self {
        self.request_count = Some(count);
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
