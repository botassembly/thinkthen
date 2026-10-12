//! Shared setup for public API cases; callers keep their own key and controls.
#![allow(
    dead_code,
    clippy::expect_used,
    reason = "each integration binary uses only its owning setup helpers"
)]
use thinkthen::{Engine, EngineBuilder, RecordInput};

// Forward fallible eager feeds without copying their per-function fixture setup.
macro_rules! whole_call {
    ($method:ident, $function:ident, $question:ty) => {
        pub(crate) fn $method<'a, I, T>(
            engine: &Engine,
            question: &$question,
            records: I,
            controls: thinkthen::CallOptions<'a>,
        ) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error>
        where
            I: IntoIterator<Item = Result<RecordInput<T>, thinkthen::Error>>,
            I::IntoIter: 'a,
            T: thinkthen::InputEvidence + 'a,
        {
            whole(
                engine,
                thinkthen::RequestFunction::$function,
                question.clone().into(),
                records,
                controls,
            )
        }
    };
}
whole_call!(try_recognize_records, Recognize, thinkthen::Recognize);
whole_call!(relation, Relate, thinkthen::Relate);
whole_call!(find, Find, thinkthen::Question);

// Borrow each typed row slice from its actual collecting Request carrier.
macro_rules! call_rows {
    ($method:ident, $variant:ident, $result:ty) => {
        pub(crate) fn $method(
            call: &thinkthen::Call<thinkthen::RequestValue>,
        ) -> Option<&[thinkthen::CompleteRecord<thinkthen::QuestionInput, $result>]> {
            match call.value() {
                thinkthen::RequestValue::$variant(rows) => Some(rows),
                _ => None,
            }
        }
    };
}
call_rows!(choices, Choices, thinkthen::CompleteChoice);
call_rows!(scores, Scores, thinkthen::CompleteScore);
call_rows!(recognized, Recognized, thinkthen::CompleteRecognized);
call_rows!(ranked, Ranked, thinkthen::CompleteRank);
call_rows!(tags, Tags, thinkthen::CompleteTags);
call_rows!(set_ranked, SetRanked, thinkthen::CompleteSetRank);
call_rows!(decisions, Decisions, thinkthen::CompleteDecision);

/// Start the actual loopback listener with the caller's exact saved reply.
#[cfg(test)]
pub(crate) fn answering(reply: &'static str) -> conformance_backend::Listener {
    conformance_backend::Listener::answering(move |_| conformance_backend::Canned::ok(reply))
        .unwrap()
}

pub(crate) fn builder(base: &str, key: &str) -> EngineBuilder {
    Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key(key))
        .expect("isolated backend")
}

/// Retain the caller's remaining settings after selecting the shared test model.
pub(crate) fn fixed(base: &str, key: &str) -> EngineBuilder {
    builder(base, key).model("fixed").expect("fixed test model")
}

/// Build an isolated fixed-model loopback engine with the caller's explicit key.
pub(crate) fn loopback(base: &str, key: &str) -> Engine {
    fixed(base, key)
        .no_cache()
        .max_retries(0)
        .build()
        .expect("isolated engine")
}

pub(crate) fn record<T>(original: T) -> RecordInput<T> {
    RecordInput {
        original,
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
    }
}

/// Supply identical plain record defaults without reading caller evidence.
pub(crate) fn plain_records<I: IntoIterator>(
    originals: I,
) -> impl Iterator<Item = Result<RecordInput<I::Item>, thinkthen::Error>> {
    plain_inputs(originals).map(Ok)
}

/// Supply identical composed defaults without reading caller evidence.
pub(crate) fn plain_inputs<I: IntoIterator>(
    originals: I,
) -> impl Iterator<Item = RecordInput<I::Item>> {
    originals.into_iter().map(record)
}

