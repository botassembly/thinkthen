//! Recognition in three steps (ADR 0056): BILOU boundaries over windowed
//! requests, kinds and edges per step-1 request, then stated relations.

use serde::Serialize;

use super::{Answered, Engine};
use crate::core::relation::count_pairs;
use crate::core::{
    Answer, AnswerOutcome, Asked, Backend, BackendProfile, Evidence, Lead, ModelName, NameOdds,
    Odds, Piece, PieceOdds, Plan, Question, RecognizeSpec, RecognizedName, RelationEdge, TAGS,
    TagRow, Usage, found_names, kind_question, name_groups, pair_edges, pieces, plan_pairs,
    settle_names, step_one_groups, step_one_questions, step_two_questions, window,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::prepared_request::{PreparedChunk, PreparedRequests, pair_chunks};

/// The default limit on one text's UTF-8 bytes, which caps spending.
pub(crate) const MAX_TEXT_BYTES: usize = 600_000;
const MAX_RELATION_NAMES: usize = 255;
const MAX_RELATION_QUESTIONS: usize = 4_000;

/// The names one text holds, and the edges between them when rules were given.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Recognized {
    pub(crate) entities: Vec<RecognizedName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) relations: Option<Vec<RelationEdge<RecognizedName>>>,
}

/// Every probability behind one text's names, as `--details` keeps them.
#[derive(Debug, Default, Serialize)]
pub(crate) struct Probabilities {
    pub(crate) pieces: Vec<PieceOdds>,
    pub(crate) names: Vec<NameOdds>,
    pub(crate) pairs: Vec<PairOdds>,
}

/// One asked pair's probability.
#[derive(Debug, Serialize)]
pub(crate) struct PairOdds {
    relation: String,
    source: Place,
    target: Place,
    probability: f64,
}

#[derive(Debug, Serialize)]
struct Place {
    start: usize,
    end: usize,
}

impl Place {
    const fn of(name: &RecognizedName) -> Self {
        Self {
            start: name.start,
            end: name.end,
        }
    }
}

/// The request metadata of every stage, in construction order. `live` says
/// whether any answer came from the backend rather than a recording or cache.
#[derive(Debug, Default)]
pub(crate) struct Aggregate {
    pub(crate) model: Option<ModelName>,
    pub(crate) usage: Option<Usage>,
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

