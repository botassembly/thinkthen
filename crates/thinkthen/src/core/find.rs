//! Pure construction and result mapping for one bounded `find` request.

use std::borrow::Cow;

use serde::{Serialize, Serializer};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::core::digest::hex;
use crate::core::plan::Descriptions;
use crate::core::{
    Answer, Evidence, Labels, Meta, ModelName, Plan, Question, QuestionText, Record,
};

/// Why a unit set cannot become one find request or result.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum FindError {
    /// The unit count falls outside the policy's settled range.
    #[error("the unit count is outside the find range")]
    Count,
    /// The aggregate evidence could not be rendered.
    #[error("the find request could not be rendered")]
    Render,
    /// The reply is not the choice distribution this request asks for.
    #[error("the find reply is not a choice distribution")]
    Answer,
}

#[derive(Serialize)]
struct Unit<'a> {
    id: String,
    evidence: Cow<'a, str>,
}

/// One constructed find request and the stable public question it represents.
#[derive(Clone, Debug)]
pub(crate) struct Find {
    plan: Plan,
    text: QuestionText,
    none: bool,
    units: usize,
}

impl Find {
    /// Build one aggregate request from ordered evidence.
    pub(crate) fn new(
        text: QuestionText,
        evidence: &[Evidence],
        model: ModelName,
        none: bool,
    ) -> Result<Self, FindError> {
        let most = if none { 254 } else { 255 };
        if !(2..=most).contains(&evidence.len()) {
            return Err(FindError::Count);
        }
        let units: Vec<Unit> = evidence
            .iter()
            .enumerate()
            .map(|(place, evidence)| {
                evidence
                    .as_text()
                    .map(|text| Unit {
                        id: unit_id(place),
                        evidence: text,
                    })
                    .map_err(|_| FindError::Render)
            })
            .collect::<Result<_, FindError>>()?;
        let aggregate = serde_json::to_string(&units).map_err(|_| FindError::Render)?;
        let mut options: Vec<_> = (0..evidence.len()).map(unit_id).collect();
        if none {
            options.push("none".to_owned());
        }
        let question = Question::Choose {
            text: text.clone(),
            options: Labels::options(options).map_err(|_| FindError::Count)?,
        };
        // A find question carries no description, so both description forms
        // write the same bytes (ADR 0115 section 3).
        let plan = Plan::new(
            Evidence::new(&aggregate).map_err(|_| FindError::Render)?,
            model,
            Descriptions::Authored,
            vec![question],
        )
        .map_err(|_| FindError::Render)?;
        Ok(Self {
            plan,
            text,
            none,
            units: evidence.len(),
        })
    }

    /// Read the internal choice plan sent to the adapter.
    #[must_use]
    pub(crate) const fn plan(&self) -> &Plan {
        &self.plan
    }

    /// Map the returned distribution under find's stable tie policy.
    pub(crate) fn select(&self, answer: &Answer) -> Result<FindAnswer, FindError> {
        let entries = answer.choice_probabilities().ok_or(FindError::Answer)?;
        if entries.len() != self.units + usize::from(self.none) {
            return Err(FindError::Answer);
        }
        let highest = entries
            .iter()
            .map(|(_, value)| *value)
            .reduce(f64::max)
            .ok_or(FindError::Answer)?;
        let leaders: Vec<_> = entries
            .iter()
            .enumerate()
            .filter(|(_, (_, value))| *value == highest)
            .collect();
        let none_leads = leaders.iter().any(|(_, (label, _))| *label == "none");
        let selected = if none_leads {
            None
        } else {
            leaders.first().map(|(place, _)| *place)
        };
        let pick = leaders
            .first()
            .map(|(_, (label, _))| (*label).to_owned())
            .ok_or(FindError::Answer)?;
        Ok(FindAnswer {
            selected,
            pick,
            probabilities: entries
                .into_iter()
                .map(|(label, value)| (label.to_owned(), value))
                .collect(),
            confidence: answer.choice_confidence(),
        })
    }

    /// Digest the public question, excluding generated ids and unit count.
    pub(crate) fn question_sha256(&self) -> Result<String, FindError> {
        let canonical = self.canonical_question()?;
        Ok(hex(&Sha256::digest(canonical.as_bytes())))
    }

    fn canonical_question(&self) -> Result<String, FindError> {
        serde_json::to_string(&FindQuestion {
            verb: "find",
            text: &self.text,
            none: self.none,
        })
        .map_err(|_| FindError::Render)
    }

