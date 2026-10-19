//! Command presentation of exact native prepared request bytes.
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use thinkthen::{Error, PlanEstimate, RequestFraming, ResolvedSettings};

/// The command's ordered setting-source labels, independent of native configuration.
#[derive(Clone, Debug, Default)]
pub(crate) struct Sources(Vec<(String, String)>);
impl Sources {
    pub(crate) fn setting(mut self, name: &str, source: &str) -> Self {
        let source = match source {
            "built_in" => "default",
            "command_line" => "command line",
            source => source,
        };
        if let Some((_, held)) = self.0.iter_mut().find(|(held, _)| held == name) {
            *held = source.to_owned();
        } else { self.0.push((name.to_owned(), source.to_owned())); }
        self
    }
}
impl Serialize for Sources {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, source)| (name, source)))
    }
}

#[derive(Clone, Debug, Serialize)]
struct ReadingPlan {
    framing: RequestFraming,
    field: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<&'static str>,
}

/// The document carries explicit evidence-bearing bytes; Debug withholds them.
#[derive(Serialize)]
pub(crate) struct PlanDocument<'a> {
    url: &'a str,
    model: &'a str,
    key_env: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<ReadingPlan>,
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
impl std::fmt::Debug for PlanDocument<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("PlanDocument")
            .field("request_bytes", &self.request.get().len()).finish_non_exhaustive()
    }
}
impl<'a> PlanDocument<'a> {
    pub(crate) fn request_body(&self) -> &[u8] { self.request.get().as_bytes() }

    /// Disclose the first actual native request without re-encoding it.
    pub(crate) fn of(settings: &'a ResolvedSettings, plan: &PlanEstimate) -> Result<Self, Error> {
        let body = plan.prepared_requests().next()
            .ok_or_else(|| Error::new(thinkthen::ErrorKind::Defect, "a displayed plan has no prepared request"))?;
        Self::of_body(settings, body.body.to_vec())
    }

    /// Retain exactly the body closed by native admission and packing.
    pub(crate) fn of_body(settings: &'a ResolvedSettings, body: Vec<u8>) -> Result<Self, Error> {
        let refused = || Error::new(thinkthen::ErrorKind::Defect, "a native prepared request is not JSON text");
        let text = String::from_utf8(body).map_err(|_| refused())?;
        let request = RawValue::from_string(text).map_err(|_| refused())?;
        Ok(Self {
            url: &settings.url, model: &settings.model, key_env: &settings.key_variable,
            input: None, from: None, on: None, request_count: None,
            group_requests: None, request,
        })
    }
    #[must_use]
    pub(crate) const fn key_env(mut self, key_env: &'a str) -> Self {
        self.key_env = key_env;
        self
    }
    #[must_use]
    pub(crate) fn reading(mut self, framing: RequestFraming, fields: Vec<String>, default: bool) -> Self {
        self.input = Some(ReadingPlan { framing, field: fields, from: default.then_some("default") });
        self
    }
    #[must_use]
    pub(crate) fn from(mut self, sources: Sources) -> Self {
        self.from = Some(sources);
        self
    }
    #[must_use]
    pub(crate) fn questions_on(mut self, pointers: Vec<(String, Vec<String>)>) -> Self {
        self.on = Some(QuestionPointers(pointers.into_iter().map(|(name, pointers)| {
            let shown = if pointers.is_empty() { vec![String::new()] } else { pointers };
            (name, shown)
        }).collect()));
        self
    }
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
