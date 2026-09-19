//! What local policy made of an answer: accepted, unsure, or unassessed.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::answer::Answer;
use crate::pass_mark::PassMark;
use crate::policy::Policy;

/// How far local policy got with the answer.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentStatus {
    /// The answer reached the pass mark, and `value` says which way.
    Accepted,
    /// The answer reached the pass mark in neither direction.
    Unsure,
    /// The user named no pass mark, so nothing was accepted.
    Unassessed,
}

/// Why three fields do not make an assessment.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("an assessment status must agree with its value and its pass mark")]
pub struct AssessmentShapeError;

/// What local policy made of the answer.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "Fields")]
pub struct Assessment {
    status: AssessmentStatus,
    value: Option<bool>,
    min_prob: Option<PassMark>,
}

/// The three fields as a document offers them, before the shape is checked.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fields {
    status: AssessmentStatus,
    value: Option<bool>,
    min_prob: Option<PassMark>,
}

/// Say whether one status agrees with the value and the mark beside it.
///
/// A yes and a no both carry a value and the mark they passed. An unsure
/// carries the mark it missed. An unassessed carries neither.
const fn consistent(
    status: AssessmentStatus,
    value: Option<bool>,
    min_prob: Option<PassMark>,
) -> bool {
    matches!(
        (status, value, min_prob),
        (AssessmentStatus::Accepted, Some(_), Some(_))
            | (AssessmentStatus::Unsure, None, Some(_))
            | (AssessmentStatus::Unassessed, None, None)
    )
}

impl TryFrom<Fields> for Assessment {
    type Error = AssessmentShapeError;

    fn try_from(fields: Fields) -> Result<Self, AssessmentShapeError> {
        if !consistent(fields.status, fields.value, fields.min_prob) {
            return Err(AssessmentShapeError);
        }
        Ok(Self {
            status: fields.status,
            value: fields.value,
            min_prob: fields.min_prob,
        })
    }
}

impl Assessment {
    /// Read the status back.
    #[must_use]
    pub const fn status(&self) -> AssessmentStatus {
        self.status
    }

    /// Read the accepted answer back, or `None` when nothing was accepted.
    #[must_use]
    pub const fn value(&self) -> Option<bool> {
        self.value
    }

    /// Read the pass mark back, or `None` when the user named none.
    #[must_use]
    pub const fn min_prob(&self) -> Option<PassMark> {
        self.min_prob
    }
}

/// Judge one answer under one policy.
///
/// The answer is yes when the probability is at or above the mark, no when one
/// minus the probability is at or above the mark, and unsure otherwise. A
/// probability exactly on the mark is accepted, so a mark of 0.9 takes 0.9 as a
/// yes and 0.1 as a no.
#[must_use]
pub fn assess(answer: Answer, policy: Policy) -> Assessment {
    let Some(mark) = policy.min_prob() else {
        return Assessment {
            status: AssessmentStatus::Unassessed,
            value: None,
            min_prob: None,
        };
    };
    let probability = answer.probability().as_f64();
    let value = if probability >= mark.as_f64() {
        Some(true)
    } else if 1.0 - probability >= mark.as_f64() {
        Some(false)
    } else {
        None
    };
    Assessment {
        status: if value.is_some() {
            AssessmentStatus::Accepted
        } else {
            AssessmentStatus::Unsure
        },
        value,
        min_prob: Some(mark),
    }
}

#[cfg(test)]
mod tests {
    use super::{Assessment, AssessmentStatus, assess, consistent};
    use crate::answer::Answer;
    use crate::pass_mark::PassMark;
    use crate::policy::Policy;
    use crate::probability::Probability;
    use proptest::strategy::Strategy;
    use proptest::{prop_assert, prop_assert_eq, proptest};

    fn mark(value: f64) -> PassMark {
        PassMark::new(value).expect("a pass mark")
    }

    fn answer(value: f64) -> Answer {
        Answer::new_yes_no(Probability::new(value).expect("a probability"))
    }

    fn judged(probability: f64, policy: Policy) -> (AssessmentStatus, Option<bool>) {
        let assessment = assess(answer(probability), policy);
        (assessment.status(), assessment.value())
    }

