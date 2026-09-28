//! A named backend's locally enforceable request limits.

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::core::json::{Json, JsonError};
use crate::core::plan::Plan;

/// A public backend name used for limits and threshold calibration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProfileName(String);

impl Serialize for ProfileName {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl ProfileName {
    /// Read a safe profile name.
    pub(crate) fn new(value: &str) -> Result<Self, ProfileError> {
        if value.is_empty()
            || !value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
            })
        {
            return Err(ProfileError::Name);
        }
        Ok(Self(value.to_owned()))
    }

    /// Read the validated name.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

/// One explicit profile and the limits it can enforce without a backend call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BackendProfile {
    name: ProfileName,
    pub(crate) max_evidence_bytes: Option<usize>,
    pub(crate) max_request_bytes: Option<usize>,
    pub(crate) max_questions: Option<usize>,
    max_options: Option<usize>,
}

impl BackendProfile {
    /// Parse the closed version-one profile object.
    pub(crate) fn parse(text: &str) -> Result<Self, ProfileError> {
        let value = Json::parse(text).map_err(ProfileError::Json)?;
        let Json::Object(members) = &value else {
            return Err(ProfileError::Object);
        };
        for (name, _) in members {
            if ![
                "schema",
                "name",
                "max_evidence_bytes",
                "max_request_bytes",
                "max_questions",
                "max_options",
            ]
            .contains(&name.as_str())
            {
                return Err(ProfileError::UnknownKey);
            }
        }
        if value.member("schema").and_then(Json::as_str) != Some("thinkthen.backend-profile/1") {
            return Err(ProfileError::Schema);
        }
        let name = value
            .member("name")
            .and_then(Json::as_str)
            .ok_or(ProfileError::Name)
            .and_then(ProfileName::new)?;
        let max_evidence_bytes = limit(&value, "max_evidence_bytes")?;
        let max_request_bytes = limit(&value, "max_request_bytes")?;
        let max_questions = limit(&value, "max_questions")?;
        let max_options = limit(&value, "max_options")?;
        if max_evidence_bytes.is_none()
            && max_request_bytes.is_none()
            && max_questions.is_none()
            && max_options.is_none()
        {
            return Err(ProfileError::NoLimit);
        }
        Ok(Self {
            name,
            max_evidence_bytes,
            max_request_bytes,
            max_questions,
            max_options,
        })
    }

    /// Read the stable name.
    pub(crate) const fn name(&self) -> &ProfileName {
        &self.name
    }

    /// Check the exact production values behind one encoded request: the
    /// evidence in its compact text form, and the body as encoded.
    pub(crate) fn check(
        &self,
        plan: &Plan,
        evidence: &str,
        body: &[u8],
    ) -> Result<(), ProfileLimit> {
        check_limit(
            &self.name,
            LimitKind::EvidenceBytes,
            self.max_evidence_bytes,
            evidence.len(),
        )?;
        check_limit(
            &self.name,
            LimitKind::Options,
            self.max_options,
            plan.questions()
                .iter()
                .filter_map(|question| match question {
                    crate::core::Question::Choose { options, .. } => Some(options.count()),
                    _ => None,
                })
                .max()
                .unwrap_or(0),
        )?;
        check_limit(
            &self.name,
            LimitKind::RequestBytes,
            self.max_request_bytes,
            body.len(),
        )?;
        check_limit(
            &self.name,
            LimitKind::Questions,
            self.max_questions,
            plan.wire_question_count(),
        )
    }
}

fn limit(value: &Json, key: &'static str) -> Result<Option<usize>, ProfileError> {
    let Some(Json::Number(number)) = value.member(key) else {
        return if value.member(key).is_none() {
            Ok(None)
        } else {
            Err(ProfileError::Limit(key))
        };
    };
    number
        .as_u64()
        .filter(|number| *number > 0)
        .and_then(|number| usize::try_from(number).ok())
        .map(Some)
        .ok_or(ProfileError::Limit(key))
}

fn check_limit(
    name: &ProfileName,
    kind: LimitKind,
    limit: Option<usize>,
    actual: usize,
) -> Result<(), ProfileLimit> {
    match limit {
        Some(limit) if actual > limit => Err(ProfileLimit {
            name: name.clone(),
            kind,
            limit,
            actual,
        }),
        _ => Ok(()),
    }
}

