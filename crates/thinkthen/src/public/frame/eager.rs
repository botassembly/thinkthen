//! Eager Series and frame calls through the shared native engine.
use super::column::{Answers, Failures, annotated, decided, kind_word, text};
use super::{PolarsCallOptions, PolarsEngine, PolarsExprOptions, complete, inputs, lazy, request};
use crate::public::{
    Annotated, Call, CallOptions, DecisionQuestion, Details, Edge, Engine, Error, Judgment,
    PlanEstimate, Probabilities, Question, QuestionKind, QuestionSet, Recognize, Recognized,
    Relate,
};
use polars::prelude::{
    BooleanChunkedBuilder, ChunkedBuilder, Column, DataFrame, Expr, Float64Type, IntoSeries,
    PrimitiveChunkedBuilder, Series,
};
impl PolarsEngine for Engine {
    fn input_column(
        &self,
        question: &Question,
        values: &[Option<crate::public::QuestionInput>],
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        inputs::column(self, question, values, options)
    }
    fn source_column(
        &self,
        question: &Question,
        paths: &[std::path::PathBuf],
        reading: crate::public::InputReaderOptions,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
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

    fn column_with(
        &self,
        question: &Question,
        texts: &Series,
        options: PolarsCallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
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
    ) -> Result<Call<Series>, Error> {
        let call = request::execute(
            self,
            question.question(),
            texts,
            crate::RequestFunction::Decide,
            options,
        )?;
        call.try_map(|value| {
            let rows = value.decisions("a decide column received another result kind")?;
            let mut answers = BooleanChunkedBuilder::new(texts.name().clone(), texts.len());
            request::project(texts, &rows, |row| {
                answers.append_option(row.and_then(|row| decided(row.value())));
                Ok(())
            })?;
            Ok(answers.finish().into_series())
        })
    }

    fn choose_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        single(self, QuestionKind::Choose, question, texts, options)
    }