    #[test]
    fn the_pass_mark_rules_follow_the_specification() {
        let nine = Policy::Symmetric(mark(0.9));
        let one = Policy::Symmetric(mark(1.0));
        let cases = [
            (1.0, nine, AssessmentStatus::Accepted, Some(true)),
            (0.92, nine, AssessmentStatus::Accepted, Some(true)),
            (0.9, nine, AssessmentStatus::Accepted, Some(true)),
            (0.899, nine, AssessmentStatus::Unsure, None),
            (0.5, nine, AssessmentStatus::Unsure, None),
            (0.101, nine, AssessmentStatus::Unsure, None),
            (0.1, nine, AssessmentStatus::Accepted, Some(false)),
            (0.08, nine, AssessmentStatus::Accepted, Some(false)),
            (0.0, nine, AssessmentStatus::Accepted, Some(false)),
            (1.0, one, AssessmentStatus::Accepted, Some(true)),
            (0.999, one, AssessmentStatus::Unsure, None),
            (0.5, one, AssessmentStatus::Unsure, None),
            (0.0, one, AssessmentStatus::Accepted, Some(false)),
            (0.92, Policy::Unassessed, AssessmentStatus::Unassessed, None),
            (0.0, Policy::Unassessed, AssessmentStatus::Unassessed, None),
        ];
        for (probability, policy, status, value) in cases {
            assert_eq!(
                judged(probability, policy),
                (status, value),
                "probability {probability} under {policy:?}"
            );
        }
    }

    #[test]
    fn an_assessment_reports_the_mark_its_policy_carried() {
        let nine = mark(0.9);
        let accepted = assess(answer(0.92), Policy::Symmetric(nine));
        assert_eq!(accepted.min_prob(), Some(nine));
        let unassessed = assess(answer(0.92), Policy::Unassessed);
        assert_eq!(unassessed.min_prob(), None);
    }

    #[test]
    fn a_document_whose_status_contradicts_its_fields_is_refused() {
        let refused = [
            r#"{"status":"accepted","value":null,"min_prob":0.9}"#,
            r#"{"status":"accepted","value":true,"min_prob":null}"#,
            r#"{"status":"unsure","value":true,"min_prob":0.9}"#,
            r#"{"status":"unsure","value":null,"min_prob":null}"#,
            r#"{"status":"unassessed","value":false,"min_prob":null}"#,
            r#"{"status":"unassessed","value":null,"min_prob":0.9}"#,
        ];
        for document in refused {
            let parsed = serde_json::from_str::<Assessment>(document);
            assert!(parsed.is_err(), "{document}");
        }
        let accepted = r#"{"status":"accepted","value":true,"min_prob":0.9}"#;
        let parsed: Assessment = serde_json::from_str(accepted).expect("a shaped assessment");
        assert_eq!(parsed.status(), AssessmentStatus::Accepted);
    }

    fn probabilities() -> impl Strategy<Value = f64> {
        0.0_f64..=1.0
    }

    fn marks() -> impl Strategy<Value = f64> {
        (0.5_f64..=1.0).prop_filter("a pass mark is above one half", |value| *value > 0.5)
    }

    /// One status, and the fields that status allows. Nothing else is legal.
    fn shaped(assessment: &Assessment) -> bool {
        consistent(
            assessment.status(),
            assessment.value(),
            assessment.min_prob(),
        )
    }

    proptest! {
        #[test]
        fn every_probability_reaches_exactly_one_status(probability in probabilities(), threshold in marks()) {
            let assessed = assess(answer(probability), Policy::Symmetric(mark(threshold)));
            prop_assert!(shaped(&assessed), "{assessed:?}");
            let unassessed = assess(answer(probability), Policy::Unassessed);
            prop_assert!(shaped(&unassessed), "{unassessed:?}");
            prop_assert_eq!(unassessed.status(), AssessmentStatus::Unassessed);
        }

        #[test]
        fn an_accepted_yes_and_an_accepted_no_never_meet(probability in probabilities(), threshold in marks()) {
            let assessed = assess(answer(probability), Policy::Symmetric(mark(threshold)));
            let yes = probability >= threshold;
            let no = 1.0 - probability >= threshold;
            prop_assert!(!(yes && no));
            prop_assert_eq!(assessed.value() == Some(true), yes);
            prop_assert_eq!(assessed.value() == Some(false), no);
        }

        #[test]
        fn mirroring_the_probability_flips_the_value(probability in probabilities(), threshold in marks()) {
            let policy = Policy::Symmetric(mark(threshold));
            let straight = assess(answer(probability), policy);
            let mirrored = assess(answer(1.0 - probability), policy);
            prop_assert_eq!(mirrored.status(), straight.status());
            prop_assert_eq!(mirrored.value(), straight.value().map(|value| !value));
        }
    }
}
