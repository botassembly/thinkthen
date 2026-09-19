//! The document `--plan` prints: what would be sent, and never a key.

use serde::Serialize;
use serde_json::value::RawValue;

use crate::adapter::Adapter;
use crate::backend::Backend;
use crate::text::{BackendName, KeyVar, ModelName, Url};

/// What the command would send, in the six fields `channels.md` fixes.
///
/// `backend` is `null` for an ad-hoc backend, and `key_env` is `null` when no
/// key would be sent, so a script can prove that a key stays home. The document
/// carries the request body, and the request body carries the evidence.
#[derive(Debug, Serialize)]
pub struct PlanDocument<'a> {
    backend: Option<&'a BackendName>,
    url: &'a Url,
    adapter: Adapter,
    model: &'a ModelName,
    key_env: Option<&'a KeyVar>,
    request: &'a RawValue,
}

impl<'a> PlanDocument<'a> {
    /// Show the request one backend would be sent.
    #[must_use]
    pub fn new(backend: &'a Backend, request: &'a RawValue) -> Self {
        Self {
            backend: backend.name(),
            url: backend.url(),
            adapter: backend.adapter(),
            model: backend.model(),
            key_env: backend.key_env(),
            request,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlanDocument;
    use crate::backend::{BackendValues, resolve_backend};
    use crate::plan::Plan;
    use crate::question::Question;
    use crate::systemone::encode_raw;
    use crate::text::{Condition, Evidence, ModelName};

    fn plan() -> Plan {
        Plan::new(
            Evidence::new("Help!").expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            vec![Question::new_if(
                Condition::new("is urgent").expect("not blank"),
            )],
        )
        .expect("a plan of one question")
    }

    #[test]
    fn a_plan_document_carries_six_fields_in_the_order_the_specification_fixes() {
        let backend = resolve_backend(BackendValues::default(), BackendValues::default())
            .expect("the built-in profile resolves");
        let request = encode_raw(&plan()).expect("the plan encodes");
        let document = PlanDocument::new(&backend, &request);

        let rendered = serde_json::to_string(&document).expect("a plan document serializes");
        assert_eq!(
            rendered,
            concat!(
                r#"{"backend":"jev","url":"https://api.typesafe.ai/v1/systemone","#,
                r#""adapter":"systemone","model":"jev-latest","key_env":"TYPESAFE_API_KEY","#,
                r#""request":{"state":"Help!","model":"jev-latest","#,
                r#""questions":{"q1":{"type":"noul","instructions":"is urgent"}}}}"#,
            )
        );
    }

    #[test]
    fn an_ad_hoc_backend_prints_a_null_name_and_a_null_key_variable() {
        let values = BackendValues::new(
            None,
            Some("http://127.0.0.1:1/v1"),
            Some("systemone"),
            Some("local-1"),
            None,
        );
        let backend =
            resolve_backend(values, BackendValues::default()).expect("an ad-hoc backend resolves");
        let request = encode_raw(&plan()).expect("the plan encodes");

        let rendered = serde_json::to_string(&PlanDocument::new(&backend, &request))
            .expect("a plan document serializes");
        assert!(rendered.starts_with(r#"{"backend":null,"#), "{rendered}");
        assert!(rendered.contains(r#""key_env":null,"#), "{rendered}");
    }
}
