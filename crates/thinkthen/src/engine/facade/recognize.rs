//! Recognition in three steps (ADR 0056): BILOU boundaries over windowed
//! requests, kinds and edges per step-1 request, then stated relations.

use super::context::contextual_requests;
use super::each::Models;
use super::{Answered, Asks, Bound, Engine, Request};
use crate::core::relation::count_pairs;
use crate::core::{
    Answer, AnswerOutcome, Asked, Backend, BackendProfile, Evidence, Lead, ModelName, NameOdds,
    Odds, Piece, PieceOdds, Plan, Question, RecognizeSpec, RecognizedName, RelationEdge, TAGS,
    TagRow, Usage, found_names, kind_question, name_groups, pair_edges, pieces, plan_pairs,
    settle_names, step_one_groups, step_one_questions, step_two_questions, window,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::pipeline::RowUsage;

/// The default limit on one text's UTF-8 bytes, which caps spending.
pub(crate) const MAX_TEXT_BYTES: usize = 600_000;
const MAX_RELATION_NAMES: usize = 255;
const MAX_RELATION_QUESTIONS: usize = 4_000;

use crate::core::{PairOdds, Place};
pub(crate) use crate::core::{RecognitionOdds as Probabilities, RecognizedValue as Recognized};

/// The request metadata of every stage, in construction order. `live` says
/// whether any answer came from the backend rather than a recording or cache.
#[derive(Debug, Default)]
pub(crate) struct Aggregate {
    pub(crate) trace: crate::core::LogicalTrace,
    pub(crate) model: Option<ModelName>,
    models: Models,
    shares: RowUsage,
    pub(crate) usage: Option<Usage>,
    pub(crate) reported_usage: Option<crate::core::ReportedUsage>,
    pub(crate) live: bool,
    pub(crate) requests_sent: u64,
    pub(crate) requests: Vec<String>,
}

/// One text's recognition, its probabilities, and the metadata.
#[derive(Debug)]
pub(crate) struct Recognition {
    pub(crate) value: Recognized,
    pub(crate) details: Probabilities,
    pub(crate) meta: Aggregate,
}

fn asked_stages(asked: &[Asked]) -> impl Iterator<Item = &'static str> + '_ {
    asked.iter().flat_map(|one| {
        [
            one.kind.then_some("kind"),
            one.edge_question().then_some("edge"),
        ]
        .into_iter()
        .flatten()
    })
}

impl Engine {
    /// Recognize the names in one text, then relate them when rules were given.
    /// A text over `limit` bytes is refused before any request.
    pub(crate) fn recognize(
        &self,
        spec: &RecognizeSpec,
        text: &str,
        limit: usize,
        cancel: &Cancel,
    ) -> Result<Recognition, Error> {
        self.recognize_observed(spec, text, limit, cancel, |_, _, _| Ok(()))
    }

    /// The same call with each answered question handed to `observe` with
    /// its stage and logical question.
    pub(crate) fn recognize_observed(
        &self,
        spec: &RecognizeSpec,
        text: &str,
        limit: usize,
        cancel: &Cancel,
        mut observe: impl FnMut(&'static str, &Question, &Answered) -> Result<(), Error>,
    ) -> Result<Recognition, Error> {
        let (pieces, asks, _) = step_one_context(
            &self.backend,
            self.profile.as_ref(),
            spec,
            text,
            limit,
            self.aggregate_context.as_deref(),
        )?;
        let mut meta = Aggregate::default();
        let mut details = Probabilities::default();
        if pieces.is_empty() {
            return Ok(Recognition {
                value: Recognized {
                    entities: Vec::new(),
                    relations: (!spec.relations.is_empty()).then(Vec::new),
                },
                details,
                meta,
            });
        }
        let stages = vec!["boundary"; asks.len()];
        let answers = self.execute(
            &asks,
            Bound::WHOLE,
            &stages,
            (&mut meta, &mut observe),
            cancel,
        )?;
        let rows = answers.iter().map(tag_row).collect::<Result<Vec<_>, _>>()?;
        details.pieces = pieces
            .iter()
            .zip(&rows)
            .map(|(piece, row)| PieceOdds::new(piece, row))
            .collect();
        let stretches = found_names(&rows);
        let (asks, asked, stages) = step_two(&self.backend, (text, &pieces), &stretches, spec)?;
        let answers = self.execute(
            &asks,
            Bound::WHOLE,
            &stages,
            (&mut meta, &mut observe),
            cancel,
        )?;
        let mut answers = answers.iter();
        let mut read = || answers.next().ok_or(Error::RecognizeLogical).and_then(odds);
        let mut settled = Vec::with_capacity(stretches.len());
        for ((first, last), held) in stretches.iter().copied().zip(asked) {
            let kinds = held.kind.then(&mut read).transpose()?;
            let edges = held.edge_question().then(&mut read).transpose()?;
            let place = |at: usize| pieces.get(at).map_or(0, |piece| piece.start);
            let odds = NameOdds {
                start: place(first),
                end: pieces.get(last).map_or(0, |piece| piece.end),
                kinds,
                edges,
            };
            details.names.push(odds.clone());
            settled.push((held, odds));
        }
        let cut = spec.threshold.cut_value().unwrap_or(0.5);
        let entities = settle_names(text, &pieces, &rows, &stretches, &settled, cut);
        let relations = self.relations(
            spec,
            text,
            &entities,
            (&mut meta, &mut details, &mut observe),
            cancel,
        )?;
        meta.model = meta.models.model().cloned();
        meta.usage = meta.shares.total()?;
        meta.reported_usage = meta.shares.reported()?;
        Ok(Recognition {
            value: Recognized {
                entities,
                relations,
            },
            details,
            meta,
        })
    }

    /// Ask each pair a rule allows about what the text itself states.
    fn relations(
        &self,
        spec: &RecognizeSpec,
        text: &str,
        entities: &[RecognizedName],
        held: (
            &mut Aggregate,
            &mut Probabilities,
            &mut impl FnMut(&'static str, &Question, &Answered) -> Result<(), Error>,
        ),
        cancel: &Cancel,
    ) -> Result<Option<Vec<RelationEdge<RecognizedName>>>, Error> {
        if spec.relations.is_empty() {
            return Ok(None);
        }
        let count = count_pairs(entities, &spec.relations);
        if count.questions == Some(0) {
            return Ok(Some(Vec::new()));
        }
        if count.admitted > MAX_RELATION_NAMES {
            return Err(Error::RecognizeRelationNames {
                count: count.admitted,
                limit: MAX_RELATION_NAMES,
            });
        }
        if count
            .questions
            .is_none_or(|questions| questions > MAX_RELATION_QUESTIONS)
        {
            return Err(Error::RecognizeRelationQuestions {
                names: count.admitted,
                count: count.questions,
                limit: MAX_RELATION_QUESTIONS,
            });
        }
        let Some(planned) = plan_pairs(Some(text), entities, &spec.relations, Lead::Stated)
            .map_err(|_| Error::Defect("relation planning failed"))?
        else {
            return Ok(Some(Vec::new()));
        };
        let bound = Bound::pairs(self.profile.as_ref());
        let mut asks = Asks::default();
        let plan = Plan::new(
            planned.evidence.clone(),
            self.backend.model().clone(),
            self.backend.descriptions(),
            planned.questions.clone(),
        )
        .map_err(|_| Error::Defect("relation planned no questions"))?;
        asks.add(&self.backend, &plan)?;
        let (meta, details, observe) = held;
        let stages = vec!["relation"; asks.len()];
        let answers = self.execute(&asks, bound, &stages, (meta, observe), cancel)?;
        let cut = spec.relation_threshold.cut_value().unwrap_or(0.5);
        for (pair, answer) in planned.pairs.iter().zip(&answers) {
            let (Some(rule), Some(source), Some(target)) = (
                spec.relations.get(pair.rule),
                entities.get(pair.source),
                entities.get(pair.target),
            ) else {
                return Err(Error::Defect("a pair names no rule or name"));
            };
            details.pairs.push(PairOdds {
                relation: rule.name.clone(),
                source: Place::of(source),
                target: Place::of(target),
                probability: answer.yes().ok_or(Error::RecognizeLogical)?,
            });
        }
        Ok(Some(pair_edges(
            entities,
            &spec.relations,
            &planned.pairs,
            &answers,
            cut,
        )))
    }

    /// Ask one step's questions; one failed question fails the text.
    fn execute(
        &self,
        asks: &Asks,
        bound: Bound,
        stages: &[&'static str],
        (meta, observe): (
            &mut Aggregate,
            &mut impl FnMut(&'static str, &Question, &Answered) -> Result<(), Error>,
        ),
        cancel: &Cancel,
    ) -> Result<Vec<Answer>, Error> {
        if stages.len() != asks.len() {
            return Err(Error::RecognizeLogical);
        }
        let mut answers = Vec::with_capacity(asks.len());
        self.ask_each(asks, bound, cancel, |place, answered| {
            let (Some(stage), Some(question)) = (stages.get(place), asks.questions().get(place))
            else {
                return Err(Error::RecognizeLogical);
            };
            meta.trace
                .take(stage, question, &answered.sources, &answered.observations);
            observe(stage, question, &answered)?;
            for outcome in answered.reply.outcomes() {
                match outcome {
                    AnswerOutcome::Answered(answer) => answers.push(answer.clone()),
                    AnswerOutcome::Failed(_) => return Err(Error::RecognizeLogical),
                }
            }
            meta.add_answered(&answered, self.backend.model())
        })?;
        Ok(answers)
    }
}

/// A text's pieces, its step-1 questions, and the requests they make with
/// nothing cached.
pub(crate) type StepOne = (Vec<Piece>, Asks, Vec<Request>);

/// Refuse a text over `limit` bytes, then split it into pieces and pack
/// every step-1 question. A kind question the profile refuses stops the text
/// here, before any request.
pub(crate) fn step_one(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    spec: &RecognizeSpec,
    text: &str,
    limit: usize,
) -> Result<StepOne, Error> {
    step_one_context(backend, profile, spec, text, limit, None)
}

fn step_one_context(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    spec: &RecognizeSpec,
    text: &str,
    limit: usize,
    context: Option<&str>,
) -> Result<StepOne, Error> {
    if text.len() > limit {
        return Err(Error::TextTooLong {
            bytes: text.len(),
            limit,
        });
    }
    let pieces = pieces(text);
    if pieces.is_empty() {
        return Ok((pieces, Asks::default(), Vec::new()));
    }
    let kinds: Vec<&str> = spec.kinds.iter().map(|(kind, _)| kind.as_str()).collect();
    if !kinds.is_empty() {
        let probe =
            kind_question(text, &pieces, (0, 0), &spec.kinds).map_err(|_| Error::RecognizeKinds)?;
        let mut alone = Asks::default();
        alone.add(
            backend,
            &window_plan(backend, (text, &pieces), (0, 0), vec![probe])?,
        )?;
        contextual_requests(&alone, backend, profile, context)?;
    }
    let mut asks = Asks::default();
    for group in step_one_groups(pieces.len()) {
        let last = group.end.saturating_sub(1);
        let questions = step_one_questions(text, &pieces, group.clone(), &kinds)
            .map_err(|_| Error::Defect("fixed boundary questions are invalid"))?;
        asks.add(
            backend,
            &window_plan(backend, (text, &pieces), (group.start, last), questions)?,
        )?;
    }
    let requests = contextual_requests(&asks, backend, profile, context)?;
    Ok((pieces, asks, requests))
}

/// Step 2's questions, what each name asked, and each question's stage.
type StepTwo = (Asks, Vec<Asked>, Vec<&'static str>);

/// Step 2's questions over the names step 1 found: each name group's kind
/// and edge questions in one window, beside what each name asked and each
/// question's stage.
fn step_two(
    backend: &Backend,
    (text, pieces): (&str, &[Piece]),
    stretches: &[(usize, usize)],
    spec: &RecognizeSpec,
) -> Result<StepTwo, Error> {
    let mut asks = Asks::default();
    let mut asked: Vec<Asked> = Vec::new();
    let mut stages = Vec::new();
    for group in name_groups(stretches) {
        let (questions, held) =
            step_two_questions(text, pieces, stretches, group.clone(), &spec.kinds)
                .map_err(|_| Error::Defect("a step-two question has invalid labels"))?;
        stages.extend(asked_stages(&held));
        asked.extend(held);
        let first = stretches.get(group.start).map_or(0, |name| name.0);
        let last = stretches
            .get(group.end.saturating_sub(1))
            .map_or(first, |name| name.1);
        if !questions.is_empty() {
            asks.add(
                backend,
                &window_plan(backend, (text, pieces), (first, last), questions)?,
            )?;
        }
    }
    Ok((asks, asked, stages))
}

/// The plan of one step-1 or step-2 window over the pieces `first` to `last`.
fn window_plan(
    backend: &Backend,
    source: (&str, &[Piece]),
    stretch: (usize, usize),
    questions: Vec<Question>,
) -> Result<Plan, Error> {
    let (text, pieces) = source;
    let evidence = text
        .get(window(pieces, stretch.0, stretch.1))
        .and_then(|part| Evidence::new(part).ok())
        .ok_or(Error::Defect("a recognize window is blank"))?;
    Plan::new(
        evidence,
        backend.model().clone(),
        backend.descriptions(),
        questions,
    )
    .map_err(|_| Error::Defect("a recognize request asks nothing"))
}

/// One piece's five tag probabilities, in table order.
fn tag_row(answer: &Answer) -> Result<TagRow, Error> {
    let held = answer
        .choice_probabilities()
        .ok_or(Error::RecognizeLogical)?;
    let mut row = [0.0; 5];
    for (tag, slot) in TAGS.iter().zip(row.iter_mut()) {
        *slot = held
            .iter()
            .find_map(|(label, value)| (label == tag).then_some(*value))
            .ok_or(Error::RecognizeLogical)?;
    }
    Ok(row)
}

fn odds(answer: &Answer) -> Result<Odds, Error> {
    answer
        .choice_probabilities()
        .map(|held| {
            Odds(
                held.into_iter()
                    .map(|(label, value)| (label.to_owned(), value))
                    .collect(),
            )
        })
        .ok_or(Error::RecognizeLogical)
}

impl Aggregate {
    /// Keep the first model the replies named and refuse a second, naming
    /// both only when each is safe to print, as `annotate` does.
    fn add_answered(&mut self, answered: &Answered, requested: &ModelName) -> Result<(), Error> {
        self.models.take(answered, |held, model| {
            super::annotate::check_model(held, model, requested)
        })?;
        self.shares.add_reported(answered.reply.reported_usage());
        self.live |= !answered.replayed;
        self.requests_sent = self
            .requests_sent
            .checked_add(answered.requests_sent)
            .ok_or(Error::UsageOverflow)?;
        self.requests.push(answered.request.as_str().to_owned());
        Ok(())
    }
}
