//! Ordered, bounded scheduling for grouped annotate requests.

use std::collections::{BTreeMap, VecDeque};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, SyncSender, channel};

use crate::engine::schedule::{Completed, Input};
use crate::engine::workers;

pub(crate) struct Prepared<S, A, W> {
    pub(crate) seed: S,
    pub(crate) accumulator: A,
    pub(crate) work: Vec<W>,
}

pub(crate) enum Outcome<E> {
    Complete {
        partial_failure: bool,
    },
    Failed(E),
    Stopped {
        at: usize,
        finished: usize,
        replayed: usize,
        cause: E,
    },
}

struct Task<W> {
    row: usize,
    group: usize,
    work: W,
}

enum Event<T, G, E> {
    Input(Input<T, E>),
    Answered {
        row: usize,
        group: usize,
        answer: Result<G, E>,
    },
}

pub(crate) struct InputPort<T, G, E>(Sender<Event<T, G, E>>);

impl<T, G, E> InputPort<T, G, E> {
    pub(crate) fn send(&self, input: Input<T, E>) -> Result<(), ()> {
        self.0.send(Event::Input(input)).map_err(|_| ())
    }
}

struct Row<S, A, G> {
    seed: S,
    accumulator: A,
    answers: Vec<Option<G>>,
    next: usize,
}

struct Run<S, A, W, G, R, E> {
    rows: BTreeMap<usize, Row<S, A, G>>,
    ready: BTreeMap<usize, Completed<R>>,
    pending: VecDeque<Task<W>>,
    failures: BTreeMap<(usize, usize), E>,
    next_row: usize,
    read_rows: usize,
    in_flight: usize,
    replayed: usize,
    reading: bool,
    exhausted: bool,
    halted: bool,
    printing: bool,
    quiet_stop: bool,
    partial_failure: bool,
}

impl<S, A, W, G, R, E> Run<S, A, W, G, R, E> {
    fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
            ready: BTreeMap::new(),
            pending: VecDeque::new(),
            failures: BTreeMap::new(),
            next_row: 0,
            read_rows: 0,
            in_flight: 0,
            replayed: 0,
            reading: false,
            exhausted: false,
            halted: false,
            printing: true,
            quiet_stop: false,
            partial_failure: false,
        }
    }

    fn stop(&mut self) {
        self.halted = true;
        self.pending.clear();
    }

    fn cancel(&mut self, error: E) {
        let (row, group) = self
            .pending
            .front()
            .map_or((self.read_rows, 0), |task| (task.row, task.group));
        self.failures.insert((row, group), error);
        self.stop();
    }

    fn failed(&mut self, row: usize, group: usize, error: E) {
        self.failures.insert((row, group), error);
        self.stop();
    }

    fn done(&self) -> bool {
        if self.halted {
            return self.in_flight == 0;
        }
        self.exhausted
            && !self.reading
            && self.in_flight == 0
            && self.pending.is_empty()
            && self.rows.is_empty()
            && self.ready.is_empty()
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "the private bridge keeps each boundary callback typed"
)]
pub(crate) fn run<T, S, A, W, G, R, E>(
    jobs: usize,
    streams: bool,
    cancel: &crate::engine::Cancel,
    start_reader: impl FnOnce(Receiver<()>, InputPort<T, G, E>),
    prepare: impl Fn(T) -> Result<Prepared<S, A, W>, E>,
    answer: &(impl Fn(W) -> Result<G, E> + Sync),
    accept: impl Fn(&mut A, G) -> Result<(), E>,
    finish: impl Fn(S, A) -> Result<Completed<R>, E>,
    mut emit: impl FnMut(R) -> Result<bool, E>,
    defect: fn(&'static str) -> E,
    cancelled: impl Fn() -> E + Sync,
) -> Result<Outcome<E>, E>
where
    T: Send + 'static,
    W: Send,
    G: Send,
    R: Send,
    E: Send,
{
    let (events, received) = channel();
    let (request, requested) = channel();
    start_reader(requested, InputPort(events.clone()));
    workers::scoped(
        jobs,
        events,
        &|task: Task<W>| Event::Answered {
            row: task.row,
            group: task.group,
            answer: if cancel.fired() {
                Err(cancelled())
            } else {
                answer(task.work)
            },
        },
        |work| {
            let mut state = Run::new();
            loop {
                drain(&mut state, &mut emit)?;
                if cancel.fired() && !state.halted {
                    state.cancel(cancelled());
                }
                dispatch(&mut state, &work, jobs, defect)?;
                request_input(&mut state, &request, jobs, streams, defect);
                if state.done() {
                    break;
                }
                cancel.observed_block();
                match received.recv_timeout(crate::engine::Cancel::poll()) {
                    Ok(event) => receive(
                        &mut state, event, streams, &prepare, &accept, &finish, defect,
                    )?,
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        return Err(defect("the annotate scheduler ended early"));
                    }
                }
            }
            drop(request);
            Ok(outcome(state, streams))
        },
    )
}

