//! Owned native requests retain DuckDB's compact result codecs.
use super::CallScope;
use thinkthen::{
    Engine, Error, Facts, LoadedQuestion, Request, RequestArguments, RequestBatch, RequestCall,
    RequestDefinition, RequestInput, RequestItem, RequestOptions, RequestOriginal, RequestQuestion,
    RequestSessionRead, RequestSessionResult, RequestSessionRow, RequestValue, Surface,
};

pub(crate) struct ResultSet {
    pub rows: Vec<RequestSessionRow>,
    pub aggregate: Option<RequestValue>,
    pub facts: Option<Facts>,
    pub error: Option<Error>,
}

pub(crate) fn texts(texts: Vec<String>) -> RequestInput {
    RequestInput::Records {
        items: texts
            .into_iter()
            .map(|text| RequestItem {
                original: Some(RequestOriginal::Text { text }),
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
                images: Vec::new(),
            })
            .collect(),
    }
}

pub(crate) fn build(
    definition: RequestDefinition,
    input: RequestInput,
    scope: CallScope<'_>,
) -> Result<Request, String> {
    let args = RequestArguments {
        question: RequestQuestion::Definition {
            value: definition.clone(),
        },
        input,
        options: RequestOptions {
            context: scope.context.map(str::to_owned),
            batch: scope
                .batch
                .map(|value| {
                    if value == "max" {
                        Ok(RequestBatch::Named(value.to_owned()))
                    } else {
                        value
                            .parse()
                            .map(RequestBatch::Count)
                            .map_err(|_| "thinkthen usage: invalid batch".to_owned())
                    }
                })
                .transpose()?,
            deadline_ms: Some(scope.due),
            max_requests_total: scope.total.map(u64::try_from).transpose().map_err(|_| {
                "thinkthen usage: a request total is a whole number of 0 or more".to_owned()
            })?,
            ..Default::default()
        },
    };
    let call = match &definition {
        RequestDefinition::Atomic(LoadedQuestion::Banded(_)) => RequestCall::Decide(args),
        RequestDefinition::Atomic(LoadedQuestion::Question(q)) => match q.kind() {
            thinkthen::QuestionKind::Decide => RequestCall::Decide(args),
            thinkthen::QuestionKind::Choose => RequestCall::Choose(args),
            thinkthen::QuestionKind::Score => RequestCall::Score(args),
            thinkthen::QuestionKind::Tag => RequestCall::Tag(args),
            thinkthen::QuestionKind::Find => RequestCall::Find(args),
            _ => return Err("thinkthen defect: unknown atomic request".to_owned()),
        },
        RequestDefinition::Annotate(_) => RequestCall::Annotate(args),
        RequestDefinition::Rank(_) | RequestDefinition::RankSet(_) => RequestCall::Rank(args),
        RequestDefinition::Recognize(_) | RequestDefinition::Recognition(_) => {
            RequestCall::Recognize(args)
        }
        RequestDefinition::Find(_) => RequestCall::Find(args),
        RequestDefinition::Relate(_) => RequestCall::Relate(args),
        _ => return Err("thinkthen defect: unsupported compact request".to_owned()),
    };
    Ok(Request::new(call))
}

pub(crate) fn run(
    engine: &Engine,
    definition: RequestDefinition,
    input: RequestInput,
    scope: CallScope<'_>,
) -> Result<ResultSet, String> {
    if let RequestInput::Records { items } = &input
        && let Some(at) = items.len().checked_sub(1)
    {
        engine
            .check_record_limit(at)
            .map_err(|e| crate::engines::call_error(e, scope.total).text)?;
    }
    let request = build(definition, input, scope)?;
    let session = engine
        .request_session_with_surface(request, Surface::Duckdb)
        .map_err(|e| crate::engines::call_error(e, scope.total).text)?;
    let mut result = ResultSet {
        rows: Vec::new(),
        aggregate: None,
        facts: None,
        error: None,
    };
    loop {
        if scope.token.is_cancelled() {
            session.cancel();
        }
        match session.try_read() {
            RequestSessionRead::Pending => std::thread::sleep(std::time::Duration::from_millis(1)),
            RequestSessionRead::End => break,
            RequestSessionRead::Result(packet) => match packet {
                RequestSessionResult::Row(row) => result.rows.push(row),
                RequestSessionResult::Aggregate(value) => match (&mut result.aggregate, value) {
                    (Some(RequestValue::Recognized(rows)), RequestValue::Recognized(mut next)) => {
                        rows.append(&mut next);
                    }
                    (slot, value) => *slot = Some(value),
                },
                RequestSessionResult::Terminal(value) => {
                    result.facts = value.facts;
                    result.error = value.error;
                }
                RequestSessionResult::Observation { .. } => {}
            },
        }
    }
    Ok(result)
}

impl ResultSet {
    pub(crate) fn success(mut self, total: Option<i64>) -> Result<Self, String> {
        if let Some(error) = self.error.take() {
            return Err(crate::engines::call_error(error, total).text);
        }
        Ok(self)
    }
}

pub(crate) fn scalar_json(row: RequestSessionRow) -> Result<String, String> {
    let json = match row {
        RequestSessionRow::Decision(row) => row.result().to_scalar_json(),
        RequestSessionRow::Choice(row) => row.result().to_scalar_json(),
        RequestSessionRow::Tags(row) => row.result().to_scalar_json(),
        RequestSessionRow::Score(row) => row.result().to_scalar_json(),
        _ => return Err("thinkthen defect: scalar returned another kind".to_owned()),
    };
    json.map_err(|e| crate::engines::call_error(e, None).text)
}

pub(crate) fn probability(value: thinkthen::Probabilities) -> Option<f64> {
    match value {
        thinkthen::Probabilities::YesNo { yes } => Some(yes),
        _ => None,
    }
}
