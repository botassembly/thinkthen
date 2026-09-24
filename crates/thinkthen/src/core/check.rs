//! The backend check's four probes and the report that grades their replies.
//!
//! The command sends each probe through the production engine and hands the
//! outcome here. `specification/check.md` fixes the rows and the sentences.

use crate::core::adapters::built_in::{wire_name, wire_type};
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question::Question;
use crate::core::question_set::QuestionSet;
use crate::core::render::json_line;
use crate::core::reply::{AnswerOutcome, FailedValue, Reply};
use crate::core::text::{Evidence, ModelName};

const TEXT: &str = "The parcel arrived on Tuesday and the box was intact.";
const OBJECT: &str = r#"{"note":"The parcel arrived on Tuesday.","box":"intact"}"#;

/// Whether each probe reads the object evidence, and its question set. A
/// one-question probe is named by its wire type, and the other is `mixed`.
const PROBES: [(bool, &str); 4] = [
    (
        false,
        r#"{"version":1,"questions":{"q":{"decide":"Did the parcel arrive undamaged?","true":"The text says the box or its contents were intact.","false":null}}}"#,
    ),
    (
        false,
        r#"{"version":1,"questions":{"q":{"choose":"Which day did the parcel arrive?","options":{"Monday":null,"Tuesday":"The second working day of the week.","Wednesday":{"what":"The middle of the week","examples":["midweek"]}}}}}"#,
    ),
    (
        false,
        r#"{"version":1,"questions":{"q":{"score":"How well was the parcel packed?","levels":{"fair":null,"good":"The box was dented but the contents were fine.","excellent":{"what":"The box was intact","not_for":"a dented box"}}}}}"#,
    ),
    (
        true,
        r#"{"version":1,"questions":{"on_time":{"decide":"Did it arrive on time?"},"day":{"choose":"Which day?","options":["Monday","Tuesday"]},"packing":{"score":"How was the packing?","levels":["poor","good"]},"fit":{"tag":"Which labels fit?","labels":{"on_time":{"what":"It arrived when promised"},"damaged":"The box or its contents were harmed."}}}}"#,
    ),
];

const NO_USAGE: &str =
    "a reply carries no token counts, so results and usage totals leave them out";

/// One fixed request: the row it reports on and the plan it sends.
pub(crate) struct Probe {
    pub(crate) name: &'static str,
    pub(crate) plan: Plan,
}

/// The four probes for one model, or `None` when a fixed probe no longer
/// parses, which only a defect can cause.
pub(crate) fn probes(model: &ModelName) -> Option<Vec<Probe>> {
    let object = Evidence::structured(Json::parse(OBJECT).ok()?).ok()?;
    let text = Evidence::new(TEXT).ok()?;
    let probe = |(structured, set): &(bool, &str)| {
        let questions = QuestionSet::parse(set).ok()?.questions().to_vec();
        let questions = questions.into_iter().map(|named| named.question().clone());
        let evidence = if *structured { &object } else { &text };
        let plan = Plan::new(evidence.clone(), model.clone(), questions.collect()).ok()?;
        let name = match plan.questions() {
            [one] => wire_type(one),
            _ => "mixed",
        };
        Some(Probe { name, plan })
    };
    PROBES.iter().map(probe).collect()
}

/// One finding: whether it is critical, and its sentence.
type Finding = (bool, String);