/// Why a profile file is not the closed public shape.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum ProfileError {
    /// The file is not valid JSON.
    #[error("is not valid JSON")]
    Json(JsonError),
    /// The top level is not an object.
    #[error("is one JSON object")]
    Object,
    /// The schema is absent or wrong.
    #[error("has schema `thinkthen.backend-profile/1`")]
    Schema,
    /// The name is absent or unsafe.
    #[error("has a nonempty lowercase name using letters, digits, hyphens, or underscores")]
    Name,
    /// An unknown member is present.
    #[error("holds no unknown keys")]
    UnknownKey,
    /// One limit is not a positive integer.
    #[error("has a positive integer `{0}`")]
    Limit(&'static str),
    /// No enforceable limit was supplied.
    #[error("names at least one limit")]
    NoLimit,
}

/// Which profile limit refused a request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LimitKind {
    EvidenceBytes,
    RequestBytes,
    Questions,
    Options,
}

impl LimitKind {
    pub(crate) const fn words(self) -> &'static str {
        match self {
            Self::EvidenceBytes => "evidence bytes",
            Self::RequestBytes => "request bytes",
            Self::Questions => "questions",
            Self::Options => "options",
        }
    }
}

/// The safe counts behind one local profile refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProfileLimit {
    pub(crate) name: ProfileName,
    pub(crate) kind: LimitKind,
    pub(crate) limit: usize,
    pub(crate) actual: usize,
}

#[cfg(test)]
mod tests {
    use super::{BackendProfile, LimitKind, ProfileError};
    use crate::core::json::JsonError;
    use crate::core::{Evidence, ModelName, Plan, Question, QuestionText};

    fn plan(evidence: &str) -> Plan {
        Plan::new(
            Evidence::new(evidence).expect("evidence"),
            ModelName::new("local-1").expect("model"),
            vec![Question::Decide {
                text: QuestionText::new("Is this relevant?").expect("question"),
                yes: None,
                no: None,
            }],
        )
        .expect("plan")
    }

    #[test]
    fn the_closed_profile_shape_accepts_independent_positive_limits() {
        let parsed = BackendProfile::parse(
            r#"{"schema":"thinkthen.backend-profile/1","name":"local_1","max_evidence_bytes":4,"max_request_bytes":200,"max_questions":1}"#,
        )
        .expect("profile");
        assert_eq!(parsed.name().as_str(), "local_1");
        let body = vec![b'x'; 200];
        assert_eq!(parsed.check(&plan("four"), "four", &body), Ok(()));
        let too_long = parsed
            .check(&plan("five!"), "five!", &body)
            .expect_err("one byte over");
        assert_eq!(too_long.kind, LimitKind::EvidenceBytes);
        assert_eq!((too_long.limit, too_long.actual), (4, 5));
    }

    #[test]
    fn malformed_and_open_profiles_are_refused_without_echoing_values() {
        assert!(matches!(
            BackendProfile::parse("not json"),
            Err(ProfileError::Json(JsonError::Syntax { .. }))
        ));
        let cases = [
            ("[]", ProfileError::Object),
            (r#"{"name":"jev","max_questions":1}"#, ProfileError::Schema),
            (
                r#"{"schema":"other","name":"jev","max_questions":1}"#,
                ProfileError::Schema,
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","max_questions":1}"#,
                ProfileError::Name,
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"Jev","max_questions":1}"#,
                ProfileError::Name,
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"jev"}"#,
                ProfileError::NoLimit,
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"jev","max_questions":0}"#,
                ProfileError::Limit("max_questions"),
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"jev","max_questions":-1}"#,
                ProfileError::Limit("max_questions"),
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"jev","max_questions":1.5}"#,
                ProfileError::Limit("max_questions"),
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"jev","max_questions":"1"}"#,
                ProfileError::Limit("max_questions"),
            ),
            (
                r#"{"schema":"thinkthen.backend-profile/1","name":"jev","max_questions":1,"url":"secret"}"#,
                ProfileError::UnknownKey,
            ),
        ];
        for (text, expected) in cases {
            assert_eq!(BackendProfile::parse(text), Err(expected));
        }
    }
}
