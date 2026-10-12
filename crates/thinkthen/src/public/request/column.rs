//! Nullable host columns share input admission and ordered result projection.
use super::{RequestFunction, RequestInput, RequestItem, RequestOriginal, RequestValue};
use crate::{Answer, Error, Probabilities};

/// Original positions of present cells in a nullable host column.
#[derive(Clone, Debug)]
pub struct RequestColumn {
    positions: Vec<usize>,
    length: usize,
}
/// Typed values for an ordinary atomic column.
#[derive(Debug)]
pub enum RequestColumnValues {
    /// Yes, no or missing.
    Decisions(Vec<Option<bool>>),
    /// Selected labels or missing.
    Choices(Vec<Option<String>>),
    /// Selected labels per cell, including empty missing cells.
    Tags(Vec<Vec<String>>),
    /// Weighted scores or missing.
    Scores(Vec<Option<f64>>),
}
/// Native presentation of an atomic nullable column.
#[derive(Debug)]
pub struct RequestColumnView {
    /// Values aligned to the caller's original cells.
    pub values: RequestColumnValues,
    /// Probabilities for decide and choose, aligned to the same cells.
    pub probability: Option<Vec<Option<f64>>>,
}
impl RequestColumn {
    /// Admit missing cells and retain original positions without host round trips.
    /// # Errors
    /// Refuses missing records for functions that require a complete collection.
    pub fn new(
        function: RequestFunction,
        cells: Vec<Option<RequestItem>>,
    ) -> Result<(Self, Vec<RequestItem>), Error> {
        let length = cells.len();
        let mut positions = Vec::new();
        let mut items = Vec::new();
        for (at, cell) in cells.into_iter().enumerate() {
            if let Some(item) = cell {
                positions.push(at);
                items.push(item);
            } else if !matches!(
                function,
                RequestFunction::Decide
                    | RequestFunction::Choose
                    | RequestFunction::Tag
                    | RequestFunction::Score
                    | RequestFunction::Recognize
            ) {
                let name = serde_json::to_value(function)
                    .map_err(|_| Error::defect("column function could not be named"))?;
                return Err(Error::usage(format!(
                    "{} takes no NA records",
                    name.as_str().unwrap_or("function")
                )));
            }
        }
        Ok((Self { positions, length }, items))
    }
    /// Original present-cell positions.
    #[must_use]
    pub fn positions(&self) -> &[usize] {
        &self.positions
    }
    /// Original cell count, including missing cells.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }
    /// Whether the original column has no cells.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }
    /// Project actual typed atomic results into the original nullable slots.
    /// # Errors
    /// Refuses a result prefix longer than the admitted present cells.
    pub fn view(&self, value: &RequestValue) -> Result<Option<RequestColumnView>, Error> {
        let view = match value {
            RequestValue::Decisions(rows) => RequestColumnView {
                values: RequestColumnValues::Decisions(self.project(rows, |row| {
                    match row.result.value() {
                        Answer::Yes => Some(true),
                        Answer::No => Some(false),
                        Answer::Unsure => None,
                    }
                })?),
                probability: Some(self.project(rows, |row| match row.result.probabilities() {
                    Probabilities::YesNo { yes } => Some(yes),
                    _ => None,
                })?),
            },
            RequestValue::Choices(rows) => RequestColumnView {
                values: RequestColumnValues::Choices(
                    self.project(rows, |row| row.result.value().map(str::to_owned))?,
                ),
                probability: Some(self.project(rows, |row| match row.result.probabilities() {
                    Probabilities::Named(values) => row.result.value().and_then(|name| {
                        values
                            .iter()
                            .find(|v| v.name() == name)
                            .map(|v| v.probability())
                    }),
                    _ => None,
                })?),
            },
            RequestValue::Tags(rows) => RequestColumnView {
                values: RequestColumnValues::Tags(
                    self.project(rows, |row| row.result.value().to_vec())?,
                ),
                probability: None,
            },
            RequestValue::Scores(rows) => RequestColumnView {
                values: RequestColumnValues::Scores(
                    self.project(rows, |row| Some(row.result.value()))?,
                ),
                probability: None,
            },
            _ => return Ok(None),
        };
        Ok(Some(view))
    }
    fn project<T, V: Clone + Default>(
        &self,
        rows: &[T],
        read: impl Fn(&T) -> V,
    ) -> Result<Vec<V>, Error> {
        if rows.len() > self.positions.len() {
            return Err(Error::defect("column results exceed present cells"));
        }
        let mut values = vec![V::default(); self.length];
        for (row, at) in rows.iter().zip(&self.positions) {
            let slot = values
                .get_mut(*at)
                .ok_or_else(|| Error::defect("column position exceeds original cells"))?;
            *slot = read(row);
        }
        Ok(values)
    }
}
impl RequestFunction {
    /// Select scalar or collection input from converted ordinary host values.
    /// Extra item fields retain record admission even for a single cell.
    #[must_use]
    pub fn host_input(self, items: Vec<RequestItem>, scalar: Option<RequestItem>) -> RequestInput {
        if let Some(item) = scalar
            && !matches!(
                self,
                Self::Filter | Self::Rank | Self::Find | Self::Annotate | Self::Relate
            )
        {
            if item.context.is_none()
                && item.options.is_none()
                && item.seed_spans.is_none()
                && item.examples.is_none()
            {
                return match item.original {
                    Some(RequestOriginal::Text { text }) => RequestInput::Text {
                        text,
                        images: item.images,
                    },
                    Some(RequestOriginal::Json { value }) => RequestInput::Json {
                        value,
                        images: item.images,
                    },
                    None => RequestInput::Records { items: vec![item] },
                };
            }
            return RequestInput::Records { items: vec![item] };
        }
        RequestInput::Records { items }
    }
}
