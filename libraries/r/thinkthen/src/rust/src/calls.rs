//! R completion handles retain the shared native session and its final packets.
use crate::{carry, defect};
use serde_json::{Value, json};
use std::sync::{Mutex, MutexGuard, PoisonError};
use thinkthen::{
    RequestSession, RequestSessionRead, RequestSessionResult, RequestSessionRow, RequestValue,
};
pub(crate) type Crossed<T> = Result<T, crate::NativeError>;

pub(crate) struct Completion(Mutex<State>);
struct State {
    name: &'static str,
    kind: Option<String>,
    session: Option<RequestSession>,
    positions: Option<Vec<usize>>,
    details: Vec<Value>,
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
            name: "unused",
            kind: None,
            session: None,
            positions: None,
            details: Vec::new(),
            packet: json!({"results": []}),
            value: None,
            error: None,
        }))
    }
    fn state(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
    pub(crate) fn claim(&self) -> bool {
        let mut state = self.state();
        if state.name != "unused" {
            return false;
        }
        state.name = "claimed";
        true
    }
    pub(crate) fn settle_early(&self, kind: String) {
        let mut state = self.state();
        if state.name == "claimed" {
            state.name = "terminal";
            state.kind = Some(kind);
        }
    }
    pub(crate) fn start(&self, session: RequestSession, positions: Option<Vec<usize>>) {
        let mut state = self.state();
        state.session = Some(session);
        state.positions = positions;
        state.name = "running";
    }
    pub(crate) fn cancel(&self) {
        if let Some(session) = &self.state().session {
            session.cancel();
        }
    }
    pub(crate) fn wait(&self) {
        if let Some(session) = &self.state().session {
            let _ = session.wait(
                thinkthen::RequestSessionWaitInterest::Settlement,
                std::time::Duration::from_millis(50),
            );
        }
    }
    pub(crate) fn drain(&self) -> Crossed<bool> {
        let mut state = self.state();
        loop {
            let Some(session) = &state.session else {
                return Ok(state.name == "terminal");
            };
            let packet = match session.try_read() {
                RequestSessionRead::Result(packet) => packet,
                RequestSessionRead::Pending => return Ok(false),
                RequestSessionRead::End => return Ok(state.name == "terminal"),
            };
            let text = packet.to_json().map_err(|e| carry(&e))?;
            let document: Value = serde_json::from_str(&text)
                .map_err(|_| defect("session packet could not be decoded"))?;
            if state.accept(packet, document)? {
                return Ok(true);
            }
        }
    }
    pub(crate) fn read(&self) -> Crossed<extendr_api::Robj> {
        self.drain()?;
        let state = self.state();
        let mut fields = vec![("state", extendr_api::Robj::from(state.name))];
        if let Some(kind) = &state.kind {
            fields.push(("kind", kind.as_str().into()));
            fields.push((
                "facts",
                match state.packet.get("facts") {
                    Some(value) => crate::native_results::facts(value)?,
                    None => ().into(),
                },
            ));
            fields.push((
                "details",
                crate::native_results::plain(&json!(state.details))?,
            ));
        }
        Ok(extendr_api::List::from_pairs(fields).into())
    }

    pub(crate) fn result(
        &self,
        function: thinkthen::RequestFunction,
        column: Option<&thinkthen::RequestColumn>,
    ) -> Crossed<extendr_api::Robj> {
        let state = self.state();
        if state.value.is_none()
            && let Some(error) = &state.error
        {
            return Err(carry(error));
        }
        let value = crate::native_results::request_packet(&state.packet.to_string(), function)?;
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
        crate::request::view(value, view)
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
                let mut detail = document
                    .get("value")
                    .ok_or_else(|| defect("session packet has no payload"))?
                    .clone();
                if detail.get("kind").and_then(Value::as_str) != Some("question") {
                    return Ok(false);
                }
                if let Some(index) = detail.get("index").and_then(Value::as_u64)
                    && let Some(position) = self
                        .positions
                        .as_ref()
                        .and_then(|p| usize::try_from(index).ok().and_then(|i| p.get(i)))
                {
                    field(&mut detail, "index", json!(position))?;
                }
                self.details.push(detail);
            }
            RequestSessionResult::Terminal(terminal) => {
                self.name = "terminal";
                self.kind = Some(
                    terminal
                        .error
                        .as_ref()
                        .map_or("success", |e| e.kind().name())
                        .into(),
                );
                if let Some(facts) = document.get("facts") {
                    field(&mut self.packet, "facts", facts.clone())?;
                }
                if let Some(failure) = document.get("failure") {
                    field(&mut self.packet, "failure", failure.clone())?;
                }
                self.error = terminal.error;
                self.session = None;
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