fn request_input<S, A, W, G, R, E>(
    state: &mut Run<S, A, W, G, R, E>,
    requests: &Sender<()>,
    jobs: usize,
    streams: bool,
    defect: fn(&'static str) -> E,
) {
    let limit = if streams { jobs } else { 1 };
    let waiting = state.read_rows.saturating_sub(state.next_row);
    if !state.halted
        && !state.exhausted
        && !state.reading
        && waiting < limit
        && state.pending.is_empty()
        && state.in_flight < jobs
    {
        state.reading = requests.send(()).is_ok();
        if !state.reading {
            state.failed(
                state.read_rows,
                0,
                defect("the annotate input reader ended early"),
            );
        }
    }
}

fn dispatch<S, A, W, G, R, E>(
    state: &mut Run<S, A, W, G, R, E>,
    send: &SyncSender<Task<W>>,
    jobs: usize,
    defect: fn(&'static str) -> E,
) -> Result<(), E> {
    while !state.halted && state.in_flight < jobs {
        let Some(work) = state.pending.pop_front() else {
            break;
        };
        send.send(work)
            .map_err(|_| defect("every annotate worker ended early"))?;
        state.in_flight += 1;
    }
    Ok(())
}

#[allow(
    clippy::too_many_arguments,
    reason = "event handling applies each typed bridge stage"
)]
fn receive<T, S, A, W, G, R, E>(
    state: &mut Run<S, A, W, G, R, E>,
    event: Event<T, G, E>,
    streams: bool,
    prepare: &impl Fn(T) -> Result<Prepared<S, A, W>, E>,
    accept: &impl Fn(&mut A, G) -> Result<(), E>,
    finish: &impl Fn(S, A) -> Result<Completed<R>, E>,
    defect: fn(&'static str) -> E,
) -> Result<(), E> {
    match event {
        Event::Input(input) => receive_input(state, input, streams, prepare),
        Event::Answered { row, group, answer } => {
            state.in_flight = state.in_flight.saturating_sub(1);
            if state.quiet_stop {
                return Ok(());
            }
            let answer = match answer {
                Ok(answer) => answer,
                Err(error) => {
                    state.failed(row, group, error);
                    return Ok(());
                }
            };
            let Some(progress) = state.rows.get_mut(&row) else {
                return Err(defect("an annotate answer names no row"));
            };
            if let Some(slot) = progress.answers.get_mut(group) {
                *slot = Some(answer);
            }
            let mut advance_error = None;
            while let Some(slot) = progress.answers.get_mut(progress.next) {
                let Some(answer) = slot.take() else { break };
                if let Err(error) = accept(&mut progress.accumulator, answer) {
                    advance_error = Some((progress.next, error));
                    break;
                }
                progress.next += 1;
            }
            if let Some((group, error)) = advance_error {
                state.failed(row, group, error);
                return Ok(());
            }
            if progress.next == progress.answers.len() {
                let progress = state
                    .rows
                    .remove(&row)
                    .ok_or_else(|| defect("a complete annotate row disappeared"))?;
                match finish(progress.seed, progress.accumulator) {
                    Ok(completed) => {
                        state.ready.insert(row, completed);
                    }
                    Err(error) => state.failed(row, group, error),
                }
            }
            Ok(())
        }
    }
}

fn receive_input<T, S, A, W, G, R, E>(
    state: &mut Run<S, A, W, G, R, E>,
    input: Input<T, E>,
    streams: bool,
    prepare: &impl Fn(T) -> Result<Prepared<S, A, W>, E>,
) -> Result<(), E> {
    state.reading = false;
    if state.halted {
        return Ok(());
    }
    let input = match input {
        Input::End => {
            state.exhausted = true;
            return Ok(());
        }
        Input::Failed(error) => {
            state.failed(state.read_rows, 0, error);
            return Ok(());
        }
        Input::Item(input) => input,
    };
    let row = state.read_rows;
    state.read_rows += 1;
    let prepared = match prepare(input) {
        Ok(prepared) => prepared,
        Err(error) => {
            state.failed(row, 0, error);
            return Ok(());
        }
    };
    let groups = prepared.work.len();
    state.rows.insert(
        row,
        Row {
            seed: prepared.seed,
            accumulator: prepared.accumulator,
            answers: std::iter::repeat_with(|| None).take(groups).collect(),
            next: 0,
        },
    );
    state.pending.extend(
        prepared
            .work
            .into_iter()
            .enumerate()
            .map(|(group, work)| Task { row, group, work }),
    );
    if !streams {
        state.exhausted = true;
    }
    Ok(())
}

fn drain<S, A, W, G, R, E>(
    state: &mut Run<S, A, W, G, R, E>,
    emit: &mut impl FnMut(R) -> Result<bool, E>,
) -> Result<(), E> {
    while state.printing {
        let Some(completed) = state.ready.remove(&state.next_row) else {
            break;
        };
        state.replayed += usize::from(completed.replayed);
        state.partial_failure |= completed.partial_failure;
        if !emit(completed.value)? {
            state.stop();
            state.printing = false;
            state.quiet_stop = true;
        }
        state.next_row += 1;
    }
    Ok(())
}

fn outcome<S, A, W, G, R, E>(mut state: Run<S, A, W, G, R, E>, streams: bool) -> Outcome<E> {
    let Some(((row, _), cause)) = state.failures.pop_first() else {
        return Outcome::Complete {
            partial_failure: state.partial_failure,
        };
    };
    if streams {
        Outcome::Stopped {
            at: row + 1,
            finished: state.next_row.min(row),
            replayed: state.replayed,
            cause,
        }
    } else {
        Outcome::Failed(cause)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering};
    use std::thread;

    use super::{Input, Outcome, Prepared};

    #[test]
    #[allow(clippy::excessive_nesting, reason = "synchronized reader fixture")]
    fn annotate_cancellation_clears_undispatched_groups_and_joins() {
        let cancel = crate::engine::Cancel::default();
        let run_cancel = cancel.clone();
        let started = Arc::new(std::sync::Barrier::new(2));
        let answer_started = Arc::clone(&started);
        let release = Arc::new(std::sync::Barrier::new(2));
        let answer_release = Arc::clone(&release);
        let starts = Arc::new(AtomicUsize::new(0));
        let answer_starts = Arc::clone(&starts);
        let active = Arc::new(AtomicUsize::new(0));
        let answer_active = Arc::clone(&active);
        let requests = Arc::new(AtomicUsize::new(0));
        let reader_requests = Arc::clone(&requests);
        let (outcome_send, outcome) = std::sync::mpsc::channel();

        let run = thread::spawn(move || {
            let result = super::run(
                1,
                true,
                &run_cancel,
                move |asked, events| {
                    thread::spawn(move || {
                        while asked.recv().is_ok() {
                            reader_requests.fetch_add(1, Ordering::SeqCst);
                            if events.send(Input::Item(())).is_err() {
                                break;
                            }
                        }
                    });
                },
                |()| {
                    Ok::<_, &'static str>(Prepared {
                        seed: (),
                        accumulator: Vec::<usize>::new(),
                        work: vec![0, 1],
                    })
                },
                &move |group| {
                    answer_starts.fetch_add(1, Ordering::SeqCst);
                    answer_active.fetch_add(1, Ordering::SeqCst);
                    answer_started.wait();
                    answer_release.wait();
                    answer_active.fetch_sub(1, Ordering::SeqCst);
                    Ok::<_, &'static str>(group)
                },
                |answers, answer| {
                    answers.push(answer);
                    Ok(())
                },
                |_, _| -> Result<crate::engine::schedule::Completed<()>, &'static str> {
                    unreachable!("a cancelled group leaves the row unfinished")
                },
                |_| Ok(true),
                |_| "defect",
                || "cancelled",
            );
            outcome_send.send(result).expect("returned outcome");
        });

        started.wait();
        cancel.fire();
        release.wait();
        let result = outcome
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("the cancelled run returns")
            .expect("the scheduler returns metadata");
        run.join().expect("scheduler thread");

        assert_eq!(requests.load(Ordering::SeqCst), 1);
        assert_eq!(starts.load(Ordering::SeqCst), 1);
        assert_eq!(active.load(Ordering::SeqCst), 0);
        assert!(matches!(
            result,
            Outcome::Stopped {
                at: 1,
                finished: 0,
                cause: "cancelled",
                ..
            }
        ));
    }

    #[test]
    fn aggregate_cancellation_while_waiting_requests_nothing_more() {
        let (waiting_send, waiting) = std::sync::mpsc::channel();
        let cancel = crate::engine::Cancel::observed(waiting_send);
        let run_cancel = cancel.clone();
        let (asked_send, asked_recv) = std::sync::mpsc::channel();
        let (outcome_send, outcome_recv) = std::sync::mpsc::channel();
        let run = thread::spawn(move || {
            let outcome = super::run(
                1,
                false,
                &run_cancel,
                move |asked, _events| asked_send.send(asked).expect("input requests"),
                |_: ()| -> Result<Prepared<(), (), ()>, &'static str> {
                    unreachable!("withheld input cannot be prepared")
                },
                &|_: ()| -> Result<(), &'static str> { unreachable!("no work can start") },
                |_, _| Ok(()),
                |_, _| -> Result<crate::engine::schedule::Completed<()>, &'static str> {
                    unreachable!("no row can finish")
                },
                |_| Ok(true),
                |_| "defect",
                || "cancelled",
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
        assert!(matches!(outcome, Outcome::Failed("cancelled")));
    }
}