    fn score_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        single(self, QuestionKind::Score, question, texts, options)
    }

    fn tag_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        single(self, QuestionKind::Tag, question, texts, options)
    }

    fn annotate_frame(
        &self,
        questions: &QuestionSet,
        frame: &DataFrame,
        on: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
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
        call.try_map(|value| {
            let rows = value.annotations("an annotation frame received another result kind")?;
            request::project(held.as_materialized_series(), &rows, |record| {
                annotated(&mut columns, &mut failures, record)
            })?;
            let mut columns = columns
                .into_iter()
                .map(|column| column.finish().into())
                .collect::<Vec<Column>>();
            columns.push(failures.finish()?.into());
            frame
                .hstack(&columns)
                .map_err(|error| Error::defect(&format!("the frame refused a new column: {error}")))
        })
    }

    fn filter_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        complete::filter(self, question, texts, options)
    }
    fn rank_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        complete::rank(self, question, texts, options)
    }
    fn find_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        complete::find(self, question, texts, options)
    }
    fn recognize_series(
        &self,
        ask: &Recognize,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Option<Recognized>>>, Error> {
        complete::recognize(self, ask, texts, options)
    }
    fn relate_frame(
        &self,
        ask: &Relate,
        frame: &DataFrame,
        name: &str,
        kind: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Edge>>, Error> {
        complete::relate(self, ask, frame, name, kind, options)
    }
}

pub(super) fn selected_probability(details: &Details) -> Result<Option<f64>, Error> {
    probability_of(details.value(), details.probabilities())
}

fn probability_of(value: &Judgment, probabilities: &Probabilities) -> Result<Option<f64>, Error> {
    match (value, probabilities) {
        (Judgment::Decision(_), Probabilities::YesNo { yes }) => Ok(Some(*yes)),
        (Judgment::Choice(Some(selected)), Probabilities::Named(options)) => options
            .iter()
            .find(|option| option.name() == selected)
            .map(|option| Some(option.probability()))
            .ok_or_else(|| Error::defect("the chosen label has no probability")),
        (Judgment::Choice(None), Probabilities::Named(_)) => Ok(None),
        (Judgment::Score(_) | Judgment::Tags(_), _) => Ok(None),
        _ => Err(Error::defect("a probability detail held another shape")),
    }
}

/// Project a named native atomic Request into the caller's column type.
fn single(
    engine: &Engine,
    wanted: QuestionKind,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<Series>, Error> {
    let named = match wanted {
        QuestionKind::Choose => crate::RequestFunction::Choose,
        QuestionKind::Score => crate::RequestFunction::Score,
        QuestionKind::Tag => crate::RequestFunction::Tag,
        _ => return Err(Error::defect("the atomic column has no named request")),
    };
    request::execute(engine, question, texts, named, options)?.try_map(|value| {
        let mut answers = Answers::new(texts.name().as_str(), wanted, texts.len())?;
        let mismatch = "an atomic column received another result kind";
        match wanted {
            QuestionKind::Choose => request::project(texts, &value.choices(mismatch)?, |row| {
                answers.push(
                    row.map(|row| Annotated::Choice(row.value().map(str::to_owned)))
                        .as_ref(),
                )
            })?,
            QuestionKind::Score => request::project(texts, &value.scores(mismatch)?, |row| {
                answers.push(row.map(|row| Annotated::Score(row.value())).as_ref())
            })?,
            QuestionKind::Tag => request::project(texts, &value.tags(mismatch)?, |row| {
                answers.push(
                    row.map(|row| Annotated::Tags(row.value().to_vec()))
                        .as_ref(),
                )
            })?,
            _ => {
                return Err(Error::defect(mismatch));
            }
        }
        Ok(answers.finish())
    })
}

fn atomic_frame(
    engine: &Engine,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<DataFrame>, Error> {
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
    let cells = text(texts)?;
    let rows = cells.len();
    let mut probability = PrimitiveChunkedBuilder::<Float64Type>::new("probability".into(), rows);
    let mut values = Answers::new("value", kind, rows)?;
    let named = match kind {
        QuestionKind::Decide => crate::RequestFunction::Decide,
        QuestionKind::Choose => crate::RequestFunction::Choose,
        QuestionKind::Score => crate::RequestFunction::Score,
        QuestionKind::Tag => crate::RequestFunction::Tag,
        _ => return Err(Error::defect("a probability column lost its question kind")),
    };
    request::execute(engine, question, texts, named, options)?.try_map(|value| {
        let mut push = |value: Option<Judgment>, probabilities: Option<Probabilities>| {
            probability.append_option(
                value
                    .as_ref()
                    .zip(probabilities.as_ref())
                    .map(|(value, probabilities)| probability_of(value, probabilities))
                    .transpose()?
                    .flatten(),
            );
            values.push_judgment(value)
        };
        let bad = "a probability column received another result kind";
        match kind {
            QuestionKind::Decide => request::project(texts, &value.decisions(bad)?, |row| {
                push(
                    row.map(|row| Judgment::Decision(row.value())),
                    row.map(|row| row.probabilities()),
                )
            })?,
            QuestionKind::Choose => request::project(texts, &value.choices(bad)?, |row| {
                push(
                    row.map(|row| Judgment::Choice(row.value().map(str::to_owned))),
                    row.map(|row| row.probabilities()),
                )
            })?,
            QuestionKind::Score => request::project(texts, &value.scores(bad)?, |row| {
                push(row.map(|row| Judgment::Score(row.value())), None)
            })?,
            QuestionKind::Tag => request::project(texts, &value.tags(bad)?, |row| {
                push(row.map(|row| Judgment::Tags(row.value().to_vec())), None)
            })?,
            _ => return Err(Error::defect(bad)),
        }
        DataFrame::new(
            rows,
            vec![
                values.finish().into(),
                probability.finish().into_series().into(),
            ],
        )
        .map_err(|error| Error::defect(&format!("the probability frame was refused: {error}")))
    })
}