/// Collect the actual public Request call, retaining its native values and facts.
pub(crate) fn request(
    engine: &Engine,
    function: thinkthen::RequestFunction,
    definition: thinkthen::RequestDefinition,
    input: thinkthen::RequestInput,
    environment: thinkthen::RequestEnvironment<'_>,
) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error> {
    let request = admitted(function, definition, input)?;
    match engine.execute_request(&request, environment)? {
        thinkthen::RequestOutcome::Complete(call) => Ok(call),
        thinkthen::RequestOutcome::Failed { error, .. } => Err(error),
    }
}

/// Retain a literal document as the public Request text input.
pub(crate) fn text(text: &str) -> thinkthen::RequestInput {
    thinkthen::RequestInput::Text {
        text: text.to_owned(),
        images: Vec::new(),
    }
}

/// Judge an actual eager composed feed through the public collecting Request.
pub(crate) fn whole<'a, I, T>(
    engine: &Engine,
    function: thinkthen::RequestFunction,
    definition: thinkthen::RequestDefinition,
    records: I,
    controls: thinkthen::CallOptions<'a>,
) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, thinkthen::Error>>,
    I::IntoIter: 'a,
    T: thinkthen::InputEvidence + 'a,
{
    let feed = thinkthen::RequestFeed::from_records(
        "whole",
        records
            .into_iter()
            .map(|row| row.map(|record| record.map_original(|original| original.question_input()))),
    )
    .eager();
    request(
        engine,
        function,
        definition,
        thinkthen::RequestInput::Feed {
            name: "whole".into(),
            framing: thinkthen::RequestFraming::Document,
            reading: thinkthen::ReaderOptions::default(),
            images: Vec::new(),
        },
        thinkthen::RequestEnvironment {
            controls,
            feed: Some(feed),
        },
    )
}

/// Collect literal recognition through the existing public Request input.
pub(crate) fn recognize(
    engine: &Engine,
    question: &thinkthen::Recognize,
    evidence: &str,
    controls: thinkthen::CallOptions<'_>,
) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error> {
    request(
        engine,
        thinkthen::RequestFunction::Recognize,
        question.clone().into(),
        text(evidence),
        thinkthen::RequestEnvironment {
            controls,
            feed: None,
        },
    )
}

pub(crate) fn recognition(
    call: &thinkthen::Call<thinkthen::RequestValue>,
) -> Option<&thinkthen::CompleteRecognized> {
    recognized(call)?.first().map(|row| row.result())
}

/// Read the actual whole-set relation carrier from a public Request call.
pub(crate) fn related(
    call: &thinkthen::Call<thinkthen::RequestValue>,
) -> Option<&thinkthen::CompleteRecord<Vec<thinkthen::QuestionInput>, thinkthen::CompleteRelated>> {
    match call.value() {
        thinkthen::RequestValue::Related(row) => Some(row),
        _ => None,
    }
}

/// Borrow the actual complete Find carrier produced by Request.
pub(crate) fn found(
    call: &thinkthen::Call<thinkthen::RequestValue>,
) -> Option<&thinkthen::CompleteFound<thinkthen::QuestionInput>> {
    match call.value() {
        thinkthen::RequestValue::Found(row) => Some(row),
        _ => None,
    }
}

fn admitted(
    function: thinkthen::RequestFunction,
    definition: thinkthen::RequestDefinition,
    input: thinkthen::RequestInput,
) -> Result<thinkthen::AdmittedRequest, thinkthen::Error> {
    thinkthen::Request::for_function(
        function,
        thinkthen::RequestArguments {
            question: thinkthen::RequestQuestion::Definition { value: definition },
            input,
            options: thinkthen::RequestOptions::default(),
        },
    )
    .admit()
}