    /// The same call with each actual answered request plan handed to its caller.
    pub(crate) fn recognize_observed(
        &self,
        spec: &RecognizeSpec,
        text: &str,
        limit: usize,
        cancel: &Cancel,
        mut observe: impl FnMut(&[&'static str], &Plan, &Answered) -> Result<(), Error>,
    ) -> Result<Recognition, Error> {
        let (pieces, prepared) = step_one(&self.backend, self.profile.as_ref(), spec, text, limit)?;
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
        let stages = vec![
            "boundary";
            prepared
                .iter()
                .map(|chunk| chunk.plan.questions().len())
                .sum()
        ];
        let answers = self.execute(prepared, stages, &mut meta, cancel, &mut observe)?;
        let rows = answers.iter().map(tag_row).collect::<Result<Vec<_>, _>>()?;
        details.pieces = pieces
            .iter()
            .zip(&rows)
            .map(|(piece, row)| PieceOdds::new(piece, row))
            .collect();
        let stretches = found_names(&rows);
        let mut prepared = Vec::new();
        let mut asked: Vec<Asked> = Vec::new();
        let mut stages = Vec::new();
        for group in name_groups(&stretches) {
            let (questions, held) =
                step_two_questions(text, &pieces, &stretches, group.clone(), &spec.kinds)
                    .map_err(|_| Error::Defect("a step-two question has invalid labels"))?;
            stages.extend(asked_stages(&held));
            asked.extend(held);
            let first = stretches.get(group.start).map_or(0, |name| name.0);
            let last = stretches
                .get(group.end.saturating_sub(1))
                .map_or(first, |name| name.1);
            if !questions.is_empty() {
                prepared.extend(prepare(
                    &self.backend,
                    self.profile.as_ref(),
                    (text, &pieces),
                    (first, last),
                    questions,
                )?);
            }
        }
        let answers = self.execute(prepared, stages, &mut meta, cancel, &mut observe)?;
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
        let entities = settle_names((text, &pieces), &rows, &stretches, &settled, cut);
        let relations = self.relations(
            spec,
            text,
            &entities,
            (&mut meta, &mut details, &mut observe),
            cancel,
        )?;
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
            &mut impl FnMut(&[&'static str], &Plan, &Answered) -> Result<(), Error>,
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
        let prepared = pair_chunks(&self.backend, self.profile.as_ref(), &planned)?;
        let (meta, details, observe) = held;
        let stages = vec![
            "relation";
            prepared
                .iter()
                .map(|chunk| chunk.plan.questions().len())
                .sum()
        ];
        let answers = self.execute(prepared, stages, meta, cancel, observe)?;
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

    /// Send requests prepared earlier; one failed question fails the text.
    fn execute(
        &self,
        prepared: Vec<PreparedChunk>,
        stages: Vec<&'static str>,
        meta: &mut Aggregate,
        cancel: &Cancel,
        observe: &mut impl FnMut(&[&'static str], &Plan, &Answered) -> Result<(), Error>,
    ) -> Result<Vec<Answer>, Error> {
        let mut answers = Vec::new();
        let mut stages = stages.into_iter();
        self.ask_chunks_with_plan(prepared, cancel, |plan, answered| {
            let chunk_stages = plan
                .questions()
                .iter()
                .map(|_| stages.next().ok_or(Error::RecognizeLogical))
                .collect::<Result<Vec<_>, _>>()?;
            observe(&chunk_stages, plan, &answered)?;
            for outcome in answered.reply.outcomes() {
                match outcome {
                    AnswerOutcome::Answered(answer) => answers.push(answer.clone()),
                    AnswerOutcome::Failed(_) => return Err(Error::RecognizeLogical),
                }
            }
            meta.add_answered(&answered, self.backend.model())
        })?;
        if stages.next().is_some() {
            return Err(Error::RecognizeLogical);
        }
        Ok(answers)
    }
}

/// A text's pieces and its prepared step-1 requests.
pub(crate) type StepOne = (Vec<Piece>, Vec<PreparedChunk>);

/// Refuse a text over `limit` bytes, then split it into pieces and prepare
/// every step-1 request. A kind question the profile refuses stops the text
/// here, before any request.
pub(crate) fn step_one(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    spec: &RecognizeSpec,
    text: &str,
    limit: usize,
) -> Result<StepOne, Error> {
    if text.len() > limit {
        return Err(Error::TextTooLong {
            bytes: text.len(),
            limit,
        });
    }
    let pieces = pieces(text);
    if pieces.is_empty() {
        return Ok((pieces, Vec::new()));
    }
    let kinds: Vec<&str> = spec.kinds.iter().map(|(kind, _)| kind.as_str()).collect();
    if !kinds.is_empty() {
        let probe =
            kind_question(text, &pieces, (0, 0), &spec.kinds).map_err(|_| Error::RecognizeKinds)?;
        prepare(backend, profile, (text, &pieces), (0, 0), vec![probe])?;
    }
    let mut prepared = Vec::new();
    for group in step_one_groups(pieces.len()) {
        let last = group.end.saturating_sub(1);
        let questions = step_one_questions(text, &pieces, group.clone(), &kinds)
            .map_err(|_| Error::Defect("fixed boundary questions are invalid"))?;
        prepared.extend(prepare(
            backend,
            profile,
            (text, &pieces),
            (group.start, last),
            questions,
        )?);
    }
    Ok((pieces, prepared))
}

/// Prepare one step-1 or step-2 request over the window of pieces `first` to `last`.
fn prepare(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    source: (&str, &[Piece]),
    stretch: (usize, usize),
    questions: Vec<Question>,
) -> Result<Vec<PreparedChunk>, Error> {
    let (text, pieces) = source;
    let evidence = text
        .get(window(pieces, stretch.0, stretch.1))
        .and_then(|part| Evidence::new(part).ok())
        .ok_or(Error::Defect("a recognize window is blank"))?;
    let plan = Plan::new(evidence, backend.model().clone(), questions)
        .map_err(|_| Error::Defect("a recognize request asks nothing"))?;
    Ok(PreparedRequests::with_profile(backend, &plan, profile, None)?.into_chunks())
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
        super::annotate::check_model(&mut self.model, answered.reply.model(), requested)?;
        self.usage = match (self.usage, answered.reply.usage()) {
            (Some(left), Some(right)) => left
                .checked_plus(right)
                .ok_or(Error::UsageOverflow)
                .map(Some)?,
            (None, held) | (held, None) => held,
        };
        self.live |= !answered.replayed;
        self.requests_sent = self
            .requests_sent
            .checked_add(answered.requests_sent)
            .ok_or(Error::UsageOverflow)?;
        self.requests.push(answered.request.as_str().to_owned());
        Ok(())
    }
}

#[cfg(test)]
mod tests;
