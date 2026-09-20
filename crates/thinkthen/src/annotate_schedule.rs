//! One ordered request queue for every record and question group.

use std::collections::{BTreeMap, VecDeque};
use std::process::ExitCode;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use thinkthen_core::{ModelName, Reading, Record};

use crate::annotate::{GroupAnswer, Judging, check_model};
use crate::failure::Failure;
use crate::schedule::{Judged, Output};

struct Work {
    row: usize,
    group: usize,
    record: Record,
    places: Vec<usize>,
}

enum Event {
    Input(Option<Result<Vec<u8>, Failure>>),
    Answered {
        row: usize,
        group: usize,
        answer: Result<GroupAnswer, Failure>,
    },
}

struct Row {
    record: Record,
    answers: Vec<Option<Result<GroupAnswer, Failure>>>,
    ordered: Vec<GroupAnswer>,
    next: usize,
    model: Option<ModelName>,
}

impl Row {
    fn new(record: Record, groups: usize) -> Self {
        Self {
            record,
            answers: std::iter::repeat_with(|| None).take(groups).collect(),
            ordered: Vec::with_capacity(groups),
            next: 0,
            model: None,
        }
    }

    fn insert(&mut self, group: usize, answer: Result<GroupAnswer, Failure>) {
        if let Some(slot) = self.answers.get_mut(group) {
            *slot = Some(answer);
        }
    }

    fn advance(&mut self, requested: &ModelName) -> Result<bool, (usize, Failure)> {
        while let Some(slot) = self.answers.get_mut(self.next) {
            let Some(answer) = slot.take() else { break };
            let answer = answer.map_err(|error| (self.next, error))?;
            check_model(&mut self.model, answer.reply.model(), requested)
                .map_err(|error| (self.next, error))?;
            self.ordered.push(answer);
            self.next += 1;
        }
        Ok(self.next == self.answers.len())
    }
}

struct Run {
    rows: BTreeMap<usize, Row>,
    ready: BTreeMap<usize, Judged>,
    pending: VecDeque<Work>,
    failures: BTreeMap<(usize, usize), Failure>,
    next_row: usize,
    read_rows: usize,
    in_flight: usize,
    replayed: usize,
    reading: bool,
    exhausted: bool,
    halted: bool,
    printing: bool,
    quiet_stop: bool,
}

impl Run {
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
        }
    }

    fn stop(&mut self) {
        self.halted = true;
        self.pending.clear();
    }

    fn failed(&mut self, row: usize, group: usize, error: Failure) {
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

pub(crate) fn run<I>(
    judging: &Judging<'_>,
    reading: &Reading,
    chunks: I,
    jobs: usize,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    I: Iterator<Item = Result<Vec<u8>, Failure>> + Send + 'static,
{
    let (work_send, work_recv) = mpsc::channel::<Work>();
    let work_recv = Mutex::new(work_recv);
    let (event_send, event_recv) = mpsc::channel::<Event>();
    let (read_send, read_recv) = mpsc::channel();
    let input_events = event_send.clone();
    thread::spawn(move || read(chunks, &read_recv, &input_events));
    thread::scope(|scope| {
        for _ in 0..jobs {
            let work_recv = &work_recv;
            let event_send = event_send.clone();
            scope.spawn(move || worker(work_recv, &event_send, judging, reading));
        }
        drop(event_send);
        let mut state = Run::new();
        loop {
            drain(&mut state, output)?;
            dispatch(&mut state, &work_send, jobs)?;
            request_input(&mut state, &read_send, jobs, reading.streams());
            if state.done() {
                break;
            }
            let event = event_recv
                .recv()
                .map_err(|_| Failure::Defect("the annotate scheduler ended early"))?;
            accept(&mut state, event, judging, reading)?;
        }
        drop(read_send);
        drop(work_send);
        finish(state, reading.streams())
    })
}

fn read<I>(mut chunks: I, requests: &Receiver<()>, events: &Sender<Event>)
where
    I: Iterator<Item = Result<Vec<u8>, Failure>>,
{
    while requests.recv().is_ok() {
        if events.send(Event::Input(chunks.next())).is_err() {
            return;
        }
    }
}

fn worker(
    work: &Mutex<Receiver<Work>>,
    events: &Sender<Event>,
    judging: &Judging<'_>,
    reading: &Reading,
) {
    while let Ok(Ok(task)) = work.lock().map(|receiver| receiver.recv()) {
        let answer = judging.answer_group(reading, &task.record, task.places);
        if events
            .send(Event::Answered {
                row: task.row,
                group: task.group,
                answer,
            })
            .is_err()
        {
            return;
        }
    }
}

fn request_input(state: &mut Run, requests: &Sender<()>, jobs: usize, streams: bool) {
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
                Failure::Defect("the annotate input reader ended early"),
            );
        }
    }
}