    /// Build the dedicated detailed result.
    #[must_use]
    pub(crate) fn result(
        &self,
        value: Option<Record>,
        answer: FindAnswer,
        meta: Meta,
    ) -> FindResult {
        FindResult {
            schema: "thinkthen.result/1",
            value,
            question: FindQuestionOwned {
                text: self.text.clone(),
                none: self.none,
            },
            answer,
            threshold: None,
            meta,
        }
    }
}

fn unit_id(place: usize) -> String {
    format!("u{:03}", place + 1)
}

#[derive(Serialize)]
struct FindQuestion<'a> {
    verb: &'static str,
    text: &'a QuestionText,
    none: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "findQuestion"))]
#[serde(tag = "verb", rename = "find")]
pub(super) struct FindQuestionOwned {
    text: QuestionText,
    none: bool,
}

impl FindQuestionOwned {
    pub(crate) const fn parts(&self) -> (&QuestionText, bool) {
        (&self.text, self.none)
    }
}

/// The mapped find answer and its selected zero-based unit.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "findAnswer"))]
#[serde(tag = "kind", rename = "find")]
pub(crate) struct FindAnswer {
    #[serde(skip)]
    selected: Option<usize>,
    pick: String,
    #[serde(serialize_with = "ordered")]
    #[cfg_attr(test, schemars(with = "std::collections::BTreeMap<String, f64>"))]
    probabilities: Vec<(String, f64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<f64>,
}

impl FindAnswer {
    pub(crate) fn pick(&self) -> &str {
        &self.pick
    }

    /// Confidence reported for this distribution, when supplied.
    pub(crate) const fn confidence(&self) -> Option<f64> {
        self.confidence
    }

    /// The selected unit, or none when `none` shares or owns the lead.
    #[must_use]
    pub(crate) const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Every candidate's probability in input order, `none` last when asked.
    #[must_use]
    pub(crate) fn probabilities(&self) -> &[(String, f64)] {
        &self.probabilities
    }
}

/// Write the probabilities as one object, in input order.
fn ordered<S: Serializer>(entries: &[(String, f64)], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_map(entries.iter().map(|(label, value)| (label, value)))
}

/// One dedicated detailed find result.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "findDetails"))]
pub(crate) struct FindResult {
    schema: &'static str,
    pub(super) value: Option<Record>,
    pub(super) question: FindQuestionOwned,
    pub(super) answer: FindAnswer,
    pub(super) threshold: Option<()>,
    pub(super) meta: Meta,
}

#[cfg(test)]
mod tests {
    use super::Find;
    use crate::core::adapters::built_in;
    use crate::core::answer::{Answer, Distribution};
    use crate::core::probability::Probability;
    use crate::core::{Evidence, ModelName, QuestionText};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct EncodedRequest {
        state: String,
    }

    #[derive(Deserialize)]
    struct EncodedUnit {
        evidence: String,
    }

    fn find(units: usize, none: bool) -> Find {
        let evidence = (0..units)
            .map(|place| Evidence::new(format!("unit {place}")).expect("evidence"))
            .collect::<Vec<_>>();
        Find::new(
            QuestionText::new("Which unit answers?").expect("question"),
            &evidence,
            ModelName::new("model").expect("model"),
            none,
        )
        .expect("find")
    }

    fn answer(entries: &[(&str, f64)]) -> Answer {
        let distribution = Distribution::with_tolerance(
            entries
                .iter()
                .map(|(label, value)| {
                    (
                        (*label).to_owned(),
                        Probability::new(*value).expect("probability"),
                    )
                })
                .collect(),
            1.0,
        )
        .expect("distribution");
        Answer::new_choice(distribution, None).expect("choice")
    }

    fn assert_last_maps(count: usize, none: bool) {
        let find = find(count, none);
        let mut entries = (0..count)
            .map(|place| {
                let probability = if place + 1 == count { 1.0 } else { 0.0 };
                (
                    super::unit_id(place),
                    Probability::new(probability).expect("probability"),
                )
            })
            .collect::<Vec<_>>();
        if none {
            entries.push(("none".to_owned(), Probability::new(0.0).expect("zero")));
        }
        let distribution = Distribution::with_tolerance(entries, 1.0).expect("distribution");
        let answer = Answer::new_choice(distribution, None).expect("choice");
        assert_eq!(
            find.select(&answer).expect("mapped").selected(),
            Some(count - 1)
        );
    }