/// Execute the actual public generic rows path without normalizing caller originals.
pub(crate) fn rows<'a, I, T>(
    engine: &Engine,
    function: thinkthen::RequestFunction,
    definition: thinkthen::RequestDefinition,
    records: I,
    controls: thinkthen::CallOptions<'a>,
) -> Result<thinkthen::RequestRows<'a, T>, thinkthen::Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, thinkthen::Error>> + 'a,
    T: thinkthen::InputEvidence + 'a,
{
    let request = admitted(
        function,
        definition,
        thinkthen::RequestInput::Feed {
            name: "rows".into(),
            framing: thinkthen::RequestFraming::Document,
            reading: thinkthen::ReaderOptions::default(),
            images: Vec::new(),
        },
    )?;
    engine.execute_request_rows(&request, records, controls)
}

// Borrow no originals while taking an actual typed batch from its public carrier.
macro_rules! batch_rows {
    ($method:ident, $variant:ident, $result:ty) => {
        pub(crate) fn $method<T>(
            rows: thinkthen::RequestRows<'_, T>,
        ) -> Option<thinkthen::Batch<'_, thinkthen::CompleteRecord<T, $result>>> {
            match rows {
                thinkthen::RequestRows::$variant(batch) => Some(batch),
                _ => None,
            }
        }
    };
}
batch_rows!(annotation_rows, Annotations, thinkthen::CompleteAnnotated);
batch_rows!(decision_rows, Decisions, thinkthen::CompleteDecision);
batch_rows!(choice_rows, Choices, thinkthen::CompleteChoice);
batch_rows!(tag_rows, Tags, thinkthen::CompleteTags);
batch_rows!(score_rows, Scores, thinkthen::CompleteScore);
batch_rows!(filter_rows, Filtered, thinkthen::CompleteFilter);

// Pull the actual typed public batch while preserving caller-owned originals.
macro_rules! pull_rows {
    ($method:ident, $function:ident, $extract:ident, $result:ty) => {
        pub(crate) fn $method<'a, I, T, Q>(
            engine: &Engine,
            question: &Q,
            records: I,
            controls: thinkthen::CallOptions<'a>,
        ) -> thinkthen::Batch<'a, thinkthen::CompleteRecord<T, $result>>
        where
            I: IntoIterator<Item = Result<RecordInput<T>, thinkthen::Error>> + 'a,
            T: thinkthen::InputEvidence + 'a,
            Q: Clone + Into<thinkthen::RequestDefinition>,
        {
            $extract(
                rows(
                    engine,
                    thinkthen::RequestFunction::$function,
                    question.clone().into(),
                    records,
                    controls,
                )
                .expect("valid row header"),
            )
            .expect("matching typed public batch")
        }
    };
}
pull_rows!(
    pull_decisions,
    Decide,
    decision_rows,
    thinkthen::CompleteDecision
);
pull_rows!(pull_choices, Choose, choice_rows, thinkthen::CompleteChoice);
pull_rows!(pull_tags, Tag, tag_rows, thinkthen::CompleteTags);
pull_rows!(pull_scores, Score, score_rows, thinkthen::CompleteScore);
pull_rows!(pull_filters, Filter, filter_rows, thinkthen::CompleteFilter);

type RowsCall<T, R> = thinkthen::Call<Vec<thinkthen::CompleteRecord<T, R>>>;

// Collect actual typed Request batches without rebuilding their Call envelopes.
macro_rules! collect_rows {
    ($method:ident, $function:ident, $variant:ident, $result:ty) => {
        pub(crate) fn $method<'a, I, T, Q>(
            engine: &Engine,
            question: &Q,
            records: I,
            controls: thinkthen::CallOptions<'a>,
        ) -> Result<RowsCall<T, $result>, thinkthen::Error>
        where
            I: IntoIterator<Item = RecordInput<T>> + 'a,
            T: thinkthen::InputEvidence + 'a,
            Q: Clone + Into<thinkthen::RequestDefinition>,
        {
            match rows(
                engine,
                thinkthen::RequestFunction::$function,
                question.clone().into(),
                records.into_iter().map(Ok),
                controls,
            )? {
                thinkthen::RequestRows::$variant(batch) => batch.into_call(),
                _ => Err(thinkthen::Error::new(
                    thinkthen::ErrorKind::Defect,
                    "typed rows",
                )),
            }
        }
    };
}
collect_rows!(
    annotate,
    Annotate,
    Annotations,
    thinkthen::CompleteAnnotated
);
collect_rows!(
    decide_records,
    Decide,
    Decisions,
    thinkthen::CompleteDecision
);
collect_rows!(score, Score, Scores, thinkthen::CompleteScore);
collect_rows!(choose, Choose, Choices, thinkthen::CompleteChoice);
collect_rows!(tag, Tag, Tags, thinkthen::CompleteTags);
collect_rows!(filter_records, Filter, Filtered, thinkthen::CompleteFilter);