fn dispatch(state: &mut Run, send: &Sender<Work>, jobs: usize) -> Result<(), Failure> {
    while !state.halted && state.in_flight < jobs {
        let Some(work) = state.pending.pop_front() else {
            break;
        };
        send.send(work)
            .map_err(|_| Failure::Defect("every annotate worker ended early"))?;
        state.in_flight += 1;
    }
    Ok(())
}

fn accept(
    state: &mut Run,
    event: Event,
    judging: &Judging<'_>,
    reading: &Reading,
) -> Result<(), Failure> {
    match event {
        Event::Input(input) => {
            state.reading = false;
            if state.halted {
                return Ok(());
            }
            let Some(bytes) = input else {
                state.exhausted = true;
                return Ok(());
            };
            let row = state.read_rows;
            state.read_rows += 1;
            let bytes = match bytes {
                Ok(bytes) => bytes,
                Err(error) => {
                    state.failed(row, 0, error);
                    return Ok(());
                }
            };
            let record = match judging.record(reading, &bytes) {
                Ok(record) => record,
                Err(error) => {
                    state.failed(row, 0, error);
                    return Ok(());
                }
            };
            let groups = judging.groups();
            state
                .rows
                .insert(row, Row::new(record.clone(), groups.len()));
            for (group, places) in groups.into_iter().enumerate() {
                state.pending.push_back(Work {
                    row,
                    group,
                    record: record.clone(),
                    places,
                });
            }
            if !reading.streams() {
                state.exhausted = true;
            }
        }
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
                return Err(Failure::Defect("an annotate answer names no row"));
            };
            progress.insert(group, Ok(answer));
            match progress.advance(judging.requested_model()) {
                Err((failed_group, error)) => state.failed(row, failed_group, error),
                Ok(false) => {}
                Ok(true) => {
                    let progress = state
                        .rows
                        .remove(&row)
                        .ok_or(Failure::Defect("a complete annotate row disappeared"))?;
                    match judging.finish(progress.record, progress.ordered) {
                        Ok(judged) => {
                            state.ready.insert(row, judged);
                        }
                        Err(error) => state.failed(row, group, error),
                    }
                }
            }
        }
    }
    Ok(())
}

fn drain(state: &mut Run, output: &mut Output<'_>) -> Result<(), Failure> {
    while state.printing {
        let Some(judged) = state.ready.remove(&state.next_row) else {
            break;
        };
        state.replayed += usize::from(judged.replayed);
        if !output.take(judged)? {
            state.stop();
            state.printing = false;
            state.quiet_stop = true;
        }
        state.next_row += 1;
    }
    Ok(())
}

fn finish(mut state: Run, streams: bool) -> Result<ExitCode, Failure> {
    let Some(((row, _), cause)) = state.failures.pop_first() else {
        return Ok(ExitCode::SUCCESS);
    };
    if !streams {
        return Err(cause);
    }
    Err(Failure::Stopped {
        at: row + 1,
        finished: state.next_row.min(row),
        replayed: state.replayed,
        held: false,
        cause: Box::new(cause),
    })
}
