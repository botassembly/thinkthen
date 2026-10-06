//! Project native typed values directly into stable owned C views.
use super::private::{
    CurrentAnnotationV1, CurrentAtomicV1, CurrentAttemptV1, CurrentFactsV1, CurrentMemberV1,
    CurrentMembersV1, CurrentMetaV1, CurrentQuestionV1, CurrentRecognitionV1, CurrentRelationsV1,
    CurrentRowV1, CurrentValueV1,
};
use super::{Content, Input, Storage};
use crate::ffi::carriers::{
    LocationV1, OptionalContentV1, OptionalDoubleV1, OptionalLocationV1, OptionalSizeV1,
    OptionalU16V1, OptionalU64V1, ProbabilitiesV1, ProbabilityV1, StringsV1,
};
use thinkthen::{
    Annotated, Answer, Details, Facts, FailureCause, Judgment, Probabilities, QuestionDetail,
    Recognized,
};
#[derive(Clone)]
pub(crate) enum Row {
    Atomic(Box<CurrentAtomicV1>),
    Original(CurrentRowV1),
    Find(super::private::CurrentFindV1),
    Annotation(CurrentAnnotationV1),
    Recognition(CurrentRecognitionV1),
    Relations(CurrentRelationsV1),
}
pub(crate) fn double(value: Option<f64>) -> OptionalDoubleV1 {
    OptionalDoubleV1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0.0),
    }
}
pub(crate) fn size(value: Option<usize>) -> OptionalSizeV1 {
    OptionalSizeV1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0),
    }
}
fn u64_view(value: Option<u64>) -> OptionalU64V1 {
    OptionalU64V1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0),
    }
}
pub(crate) fn facts(s: &mut Storage, f: &Facts) -> CurrentFactsV1 {
    CurrentFactsV1 {
        cache_answers: f.cache_answers(),
        estimated_cost_usd: s.optional_string(f.estimated_cost_usd()),
        input_tokens: u64_view(f.input_tokens()),
        output_tokens: u64_view(f.output_tokens()),
        model: s.optional_string(f.model()),
        records: f.records(),
        requests_sent: f.requests_sent(),
        seconds: f.seconds(),
    }
}
fn strings(s: &mut Storage, items: &[String]) -> StringsV1 {
    let values = items.iter().map(|value| s.string(value)).collect();
    let (data, len) = s.array(values);
    StringsV1 { data, len }
}
pub(crate) fn value(s: &mut Storage, v: &Judgment) -> CurrentValueV1 {
    let mut out = CurrentValueV1::default();
    match v {
        Judgment::Decision(answer) => {
            out.kind = 1;
            if *answer != Answer::Unsure {
                out.decide_kind = 1;
                out.boolean = i32::from(*answer == Answer::Yes);
            }
        }
        Judgment::Choice(choice) => {
            out.kind = 2;
            out.choice = s.optional_string(choice.as_deref());
        }
        Judgment::Tags(tags) => {
            out.kind = 3;
            out.tags = strings(s, tags);
        }
        Judgment::Score(score) => {
            out.kind = 4;
            out.score = *score;
        }
    }
    out
}
pub(crate) fn authored(s: &mut Storage, v: &mut CurrentValueV1, meaning: Option<&Content>) {
    if v.decide_kind == 1
        && let Some(meaning) = meaning
    {
        v.decide_kind = 2;
        v.boolean = 0;
        v.authored = OptionalContentV1 {
            present: 1,
            value: s.content(meaning),
        };
    }
}
pub(crate) fn probabilities(
    s: &mut Storage,
    p: Option<&Probabilities>,
) -> (OptionalDoubleV1, ProbabilitiesV1) {
    match p {
        Some(Probabilities::YesNo { yes }) => (double(Some(*yes)), ProbabilitiesV1::default()),
        Some(Probabilities::Named(items)) => {
            let items = items
                .iter()
                .map(|item| ProbabilityV1 {
                    name: s.string(item.name()),
                    probability: item.probability(),
                })
                .collect();
            let (data, len) = s.array(items);
            (double(None), ProbabilitiesV1 { data, len })
        }
        None => (double(None), ProbabilitiesV1::default()),
    }
}
pub(crate) fn row(s: &mut Storage, input: Option<&Input>) -> CurrentRowV1 {
    let Some(input) = input else {
        return CurrentRowV1::default();
    };
    let position = input
        .position
        .as_ref()
        .map_or_else(OptionalLocationV1::default, |p| OptionalLocationV1 {
            present: 1,
            value: LocationV1 {
                file: s.optional_string(Some(&p.file)),
                first_line: size(p.first_line),
                last_line: size(p.last_line),
            },
        });
    CurrentRowV1 {
        input: input
            .original
            .as_ref()
            .map_or_else(OptionalContentV1::default, |original| OptionalContentV1 {
                present: 1,
                value: s.content(original),
            }),
        position,
        index: size(Some(input.index)),
        ..CurrentRowV1::default()
    }
}
pub(crate) fn atomic(s: &mut Storage, input: &Input, d: &Details) -> CurrentAtomicV1 {
    let (yes_probability, probabilities) = probabilities(s, Some(d.probabilities()));
    let warning = d.profile_warning();
    let usage = d.usage();
    let meta = CurrentMetaV1 {
        question_sha256: s.string(d.question_sha256()),
        model: s.string(d.model()),
        url: s.string(d.url()),
        requests: strings(s, d.requests()),
        requests_sent: d.requests_sent(),
        cached: i32::from(d.cached()),
        usage_present: i32::from(usage.is_some()),
        input_tokens: usage.map_or(0, thinkthen::Usage::input_tokens),
        output_tokens: usage.map_or(0, thinkthen::Usage::output_tokens),
        tuned_for: s.optional_string(warning.map(|(t, _)| t)),
        running: s.optional_string(warning.map(|(_, r)| r)),
    };
    CurrentAtomicV1 {
        common: row(s, Some(input)),
        value: value(s, d.value()),
        yes_probability,
        probabilities,
        nearest: s.optional_string(d.nearest()),
        confidence: double(d.confidence()),
        meta,
    }
}
pub(crate) const fn failure(cause: FailureCause) -> u32 {
    match cause {
        FailureCause::MissingAnswer => 1,
        FailureCause::WrongKind => 2,
        FailureCause::MissingProbability => 3,
        FailureCause::InvalidProbability => 4,
        FailureCause::InvalidDistribution => 5,
        FailureCause::UnexpectedProbability => 6,
    }
}
pub(crate) fn annotation(
    s: &mut Storage,
    input: &Input,
    members: &[thinkthen::NamedAnnotation],
    grammar: &str,
) -> Result<CurrentAnnotationV1, crate::failures::Failure> {
    let body: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(grammar)
            .map_err(|_| crate::failures::Failure::defect("saved set cannot be read"))?;
    let questions = body
        .get("questions")
        .ok_or_else(|| crate::failures::Failure::defect("saved set lost its questions"))?;
    let questions: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(questions.get())
            .map_err(|_| crate::failures::Failure::defect("saved set lost its members"))?;
    let members = members
        .iter()
        .map(|m| {
            let name = s.string(m.name());
            let v = match m.value() {
                Annotated::Decision(v) => Judgment::Decision(*v),
                Annotated::Choice(v) => Judgment::Choice(v.clone()),
                Annotated::Score(v) => Judgment::Score(*v),
                Annotated::Tags(v) => Judgment::Tags(v.clone()),
                Annotated::Failed(f) => {
                    return Ok(CurrentMemberV1 {
                        name,
                        state: 2,
                        failure: failure(f.cause()),
                        ..CurrentMemberV1::default()
                    });
                }
            };
            let mut projected = value(s, &v);
            if let Some(grammar) = questions.get(m.name()) {
                let meaning = super::execute::meaning(grammar.get(), &v)?;
                authored(s, &mut projected, meaning.as_ref());
            }
            Ok(CurrentMemberV1 {
                name,
                state: 1,
                value: projected,
                failure: 0,
            })
        })
        .collect::<Result<Vec<_>, crate::failures::Failure>>()?;
    let (data, len) = s.array(members);
    Ok(CurrentAnnotationV1 {
        common: row(s, Some(input)),
        members: CurrentMembersV1 { data, len },
    })
}
pub(crate) fn recognition(s: &mut Storage, input: &Input, r: &Recognized) -> CurrentRecognitionV1 {
    let value = s.recognition_value(r);
    CurrentRecognitionV1 {
        common: row(s, Some(input)),
        entities: value.entities,
        relations_present: value.relations.present,
        relations: value.relations.value,
    }
}
pub(crate) fn relations(s: &mut Storage, edges: &[thinkthen::Edge]) -> CurrentRelationsV1 {
    CurrentRelationsV1 {
        edges: s.edges(edges),
        ..CurrentRelationsV1::default()
    }
}
pub(crate) struct SavedQuestion {
    pub(crate) index: usize,
    member: Option<String>,
    stage: Option<String>,
    position: usize,
    digest: String,
    model: String,
    url: String,
    requests: Vec<String>,
    sends: u64,
    cached: bool,
    usage: Option<thinkthen::Usage>,
    value: Option<Judgment>,
    failure: Option<FailureCause>,
    probabilities: Option<Probabilities>,
    confidence: Option<f64>,
    failed: usize,
}
impl SavedQuestion {
    pub(crate) fn new(
        index: usize,
        member: Option<&str>,
        stage: Option<&str>,
        position: usize,
        d: QuestionDetail<'_>,
    ) -> Self {
        Self {
            index,
            member: member.map(str::to_owned),
            stage: stage.map(str::to_owned),
            position,
            digest: d.question_sha256().to_owned(),
            model: d.model().to_owned(),
            url: d.url().to_owned(),
            requests: d.requests().to_vec(),
            sends: d.requests_sent(),
            cached: d.cached(),
            usage: d.usage(),
            value: d.value().cloned(),
            failure: d.failure(),
            probabilities: d.probabilities().cloned(),
            confidence: d.confidence(),
            failed: d.failed_questions(),
        }
    }
    pub(crate) fn view(&self, s: &mut Storage) -> CurrentQuestionV1 {
        let (yes_probability, probabilities) = probabilities(s, self.probabilities.as_ref());
        CurrentQuestionV1 {
            index: self.index,
            member: s.optional_string(self.member.as_deref()),
            stage: s.optional_string(self.stage.as_deref()),
            position: self.position,
            meta: CurrentMetaV1 {
                question_sha256: s.string(&self.digest),
                model: s.string(&self.model),
                url: s.string(&self.url),
                requests: strings(s, &self.requests),
                requests_sent: self.sends,
                cached: i32::from(self.cached),
                usage_present: i32::from(self.usage.is_some()),
                input_tokens: self.usage.map_or(0, |u| u.input_tokens()),
                output_tokens: self.usage.map_or(0, |u| u.output_tokens()),
                ..CurrentMetaV1::default()
            },
            state: if self.failure.is_some() { 2 } else { 1 },
            value: self
                .value
                .as_ref()
                .map_or_else(CurrentValueV1::default, |v| value(s, v)),
            failure: self.failure.map_or(0, failure),
            yes_probability,
            probabilities,
            confidence: double(self.confidence),
            failed_questions: self.failed,
        }
    }
}
pub(crate) fn attempt(s: &mut Storage, a: &thinkthen::AttemptObservation) -> CurrentAttemptV1 {
    let outcome = match a.outcome() {
        thinkthen::AttemptOutcome::Ok => 1,
        thinkthen::AttemptOutcome::Status => 2,
        thinkthen::AttemptOutcome::Transport => 3,
    };
    let status = a.status();
    CurrentAttemptV1 {
        ordinal: a.ordinal(),
        request_sha256: s.string(a.request_sha256()),
        wall_ms: a.wall_ms(),
        outcome,
        status: OptionalU16V1 {
            present: i32::from(status.is_some()),
            value: status.unwrap_or(0),
        },
        server_ms: u64_view(a.server_ms()),
        request_id: s.optional_string(a.request_id()),
    }
}
