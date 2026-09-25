//! The engine calls behind the scalars and the rows their answers become.

use thinkthen::{
    Annotated, AnnotatedRecord, Answer, CallOptions, CancelToken, Details, Engine, Error,
    FailureCause, Judgment, Kind, LoadedQuestion, NamedAnnotation, Probabilities, QuestionSet,
    Recognize, Recognized,
};

use crate::errors::{failure, prefix};
use crate::ffi::Value;

/// The controls one call carries: its token and its deadline in milliseconds.
pub(super) fn options(token: &CancelToken, due: Option<i64>) -> Result<CallOptions<'_>, Error> {
    let options = CallOptions::new().cancel(token);
    due.map_or(Ok(options), |millis| options.deadline_millis(millis))
}

pub(crate) fn decided(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: Vec<String>,
    token: &CancelToken,
    due: Option<i64>,
    probability: bool,
) -> Result<Vec<Value>, Error> {
    let options = options(token, due)?;
    let rows: Vec<(Answer, f64)> = match question {
        LoadedQuestion::Question(question) => engine
            .decide_many_with(question, texts, options)
            .map(|row| row.map(|row| (*row.value(), row.probability())))
            .collect::<Result<_, _>>()?,
        LoadedQuestion::Banded(question) => engine
            .decide_many_with(question, texts, options)
            .map(|row| row.map(|row| (*row.value(), row.probability())))
            .collect::<Result<_, _>>()?,
    };
    Ok(rows
        .into_iter()
        .map(|(answer, yes)| match (probability, answer) {
            (true, _) => Value::Double(yes),
            (false, Answer::Yes) => Value::Bool(true),
            (false, Answer::No) => Value::Bool(false),
            (false, Answer::Unsure) => Value::Null,
        })
        .collect())
}

pub(super) fn annotated(
    engine: &Engine,
    set: &QuestionSet,
    texts: Vec<String>,
    token: &CancelToken,
    due: Option<i64>,
) -> Result<Vec<AnnotatedRecord<String>>, Error> {
    engine
        .annotate_with(set, texts, options(token, due)?)
        .collect()
}

/// A member verb's one value, from its one-question set. A question the
/// backend failed reads `backend` with the cause.
pub(super) fn member(record: &AnnotatedRecord<String>) -> Result<Value, String> {
    Ok(match record.values().first().map(NamedAnnotation::value) {
        Some(Annotated::Choice(Some(picked))) => Value::Text(picked.clone()),
        Some(Annotated::Score(position)) => Value::Double(*position),
        Some(Annotated::Tags(held)) => Value::List(held.iter().cloned().map(Value::Text).collect()),
        Some(Annotated::Decision(Answer::Yes)) => Value::Bool(true),
        Some(Annotated::Decision(Answer::No)) => Value::Bool(false),
        Some(Annotated::Failed(failed)) => {
            return Err(format!(
                "{}the backend's answer could not be read: {}",
                prefix(failed.kind()),
                cause(failed.cause())
            ));
        }
        Some(Annotated::Choice(None) | Annotated::Decision(Answer::Unsure)) | None => Value::Null,
    })
}

pub(super) const fn cause(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "the reply omitted the answer",
        FailureCause::WrongKind => "the reply answered another kind of question",
        FailureCause::MissingProbability => "an option or level had no probability",
        FailureCause::InvalidProbability => "a probability fell outside zero to one",
        FailureCause::InvalidDistribution => "a distribution did not total one",
        FailureCause::UnexpectedProbability => "a distribution named an option that was not sent",
    }
}

pub(super) fn detailed(
    engine: &Engine,
    question: &LoadedQuestion,
    texts: &[String],
    token: &CancelToken,
    due: Option<i64>,
) -> Result<Vec<Value>, Error> {
    texts
        .iter()
        .map(|text| {
            let options = options(token, due)?;
            let details = match question {
                LoadedQuestion::Question(question) => {
                    engine.details_with(question, text, options)?
                }
                LoadedQuestion::Banded(question) => engine.details_with(question, text, options)?,
            };
            Ok(details_row(&details))
        })
        .collect()
}

pub(super) fn details_row(details: &Details) -> Value {
    let text =
        |value: Option<&str>| value.map_or(Value::Null, |value| Value::Text(value.to_owned()));
    let (yes, word) = match (details.probabilities(), details.value()) {
        (Probabilities::YesNo { yes }, Judgment::Decision(answer)) => {
            (Value::Double(*yes), text(Some(answer_word(*answer))))
        }
        _ => (Value::Null, Value::Null),
    };
    let value = match details.value() {
        Judgment::Decision(answer) => serde_json::Value::from(answer_word(*answer)),
        Judgment::Choice(picked) => picked
            .clone()
            .map_or(serde_json::Value::Null, serde_json::Value::from),
        Judgment::Score(position) => serde_json::Value::from(*position),
        Judgment::Tags(held) => serde_json::Value::from(held.clone()),
    };
    Value::Struct(vec![
        yes,
        word,
        Value::Text(value.to_string()),
        text(details.nearest()),
        Value::Text(details.model().to_owned()),
        Value::Text(details.question_sha256().to_owned()),
        Value::Int(i64::try_from(details.requests_sent()).unwrap_or(i64::MAX)),
        Value::Bool(details.cached()),
    ])
}

pub(super) const fn answer_word(answer: Answer) -> &'static str {
    match answer {
        Answer::Yes => "yes",
        Answer::No => "no",
        Answer::Unsure => "unsure",
    }
}

pub(super) fn recognize_kinds(kinds: &[String]) -> Result<Recognize, String> {
    kinds
        .iter()
        .try_fold(Recognize::builder(), |built, kind| {
            built.kind(Kind::new(kind, None)?)
        })
        .and_then(thinkthen::RecognizeBuilder::build)
        .map_err(|error| failure(&error))
}

pub(super) fn recognized(
    engine: &Engine,
    ask: &Recognize,
    texts: Vec<String>,
    token: &CancelToken,
    due: Option<i64>,
    read: fn(&Recognized) -> Value,
) -> Result<Vec<Value>, Error> {
    texts
        .iter()
        .map(|text| {
            Ok(read(&engine.recognize_with(
                ask,
                text,
                options(token, due)?,
            )?))
        })
        .collect()
}

pub(super) fn entities(found: &Recognized) -> Value {
    let place = |at: usize| Value::Int(i64::try_from(at).unwrap_or(i64::MAX));
    Value::List(
        found
            .entities()
            .iter()
            .map(|name| {
                Value::Struct(vec![
                    Value::Text(name.name().to_owned()),
                    Value::Text(name.kind().to_owned()),
                    place(name.start()),
                    place(name.end()),
                    Value::Double(name.strength()),
                ])
            })
            .collect(),
    )
}

pub(super) fn relations(found: &Recognized) -> Value {
    Value::List(
        found
            .relations()
            .unwrap_or_default()
            .iter()
            .map(|relation| {
                Value::Struct(vec![
                    Value::Text(relation.relation().to_owned()),
                    Value::Text(relation.source().name().to_owned()),
                    Value::Text(relation.source().kind().to_owned()),
                    Value::Text(relation.target().name().to_owned()),
                    Value::Text(relation.target().kind().to_owned()),
                    Value::Double(relation.probability()),
                ])
            })
            .collect(),
    )
}