/// Execute a literal decision through its actual public Request carrier.
pub(crate) fn decide<Q>(
    engine: &Engine,
    question: &Q,
    input: &str,
    controls: thinkthen::CallOptions<'_>,
) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error>
where
    Q: Clone + Into<thinkthen::RequestDefinition>,
{
    whole(
        engine,
        thinkthen::RequestFunction::Decide,
        question.clone().into(),
        [Ok(record(input))],
        controls,
    )
}

/// Borrow the actual decision from a successful one-input Request.
pub(crate) fn decision(
    call: &thinkthen::Call<thinkthen::RequestValue>,
) -> Option<&thinkthen::CompleteDecision> {
    decisions(call)?.first().map(|row| row.result())
}

// Share the actual public settlement reader across independent integration binaries.
pub(crate) fn session_next(session: &thinkthen::RequestSession) -> thinkthen::RequestSessionRead {
    let until = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let packet = session.try_read();
        if !matches!(packet, thinkthen::RequestSessionRead::Pending) {
            return packet;
        }
        assert!(std::time::Instant::now() < until, "session did not settle");
        let _ = session.wait(
            thinkthen::RequestSessionWaitInterest::Settlement,
            until.saturating_duration_since(std::time::Instant::now()),
        );
    }
}
pub(crate) fn session_drain(
    session: &thinkthen::RequestSession,
) -> Vec<thinkthen::RequestSessionResult> {
    let mut packets = vec![];
    loop {
        match session_next(session) {
            thinkthen::RequestSessionRead::Result(packet) => packets.push(packet),
            thinkthen::RequestSessionRead::End => return packets,
            thinkthen::RequestSessionRead::Pending => unreachable!(),
        }
    }
}

// Admit a whole composed feed through the actual public Request before any send.
macro_rules! eager_rows {
    ($method:ident, $function:ident) => {
        pub(crate) fn $method<'a, I, T, Q>(
            engine: &Engine,
            question: &Q,
            records: I,
            controls: thinkthen::CallOptions<'a>,
        ) -> Result<thinkthen::Call<thinkthen::RequestValue>, thinkthen::Error>
        where
            I: IntoIterator<Item = RecordInput<T>>,
            I::IntoIter: 'a,
            T: thinkthen::InputEvidence + 'a,
            Q: Clone + Into<thinkthen::RequestDefinition>,
        {
            whole(
                engine,
                thinkthen::RequestFunction::$function,
                question.clone().into(),
                records.into_iter().map(Ok),
                controls,
            )
        }
    };
}
eager_rows!(eager_decisions, Decide);
eager_rows!(eager_choices, Choose);
eager_rows!(eager_rank, Rank);
eager_rows!(recognize_records, Recognize);

#[cfg(test)]
pub(crate) fn dynamic_one() -> thinkthen::CallOptions<'static> {
    thinkthen::CallOptions::new()
        .batch(thinkthen::BatchSetting::Records(
            std::num::NonZeroUsize::new(1).unwrap(),
        ))
        .attempts(true)
}
#[cfg(test)]
pub(crate) fn dynamic_candidates(text: &str) -> thinkthen::RecordOptions {
    thinkthen::RecordOptions::project(text, "").unwrap()
}
