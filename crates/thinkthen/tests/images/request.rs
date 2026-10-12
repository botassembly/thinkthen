//! Requests keep image tests on the public native execution edge.
use thinkthen::{
    AdmittedRequest, Batch, Call, CallOptions, CompleteChoice, CompleteDecision, CompleteRecord,
    CompleteScore, Engine, Error, InputEvidence, Question, QuestionInput, RecordInput, Request,
    RequestDefinition, RequestFraming, RequestInput, RequestOptions, RequestRows,
};

pub(super) fn request(definition: RequestDefinition) -> Result<AdmittedRequest, Error> {
    Request::for_definition(
        definition,
        RequestInput::Feed {
            name: "images".into(),
            framing: RequestFraming::Document,
            reading: Default::default(),
            images: vec![],
        },
        RequestOptions::default(),
    )
    .admit()
}
pub(super) fn rows<'a, T: InputEvidence + 'a>(
    engine: &Engine,
    definition: RequestDefinition,
    inputs: impl IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
    options: CallOptions<'a>,
) -> Result<RequestRows<'a, T>, Error> {
    engine.execute_request_rows(&request(definition)?, inputs, options)
}
macro_rules! scalar {
    ($method:ident, $variant:ident, $result:ty) => {
        fn $method(
            &self,
            question: &Question,
            input: &QuestionInput,
        ) -> Result<Call<Vec<CompleteRecord<QuestionInput, $result>>>, Error> {
            let RequestRows::$variant(batch) = rows(
                self.engine(),
                question.clone().into(),
                [Ok(record(input.clone()))],
                CallOptions::new(),
            )?
            else {
                unreachable!("request returns the selected result type")
            };
            batch.into_call()
        }
    };
}
pub(super) trait ImageRequests {
    fn engine(&self) -> &Engine;
    scalar!(request_decide, Decisions, CompleteDecision);
    scalar!(request_choose, Choices, CompleteChoice);
    scalar!(request_score, Scores, CompleteScore);
}
impl ImageRequests for Engine {
    fn engine(&self) -> &Engine {
        self
    }
}

pub(super) fn record<T>(original: T) -> RecordInput<T> {
    RecordInput {
        original,
        context: None,
        options: None,
        seed_spans: None,
        examples: None,
    }
}
macro_rules! many {
    ($method:ident, $variant:ident, $result:ty) => {
        pub(super) fn $method<'a, T: InputEvidence + 'a>(
            engine: &Engine,
            definition: RequestDefinition,
            inputs: impl IntoIterator<Item = Result<T, Error>> + 'a,
            options: CallOptions<'a>,
        ) -> Result<Batch<'a, CompleteRecord<T, $result>>, Error> {
            let RequestRows::$variant(batch) = rows(
                engine,
                definition,
                inputs.into_iter().map(|row| row.map(record)),
                options,
            )?
            else {
                unreachable!("request returns the selected result type")
            };
            Ok(batch)
        }
    };
}
many!(decisions, Decisions, CompleteDecision);
many!(choices, Choices, CompleteChoice);
many!(scores, Scores, CompleteScore);