    #[test]
    fn exact_count_edges_build_one_aggregate_choice_plan() {
        assert!(find(2, false).plan().questions().len() == 1);
        assert!(find(255, false).plan().questions().len() == 1);
        assert!(find(2, true).plan().questions().len() == 1);
        assert!(find(254, true).plan().questions().len() == 1);
    }

    #[test]
    fn every_outside_count_is_refused_and_hostile_text_stays_json_data() {
        for (units, none) in [(1, false), (256, false), (1, true), (255, true)] {
            let evidence = (0..units)
                .map(|place| Evidence::new(format!("unit {place}")).expect("evidence"))
                .collect::<Vec<_>>();
            assert!(
                Find::new(
                    QuestionText::new("Which unit answers?").expect("question"),
                    &evidence,
                    ModelName::new("model").expect("model"),
                    none,
                )
                .is_err()
            );
        }
        let hostile_text = "quote: \" and slash: \\";
        let hostile = vec![
            Evidence::new(hostile_text).expect("hostile"),
            Evidence::new("second").expect("evidence"),
        ];
        let find = Find::new(
            QuestionText::new("Which unit answers?").expect("question"),
            &hostile,
            ModelName::new("model").expect("model"),
            false,
        )
        .expect("find");
        let request: EncodedRequest =
            serde_json::from_slice(&built_in::encode(find.plan()).expect("request"))
                .expect("request JSON");
        let units: Vec<EncodedUnit> = serde_json::from_str(&request.state).expect("aggregate JSON");
        assert_eq!(units.first().expect("first unit").evidence, hostile_text);
    }

    #[test]
    fn the_complete_two_unit_request_matches_its_public_fixture() {
        let evidence = [
            Evidence::new("alpha \"quote\"\\path").expect("evidence"),
            Evidence::new("beta").expect("evidence"),
        ];
        let find = Find::new(
            QuestionText::new("Which unit answers?").expect("question"),
            &evidence,
            ModelName::new("local-1").expect("model"),
            false,
        )
        .expect("find");
        let request = built_in::encode(find.plan()).expect("request");
        assert_eq!(
            request,
            include_bytes!("../../../../specification/fixtures/systemone/find-two.request.json")
                .strip_suffix(b"\n")
                .expect("fixture line ending")
        );
    }

    #[test]
    fn real_ties_take_the_first_and_any_none_tie_is_unresolved() {
        let plain = find(2, false);
        assert_eq!(
            plain
                .select(&answer(&[("u001", 0.5), ("u002", 0.5)]))
                .expect("mapped")
                .selected(),
            Some(0)
        );
        let with_none = find(2, true);
        assert_eq!(
            with_none
                .select(&answer(&[("u001", 0.5), ("u002", 0.0), ("none", 0.5)]))
                .expect("mapped")
                .selected(),
            None
        );
        assert_eq!(
            with_none
                .select(&answer(&[("u001", 0.0), ("u002", 0.0), ("none", 1.0)]))
                .expect("mapped")
                .selected(),
            None
        );
    }

    #[test]
    fn digest_depends_on_question_and_none_policy_but_not_unit_count() {
        let canonical = find(2, false).canonical_question().expect("canonical");
        assert_eq!(
            canonical,
            r#"{"verb":"find","text":"Which unit answers?","none":false}"#
        );
        assert_eq!(
            find(2, false).question_sha256().expect("digest"),
            "01456d0e17c98c801c2ad9b2a9b56e47aeb33ff0eacde8d44bd6f55e4d0ab9ef"
        );
        assert_eq!(
            find(2, false).question_sha256().expect("digest"),
            find(255, false).question_sha256().expect("digest")
        );
        assert_ne!(
            find(2, false).question_sha256().expect("digest"),
            find(2, true).question_sha256().expect("digest")
        );
        let different = Find::new(
            QuestionText::new("Which different unit answers?").expect("question"),
            &[
                Evidence::new("one").expect("evidence"),
                Evidence::new("two").expect("evidence"),
            ],
            ModelName::new("model").expect("model"),
            false,
        )
        .expect("find");
        assert_ne!(
            find(2, false).question_sha256().expect("digest"),
            different.question_sha256().expect("digest")
        );
    }

    #[test]
    fn every_valid_count_maps_its_last_real_unit_back_to_the_same_place() {
        for none in [false, true] {
            let most = if none { 254 } else { 255 };
            (2..=most).for_each(|count| assert_last_maps(count, none));
        }
    }
}
