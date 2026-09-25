//! Staged recognition over the shared splitter, name assembler, and relation planner.

use serde::Serialize;

use super::{Answered, Engine};
use crate::core::{
    Answer, AnswerOutcome, Description, Evidence, ModelName, Plan, RecognizeSpec, RecognizedName,
    RelationEdge, TokenAnswer, Usage, Withheld, assemble_edges, assemble_names, kind_questions,
    plan_relation, recognition_questions, tokenize,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::prepared_request::{PreparedRequests, SettledRelation};

/// The names one text holds, and the edges between them when rules were given.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Recognized {
    pub(crate) entities: Vec<RecognizedName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) relations: Option<Vec<RelationEdge<RecognizedName>>>,
}

/// The probabilities behind one token's strength.
#[derive(Serialize)]
pub(crate) struct TokenInput {
    pub(crate) token: String,
    pub(crate) detection_probability: f64,
    pub(crate) kind_probabilities: Vec<f64>,
}

/// A token is evidence, so `Debug` withholds it and keeps the probabilities.
impl std::fmt::Debug for TokenInput {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TokenInput")
            .field("token", &Withheld(self.token.len()))
            .field("detection_probability", &self.detection_probability)
            .field("kind_probabilities", &self.kind_probabilities)
            .finish()
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

/// One text's recognition, the token probabilities, and the metadata.
#[derive(Debug)]
pub(crate) struct Recognition {
    pub(crate) value: Recognized,
    pub(crate) inputs: Vec<TokenInput>,
    pub(crate) meta: Aggregate,
}

impl Engine {
    /// Recognize the names in one text, then relate them when rules were given.
    pub(crate) fn recognize(
        &self,
        spec: &RecognizeSpec,
        text: &str,
        cancel: &Cancel,
    ) -> Result<Recognition, Error> {
        let tokens = tokenize(text);
        if tokens.is_empty() {
            return Ok(Recognition {
                value: Recognized {
                    entities: Vec::new(),
                    relations: (!spec.relations.is_empty()).then(Vec::new),
                },
                inputs: Vec::new(),
                meta: Aggregate::default(),
            });
        }
        let mut questions = recognition_questions(&tokens)
            .map_err(|_| Error::Defect("fixed detection questions are invalid"))?;
        questions.extend(kind_questions(&tokens, &spec.kinds).map_err(|_| Error::RecognizeKinds)?);
        let evidence =
            Evidence::new(text).map_err(|_| Error::Defect("record evidence became blank"))?;
        let plan = Plan::new(evidence, self.backend.model().clone(), questions)
            .map_err(|_| Error::Defect("recognize planned no token questions"))?;
        let prepared =
            PreparedRequests::with_profile(&self.backend, &plan, self.profile.as_ref(), None)?;
        let (answers, mut meta) = self.execute(prepared, cancel)?;
        let token_answers = token_answers(&answers, tokens.len(), &spec.kinds)?;
        let kind_names = spec
            .kinds
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        let entities = assemble_names(
            text,
            &tokens,
            &token_answers,
            &kind_names,
            spec.threshold.cut_value().unwrap_or(0.5),
        );
        let inputs = tokens
            .iter()
            .zip(&token_answers)
            .map(|(token, answer)| TokenInput {
                token: token.text().to_owned(),
                detection_probability: answer.detection_probability,
                kind_probabilities: answer.kind_probabilities.clone(),
            })
            .collect();
        let relations = self.relations(spec, text, &entities, &mut meta, cancel)?;
        Ok(Recognition {
            value: Recognized {
                entities,
                relations,
            },
            inputs,
            meta,
        })
    }

