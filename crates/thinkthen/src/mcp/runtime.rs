//! One bounded protocol inbox, one native dispatch worker and responsive input.

use super::{
    admission::{CallParams, Invocation},
    output::{NativeObject, Output, native_error, tool_result},
    protocol::{self, Fault, Id, Message},
    tools,
};
use crate::CancelToken;
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Implement only with the settled complete native execution path. The object
/// owns one resolved Engine and its default cache; no subprocess or host helper.
pub(super) trait Executor: Send + Sync + 'static {
    fn output_schema(&self) -> Value;
    fn execute(
        &self,
        invocation: Invocation,
        token: &CancelToken,
    ) -> Result<NativeReply, crate::Error>;
}

pub(super) struct NativeReply {
    pub(super) object: NativeObject,
    pub(super) failed: bool,
}

struct Active {
    id: Id,
    token: CancelToken,
}
type Registry = Arc<Mutex<Option<Active>>>;
enum Event {
    Message {
        message: Message,
        token: Option<CancelToken>,
    },
    Fault {
        id: Option<Id>,
        fault: Fault,
    },
}

#[derive(Default)]
enum Phase {
    #[default]
    New,
    Negotiated,
    Ready,
}

#[derive(Default)]
struct State {
    phase: Phase,
    worker: Option<JoinHandle<io::Result<()>>>,
}

/// The reader factory MUST produce a reader whose blocked reads return when
/// stop fires. This allows joining input even on broken stdout. No engine/key
/// is captured by the reader. Dispatch and all native workers join before exit.
pub(super) fn serve<R, W, D>(
    reader: impl FnOnce(CancelToken) -> io::Result<R> + Send + 'static,
    writer: impl FnOnce(CancelToken) -> io::Result<W>,
    executor: D,
) -> io::Result<()>
where
    R: BufRead + Send + 'static,
    W: Write + Send + 'static,
    D: Executor,
{
    let stop = CancelToken::new();
    let output = Output::new(writer(stop.clone())?);
    let active: Registry = Arc::new(Mutex::new(None));
    let (sender, receiver) = mpsc::sync_channel(4);
    let input = spawn_reader(reader, sender, stop.clone(), Arc::clone(&active))?;
    let executor = Arc::new(executor);
    let mut state = State::default();
    let result = supervise(&receiver, &output, &executor, &stop, &active, &mut state);
    stop.cancel();
    cancel_active(&active);
    let joined = state.worker.map_or(Ok(()), join);
    let read = input
        .join()
        .map_err(|_| io::Error::other("MCP reader failed"))?;
    result.and(joined).and(read)
}

fn supervise<W: Write + Send + 'static, D: Executor>(
    receiver: &mpsc::Receiver<Event>,
    output: &Output<W>,
    executor: &Arc<D>,
    stop: &CancelToken,
    active: &Registry,
    state: &mut State,
) -> io::Result<()> {
    while !stop.is_cancelled() {
        if state.worker.as_ref().is_some_and(JoinHandle::is_finished)
            && let Some(done) = state.worker.take()
        {
            join(done)?;
        }
        let event = match receiver.recv_timeout(Duration::from_millis(20)) {
            Ok(event) => event,
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        };
        if stop.is_cancelled() {
            break;
        }
        let (message, token) = match event {
            Event::Message { message, token } => (message, token),
            Event::Fault { id, fault } => {
                output.fault(id.as_ref(), fault)?;
                continue;
            }
        };
        let Some(id) = message.id.as_ref() else {
            if message.method == "notifications/initialized"
                && matches!(state.phase, Phase::Negotiated)
                && protocol::empty_params(&message).is_ok()
            {
                state.phase = Phase::Ready;
            }
            continue;
        };
        if message.method == "tools/call" {
            let outcome = start_call(message, token, output, executor, state, active);
            if let Err((id, fault)) = outcome {
                clear_active(active);
                output.fault(Some(&id), fault)?;
            }
        } else {
            let outcome = immediate(&message, executor, &mut state.phase);
            match outcome {
                Ok(value) => output.result(id, value)?,
                Err(fault) => output.fault(Some(id), fault)?,
            }
        }
    }
    Ok(())
}

fn start_call<W: Write + Send + 'static, D: Executor>(
    message: Message,
    token: Option<CancelToken>,
    output: &Output<W>,
    executor: &Arc<D>,
    state: &mut State,
    active: &Registry,
) -> Result<(), (Id, Fault)> {
    let Some(id) = message.id.clone() else {
        return Ok(());
    };
    if !matches!(state.phase, Phase::Ready) {
        return Err((id, Fault::STATE));
    }
    let params = protocol::params::<CallParams>(&message).map_err(|fault| (id.clone(), fault))?;
    let token = token.ok_or_else(|| (id.clone(), Fault::BUSY))?;
    if let Some(done) = state.worker.take() {
        join(done).map_err(|_| {
            (
                id.clone(),
                Fault {
                    code: -32603,
                    message: "MCP dispatch failed",
                },
            )
        })?;
    }
    let registry = Arc::clone(active);
    let executor = Arc::clone(executor);
    let output = output.clone();
    let refusal_id = id.clone();
    let handle = thread::Builder::new()
        .name("thinkthen-mcp-call".into())
        .spawn(move || {
            let mut owned = ActiveCall {
                registry,
                id: Some(id.clone()),
            };
            if token.is_cancelled() {
                return Ok(());
            }
            let invocation = match Invocation::admit(params) {
                Ok(invocation) => invocation,
                Err(error) => {
                    if token.is_cancelled() {
                        return Ok(());
                    }
                    let error = encode_error(&error)?;
                    owned.retire();
                    return output.result(&id, tool_result(error.object, true));
                }
            };
            let result = match executor.execute(invocation, &token) {
                Ok(result) => result,
                Err(error) => encode_error(&error)?,
            };
            // Client cancellation is fire-and-forget. Retain native cleanup/facts
            // internally; do not fabricate an answer after cancelling the request.
            if token.is_cancelled() {
                return Ok(());
            }
            owned.retire();
            output.result(&id, tool_result(result.object, result.failed))
        })
        .map_err(|_| {
            (
                refusal_id,
                Fault {
                    code: -32603,
                    message: "MCP worker unavailable",
                },
            )
        })?;
    state.worker = Some(handle);
    Ok(())
}

