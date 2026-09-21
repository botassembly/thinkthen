//! Ordered, bounded scheduling for grouped annotate requests.

use std::collections::{BTreeMap, VecDeque};
use std::sync::mpsc::{Receiver, Sender, SyncSender, channel};

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
    start_reader: impl FnOnce(Receiver<()>, InputPort<T, G, E>),
    prepare: impl Fn(T) -> Result<Prepared<S, A, W>, E>,
    answer: &(impl Fn(W) -> Result<G, E> + Sync),
    accept: impl Fn(&mut A, G) -> Result<(), E>,
    finish: impl Fn(S, A) -> Result<Completed<R>, E>,
    mut emit: impl FnMut(R) -> Result<bool, E>,
    defect: fn(&'static str) -> E,
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
            answer: answer(task.work),
        },
        |work| {
            let mut state = Run::new();
            loop {
                drain(&mut state, &mut emit)?;
                dispatch(&mut state, &work, jobs, defect)?;
                request_input(&mut state, &request, jobs, streams, defect);
                if state.done() {
                    break;
                }
                let event = received
                    .recv()
                    .map_err(|_| defect("the annotate scheduler ended early"))?;
                receive(
                    &mut state, event, streams, &prepare, &accept, &finish, defect,
                )?;
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
