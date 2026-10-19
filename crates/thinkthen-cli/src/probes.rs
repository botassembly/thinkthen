//! The backend check's four probes and the report that grades their replies.
//!
//! The command sends each probe through the production engine and hands the
//! outcome here. `specification/check.md` fixes the rows and the sentences.

use serde::Serialize;

use crate::render::json_line;
use thinkthen::{
    BatchSetting, CallOptions, CompleteAnnotated, CompleteAnnotationMember, Engine, Error,
    ErrorKind, FailureCause, PlanEstimate, Probabilities, QuestionKind, QuestionSet, RawRecord,
    Request, RequestArguments, RequestEnvironment, RequestInput, RequestOptions, RequestQuestion,
};

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

/// One fixed request: the row it reports on, the plan it sends, and whether
/// sending it turns a description object into text.
pub(crate) struct Probe {
    pub(crate) name: &'static str,
    pub(crate) arguments: RequestArguments,
    pub(crate) plan: PlanEstimate,
    pub(crate) drops_detail: bool,
    pub(crate) warning: &'static str,
}

/// Prepare the four fixed, typed probes through the native public call family.
pub(crate) fn probes(engine: &Engine, backend_name: Option<&str>) -> Result<Vec<Probe>, Error> {
    let names = ["noul", "choice", "score", "mixed"];
    PROBES
        .iter()
        .zip(names)
        .map(|((structured, set), name)| {
            let arguments = RequestArguments {
                question: RequestQuestion::Definition {
                    value: QuestionSet::from_json(set)
                        .map_err(|_| broken_probe())?
                        .into(),
                },
                input: if *structured {
                    RequestInput::Json {
                        value: RawRecord::json(OBJECT).map_err(|_| broken_probe())?,
                        images: vec![],
                    }
                } else {
                    RequestInput::Text {
                        text: TEXT.into(),
                        images: vec![],
                    }
                },
                options: RequestOptions::default(),
            };
            let request = Request::named("annotate", arguments.clone())
                .and_then(Request::admit)
                .map_err(|_| broken_probe())?;
            let plan = engine.plan_request(
                &request,
                RequestEnvironment {
                    controls: CallOptions::new()
                        .shared_state_groups()
                        .batch(BatchSetting::Max),
                    feed: None,
                },
            )?;
            Ok(Probe {
                name,
                arguments,
                drops_detail: plan.drops_detail(),
                plan,
                warning: detail_warning(backend_name),
            })
        })
        .collect()
}

fn broken_probe() -> Error {
    Error::new(ErrorKind::Defect, "a check probe no longer parses")
}

#[derive(Serialize)]
struct Said<'a> {
    model: &'a str,
    answers: Vec<Outcome>,
    usage: Option<ReplyUsage>,
}
#[derive(Serialize)]
struct ReplyUsage {
    input_tokens: u64,
    output_tokens: u64,
}
fn usage(reply: &CompleteAnnotated) -> Option<ReplyUsage> {
    let usage = reply.meta().usage()?;
    Some(ReplyUsage {
        input_tokens: usage.input_tokens()?,
        output_tokens: usage.output_tokens()?,
    })
}
#[derive(Serialize)]
#[serde(untagged)]
enum Outcome {
    Answered(Answer),
    Failed { failed: Failure },
}
#[derive(Serialize)]
struct Failure {
    kind: &'static str,
    cause: FailureCause,
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Answer {
    YesNo {
        probability: f64,
    },
    Choice {
        pick: String,
        probabilities: Odds,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<f64>,
    },
    Tag {
        probabilities: Odds,
    },
    Score {
        level: String,
        probabilities: Odds,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<f64>,
    },
}
struct Odds(Vec<(String, f64)>);
impl Serialize for Odds {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, probability)| (name, probability)))
    }
}
fn outcome(member: CompleteAnnotationMember<'_>) -> Option<Outcome> {
    if let Some(cause) = member.failure() {
        return Some(Outcome::Failed {
            failed: Failure {
                kind: "backend",
                cause,
            },
        });
    }
    let answer = match member.probabilities()? {
        Probabilities::YesNo { yes } => Answer::YesNo { probability: yes },
        Probabilities::Named(values) => {
            let probabilities = Odds(
                values
                    .into_iter()
                    .map(|value| (value.name().to_owned(), value.probability()))
                    .collect(),
            );
            let confidence = member.confidence();
            match member.question().kind() {
                QuestionKind::Choose => Answer::Choice {
                    pick: member.leader()?.to_owned(),
                    probabilities,
                    confidence,
                },
                QuestionKind::Score => Answer::Score {
                    level: member.leader()?.to_owned(),
                    probabilities,
                    confidence,
                },
                QuestionKind::Tag => Answer::Tag { probabilities },
                _ => return None,
            }
        }
    };
    Some(Outcome::Answered(answer))
}

