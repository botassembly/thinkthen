//! Collection expressions evaluate one logical set through the native request.
use super::{
    PolarsExprOptions,
    lazy::{caught, compute_error},
    request, result,
};
use crate::{
    Engine, Error, Question, QuestionSet, Recognize, Relate, RequestDefinition, RequestFunction,
};
use polars::prelude::{Column, DataFrame, DataType, Expr, Field};

fn expression(
    engine: &Engine,
    function: RequestFunction,
    definition: RequestDefinition,
    input: Expr,
    options: PolarsExprOptions,
) -> Result<Expr, Error> {
    options.validate()?;
    let dtype = result::expression_type(function, &definition, &DataType::String)?;
    let engine = engine.clone();
    let evaluate =
        move |column: Column| caught(|| evaluate(&engine, function, &definition, &options, column));
    // Groupwise execution materializes the complete logical collection. It
    // prevents streaming morsels from becoming independent rank/find sets.
    Ok(input.apply_with_fmt_str(
        evaluate,
        move |_, field| Ok(Field::new(field.name().clone(), dtype.clone())),
        "thinkthen",
    ))
}
fn evaluate(
    engine: &Engine,
    function: RequestFunction,
    definition: &RequestDefinition,
    options: &PolarsExprOptions,
    column: Column,
) -> Result<Column, polars::prelude::PolarsError> {
    let source = column.as_materialized_series();
    let call = options
        .run(|| {
            request::execute_definition(
                engine,
                definition.clone(),
                source,
                function,
                options.call()?,
            )
        })
        .map_err(compute_error)?;
    let value = call
        .try_map(|value| result::value_column(&value, function, definition, source))
        .map_err(compute_error)?;
    Ok(value.into_value().into())
}
fn related(
    engine: &Engine,
    question: &Relate,
    definition: &RequestDefinition,
    options: &PolarsExprOptions,
    columns: &mut [Column],
) -> Result<Column, polars::prelude::PolarsError> {
    let first = columns
        .first()
        .ok_or_else(|| compute_error(Error::defect("a relation lost its names")))?;
    let second = columns
        .get(1)
        .ok_or_else(|| compute_error(Error::defect("a relation lost its kinds")))?;
    let mut first = first.clone();
    first.rename("name".into());
    let mut second = second.clone();
    second.rename("kind".into());
    let frame = DataFrame::new(first.len(), vec![first, second])?;
    let source = frame.column("name")?.as_materialized_series();
    let call = options
        .run(|| {
            super::complete::related_call(engine, question, &frame, "name", "kind", options.call()?)
        })
        .map_err(compute_error)?;
    let value = call
        .try_map(|value| result::value_column(&value, RequestFunction::Relate, definition, source))
        .map_err(compute_error)?;
    Ok(value.into_value().into())
}
impl Engine {
    /// Keep matching rows as value/probability structs in their original order.
    /// # Errors
    /// Refuses invalid controls or non-decide questions before collection.
    pub fn filter_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        let question = options.question(question)?;
        if question.kind() != crate::QuestionKind::Decide {
            return Err(Error::usage("filter_expr needs a decide question"));
        }
        expression(
            self,
            RequestFunction::Filter,
            question.into(),
            input,
            options,
        )
    }
    /// Rank the whole logical collection once and retain original row indices.
    /// # Errors
    /// Refuses a non-rank question or invalid controls before collection.
    pub fn rank_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        let question = options.question(question)?;
        if question.kind() != crate::QuestionKind::Rank {
            return Err(Error::usage("rank_expr needs a rank question"));
        }
        expression(self, RequestFunction::Rank, question.into(), input, options)
    }
    /// Find one original unit in a complete logical collection.
    /// # Errors
    /// Refuses a non-find question or invalid controls before collection.
    pub fn find_expr(
        &self,
        question: &Question,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        let question = options.question(question)?;
        if question.kind() != crate::QuestionKind::Find {
            return Err(Error::usage("find_expr needs a find question"));
        }
        expression(self, RequestFunction::Find, question.into(), input, options)
    }
    /// Annotate every present text with typed named values and null probability.
    /// # Errors
    /// Refuses invalid call controls before collection.
    pub fn annotate_expr(
        &self,
        questions: &QuestionSet,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        options.aggregate()?;
        expression(
            self,
            RequestFunction::Annotate,
            questions.clone().into(),
            input,
            options,
        )
    }
    /// Recognize present texts with a typed probability list for each row.
    /// # Errors
    /// Refuses invalid call controls before collection.
    pub fn recognize_expr(
        &self,
        question: &Recognize,
        input: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        options.aggregate()?;
        expression(
            self,
            RequestFunction::Recognize,
            question.clone().into(),
            input,
            options,
        )
    }
    /// Relate the complete paired name/kind columns in one native call.
    /// Paired null rows are omitted; partly null entities are refused.
    /// # Errors
    /// Refuses invalid controls; collection returns native input/call errors.
    pub fn relate_expr(
        &self,
        question: &Relate,
        names: Expr,
        kinds: Expr,
        options: PolarsExprOptions,
    ) -> Result<Expr, Error> {
        options.aggregate()?;
        let definition = RequestDefinition::from(question.clone());
        let dtype =
            result::expression_type(RequestFunction::Relate, &definition, &DataType::String)?;
        let engine = self.clone();
        let question = question.clone();
        Ok(names.apply_many(
            move |columns| caught(|| related(&engine, &question, &definition, &options, columns)),
            &[kinds],
            move |_, fields| {
                let field = fields.first().ok_or_else(|| {
                    compute_error(Error::defect("a relation lost its column type"))
                })?;
                Ok(Field::new(field.name().clone(), dtype.clone()))
            },
        ))
    }
}
