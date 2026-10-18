//! One eager result and the count-only facts of the call that produced it.

use std::fmt;

use serde::{Serialize, Serializer, ser::SerializeStruct};
use serde_json::value::RawValue;

use crate::core::{CallId, Prices};
use crate::engine::call_facts::Snapshot;
use crate::public::error::Error;

/// Count-only facts fixed when one call and all of its workers finish.
///
/// It serializes with its members in name order, as the C interface prints them.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "facts"))]
pub struct Facts {
    #[serde(skip)]
    #[cfg_attr(test, schemars(skip))]
    pub(super) attempts: Option<Vec<crate::public::AttemptObservation>>,
    // Explicit facts serialization stays count-only; complete projection adopts this ID.
    #[serde(skip)]
    #[cfg_attr(test, schemars(skip))]
    pub(super) call_id: Option<CallId>,
    #[serde(skip)]
    #[cfg_attr(test, schemars(skip))]
    pub(super) usage_persistence: Option<super::UsagePersistence>,
    #[serde(skip)]
    #[cfg_attr(test, schemars(skip))]
    pub(super) held_model_mismatch: bool,
    pub(super) cache_answers: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "String"))]
    pub(super) estimated_cost_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "u64"))]
    pub(super) input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "String"))]
    pub(super) model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "u64"))]
    pub(super) output_tokens: Option<u64>,
    pub(super) records: u64,
    pub(super) requests_sent: u64,
    pub(super) largest_request_bytes: usize,
    pub(super) largest_request_estimated_input_tokens: Option<u64>,
    pub(super) token_estimate_method: &'static str,
    pub(super) seconds: f64,
}

impl fmt::Debug for Facts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Facts")
            .field("records", &self.records)
            .field("requests_sent", &self.requests_sent)
            .field("cache_answers", &self.cache_answers)
            .field("input_tokens", &self.input_tokens)
            .field("output_tokens", &self.output_tokens)
            .field("estimated_cost_usd", &self.estimated_cost_usd.is_some())
            .field("seconds", &self.seconds)
            .field("model", &self.model.is_some())
            .finish()
    }
}

impl Facts {
    pub(crate) fn of(snapshot: Snapshot, prices: Option<Prices>) -> Self {
        let tokens = snapshot.tokens.map(crate::core::Usage::token_counts);
        let estimated_cost_usd = prices.and_then(|prices| {
            if !snapshot.cost_complete {
                return None;
            }
            let (input, output) = tokens.unwrap_or((0, 0));
            prices.estimate(input, output)
        });
        Self {
            attempts: snapshot.attempts,
            call_id: snapshot.call_id,
            usage_persistence: snapshot.usage_persistence,
            held_model_mismatch: snapshot.held_model_mismatch,
            records: snapshot.records,
            requests_sent: snapshot.requests_sent,
            largest_request_bytes: snapshot.largest_request_bytes,
            largest_request_estimated_input_tokens: snapshot.largest_request_estimated_input_tokens,
            token_estimate_method: crate::core::PlanSummary::TOKEN_ESTIMATE_METHOD,
            cache_answers: snapshot.cache_answers,
            input_tokens: snapshot
                .reported
                .and_then(crate::core::ReportedUsage::input_tokens),
            output_tokens: snapshot
                .reported
                .and_then(crate::core::ReportedUsage::output_tokens),
            estimated_cost_usd,
            seconds: snapshot.elapsed.as_secs_f64(),
            model: snapshot.model,
        }
    }

    /// This engine's persistence observed when these facts were snapshotted.
    /// This immutable observation cannot describe a later asynchronous write.
    /// Aggregate tallies and calls without engine state have no observation.
    #[must_use]
    pub const fn usage_persistence(&self) -> Option<super::UsagePersistence> {
        self.usage_persistence
    }

    /// Whether an excluded held answer reported a different model on this route.
    /// This is call-scoped and contains no stored model, address or credential.
    #[must_use]
    pub const fn held_model_mismatch(&self) -> bool {
        self.held_model_mismatch
    }

    /// Ordered input records finished before a stop, including filtered rows.
    #[must_use]
    pub const fn records(&self) -> u64 {
        self.records
    }

