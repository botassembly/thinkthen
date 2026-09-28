//! Ordered, bounded scheduling below every ordinary record command.

use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, SyncSender, channel};

use crate::engine::error::Error;
use crate::engine::workers;

/// One framed input event from the command-owned reader.
pub(crate) enum Input<T, E> {
    Item(T),
    Failed(E),
    End,
}

/// One completed item, the records it finished, and the counters the
/// scheduler owns. A `stop` ends the run at the record after those.
pub(crate) struct Completed<R, E> {
    pub(crate) value: R,
    /// Number of finished records answered from a recording.
    pub(crate) replayed: usize,
    pub(crate) partial_failure: bool,
    pub(crate) records: usize,
    pub(crate) stop: Option<E>,
}

impl<R, E> Completed<R, E> {
    /// One record's row, with no stop after it.
    pub(crate) const fn one(value: R, replayed: bool, partial_failure: bool) -> Self {
        Self {
            value,
            replayed: if replayed { 1 } else { 0 },
            partial_failure,
            records: 1,
            stop: None,
        }
    }
}

/// The ordered stop metadata returned after all request workers have joined.
#[derive(Debug)]
pub(crate) enum Outcome<E> {
    Complete,
    Stopped {
        finished: usize,
        replayed: usize,
        held: bool,
        cause: E,
    },
}

enum Event<T, R, E> {
    Input(Input<T, E>),
    Answered(usize, Result<Completed<R, E>, E>),
}

/// The typed command-to-engine side of the framed-input bridge.
pub(crate) struct InputPort<T, R, E>(Sender<Event<T, R, E>>);

impl<T, R, E> InputPort<T, R, E> {
    pub(crate) fn send(&self, input: Input<T, E>) -> Result<(), ()> {
        self.0.send(Event::Input(input)).map_err(|_| ())
    }
}

type Done<R, E> = Result<Completed<R, E>, E>;

/// Output retention and the scheduler's admission window are separate choices.
#[derive(Clone, Copy)]
pub(crate) enum RecordFlow {
    Streaming,
    HeldAll,
    HeldWindowed,
}

impl RecordFlow {
    const fn held(self) -> bool {
        !matches!(self, Self::Streaming)
    }
}

struct Run<R, E> {
    pending: BTreeMap<usize, Done<R, E>>,
    next: usize,
    finished: usize,
    dispatched: usize,
    in_flight: usize,
    replayed: usize,
    reading: bool,
    exhausted: bool,
    halted: bool,
    printing: bool,
    stop: Option<E>,
}

impl<R, E> Run<R, E> {
    const fn new() -> Self {
        Self {
            pending: BTreeMap::new(),
            next: 0,
            finished: 0,
            dispatched: 0,
            in_flight: 0,
            replayed: 0,
            reading: false,
            exhausted: false,
            halted: false,
            printing: true,
            stop: None,
        }
    }

