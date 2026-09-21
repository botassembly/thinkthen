//! Ordered, bounded scheduling below every ordinary record command.

use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, Sender, SyncSender, channel};

use crate::engine::workers;

/// One framed input event from the command-owned reader.
pub(crate) enum Input<T, E> {
    Item(T),
    Failed(E),
    End,
}

/// One completed row and the counters the scheduler owns.
pub(crate) struct Completed<R> {
    pub(crate) value: R,
    pub(crate) replayed: bool,
    pub(crate) partial_failure: bool,
}

/// The ordered stop metadata returned after all request workers have joined.
pub(crate) enum Outcome<E> {
    Complete,
    Stopped {
        at: usize,
        finished: usize,
        replayed: usize,
        held: bool,
        cause: E,
    },
}

enum Event<T, R, E> {
    Input(Input<T, E>),
    Answered(usize, Result<Completed<R>, E>),
}

/// The typed command-to-engine side of the framed-input bridge.
pub(crate) struct InputPort<T, R, E>(Sender<Event<T, R, E>>);

impl<T, R, E> InputPort<T, R, E> {
    pub(crate) fn send(&self, input: Input<T, E>) -> Result<(), ()> {
        self.0.send(Event::Input(input)).map_err(|_| ())
    }
}

struct Run<R, E> {
    pending: BTreeMap<usize, Result<Completed<R>, E>>,
    next: usize,
    dispatched: usize,
    in_flight: usize,
    replayed: usize,
    reading: bool,
    exhausted: bool,
    halted: bool,
    printing: bool,
    stop: Option<(usize, E)>,
}

impl<R, E> Run<R, E> {
    const fn new() -> Self {
        Self {
            pending: BTreeMap::new(),
            next: 0,
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
        held: bool,
        defect: fn(&'static str) -> E,
    ) {
        if !self.reading && !self.halted && !self.exhausted && self.waiting(held) < jobs {
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
                self.halted |= result.is_err();
                self.pending.insert(place, result);
            }
        }
    }

    const fn done(&self) -> bool {
        self.in_flight == 0 && (self.halted || self.exhausted)
    }

    const fn waiting(&self, held: bool) -> usize {
        if held {
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
                    self.stop = Some((self.next, cause));
                    self.halted = true;
                    self.printing = false;
                }
                Ok(completed) => {
                    self.replayed += usize::from(completed.replayed);
                    if !emit(completed.value)? {
                        self.halted = true;
                        self.printing = false;
                    }
                    self.next += 1;
                }
            }
        }
        Ok(())
    }

    fn finish(self, held: bool) -> Outcome<E> {
        match self.stop {
            Some((place, cause)) => Outcome::Stopped {
                at: place + 1,
                finished: place,
                replayed: self.replayed,
                held,
                cause,
            },
            None => Outcome::Complete,
        }
    }
}

/// Schedule framed inputs over scoped workers and emit results in input order.
pub(crate) fn run<T, R, E>(
    jobs: usize,
    held: bool,
    start_reader: impl FnOnce(Receiver<()>, InputPort<T, R, E>),
    answer: &(impl Fn(&T) -> Result<Completed<R>, E> + Sync),
    emit: impl FnMut(R) -> Result<bool, E>,
    defect: fn(&'static str) -> E,
) -> Result<Outcome<E>, E>
where
    T: Send + 'static,
    R: Send,
    E: Send,
{
    run_observed(jobs, held, start_reader, answer, emit, defect, &|| ())
}

#[allow(
    clippy::too_many_arguments,
    reason = "the test observer proves scheduler worker lifetime"
)]
fn run_observed<T, R, E, G>(
    jobs: usize,
    held: bool,
    start_reader: impl FnOnce(Receiver<()>, InputPort<T, R, E>),
    answer: &(impl Fn(&T) -> Result<Completed<R>, E> + Sync),
    mut emit: impl FnMut(R) -> Result<bool, E>,
    defect: fn(&'static str) -> E,
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
        &|(place, value)| Event::Answered(place, answer(&value)),
        begin,
        |work| {
            let mut state = Run::new();
            loop {
                state.drain(&mut emit)?;
                if state.done() {
                    break;
                }
                state.request(&ask, jobs, held, defect);
                let event = received
                    .recv()
                    .map_err(|_| defect("the record scheduler ended early"))?;
                state.accept(event, &work, defect);
            }
            drop(ask);
            Ok(state.finish(held))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{Completed, Input, Outcome, run_observed};
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
            true,
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
                Ok::<_, ()>(Completed {
                    value: *item,
                    replayed: false,
                    partial_failure: false,
                })
            },
            |_| Ok(true),
            |_| (),
            &worker_lifetime,
        )
        .expect("the scheduler runs");
        assert!(matches!(outcome, Outcome::Complete));
        assert_eq!(active.load(Ordering::SeqCst), 0, "all worker threads ended");
        assert_eq!(2 + 2, 4, "the test process remains alive after return");
    }
}