    /// This invocation's opaque ID, absent on a caller's aggregate tally.
    #[must_use]
    pub const fn call_id(&self) -> Option<&CallId> {
        self.call_id.as_ref()
    }

    /// Requested ordered live attempts, retained even after a started failure.
    /// None means unrequested; Some([]) means requested with zero sends.
    #[must_use]
    pub fn attempts(&self) -> Option<&[crate::public::AttemptObservation]> {
        self.attempts.as_deref()
    }

    /// Live transport attempts, including retries and failed attempts.
    #[must_use]
    pub const fn requests_sent(&self) -> u64 {
        self.requests_sent
    }

    /// Answers obtained from the answer cache without a live send.
    #[must_use]
    pub const fn cache_answers(&self) -> u64 {
        self.cache_answers
    }

    /// Backend-reported input tokens, absent when a live reply omitted usage.
    #[must_use]
    pub const fn input_tokens(&self) -> Option<u64> {
        self.input_tokens
    }

    /// Backend-reported output tokens, absent when a live reply omitted usage.
    #[must_use]
    pub const fn output_tokens(&self) -> Option<u64> {
        self.output_tokens
    }

    /// Caller-priced estimate from complete reported usage, if configured.
    #[must_use]
    pub fn estimated_cost_usd(&self) -> Option<&str> {
        self.estimated_cost_usd.as_deref()
    }

    /// Wall seconds from call start through worker join and final stop check.
    #[must_use]
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The reported model, only when all live or stored replies agree.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }
}

/// One eager value with its call-scoped facts.
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "CompleteCall"))]
pub struct Call<T, R = (), P = Option<f64>> {
    value: T,
    answer: Option<Box<super::SettledAnswer>>,
    #[cfg_attr(test, schemars(skip))]
    results: R,
    #[cfg_attr(test, schemars(skip))]
    probability: P,
    #[cfg_attr(test, schemars(skip))]
    projected: bool,
    #[cfg_attr(test, schemars(with = "super::CompleteFacts<'static>"))]
    facts: Facts,
}

impl<T: Serialize, R: Serialize, P: Serialize> Serialize for Call<T, R, P> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let facts = self
            .facts
            .complete()
            .ok_or_else(|| serde::ser::Error::custom("a completed call has no identity"))?;
        let mut document =
            serializer.serialize_struct("Call", if self.projected { 5 } else { 3 })?;
        document.serialize_field("value", &self.value)?;
        if let Some(answer) = &self.answer {
            document.serialize_field("answer", answer)?;
        }
        if self.projected {
            document.serialize_field("probability", &self.probability)?;
            document.serialize_field("results", &self.results)?;
        }
        document.serialize_field("facts", &facts)?;
        document.end()
    }
}

impl<T, R, P> fmt::Debug for Call<T, R, P> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Call")
            .field("facts", &self.facts)
            .finish_non_exhaustive()
    }
}

impl<T> Call<T> {
    pub(crate) const fn new(value: T, facts: Facts) -> Self {
        Self {
            value,
            answer: None,
            facts,
            results: (),
            probability: None,
            projected: false,
        }
    }
}

impl<T, R, P> Call<T, R, P> {
    /// The Rust-settled answer of a canonical native request.
    #[must_use]
    pub fn answer(&self) -> Option<&super::SettledAnswer> {
        self.answer.as_deref()
    }

    pub(crate) fn with_answer(mut self, answer: super::SettledAnswer) -> Self {
        self.answer = Some(Box::new(answer));
        self
    }

    /// Canonical complete records retained by this call.
    #[must_use]
    pub const fn results(&self) -> &R {
        &self.results
    }

    /// The scalar or aligned item probabilities of the plain answer.
    #[must_use]
    pub const fn probability(&self) -> &P {
        &self.probability
    }

    pub(crate) fn project<U, S, Q>(
        self,
        project: impl FnOnce(T) -> Result<(U, S, Q), Error>,
    ) -> Result<Call<U, S, Q>, Error>
    where
        S: super::settled_carriers::SettledCarrier,
    {
        match project(self.value) {
            Ok((value, results, probability)) => {
                let answer = match self.answer {
                    Some(answer) => answer,
                    None => Box::new(results
                        .settled_answer()
                        .map_err(|error| error.with_facts(self.facts.clone()))?),
                };
                Ok(Call {
                    value,
                    answer: Some(answer),
                    results,
                    probability,
                    facts: self.facts,
                    projected: true,
                })
            }
            Err(error) => Err(error.with_facts(self.facts)),
        }
    }