    fn relations(
        &self,
        spec: &RecognizeSpec,
        source: &str,
        entities: &[RecognizedName],
        meta: &mut Aggregate,
        cancel: &Cancel,
    ) -> Result<Option<Vec<RelationEdge<RecognizedName>>>, Error> {
        if spec.relations.is_empty() {
            return Ok(None);
        }
        let mut prepared = Vec::new();
        for rule in &spec.relations {
            let plans = plan_relation(entities, rule)
                .map_err(|_| Error::Defect("relation planning failed"))?;
            for planned in plans
                .into_iter()
                .filter(|planned| !planned.questions.is_empty())
            {
                let settled = SettledRelation::settle(
                    &self.backend,
                    self.profile.as_ref(),
                    Some(source),
                    entities,
                    planned,
                )?;
                prepared.push((settled.planned, settled.requests));
            }
        }
        let mut edges = Vec::new();
        for (planned, requests) in prepared {
            let (answers, stage) = self.execute(requests, cancel)?;
            meta.add(stage)?;
            edges.extend(assemble_edges(
                entities,
                &planned.relation,
                &planned.mappings,
                &answers,
                spec.relation_threshold.cut_value().unwrap_or(0.5),
            ));
        }
        Ok(Some(edges))
    }

    /// Send requests prepared earlier; one failed question fails the record.
    fn execute(
        &self,
        prepared: PreparedRequests,
        cancel: &Cancel,
    ) -> Result<(Vec<Answer>, Aggregate), Error> {
        let mut answers = Vec::new();
        let mut meta = Aggregate::default();
        self.ask_chunks(prepared.into_chunks(), cancel, |answered| {
            for outcome in answered.reply.outcomes() {
                match outcome {
                    AnswerOutcome::Answered(answer) => answers.push(answer.clone()),
                    AnswerOutcome::Failed(_) => return Err(Error::RecognizeLogical),
                }
            }
            meta.add_answered(&answered)
        })?;
        Ok((answers, meta))
    }
}

fn token_answers(
    answers: &[Answer],
    count: usize,
    kinds: &[(String, Option<Description>)],
) -> Result<Vec<TokenAnswer>, Error> {
    let mut built = Vec::with_capacity(count);
    for place in 0..count {
        let detection = answers.get(place).ok_or(Error::RecognizeLogical)?;
        let detection_probabilities = detection
            .choice_probabilities()
            .ok_or(Error::RecognizeLogical)?;
        let in_probability = detection_probabilities
            .iter()
            .find_map(|(name, value)| (*name == "IN").then_some(*value))
            .ok_or(Error::RecognizeLogical)?;
        let out_probability = detection_probabilities
            .iter()
            .find_map(|(name, value)| (*name == "OUT").then_some(*value))
            .ok_or(Error::RecognizeLogical)?;
        let kind_probabilities = if kinds.len() == 1 {
            Vec::new()
        } else {
            answers
                .get(count + place)
                .and_then(Answer::choice_probabilities)
                .ok_or(Error::RecognizeLogical)?
        };
        let values = kinds
            .iter()
            .map(|(name, _)| {
                if kinds.len() == 1 {
                    return 1.0;
                }
                kind_probabilities
                    .iter()
                    .find_map(|(held, value)| (*held == name).then_some(*value))
                    .unwrap_or(0.0)
            })
            .collect::<Vec<_>>();
        let winner = values
            .iter()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
            .map_or(0, |(index, _)| index);
        built.push(TokenAnswer {
            detected: in_probability > out_probability,
            detection_probability: in_probability,
            kind: winner,
            kind_probabilities: values,
        });
    }
    Ok(built)
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

    fn add(&mut self, other: Self) -> Result<(), Error> {
        if let Some(model) = other.model {
            if self.model.as_ref().is_some_and(|held| held != &model) {
                return Err(Error::ModelsDiffer(None));
            }
            self.model.get_or_insert(model);
        }
        self.usage = match (self.usage, other.usage) {
            (Some(left), Some(right)) => left
                .checked_plus(right)
                .ok_or(Error::UsageOverflow)
                .map(Some)?,
            (None, held) | (held, None) => held,
        };
        self.replayed &= other.replayed;
        self.requests_sent = self
            .requests_sent
            .checked_add(other.requests_sent)
            .ok_or(Error::UsageOverflow)?;
        self.requests.extend(other.requests);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
