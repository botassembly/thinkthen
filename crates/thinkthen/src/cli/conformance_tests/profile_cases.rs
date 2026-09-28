use super::{asked, conformance_support::Document};
use crate::core::{Backend, BackendProfile, DEFAULT_MODEL, ProfileName, ProfileWarning};
use crate::engine::error::Error;
use crate::engine::facade;
use serde::Deserialize;
use serde_json::value::RawValue;

const CASES: &str = include_str!("../../../../../conformance/cases.json");
const PROFILES: &str = include_str!("../../../../../conformance/backend-profiles.json");

#[derive(Deserialize)]
struct ProfileDocument {
    schema: String,
    cases: Vec<ProfileCase>,
    mismatches: Vec<MismatchCase>,
}

#[derive(Deserialize)]
struct ProfileCase {
    id: String,
    source_case: String,
    exchange: usize,
    profile: Box<RawValue>,
    expect: Expectation,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Expectation {
    Pass(String),
    Limit {
        kind: String,
        limit: usize,
        actual: usize,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MismatchCase {
    id: String,
    tuned_for: Option<String>,
    running: Option<String>,
    warning: bool,
}

#[test]
fn shared_profile_cases_cross_the_facade_preparation() {
    let source: Document = serde_json::from_str(CASES).expect("main cases");
    let profiles: ProfileDocument = serde_json::from_str(PROFILES).expect("profile cases");
    assert_eq!(profiles.schema, "thinkthen.backend-profile-conformance/1");
    let backend = Backend::resolve(None, None, DEFAULT_MODEL).expect("backend");
    for profile_case in profiles.cases {
        let case = source
            .cases
            .iter()
            .find(|case| case.id == profile_case.source_case)
            .expect("source case");
        let exchange = case.exchanges.get(profile_case.exchange).expect("exchange");
        let plan = asked(case, profile_case.exchange, exchange)
            .expect("production question grammar")
            .plan;
        let profile = BackendProfile::parse(profile_case.profile.get()).expect("profile parser");
        let result = facade::split(&backend, Some(&profile), &plan);
        match profile_case.expect {
            Expectation::Pass(word) => {
                assert_eq!(word, "pass", "{}", profile_case.id);
                assert!(
                    result.is_ok(),
                    "{}: {:?}",
                    profile_case.id,
                    result.as_ref().err()
                );
            }
            Expectation::Limit {
                kind,
                limit,
                actual,
            } => {
                let Err(Error::ProfileLimit(found)) = result else {
                    panic!("{} returned another error", profile_case.id);
                };
                assert_eq!(found.kind.words(), kind, "{}", profile_case.id);
                assert_eq!(
                    (found.limit, found.actual),
                    (limit, actual),
                    "{}",
                    profile_case.id
                );
            }
        }
    }

    for case in profiles.mismatches {
        let tuned_for = case
            .tuned_for
            .as_deref()
            .map(ProfileName::new)
            .transpose()
            .expect("tuned_for name");
        let running = case
            .running
            .as_deref()
            .map(ProfileName::new)
            .transpose()
            .expect("running name");
        assert_eq!(
            ProfileWarning::between(tuned_for.as_ref(), running.as_ref()).is_some(),
            case.warning,
            "{}",
            case.id
        );
    }
}

#[test]
fn a_mismatch_case_with_the_old_calibrated_key_is_refused() {
    let old = r#"{"id":"old","calibrated":"jev","running":"jev","warning":false}"#;
    assert!(serde_json::from_str::<MismatchCase>(old).is_err());
}
