//! Eager Series and frame calls through the shared native engine.
use super::column::{Answers, Failures, annotated, kind_word, text};
use super::{
    PolarsCallOptions, PolarsEngine, PolarsExprOptions, complete, inputs, lazy, request, result,
};
use crate::public::{
    Call, CallOptions, DecisionQuestion, Edge, Engine, Error, Judgment, PlanEstimate, Question,
    QuestionKind, QuestionSet, Recognize, Recognized, Relate,
};
use polars::prelude::{
    ChunkedBuilder, Column, DataFrame, Expr, Float64Type, IntoSeries, PrimitiveChunkedBuilder,
    Series,
};
impl PolarsEngine for Engine {
    fn input_column(
        &self,
        question: &Question,
        values: &[Option<crate::public::QuestionInput>],
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
        inputs::column(self, question, values, options)
    }
    fn source_column(
        &self,
        question: &Question,
        paths: &[std::path::PathBuf],
        reading: crate::public::InputReaderOptions,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
        inputs::sources(self, question, paths, reading, options)
    }
    fn decide_expr<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(
            self,
            QuestionKind::Decide,
            question.question(),
            input,
            options,
        )
    }

    fn choose_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(self, QuestionKind::Choose, question, input, options)
    }

    fn score_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(self, QuestionKind::Score, question, input, options)
    }

    fn tag_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        lazy::expression(self, QuestionKind::Tag, question, input, options)
    }

    fn filter_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        Engine::filter_expr(self, question, input, options)
    }
    fn rank_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        Engine::rank_expr(self, question, input, options)
    }
    fn find_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        Engine::find_expr(self, question, input, options)
    }
    fn annotate_expr(
        &self,
        questions: &QuestionSet,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        Engine::annotate_expr(self, questions, input, options)
    }
    fn recognize_expr(
        &self,
        question: &Recognize,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        Engine::recognize_expr(self, question, input, options)
    }
    fn relate_expr(
        &self,
        question: &Relate,
        names: Expr,
        kinds: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        Engine::relate_expr(self, question, names, kinds, options)
    }

    fn column_with(
        &self,
        question: &Question,
        texts: &Series,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
        let (question, call) = options.apply(question)?;
        atomic_frame(self, &question, texts, call)
    }

    fn plan_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<PlanEstimate, Error> {
        self.plan_with(question, text(texts)?.iter().flatten(), options)
    }

    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
        scalar(
            self,
            QuestionKind::Decide,
            question.question(),
            texts,
            options,
        )
    }

    fn choose_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
        scalar(self, QuestionKind::Choose, question, texts, options)
    }

    fn score_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
        scalar(self, QuestionKind::Score, question, texts, options)
    }

    fn tag_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
        scalar(self, QuestionKind::Tag, question, texts, options)
    }

    fn annotate_frame(
        &self,
        questions: &QuestionSet,
        frame: &DataFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
        let held = frame
            .column(on)
            .map_err(|_| Error::usage(format!("the frame holds no column {on}")))?;
        if questions.members().any(|(name, _)| name == "failed") {
            return Err(Error::usage(
                "the question name failed is reserved for frame failures",
            ));
        }
        for (name, _) in questions
            .members()
            .chain(std::iter::once(("failed", QuestionKind::Decide)))
        {
            if frame.column(name).is_ok() {
                return Err(Error::usage(format!(
                    "the frame already holds a column named {name}"
                )));
            }
        }
        let cells = text(held.as_materialized_series())?;
        let names = questions
            .members()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        let mut columns = questions
            .members()
            .map(|(name, kind)| Answers::new(name, kind, cells.len()))
            .collect::<Result<Vec<_>, _>>()?;
        let mut failures = Failures::new(&names, cells.len());
        let call = request::execute_definition(
            self,
            questions.clone().into(),
            held.as_materialized_series(),
            crate::RequestFunction::Annotate,
            options,
        )?;
        result::enrich(
            call,
            crate::RequestFunction::Annotate,
            held.as_materialized_series(),
            |value| {
                let crate::RequestValue::Annotations(rows) = value else {
                    return Err(Error::defect(
                        "an annotation frame received another result kind",
                    ));
                };
                request::project(held.as_materialized_series(), rows, |record| {
                    annotated(&mut columns, &mut failures, record)
                })?;
                let mut columns = columns
                    .into_iter()
                    .map(|column| column.finish().into())
                    .collect::<Vec<Column>>();
                columns.push(failures.finish()?.into());
                frame.hstack(&columns).map_err(|error| {
                    Error::defect(&format!("the frame refused a new column: {error}"))
                })
            },
        )
    }

    fn filter_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
        complete::filter(self, question, texts, options)
    }
    fn rank_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
        complete::rank(self, question, texts, options)
    }
    fn find_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
        complete::find(self, question, texts, options)
    }
    fn recognize_series(
        &self,
        ask: &Recognize,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<super::FrameCall<Vec<Option<Recognized>>>, Error> {
        complete::recognize(self, ask, texts, options)
    }
    fn relate_frame(
        &self,
        ask: &Relate,
        frame: &DataFrame,
        name: &str,
        kind: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Edge>, crate::RequestValue, Series>, Error> {
        complete::relate(self, ask, frame, name, kind, options)
    }
}

fn scalar(
    engine: &Engine,
    wanted: QuestionKind,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<Series, crate::RequestValue, Series>, Error> {
    if question.kind() != wanted {
        return Err(Error::usage(
            "question kind does not match the requested function",
        ));
    }
    atomic_frame(engine, question, texts, options)?.try_map(|frame| {
        let mut value = frame
            .column("value")
            .map_err(|_| Error::defect("an atomic frame lost its value"))?
            .as_materialized_series()
            .clone();
        value.rename(texts.name().clone());
        Ok(value)
    })
}

pub(super) fn atomic_frame(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<DataFrame, crate::RequestValue, Series>, Error> {
    let kind = question.kind();
    if !matches!(
        kind,
        QuestionKind::Decide | QuestionKind::Choose | QuestionKind::Tag | QuestionKind::Score
    ) {
        return Err(Error::usage(format!(
            "the atomic column does not accept {}",
            kind_word(kind)
        )));
    }
    let named = match kind {
        QuestionKind::Decide => crate::RequestFunction::Decide,
        QuestionKind::Choose => crate::RequestFunction::Choose,
        QuestionKind::Score => crate::RequestFunction::Score,
        QuestionKind::Tag => crate::RequestFunction::Tag,
        _ => return Err(Error::defect("an atomic column lost its kind")),
    };
    result::enrich(
        request::execute(engine, question, texts, named, options)?,
        named,
        texts,
        |value| project_atomic(value, texts, kind),
    )
}

pub(super) fn project_atomic(
    value: &crate::RequestValue,
    texts: &Series,
    kind: QuestionKind,
) -> Result<DataFrame, Error> {
    let rows = texts.len();
    let mut probabilities = PrimitiveChunkedBuilder::<Float64Type>::new("probability".into(), rows);
    let mut values = Answers::new("value", kind, rows)?;
    let mut push = |value: Option<Judgment>, probability: Option<f64>| {
        probabilities.append_option(probability);
        values.push_judgment(value)
    };
    match value {
        crate::RequestValue::Decisions(rows) => request::project(texts, rows, |row| {
            push(
                row.map(|row| Judgment::Decision(row.value())),
                row.and_then(|row| row.probability()),
            )
        })?,
        crate::RequestValue::Choices(rows) => request::project(texts, rows, |row| {
            push(
                row.map(|row| Judgment::Choice(row.value().map(str::to_owned))),
                row.and_then(|row| row.probability()),
            )
        })?,
        crate::RequestValue::Scores(rows) => request::project(texts, rows, |row| {
            push(row.map(|row| Judgment::Score(row.value())), None)
        })?,
        crate::RequestValue::Tags(rows) => request::project(texts, rows, |row| {
            push(row.map(|row| Judgment::Tags(row.value().to_vec())), None)
        })?,
        _ => {
            return Err(Error::defect(
                "an atomic column received another result kind",
            ));
        }
    }
    DataFrame::new(
        rows,
        vec![
            values.finish().into(),
            probabilities.finish().into_series().into(),
        ],
    )
    .map_err(|_| Error::defect("an atomic frame could not be built"))
}