    fn request(
        &mut self,
        ask: &Sender<()>,
        jobs: usize,
        flow: RecordFlow,
        defect: fn(&'static str) -> E,
    ) {
        if !self.reading && !self.halted && !self.exhausted && self.waiting(flow) < jobs {
            self.reading = ask.send(()).is_ok();
            if !self.reading {
                self.refuse(defect("the record reader ended early"));
            }
        }
    }

    fn accept<T>(
        &mut self,
        event: Event<T, R, E>,
        work: &SyncSender<(usize, T)>,
        defect: fn(&'static str) -> E,
    ) {
        match event {
            Event::Input(input) => {
                self.reading = false;
                if self.halted {
                    return;
                }
                match input {
                    Input::End => self.exhausted = true,
                    Input::Failed(error) => self.refuse(error),
                    Input::Item(value) => self.send(value, work, defect),
                }
            }
            Event::Answered(place, result) => {
                self.in_flight -= 1;
                self.halted |= !matches!(&result, Ok(done) if done.stop.is_none());
                self.pending.insert(place, result);
            }
        }
    }

    const fn done(&self) -> bool {
        self.in_flight == 0 && (self.halted || self.exhausted)
    }

    const fn waiting(&self, flow: RecordFlow) -> usize {
        if matches!(flow, RecordFlow::HeldAll) {
            self.in_flight
        } else {
            self.dispatched - self.next
        }
    }

    fn refuse(&mut self, error: E) {
        self.pending.insert(self.dispatched, Err(error));
        self.dispatched += 1;
        self.halted = true;
    }

    fn send<T>(&mut self, value: T, work: &SyncSender<(usize, T)>, defect: fn(&'static str) -> E) {
        if work.send((self.dispatched, value)).is_err() {
            self.refuse(defect("every worker ended before the records did"));
            return;
        }
        self.dispatched += 1;
        self.in_flight += 1;
    }

    #[allow(
        clippy::excessive_nesting,
        reason = "the ordered state machine handles one result inside one draining loop"
    )]
    fn drain(&mut self, emit: &mut impl FnMut(R) -> Result<bool, E>) -> Result<(), E> {
        while self.printing {
            let Some(result) = self.pending.remove(&self.next) else {
                return Ok(());
            };
            match result {
                Err(cause) => {
                    self.stop = Some(cause);
                    self.halted = true;
                    self.printing = false;
                }
                Ok(completed) => {
                    self.replayed += completed.replayed;
                    let more = emit(completed.value)?;
                    self.finished += completed.records;
                    self.next += 1;
                    self.stop = completed.stop;
                    if !more || self.stop.is_some() {
                        self.halted = true;
                        self.printing = false;
                    }
                }
            }
        }
        Ok(())
    }

    fn finish(self, flow: RecordFlow) -> Outcome<E> {
        match self.stop {
            Some(cause) => Outcome::Stopped {
                finished: self.finished,
                replayed: self.replayed,
                held: flow.held(),
                cause,
            },
            None => Outcome::Complete,
        }
    }
}

/// Schedule framed inputs over scoped workers and emit results in input order.
#[expect(
    clippy::too_many_arguments,
    reason = "the private scheduler keeps cancellation and each typed bridge callback explicit"
)]
pub(crate) fn run_cancelled<T, R, E>(
    jobs: usize,
    flow: RecordFlow,
    cancel: &crate::engine::Cancel,
    start_reader: impl FnOnce(Receiver<()>, InputPort<T, R, E>),
    answer: &(impl Fn(&T) -> Result<Completed<R, E>, E> + Sync),
    emit: impl FnMut(R) -> Result<bool, E>,
    defect: fn(&'static str) -> E,
    stopped: impl Fn(Error) -> E + Sync,
) -> Result<Outcome<E>, E>
where
    T: Send + 'static,
    R: Send,
    E: Send,
{
    run_observed(
        jobs,
        flow,
        cancel,
        start_reader,
        answer,
        emit,
        defect,
        &stopped,
        &|| (),
    )
}

#[allow(
    clippy::too_many_arguments,
    reason = "the test observer proves scheduler worker lifetime"
)]
fn run_observed<T, R, E, G>(
    jobs: usize,
    flow: RecordFlow,
    cancel: &crate::engine::Cancel,
    start_reader: impl FnOnce(Receiver<()>, InputPort<T, R, E>),
    answer: &(impl Fn(&T) -> Result<Completed<R, E>, E> + Sync),
    mut emit: impl FnMut(R) -> Result<bool, E>,
    defect: fn(&'static str) -> E,
    stopped: &(impl Fn(Error) -> E + Sync),
    begin: &(impl Fn() -> G + Sync),
) -> Result<Outcome<E>, E>
where
    T: Send + 'static,
    R: Send,
    E: Send,
    G: Send,
{
    let (events, received) = channel();
    let (ask, asked) = channel();
    start_reader(asked, InputPort(events.clone()));
    workers::scoped_observed(
        jobs,
        events,
        &|(place, value)| {
            Event::Answered(
                place,
                match cancel.stop() {
                    Some(stop) => Err(stopped(stop)),
                    None => answer(&value),
                },
            )
        },
        begin,
        |work| {
            let mut state = Run::new();
            loop {
                state.drain(&mut emit)?;
                if let Some(stop) = cancel.stop().filter(|_| !state.halted) {
                    state.refuse(stopped(stop));
                    continue;
                }
                if state.done() {
                    break;
                }
                state.request(&ask, jobs, flow, defect);
                cancel.observed_block();
                match received.recv_timeout(crate::engine::Cancel::poll()) {
                    Ok(event) => state.accept(event, &work, defect),
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        return Err(defect("the record scheduler ended early"));
                    }
                }
            }
            drop(ask);
            Ok(state.finish(flow))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{Completed, Input, Outcome, RecordFlow, run_observed};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    struct Lifetime(Arc<AtomicUsize>);

    impl Drop for Lifetime {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn cancellation_while_waiting_for_input_requests_nothing_more() {
        let (waiting_send, waiting) = std::sync::mpsc::channel();
        let cancel = crate::engine::Cancel::observed(waiting_send);
        let run_cancel = cancel.clone();
        let (asked_send, asked_recv) = std::sync::mpsc::channel();
        let (outcome_send, outcome_recv) = std::sync::mpsc::channel();
        let run = thread::spawn(move || {
            let outcome = super::run_cancelled(
                1,
                RecordFlow::Streaming,
                &run_cancel,
                move |asked, _events| asked_send.send(asked).expect("input requests"),
                &|_: &()| -> Result<Completed<(), &'static str>, &'static str> {
                    unreachable!("withheld input cannot be dispatched")
                },
                |_| Ok(true),
                |_| "defect",
                |_| "cancelled",
            );
            outcome_send.send(outcome).expect("returned outcome");
        });
        let asked = asked_recv
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("request receiver");
        asked
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("first input request");
        waiting
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("scheduler entered recv_timeout");

        cancel.fire();
        let outcome = outcome_recv
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("cancelled scheduler returns")
            .expect("cancelled outcome");
        run.join().expect("scheduler thread");

        assert_eq!(asked.try_iter().count(), 0);
        assert!(matches!(
            outcome,
            Outcome::Stopped {
                finished: 0,
                cause: "cancelled",
                ..
            }
        ));
    }

    #[test]
    #[allow(clippy::excessive_nesting, reason = "synchronized reader fixture")]
    fn cancellation_with_work_in_flight_ignores_later_input_and_joins() {
        let cancel = crate::engine::Cancel::default();
        let run_cancel = cancel.clone();
        let started = Arc::new(Barrier::new(2));
        let answer_started = Arc::clone(&started);
        let second_requested = Arc::new(Barrier::new(2));
        let reader_requested = Arc::clone(&second_requested);
        let later_input = Arc::new(Barrier::new(2));
        let reader_later = Arc::clone(&later_input);
        let release = Arc::new(Barrier::new(2));
        let answer_release = Arc::clone(&release);
        let active = Arc::new(AtomicUsize::new(0));
        let answer_active = Arc::clone(&active);
        let starts = Arc::new(AtomicUsize::new(0));
        let answer_starts = Arc::clone(&starts);
        let (emitted_send, emitted) = std::sync::mpsc::channel();
        let (outcome_send, outcome) = std::sync::mpsc::channel();

        let run = thread::spawn(move || {
            let result = super::run_cancelled(
                2,
                RecordFlow::Streaming,
                &run_cancel,
                move |asked, events| {
                    thread::spawn(move || {
                        asked.recv().expect("first input request");
                        events.send(Input::Item(0)).expect("first input");
                        asked.recv().expect("second input request");
                        reader_requested.wait();
                        reader_later.wait();
                        let _ignored = events.send(Input::Item(1));
                    });
                },
                &move |item| {
                    answer_starts.fetch_add(1, Ordering::SeqCst);
                    answer_active.fetch_add(1, Ordering::SeqCst);
                    answer_started.wait();
                    answer_release.wait();
                    answer_active.fetch_sub(1, Ordering::SeqCst);
                    Ok::<_, &'static str>(Completed::one(*item, false, false))
                },
                |value| {
                    emitted_send.send(value).expect("emitted result");
                    Ok(true)
                },
                |_| "defect",
                |_| "cancelled",
            );
            outcome_send.send(result).expect("returned outcome");
        });

        started.wait();
        second_requested.wait();
        cancel.fire();
        later_input.wait();
        release.wait();
        let result = outcome
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("the cancelled run returns")
            .expect("the scheduler returns metadata");
        run.join().expect("scheduler thread");

        assert_eq!(emitted.try_iter().collect::<Vec<_>>(), [0]);
        assert_eq!(starts.load(Ordering::SeqCst), 1);
        assert_eq!(active.load(Ordering::SeqCst), 0);
        assert!(matches!(
            result,
            Outcome::Stopped {
                finished: 1,
                cause: "cancelled",
                ..
            }
        ));
    }

    #[test]
    #[allow(
        clippy::excessive_nesting,
        reason = "the actual scheduler test starts its command-owned reader inline"
    )]
    fn the_process_survives_after_the_actual_scheduler_joins_every_worker() {
        let active = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&active);
        let started = Arc::new(Barrier::new(3));
        let gathered = Arc::clone(&started);
        let worker_lifetime = move || {
            observed.fetch_add(1, Ordering::SeqCst);
            Lifetime(Arc::clone(&observed))
        };
        let outcome = run_observed(
            3,
            RecordFlow::HeldAll,
            &crate::engine::Cancel::default(),
            |requests, events| {
                thread::spawn(move || {
                    for item in 0..3 {
                        requests.recv().expect("one request for each item");
                        events.send(Input::Item(item)).expect("the scheduler waits");
                    }
                    requests.recv().expect("one request for the end");
                    events.send(Input::End).expect("the scheduler waits");
                });
            },
            &move |item| {
                gathered.wait();
                Ok::<_, ()>(Completed::one(*item, false, false))
            },
            |_| Ok(true),
            |_| (),
            &|_| (),
            &worker_lifetime,
        )
        .expect("the scheduler runs");
        assert!(matches!(outcome, Outcome::Complete));
        assert_eq!(active.load(Ordering::SeqCst), 0, "all worker threads ended");
        assert_eq!(2 + 2, 4, "the test process remains alive after return");
    }
}
