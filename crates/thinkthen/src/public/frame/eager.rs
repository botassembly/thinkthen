//! Eager Series and frame calls through the shared native engine.
use super::column::{Answers, Failures, decided, kind_word, streamed, text};
use super::{MEMBER, PolarsCallOptions, PolarsEngine, PolarsExprOptions, complete, inputs, lazy};
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
    fn input_details_column(
        &self,
        question: &Question,
        values: &[Option<crate::public::QuestionInput>],
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Option<Details>>>, Error> {
        inputs::details(self, question, values, options)
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
        let (question, call, probability) = options.apply(question)?;
        if probability {
            return self.probability_frame(&question, texts, call);
        }
        let value = match question.kind() {
            QuestionKind::Decide => self.decide_series(&question, texts, call)?,
            QuestionKind::Choose => self.choose_series(&question, texts, call)?,
            QuestionKind::Score => self.score_series(&question, texts, call)?,
            QuestionKind::Tag => self.tag_series(&question, texts, call)?,
            _ => {
                return Err(Error::usage(
                    "this column door takes decide, choose, score or tag",
                ));
            }
        };
        value.try_map(|value| {
            DataFrame::new(value.len(), vec![value.into()])
                .map_err(|error| Error::defect(&format!("the value frame was refused: {error}")))
        })
    }

    fn plan_series(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<PlanEstimate, Error> {
        self.plan_with(question, text(texts)?.iter().flatten(), options)
    }

    fn probability_frame(
        &self,
        question: &Question,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<DataFrame>, Error> {
        let kind = question.kind();
        if !matches!(kind, QuestionKind::Decide | QuestionKind::Choose) {
            return Err(Error::usage(format!(
                "probability belongs to decide and choose, not {}",
                kind_word(kind)
            )));
        }
        let cells = text(texts)?;
        let rows = cells.len();
        let mut probability =
            PrimitiveChunkedBuilder::<Float64Type>::new("probability".into(), rows);
        let mut values = Answers::new("value", kind, rows)?;
        let batch = self.details_many_with(question, cells.iter().flatten(), options);
        let facts = streamed(batch, cells, |row| {
            let Some(row) = row else {
                probability.append_null();
                return values.push(None);
            };
            probability.append_option(selected_probability(row.value())?);
            let value = match row.value().value() {
                Judgment::Decision(answer) => Annotated::Decision(*answer),
                Judgment::Choice(label) => Annotated::Choice(label.clone()),
                _ => return Err(Error::defect("a probability detail held another value")),
            };
            values.push(Some(&value))
        })?;
        let columns = vec![
            values.finish().into(),
            probability.finish().into_series().into(),
        ];
        Call::new(columns, facts).try_map(|columns| {
            DataFrame::new(rows, columns).map_err(|error| {
                Error::defect(&format!("the probability frame was refused: {error}"))
            })
        })
    }

    fn decide_series<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &Series,
        options: CallOptions<'_>,
    ) -> Result<Call<Series>, Error> {
        let cells = text(texts)?;
        let mut answers = BooleanChunkedBuilder::new(texts.name().clone(), cells.len());
        let batch = self.decide_many_with(question, cells.iter().flatten(), options);
        let facts = streamed(batch, cells, |row| {
            answers.append_option(row.and_then(|row| decided(*row.value())));
            Ok(())
        })?;
        Ok(Call::new(answers.finish().into_series(), facts))
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
        let batch = self.annotate_with(questions, cells.iter().flatten(), options);
        let facts = streamed(batch, cells, |record| {
            for (place, column) in columns.iter_mut().enumerate() {
                column.push_record(record.as_ref(), place)?;
            }
            failures.push(record.as_ref())
        })?;
        Call::new((columns, failures), facts).try_map(|(columns, failures)| {
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
    match (details.value(), details.probabilities()) {
        (Judgment::Decision(_), Probabilities::YesNo { yes }) => Ok(Some(*yes)),
        (Judgment::Choice(Some(selected)), Probabilities::Named(options)) => options
            .iter()
            .find(|option| option.name() == selected)
            .map(|option| Some(option.probability()))
            .ok_or_else(|| Error::defect("the chosen label has no probability")),
        (Judgment::Choice(None), Probabilities::Named(_)) => Ok(None),
        _ => Err(Error::defect("a probability detail held another shape")),
    }
}

/// One choose, score, or tag column through a one-question set.
fn single(
    engine: &Engine,
    wanted: QuestionKind,
    question: &Question,
    texts: &Series,
    options: CallOptions<'_>,
) -> Result<Call<Series>, Error> {
    if question.kind() != wanted {
        let word = kind_word(wanted);
        return Err(Error::usage(format!(
            "{word}_series needs a {word} question, and this one is a {} question",
            kind_word(question.kind())
        )));
    }
    let cells = text(texts)?;
    let set = QuestionSet::builder()
        .question(MEMBER, question.clone())?
        .build()?;
    let mut answers = Answers::new(texts.name().as_str(), wanted, cells.len())?;
    let batch = engine.annotate_with(&set, cells.iter().flatten(), options);
    let facts = streamed(batch, cells, |record| {
        answers.push_record(record.as_ref(), 0)
    })?;
    Ok(Call::new(answers.finish(), facts))
}
