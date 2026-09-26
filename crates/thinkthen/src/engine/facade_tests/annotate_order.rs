//! The facade's annotate prepares every group of a record before it sends any.

use conformance_backend::Backend as Loopback;

use crate::core::{BackendProfile, Evidence, ModelName, Plan, QuestionSet};
use crate::engine::Cancel;
use crate::engine::error::{Error, Kind};
use crate::engine::facade::{Engine, Settings};

/// A later group whose part is over the profile's evidence limit sends nothing,
/// as `conformance/backend-profiles.json` case `annotate-later-group-over`
/// refuses it on the command.
#[test]
fn a_later_group_over_the_profile_sends_nothing() {
    let loopback = Loopback::start().expect("loopback");
    let profile = BackendProfile::parse(
        r#"{"schema":"thinkthen.backend-profile/1","name":"edge","max_evidence_bytes":16}"#,
    )
    .expect("profile");
    let engine = Engine::new(Settings {
        profile: Some(profile),
        ..super::settings(&format!("{}/generic/v1", loopback.origin()))
    })
    .expect("engine");
    let set = QuestionSet::parse(
        r#"{"version":1,"questions":{"summary":{"decide":"Is this concise?","on":"/summary"},"body":{"decide":"Does this ask for a refund?","on":"/body"}}}"#,
    )
    .expect("set");
    let record =
        Evidence::new(r#"{"summary":"Short note.","body":"Please refund me."}"#).expect("record");
    let plan = |places: &[usize]| {
        let questions = places
            .iter()
            .map(|place| set.questions()[*place].question().clone());
        Ok(Plan::new(
            set.group_evidence(places, &record).expect("a part"),
            ModelName::new("jev-latest").expect("model"),
            questions.collect(),
        )
        .expect("plan"))
    };
    let refused = engine.annotate(&set, plan, &Cancel::default());
    assert!(
        matches!(&refused, Err(error @ Error::ProfileLimit(_)) if error.kind() == Kind::Usage),
        "{:?}",
        refused.err()
    );
    assert_eq!(loopback.count(), 0, "the first group was sent");
}