/// A row and its findings. `None` marks a row the check did not reach.
type Row = (&'static str, Option<Vec<Finding>>);

/// Every row, in the order the report prints them.
pub(crate) struct Report(Vec<Row>);

impl Report {
    /// The gate rows, one row per probe, and the usage row, none reached yet.
    pub(crate) fn new(probes: &[Probe]) -> Self {
        let probes = probes.iter().map(|probe| probe.name);
        let rows = ["connection", "key", "endpoint"].into_iter().chain(probes);
        Self(rows.chain(["usage"]).map(|name| (name, None)).collect())
    }

    /// A probe's reply was decoded: grade each answer in wire order.
    pub(crate) fn replied(&mut self, probe: &Probe, reply: &Reply) {
        self.reached();
        let mut place = 0;
        let mut findings = Vec::new();
        for (question, outcome) in probe.plan.questions().iter().zip(reply.outcomes()) {
            let width = match question {
                Question::Tag { labels, .. } => labels.count(),
                _ => 1,
            };
            findings.extend(graded(question, outcome, place, width));
            place += width;
        }
        self.set(probe.name, findings);
        let warned = self.row("usage").is_some_and(|held| !held.is_empty());
        let warning = (warned || reply.usage().is_none()).then(|| (false, NO_USAGE.to_owned()));
        self.set("usage", warning.into_iter().collect());
    }

    /// A probe failed for a reason its own body may cause. The check goes on.
    pub(crate) fn failed(&mut self, probe: &Probe, sentence: String) {
        self.reached();
        self.set(probe.name, vec![(true, sentence)]);
    }

    /// A probe met a failure every later probe would meet. The check stops.
    /// At the first probe the finding lands on `gate`: `connection`, `key`,
    /// or `endpoint`.
    pub(crate) fn stopped(&mut self, probe: &Probe, gate: &'static str, sentence: String) {
        let row = if self.row("connection").is_some() {
            probe.name
        } else {
            gate
        };
        if row != "connection" {
            self.set("connection", Vec::new());
        }
        self.set(row, vec![(true, sentence)]);
    }

    /// The row lines, the closing count line, and whether any line is critical.
    pub(crate) fn lines(&self) -> (Vec<String>, bool) {
        let mut lines = Vec::new();
        for (name, row) in &self.0 {
            match row.as_deref() {
                None => lines.push(format!("unchecked {name}")),
                Some([]) => lines.push(format!("ok {name}")),
                Some(held) => lines.extend(
                    held.iter()
                        .map(|(critical, said)| format!("{} {name}: {said}", level(*critical))),
                ),
            }
        }
        let all = self.0.iter().filter_map(|(_, row)| row.as_ref()).flatten();
        let critical = all.clone().filter(|(critical, _)| *critical).count();
        let warning = all.count() - critical;
        lines.push(format!("critical {critical}, warning {warning}"));
        (lines, critical > 0)
    }

    /// A reply of any status arrived, so the three gate rows are ok.
    fn reached(&mut self) {
        for name in ["connection", "key", "endpoint"] {
            if self.row(name).is_none() {
                self.set(name, Vec::new());
            }
        }
    }

    fn row(&self, name: &str) -> Option<&Vec<Finding>> {
        self.0.iter().find(|(held, _)| *held == name)?.1.as_ref()
    }

    fn set(&mut self, name: &str, findings: Vec<Finding>) {
        if let Some((_, row)) = self.0.iter_mut().find(|(held, _)| *held == name) {
            *row = Some(findings);
        }
    }
}

const fn level(critical: bool) -> &'static str {
    if critical { "critical" } else { "warning" }
}

/// One answer's finding: a failed logical question named by its wire range,
/// or a choice or score answer that carries no confidence.
fn graded(
    question: &Question,
    outcome: &AnswerOutcome,
    place: usize,
    width: usize,
) -> Option<Finding> {
    let first = wire_name(place);
    let answer = match outcome {
        AnswerOutcome::Answered(answer) => answer,
        AnswerOutcome::Failed(failure) => {
            let cause = json_line(&FailedValue::new(*failure).cause()).unwrap_or_default();
            let named = match width {
                1 => format!("question `{first}`"),
                _ => format!("questions `{first}` to `{}`", wire_name(place + width - 1)),
            };
            let cause = cause.trim_matches('"');
            return Some((true, format!("the answer to {named} failed as `{cause}`")));
        }
    };
    let said = format!("the answer to question `{first}` carries no confidence");
    let graded = matches!(question, Question::Choose { .. } | Question::Score { .. });
    (graded && answer.confidence().is_none()).then_some((false, said))
}
