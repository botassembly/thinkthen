//! Pulled complete batches stay on their native worker; the host only advances them.
use super::{Request, defect, inputs, questions, usage};
use questions::Asked;
use serde::Serialize;
use std::sync::{Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use thinkthen::fork_safe::{Receiver, RecvTimeoutError, Sender, channel};
use thinkthen::{
    Batch, CallOptions, CancelToken, CompleteRecord, Engine, Error, Facts, LoadedQuestion,
};

pub(crate) fn failure(error: &Error) -> Result<String, Error> {
    #[derive(Serialize)]
    struct Failed<'a> {
        kind: thinkthen::ErrorKind,
        message: String,
        retryable: bool,
        stopped: thinkthen::Stopped,
        #[serde(skip_serializing_if = "Option::is_none")]
        facts: Option<thinkthen::CompleteFacts<'a>>,
    }
    serde_json::to_string(&Failed {
        kind: error.kind(),
        message: error.to_string(),
        retryable: error.retryable(),
        stopped: error.stopped(),
        facts: error.facts().and_then(Facts::complete),
    })
    .map_err(|_| defect("complete failure could not be written"))
}

macro_rules! batch {
    ($engine:expr, $verb:expr, $asked:expr, $records:expr, $options:expr, $run:ident $(, $arg:expr)*) => {
        match ($verb, $asked) {
            ("decide", Asked::Atomic(LoadedQuestion::Question(q))) => $run($engine.try_decide_records_complete_with(&q,$records,$options) $(,$arg)*),
            ("decide", Asked::Atomic(LoadedQuestion::Banded(q))) => $run($engine.try_decide_records_complete_with(&q,$records,$options) $(,$arg)*),
            ("choose", Asked::Atomic(LoadedQuestion::Question(q))) => $run($engine.try_choose_records_complete_with(&q,$records,$options) $(,$arg)*),
            ("tag", Asked::Atomic(LoadedQuestion::Question(q))) => $run($engine.try_tag_records_complete_with(&q,$records,$options) $(,$arg)*),
            ("score", Asked::Atomic(LoadedQuestion::Question(q))) => $run($engine.try_score_records_complete_with(&q,$records,$options) $(,$arg)*),
            ("filter", Asked::Atomic(LoadedQuestion::Question(q))) => $run($engine.try_filter_records_complete_with(&q,$records,$options) $(,$arg)*),
            ("annotate", Asked::Set(q)) => $run($engine.try_annotate_records_complete_with(&q,$records,$options) $(,$arg)*),
            _ => Err(usage("a complete batch requires decide, choose, tag, score, filter or annotate")),
        }
    };
}
struct State {
    credit: Option<Sender<()>>,
    worker: Option<JoinHandle<()>>,
}
pub(crate) struct Session {
    state: Mutex<State>,
    output: Mutex<Receiver<String>>,
    stop: CancelToken,
}
impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CompleteBatch")
    }
}
impl Session {
    pub(crate) fn start(
        engine: Engine,
        text: String,
        deadline: Option<i64>,
        caller: Option<CancelToken>,
        context: Option<String>,
    ) -> Result<Self, Error> {
        let request: Request = super::parse(&text)?;
        let due = deadline
            .filter(|ms| *ms != -1)
            .map(|ms| {
                let ms = u64::try_from(ms).map_err(|_| usage("deadline must be nonnegative"))?;
                Instant::now()
                    .checked_add(Duration::from_millis(ms))
                    .ok_or_else(|| usage("deadline does not fit the clock"))
            })
            .transpose()?;
        let stop = CancelToken::new();
        let token = stop.clone();
        let (credit, input) = channel();
        let (output, receiver) = channel();
        let worker = std::thread::Builder::new()
            .name("thinkthen-complete-batch".into())
            .spawn(move || {
                Work {
                    engine,
                    request,
                    due,
                    caller,
                    context,
                    token,
                }
                .run(input, output)
            })
            .map_err(|_| defect("complete batch worker did not start"))?;
        Ok(Self {
            state: Mutex::new(State {
                credit: Some(credit),
                worker: Some(worker),
            }),
            output: Mutex::new(receiver),
            stop,
        })
    }
    pub(crate) fn advance(&self) -> Result<(), Error> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state
            .credit
            .as_ref()
            .ok_or_else(|| usage("complete batch is closed"))?
            .send(())
            .map_err(|_| usage("complete batch is exhausted"))
    }
    pub(crate) fn poll(&self) -> Result<Option<String>, Error> {
        match self
            .output
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .recv_timeout(Duration::from_millis(50))
        {
            Ok(value) => Ok(Some(value)),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => Err(usage("complete batch is exhausted")),
        }
    }
    pub(crate) fn cancel(&self) {
        self.stop.cancel();
    }
    pub(crate) fn close(&self) {
        self.stop.cancel();
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.credit.take();
        if let Some(worker) = state.worker.take() {
            let _joined = worker.join();
        }
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        self.close();
    }
}
struct Work {
    engine: Engine,
    request: Request,
    due: Option<Instant>,
    caller: Option<CancelToken>,
    context: Option<String>,
    token: CancelToken,
}
impl Work {
    fn run(self, input: Receiver<()>, output: Sender<String>) {
        let result = thinkthen::contained(|| self.execute(&input, &output))
            .unwrap_or_else(|| Err(defect("complete batch worker panicked")));
        if let Err(error) = result
            && let Ok(error) = failure(&error)
        {
            let _ignored = output.send(format!("{{\"error\":{error}}}"));
        }
    }
    fn execute(self, input: &Receiver<()>, output: &Sender<String>) -> Result<(), Error> {
        // Even native preparation waits for the first host pull.
        if input.recv().is_err() {
            return Ok(());
        }
        let interrupt =
            || self.request.cancel || self.caller.as_ref().is_some_and(CancelToken::is_cancelled);
        let mut options = CallOptions::new()
            .cancel(&self.token)
            .interrupt(&interrupt)
            .attempts(self.request.attempts);
        if let Some(due) = self.due {
            options = options.deadline_at(due);
        }
        if let Some(context) = self.request.context.as_deref().or(self.context.as_deref()) {
            options = options.context(context);
        }
        let asked = questions::load(&self.request.verb, self.request.question)?;
        let records = inputs::iter(
            &self.engine,
            self.request.input,
            asked.reading(),
            self.request.verb == "annotate",
        )?;
        batch!(
            &self.engine,
            self.request.verb.as_str(),
            asked,
            records,
            options,
            emit,
            input,
            output
        )
    }
}
fn emit<R>(
    mut rows: Batch<'_, CompleteRecord<inputs::Original, R>>,
    credit: &Receiver<()>,
    output: &Sender<String>,
) -> Result<(), Error>
where
    CompleteRecord<inputs::Original, R>: Serialize,
{
    loop {
        match rows.next() {
            Some(Ok(row)) => {
                #[derive(Serialize)]
                struct Row<'a, T> {
                    row: &'a T,
                    ordinal: usize,
                    input: inputs::InputView,
                }
                let packet = serde_json::to_string(&Row {
                    row: &row,
                    ordinal: row.ordinal(),
                    input: row.original().view(),
                })
                .map_err(|_| defect("complete batch row could not be written"))?;
                if output.send(packet).is_err() || credit.recv().is_err() {
                    return Ok(());
                }
            }
            Some(Err(error)) => return Err(error),
            None => {
                let facts = rows
                    .facts()
                    .and_then(Facts::complete)
                    .ok_or_else(|| defect("complete batch has no final facts"))?;
                let facts = serde_json::to_string(&facts)
                    .map_err(|_| defect("complete batch facts could not be written"))?;
                let _ignored = output.send(format!("{{\"facts\":{facts}}}"));
                return Ok(());
            }
        }
    }
}
