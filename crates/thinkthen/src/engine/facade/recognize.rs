//! Recognition in three steps (ADR 0056): BILOU boundaries over windowed
//! requests, kinds and edges per step-1 request, then stated relations.

use serde::Serialize;

use super::{Answered, Engine};
use crate::core::{
    Answer, AnswerOutcome, Asked, Backend, BackendProfile, Evidence, ModelName, NameOdds, Odds,
    Piece, PieceOdds, Plan, Question, RecognizeSpec, RecognizedName, RelationEdge, TAGS, TagRow,
    Usage, found_names, kind_question, name_groups, pieces, plan_stated, settle_names,
    stated_edges, step_one_groups, step_one_questions, step_two_questions, window,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::prepared_request::{PreparedChunk, PreparedRequests, relation_ceiling};

/// The default limit on one text's UTF-8 bytes, which caps spending.
pub(crate) const MAX_TEXT_BYTES: usize = 600_000;

/// The most pair questions one step-3 request holds.
const MOST_PAIRS: usize = 400;

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

/// The request metadata of every stage, in construction order.
#[derive(Debug)]
pub(crate) struct Aggregate {
    pub(crate) model: Option<ModelName>,
    pub(crate) usage: Option<Usage>,
    pub(crate) replayed: bool,
    pub(crate) requests_sent: u64,
    pub(crate) requests: Vec<String>,
}

impl Default for Aggregate {
    fn default() -> Self {
        Self {
            model: None,
            usage: None,
            replayed: true,
            requests_sent: 0,
            requests: Vec::new(),
        }
    }
}

/// One text's recognition, its probabilities, and the metadata.
#[derive(Debug)]
pub(crate) struct Recognition {
    pub(crate) value: Recognized,
    pub(crate) details: Probabilities,
    pub(crate) meta: Aggregate,
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
        let answers = self.execute(prepared, &mut meta, cancel)?;
        let rows = answers.iter().map(tag_row).collect::<Result<Vec<_>, _>>()?;
        details.pieces = pieces
            .iter()
            .zip(&rows)
            .map(|(piece, row)| PieceOdds::new(piece, row))
            .collect();
        let stretches = found_names(&rows);
        let mut prepared = Vec::new();
        let mut asked: Vec<Asked> = Vec::new();
        for group in name_groups(&stretches) {
            let (questions, held) =
                step_two_questions(text, &pieces, &stretches, group.clone(), &spec.kinds)
                    .map_err(|_| Error::RecognizeKinds)?;
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
        let answers = self.execute(prepared, &mut meta, cancel)?;
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
        let relations = self.relations(spec, text, &entities, (&mut meta, &mut details), cancel)?;
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
        held: (&mut Aggregate, &mut Probabilities),
        cancel: &Cancel,
    ) -> Result<Option<Vec<RelationEdge<RecognizedName>>>, Error> {
        if spec.relations.is_empty() {
            return Ok(None);
        }
        let Some(planned) = plan_stated(text, entities, &spec.relations)
            .map_err(|_| Error::Defect("relation planning failed"))?
        else {
            return Ok(Some(Vec::new()));
        };
        let ceiling = relation_ceiling(&self.backend, self.profile.as_ref());
        let mut prepared = Vec::new();
        for questions in planned.questions.chunks(MOST_PAIRS) {
            let plan = Plan::new(
                planned.evidence.clone(),
                self.backend.model().clone(),
                questions.to_vec(),
            )
            .map_err(|_| Error::Defect("relation planned no questions"))?;
            prepared.extend(
                PreparedRequests::with_profile(
                    &self.backend,
                    &plan,
                    self.profile.as_ref(),
                    ceiling,
                )?
                .into_chunks(),
            );
        }
        let (meta, details) = held;
        let answers = self.execute(prepared, meta, cancel)?;
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
        Ok(Some(stated_edges(
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
        meta: &mut Aggregate,
        cancel: &Cancel,
    ) -> Result<Vec<Answer>, Error> {
        let mut answers = Vec::new();
        self.ask_chunks(prepared, cancel, |answered| {
            for outcome in answered.reply.outcomes() {
                match outcome {
                    AnswerOutcome::Answered(answer) => answers.push(answer.clone()),
                    AnswerOutcome::Failed(_) => return Err(Error::RecognizeLogical),
                }
            }
            meta.add_answered(&answered)
        })?;
        Ok(answers)
    }
}

/// Refuse a text over `limit` bytes, then split it into pieces and prepare
/// every step-1 request. A kind question the profile refuses stops the text
/// here, before any request.
pub(crate) fn step_one(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    spec: &RecognizeSpec,
    text: &str,
    limit: usize,
) -> Result<(Vec<Piece>, Vec<PreparedChunk>), Error> {
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
    fn add_answered(&mut self, answered: &Answered) -> Result<(), Error> {
        if let Some(model) = &self.model {
            if model != answered.reply.model() {
                return Err(Error::ModelsDiffer(Some((
                    model.as_str().to_owned(),
                    answered.reply.model().as_str().to_owned(),
                ))));
            }
        } else {
            self.model = Some(answered.reply.model().clone());
        }
        self.usage = match (self.usage, answered.reply.usage()) {
            (Some(left), Some(right)) => left
                .checked_plus(right)
                .ok_or(Error::UsageOverflow)
                .map(Some)?,
            (None, held) | (held, None) => held,
        };
        self.replayed &= answered.replayed;
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