fn immediate<D: Executor>(
    message: &Message,
    executor: &Arc<D>,
    phase: &mut Phase,
) -> Result<Value, Fault> {
    match message.method.as_str() {
        "initialize" if matches!(phase, Phase::New) => {
            let params: protocol::Initialize = protocol::params(message)?;
            if params.version.is_empty()
                || params.client.name.is_empty()
                || params.client.version.is_empty()
            {
                return Err(Fault::PARAMS);
            }
            *phase = Phase::Negotiated;
            Ok(
                json!({"protocolVersion":protocol::VERSION,"capabilities":{"tools":{"listChanged":false}},
                "serverInfo":{"name":"thinkthen","version":env!("CARGO_PKG_VERSION")}}),
            )
        }
        "initialize" => Err(Fault::REQUEST),
        "ping" => {
            protocol::empty_params(message)?;
            Ok(json!({}))
        }
        "tools/list" if matches!(phase, Phase::Ready) => {
            protocol::empty_params(message)?;
            Ok(tools::list(&executor.output_schema()))
        }
        "tools/list" => Err(Fault::STATE),
        _ => Err(Fault::METHOD),
    }
}

fn spawn_reader<R>(
    factory: impl FnOnce(CancelToken) -> io::Result<R> + Send + 'static,
    sender: mpsc::SyncSender<Event>,
    stop: CancelToken,
    active: Registry,
) -> io::Result<JoinHandle<io::Result<()>>>
where
    R: BufRead + Send + 'static,
{
    thread::Builder::new()
        .name("thinkthen-mcp-input".into())
        .spawn(move || {
            let result = factory(stop.clone())
                .and_then(|mut reader| read_messages(&mut reader, &sender, &stop, &active));
            stop.cancel();
            cancel_active(&active);
            result
        })
}

fn read_messages(
    reader: &mut impl BufRead,
    sender: &mpsc::SyncSender<Event>,
    stop: &CancelToken,
    active: &Registry,
) -> io::Result<()> {
    while !stop.is_cancelled() {
        let Some(line) = protocol::read_line(reader)? else {
            break;
        };
        let bytes = match line {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "MCP framing refused",
                ));
            }
        };
        let message = match protocol::parse(&bytes) {
            Ok(message) => message,
            Err(fault) => {
                enqueue(sender, Event::Fault { id: None, fault })?;
                continue;
            }
        };
        if message.method == "notifications/cancelled" && message.id.is_none() {
            if let Ok(params) = protocol::params::<protocol::Cancellation>(&message)
                && let Ok(registry) = active.lock()
                && let Some(call) = registry.as_ref().filter(|call| call.id == params.id)
            {
                call.token.cancel();
            }
            continue;
        }
        let token = reserve(&message, active);
        if message.method == "tools/call" && message.id.is_some() && token.is_none() {
            enqueue(
                sender,
                Event::Fault {
                    id: message.id,
                    fault: Fault::BUSY,
                },
            )?;
            continue;
        }
        enqueue(sender, Event::Message { message, token })?;
    }
    Ok(())
}

/// Overflow closes the session rather than blocking cancellation behind reply
/// output or retaining an unbounded backlog. The reader never writes stdout.
fn enqueue(sender: &mpsc::SyncSender<Event>, event: Event) -> io::Result<()> {
    sender
        .try_send(event)
        .map_err(|_| io::Error::other("MCP protocol inbox closed or full"))
}

fn reserve(message: &Message, active: &Registry) -> Option<CancelToken> {
    if message.method != "tools/call" {
        return None;
    }
    let id = message.id.clone()?;
    let mut registry = active.lock().ok()?;
    if registry.is_some() {
        return None;
    }
    let token = CancelToken::new();
    *registry = Some(Active {
        id,
        token: token.clone(),
    });
    Some(token)
}
fn clear_active(active: &Registry) {
    if let Ok(mut registry) = active.lock() {
        *registry = None;
    }
}
fn cancel_active(active: &Registry) {
    if let Ok(registry) = active.lock()
        && let Some(call) = registry.as_ref()
    {
        call.token.cancel();
    }
}
fn join(handle: JoinHandle<io::Result<()>>) -> io::Result<()> {
    handle
        .join()
        .map_err(|_| io::Error::other("MCP dispatch failed"))?
}

fn encode_error(error: &crate::Error) -> io::Result<NativeReply> {
    let object = NativeObject::new(&native_error(error))
        .map_err(|_| io::Error::other("MCP error serialization failed"))?;
    Ok(NativeReply {
        object,
        failed: true,
    })
}

// Retire only this call before publishing its response. The next sequential
// call may arrive immediately; joining the previous worker cannot clear it.
struct ActiveCall {
    registry: Registry,
    id: Option<Id>,
}
impl ActiveCall {
    fn retire(&mut self) {
        let Some(id) = self.id.take() else {
            return;
        };
        if let Ok(mut registry) = self.registry.lock()
            && registry.as_ref().is_some_and(|call| call.id == id)
        {
            *registry = None;
        }
    }
}
impl Drop for ActiveCall {
    fn drop(&mut self) {
        self.retire();
    }
}