    /// Borrow the value without losing its facts.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Take the value when an adapter retains an older outward shape.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// The immutable facts of this completed call.
    #[must_use]
    pub const fn facts(&self) -> &Facts {
        &self.facts
    }

    pub(crate) fn map<U>(self, map: impl FnOnce(T) -> U) -> Call<U, R, P> {
        Call {
            value: map(self.value),
            answer: self.answer,
            facts: self.facts,
            results: self.results,
            probability: self.probability,
            projected: self.projected,
        }
    }

    pub(crate) fn try_map<U>(
        self,
        map: impl FnOnce(T) -> Result<U, Error>,
    ) -> Result<Call<U, R, P>, Error> {
        match map(self.value) {
            Ok(value) => Ok(Call {
                value,
                answer: self.answer,
                facts: self.facts,
                results: self.results,
                probability: self.probability,
                projected: self.projected,
            }),
            Err(error) => Err(error.with_facts(self.facts)),
        }
    }
}

/// The C interface's success reply: the bare value, the call's facts, and the
/// attempts when the request asked for them.
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "callSuccess"))]
pub struct CallReply {
    value: Box<RawValue>,
    facts: Facts,
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<Vec<crate::public::AttemptObservation>>,
}

/// The value may hold record text, so `Debug` shows the facts alone.
impl fmt::Debug for CallReply {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CallReply")
            .field("facts", &self.facts)
            .finish_non_exhaustive()
    }
}

impl CallReply {
    /// Gather one reply from the value's JSON text, its facts, and any attempts.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Defect`] when `value` is not one JSON value.
    pub fn new(
        value: String,
        facts: Facts,
        attempts: Option<Vec<crate::public::AttemptObservation>>,
    ) -> Result<Self, Error> {
        let value =
            RawValue::from_string(value).map_err(|_| Error::defect("a reply value is not JSON"))?;
        Ok(Self {
            value,
            facts,
            attempts,
        })
    }
}

/// One nullable yes/no answer and its complete record.
pub type DecisionCall = Call<Option<bool>, Vec<super::CompleteDecision>>;
/// One typed choice and its complete record.
pub type ChoiceCall<C> = Call<Option<C>, Vec<super::CompleteChoice>>;
/// One weighted score and its complete record.
pub type ScoreCall = Call<f64, Vec<super::CompleteScore>>;
/// Selected typed labels and the complete tag record.
pub type TagCall<C> = Call<Vec<C>, Vec<super::CompleteTags>>;
/// Selected original records with every input's filter judgment.
pub type FilterCall<T> =
    Call<Vec<T>, Vec<super::CompleteRecord<T, super::CompleteFilter>>, Vec<Option<f64>>>;
/// Original indexes in ranked order with complete ranked records.
pub type RankCall<T> =
    Call<Vec<usize>, Vec<super::CompleteRecord<T, super::CompleteRank>>, Vec<Option<f64>>>;
/// Original indexes selected by a question set and their complete records.
pub type SetRankCall<T> =
    Call<Vec<usize>, Vec<super::CompleteRecord<T, super::CompleteSetRank>>, Vec<Option<f64>>>;
/// One optional original unit and its complete candidate set.
pub type FindCall<T> = Call<Option<T>, super::CompleteFound<T>>;
/// Named values on every input and the complete annotation records.
pub type AnnotationCall<T> =
    Call<Vec<Vec<crate::NamedAnnotation>>, Vec<super::CompleteRecord<T, super::CompleteAnnotated>>>;
/// Recognized names and the complete staged record.
pub type RecognitionCall =
    Call<crate::Recognized, Vec<super::CompleteRecognized>, Vec<Option<f64>>>;
/// Selected relation edges and the complete aggregate record.
pub type RelationCall = Call<Vec<crate::Edge>, Vec<super::CompleteRelated>, Vec<Option<f64>>>;
