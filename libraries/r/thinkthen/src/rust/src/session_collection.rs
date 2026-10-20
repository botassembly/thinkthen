//! Feed collection retains native packets until the shared pull-call bridge exists.
use crate::{carry, defect};
use serde_json::{Value, json};
use std::sync::{Mutex, MutexGuard, PoisonError};
use thinkthen::{
    RequestSessionResult, RequestSessionRow, RequestValue,
};
use crate::calls::Crossed;

pub(crate) struct Completion(Mutex<State>);
struct State {
    observations: Vec<Value>,
    packet: Value,
    value: Option<RequestValue>,
    error: Option<thinkthen::Error>,
}
impl std::fmt::Debug for Completion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Completion { content withheld }")
    }
}
impl Completion {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(State {
            observations: Vec::new(),
            packet: json!({"results": []}),
            value: None,
            error: None,
        }))
    }
    fn state(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
    pub(crate) fn accept(&self, packet: RequestSessionResult, document: Value) -> Crossed<()> {
        self.state().accept(packet, document)?;
        Ok(())
    }
    pub(crate) fn result(
        &self,
        function: thinkthen::RequestFunction,
        column: Option<&thinkthen::RequestColumn>,
        scalar: bool,
    ) -> Crossed<extendr_api::Robj> {
        let state = self.state();
        if state.value.is_none()
            && let Some(error) = &state.error
        {
            return Err(carry(error));
        }
        let value = crate::native_results::request_packet(
            &state.packet.to_string(),
            function,
            &state.observations,
            scalar,
        )?;
        let empty = match function {
            thinkthen::RequestFunction::Decide => Some(RequestValue::Decisions(Vec::new())),
            thinkthen::RequestFunction::Choose => Some(RequestValue::Choices(Vec::new())),
            thinkthen::RequestFunction::Tag => Some(RequestValue::Tags(Vec::new())),
            thinkthen::RequestFunction::Score => Some(RequestValue::Scores(Vec::new())),
            _ => None,
        };
        let view = column
            .zip(state.value.as_ref().or(empty.as_ref()))
            .map(|(column, value)| column.view(value).map_err(|e| carry(&e)))
            .transpose()?
            .flatten();
        crate::request::view(value, view, column, scalar)
    }
}
impl State {
    fn accept(&mut self, packet: RequestSessionResult, document: Value) -> Crossed<bool> {
        match packet {
            RequestSessionResult::Row(row) => {
                self.packet
                    .get_mut("results")
                    .and_then(Value::as_array_mut)
                    .ok_or_else(|| defect("row results are not a list"))?
                    .push(
                        document
                            .get("value")
                            .ok_or_else(|| defect("session packet has no payload"))?
                            .clone(),
                    );
                append(&mut self.value, row)?;
            }
            RequestSessionResult::Aggregate(value) => {
                field(
                    &mut self.packet,
                    "results",
                    document
                        .get("value")
                        .ok_or_else(|| defect("session packet has no payload"))?
                        .clone(),
                )?;
                self.value = Some(value);
            }
            RequestSessionResult::Observation { .. } => {
                self.observations.push(document.clone());
            }
            RequestSessionResult::Terminal(terminal) => {
                if let Some(facts) = document.get("facts") {
                    field(&mut self.packet, "facts", facts.clone())?;
                }
                if let Some(failure) = document.get("failure") {
                    field(&mut self.packet, "failure", failure.clone())?;
                }
                self.error = terminal.error;
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn append(value: &mut Option<RequestValue>, row: RequestSessionRow) -> Crossed<()> {
    macro_rules! add {
        ($row:expr, $kind:ident) => {{
            let rows = value.get_or_insert_with(|| RequestValue::$kind(Vec::new()));
            let RequestValue::$kind(rows) = rows else {
                return Err(defect("session row function changed"));
            };
            rows.push($row);
        }};
    }
    match row {
        RequestSessionRow::Decision(row) => add!(row, Decisions),
        RequestSessionRow::Choice(row) => add!(row, Choices),
        RequestSessionRow::Tags(row) => add!(row, Tags),
        RequestSessionRow::Score(row) => add!(row, Scores),
        RequestSessionRow::Filter(row) => add!(row, Filtered),
        RequestSessionRow::Annotation(row) => add!(row, Annotations),
    }
    Ok(())
}

fn field(object: &mut Value, name: &str, value: Value) -> Crossed<()> {
    object
        .as_object_mut()
        .ok_or_else(|| defect("session document is not an object"))?
        .insert(name.into(), value);
    Ok(())
}