/// Render retained native answers; no provider bytes or inferred leaders enter output.
pub(crate) fn reply_line(probe: &Probe, reply: &CompleteAnnotated) -> Option<String> {
    let metadata = reply.meta();
    let said = Said {
        model: metadata.model(),
        answers: reply.members().map(outcome).collect::<Option<Vec<_>>>()?,
        usage: usage(reply),
    };
    Some(format!("reply {} {}", probe.name, json_line(&said).ok()?))
}

/// One finding: whether it is critical, and its sentence.
type Finding = (bool, String);

/// A row and its findings. `None` marks a row the check did not reach.
type Row = (&'static str, Option<Vec<Finding>>);

/// Every row, in the order the report prints them.
pub(crate) struct Report(Vec<Row>);

impl Report {
    /// The gate rows, one row per probe, and the usage row, none reached yet.
    pub(crate) fn new(probes: &[Probe], functions: &[&'static str]) -> Self {
        let probes = probes.iter().map(|probe| probe.name);
        let rows = ["connection", "key", "endpoint"].into_iter().chain(probes);
        Self(
            rows.chain(["usage"])
                .chain(functions.iter().copied())
                .map(|name| (name, None))
                .collect(),
        )
    }

    /// A probe's reply was decoded: grade each answer in wire order.
    pub(crate) fn replied(&mut self, probe: &Probe, reply: &CompleteAnnotated) {
        self.reached();
        let mut place = 0;
        let mut findings = Vec::new();
        for member in reply.members() {
            let width = if member.question().kind() == QuestionKind::Tag {
                member.question().options().count()
            } else {
                1
            };
            findings.extend(graded(&member, place, width));
            place += width;
        }
        findings.extend(dropped(probe));
        self.set(probe.name, findings);
        let warned = self.row("usage").is_some_and(|held| !held.is_empty());
        let warning = (warned || usage(reply).is_none()).then(|| (false, NO_USAGE.to_owned()));
        self.set("usage", warning.into_iter().collect());
    }

    /// A probe failed for a reason its own body may cause. The check goes on.
    pub(crate) fn failed(&mut self, probe: &Probe, sentence: String) {
        self.reached();
        let findings = std::iter::once((true, sentence)).chain(dropped(probe));
        self.set(probe.name, findings.collect());
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

    /// A real function path answered, or failed with a safe public error.
    pub(crate) fn function(&mut self, name: &str, failure: Option<String>) {
        self.set(name, failure.into_iter().map(|said| (true, said)).collect());
    }

    /// The row lines, the closing count line, and whether any line is critical.
    pub(crate) fn lines(&self) -> (Vec<String>, bool) {
        let mut lines = Vec::new();
        for (name, row) in &self.0 {
            match row.as_deref() {
                None => lines.push(format!("unchecked {name}")),
                Some([]) => lines.push(format!(
                    "{} {name}",
                    if name.starts_with("function ") {
                        "answered"
                    } else {
                        "ok"
                    }
                )),
                Some(held) => lines.extend(held.iter().map(|(critical, said)| {
                    format!(
                        "{} {name}: {said}",
                        if name.starts_with("function ") {
                            "incompatible"
                        } else {
                            level(*critical)
                        }
                    )
                })),
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

/// The warning a probe row carries when the Ollama workaround turned one of its
/// description objects into text.
fn dropped(probe: &Probe) -> Option<Finding> {
    probe
        .drops_detail
        .then(|| (false, probe.warning.to_owned()))
}

const fn level(critical: bool) -> &'static str {
    if critical { "critical" } else { "warning" }
}

/// One answer's finding: a failed logical question named by its wire range,
/// or a choice or score answer that carries no confidence.
fn graded(member: &CompleteAnnotationMember<'_>, place: usize, width: usize) -> Option<Finding> {
    let first = format!("q{}", place + 1);
    if let Some(failure) = member.failure() {
        let cause = json_line(&failure).unwrap_or_default();
        let named = match width {
            1 => format!("question `{first}`"),
            _ => format!("questions `{first}` to `q{}`", place + width),
        };
        return Some((
            true,
            format!(
                "the answer to {named} failed as `{}`",
                cause.trim_matches('"')
            ),
        ));
    }
    let said = format!("the answer to question `{first}` carries no confidence");
    let graded = matches!(
        member.question().kind(),
        QuestionKind::Choose | QuestionKind::Score
    );
    (graded && member.confidence().is_none()).then_some((false, said))
}

/// Existing command advice for adapters that flatten authored descriptions.
pub(crate) fn detail_warning(name: Option<&str>) -> &'static str {
    if name == Some("mlx") {
        "backend `mlx` sends each description object as its `what` text, a temporary workaround for the Strands server schema, so its other fields are left out"
    } else {
        "backend `ollama` sends each description object as its `what` text, a temporary workaround for an Ollama bug, so its other fields are left out"
    }
}
