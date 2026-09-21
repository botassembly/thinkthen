//! Asking every record at once, and printing the answers in input order.
//!
//! The workers share one queue of records and one channel back. Nothing here
//! knows what a judgment is: it takes a function over one record's bytes and
//! the writer that owns standard output.

use std::collections::BTreeMap;
use std::fmt;
use std::io::Write;
use std::process::ExitCode;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::thread;

use thinkthen_core::{Outcome, ranking};

use crate::edge;
use crate::failure::Failure;

/// How many requests are in flight when `--jobs` names no number.
///
/// ADR 0010 fixes it. The vendor's own example code uses 4 to 12 workers and
/// says the public endpoint limits concurrency above about eight, so 4 is safe
/// everywhere and a measured run can raise it.
const DEFAULT_JOBS: usize = 4;

/// One record's place and value, as the queue carries them.
type Work<T> = (usize, T);

/// One record's place and the answer it earned.
type Answered = (usize, Result<Judged, Failure>);

/// What one worker calls on one input value.
type Asking<'a, T> = dyn Fn(&T) -> Result<Judged, Failure> + Sync + 'a;

/// One input or worker event the scheduler can act on without blocking either.
enum Event<T> {
    Input(Option<Result<T, Failure>>),
    Answered(Answered),
}

/// One record's answer, as the line it prints and what the run counts.
pub(crate) struct Judged {
    /// The line standard output takes, or nothing when the view prints none.
    pub(crate) printed: Option<String>,
    /// What the answer earns a run over one document.
    pub(crate) outcome: Outcome,
    /// True when a recording answered rather than a backend.
    pub(crate) replayed: bool,
    /// The probability of yes, which `rank` sorts on and no other verb reads.
    pub(crate) probability: Option<f64>,
    /// True when this completed row contains one or more failed questions.
    pub(crate) partial_failure: bool,
}

impl fmt::Debug for Judged {
    /// Show what the row carries and never the row, which holds the evidence.
    ///
    /// Under `--details` the printed line holds the whole record. Nothing shows
    /// a `Judged` today, and a later line that does must not print a record.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Judged")
            .field(
                "printed",
                &format_args!(
                    "<{} bytes withheld>",
                    self.printed.as_ref().map_or(0, String::len)
                ),
            )
            .field("outcome", &self.outcome)
            .field("replayed", &self.replayed)
            .field("probability", &self.probability)
            .field("partial_failure", &self.partial_failure)
            .finish()
    }
}

/// Where the finished rows go, in input order.
///
/// Every verb but one prints a row as its place comes. `rank` holds every row
/// until the input ends, because a final order needs the whole set, and then
/// prints the order [`ranking`] gives.
pub(crate) enum Output<'a> {
    /// Each row prints as its place comes.
    Streaming(&'a mut dyn Write),
    /// Every row is held, and the order prints once the run has finished.
    Ordered {
        /// The rows so far, in input order.
        held: Vec<Judged>,
        /// How many places of the order print, or every one.
        top: Option<usize>,
        /// Where the order goes once the input has ended.
        writer: &'a mut dyn Write,
    },
}

impl fmt::Debug for Output<'_> {
    /// Show how many rows are held and never a row, which holds the evidence.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Streaming(_) => formatter.write_str("Streaming"),
            Self::Ordered { held, top, .. } => formatter
                .debug_struct("Ordered")
                .field("held", &format_args!("<{} rows withheld>", held.len()))
                .field("top", top)
                .finish_non_exhaustive(),
        }
    }
}

impl Output<'_> {
    /// Take one finished row, whose place in the input order has come.
    ///
    /// Returns false when the reader downstream has closed the pipe.
    pub(crate) fn take(&mut self, judged: Judged) -> Result<bool, Failure> {
        match self {
            Self::Streaming(writer) => match judged.printed.as_deref() {
                Some(line) => edge::write_line(&mut **writer, line),
                None => Ok(true),
            },
            Self::Ordered { held, .. } => {
                held.push(judged);
                Ok(true)
            }
        }
    }

    /// Write whatever was held back, once the whole run has finished.
    ///
    /// # Errors
    ///
    /// Returns [`Failure::Defect`] when a held row carries no probability to
    /// sort on, which no `rank` row can reach this point without.
    pub(crate) fn ended(&mut self) -> Result<(), Failure> {
        let Self::Ordered { held, top, writer } = self else {
            return Ok(());
        };
        let odds = held
            .iter()
            .map(|judged| {
                judged
                    .probability
                    .ok_or(Failure::Defect("a ranked row carries no probability"))
            })
            .collect::<Result<Vec<f64>, Failure>>()?;
        for place in ranking(&odds, *top) {
            let Some(line) = held.get(place).and_then(|judged| judged.printed.as_deref()) else {
                continue;
            };
            if !edge::write_line(&mut **writer, line)? {
                return Ok(());
            }
        }
        Ok(())
    }

    /// The writer standard output goes through, which a plan writes one line to.
    pub(crate) fn writer(&mut self) -> &mut dyn Write {
        match self {
            Self::Streaming(writer) | Self::Ordered { writer, .. } => *writer,
        }
    }

    /// True when nothing has printed yet, which the line about a stop reports.
    const fn holds(&self) -> bool {
        matches!(*self, Self::Ordered { .. })
    }
}

/// How many requests this run keeps in flight, or why the number cannot act.
///
/// # Errors
///
/// Returns [`Failure::JobsOutsideRecords`] when `--jobs` was given to a run
/// over one document, which sends one request and has nothing to bound.
pub(crate) fn jobs_of(asked: Option<u8>, streams: bool) -> Result<usize, Failure> {
    match asked {
        Some(_) if !streams => Err(Failure::JobsOutsideRecords),
        Some(number) => Ok(usize::from(number)),
        None => Ok(DEFAULT_JOBS),
    }
}

/// Ask every record with at most `jobs` requests in flight, printing in order.
///
/// The workers share one queue of records and one channel back. The records go
/// out in input order and no more than `jobs` are outstanding, so the buffer of
/// finished rows waiting for the rows before them holds at most `jobs` and the
/// memory of a long run stays flat.
pub(crate) fn over_records<T, I>(
    row: &Asking<'_, T>,
    chunks: I,
    jobs: usize,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    T: Send + 'static,
    I: Iterator<Item = Result<T, Failure>> + Send + 'static,
{
    let (hand, queue) = mpsc::sync_channel::<Work<T>>(jobs);
    let queue = Mutex::new(queue);
    let (give, gathered) = mpsc::channel::<Event<T>>();
    let (ask, asked) = mpsc::channel();
    let input_give = give.clone();
    thread::spawn(move || read_records(chunks, &asked, &input_give));
    thread::scope(|scope| {
        for _ in 0..jobs {
            let queue = &queue;
            let give = give.clone();
            scope.spawn(move || work(queue, &give, row));
        }
        drop(give);
        let mut run = Run::new();
        loop {
            run.drain(output)?;
            if run.done() {
                break;
            }
            run.request(&ask, jobs, output.holds());
            let Ok(event) = gathered.recv() else {
                return Err(Failure::Defect("the record scheduler ended early"));
            };
            run.accept(event, &hand);
        }
        drop(ask);
        drop(hand);
        run.finished(output)
    })
}

/// Read one record for each place the scheduler opens.
fn read_records<T, I>(mut chunks: I, asked: &Receiver<()>, give: &Sender<Event<T>>)
where
    I: Iterator<Item = Result<T, Failure>>,
{
    while asked.recv().is_ok() {
        if give.send(Event::Input(chunks.next())).is_err() {
            return;
        }
    }
}

/// Take one record at a time off the queue and answer it, until the queue ends.
///
/// The lock is held over the wait, so one worker takes one record and the next
/// worker takes the next. A worker that finds the queue closed is done.
fn work<T>(queue: &Mutex<Receiver<Work<T>>>, give: &Sender<Event<T>>, row: &Asking<'_, T>) {
    while let Ok(Ok((place, value))) = queue.lock().map(|taken| taken.recv()) {
        if give.send(Event::Answered((place, row(&value)))).is_err() {
            return;
        }
    }
}

/// What has gone out, what has come back, and what has been printed.
#[derive(Debug)]
struct Run {
    pending: BTreeMap<usize, Result<Judged, Failure>>,
    next: usize,
    dispatched: usize,
    in_flight: usize,
    replayed: usize,
    reading: bool,
    exhausted: bool,
    halted: bool,
    printing: bool,
    stop: Option<(usize, Failure)>,
}

impl Run {
    /// Start with nothing sent and nothing printed.
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

    /// Ask the input thread for one row when the schedule has a place for it.
    fn request(&mut self, ask: &Sender<()>, jobs: usize, holds_order: bool) {
        if !self.reading && !self.halted && !self.exhausted && self.waiting(holds_order) < jobs {
            self.reading = ask.send(()).is_ok();
            if !self.reading {
                self.refuse(Failure::Defect("the record reader ended early"));
            }
        }
    }

    /// Take one event without letting a blocked input read hide an answer.
    fn accept<T>(&mut self, event: Event<T>, hand: &SyncSender<Work<T>>) {
        match event {
            Event::Input(input) => {
                self.reading = false;
                if self.halted {
                    return;
                }
                match input {
                    None => self.exhausted = true,
                    Some(Err(error)) => self.refuse(error),
                    Some(Ok(value)) => self.send(value, hand),
                }
            }
            Event::Answered((place, judged)) => {
                self.in_flight -= 1;
                self.halted |= judged.is_err();
                self.pending.insert(place, judged);
            }
        }
    }

    /// True when no event that changes this run still needs to arrive.
    const fn done(&self) -> bool {
        self.in_flight == 0 && (self.halted || self.exhausted)
    }

    /// Count rows that occupy one scheduling place.
    ///
    /// Streaming keeps every dispatched row inside the bound until its place
    /// prints. A final order must read the whole input, so only live requests
    /// occupy its worker bound.
    const fn waiting(&self, holds_order: bool) -> usize {
        if holds_order {
            self.in_flight
        } else {
            self.dispatched - self.next
        }
    }

    /// Give this record its place in the order without sending it anywhere.
    ///
    /// A record the reader could not hand over stops the run where it sits
    /// rather than where it was read.
    fn refuse(&mut self, error: Failure) {
        self.pending.insert(self.dispatched, Err(error));
        self.dispatched += 1;
        self.halted = true;
    }

    /// Hand this record to the workers, and refuse it when none is left to take it.
    ///
    /// No worker to take a record means no answer for it, and the run reports
    /// a defect rather than printing a short file that reads as a whole one.
    fn send<T>(&mut self, value: T, hand: &SyncSender<Work<T>>) {
        if hand.send((self.dispatched, value)).is_err() {
            self.refuse(Failure::Defect("every worker ended before the records did"));
            return;
        }
        self.dispatched += 1;
        self.in_flight += 1;
    }

    /// Print every row whose place has come, and stop at the first failure.
    ///
    /// A reader that closed the pipe ends the printing and the scheduling with
    /// it, and the run keeps the exit code it had earned.
    fn drain(&mut self, output: &mut Output<'_>) -> Result<(), Failure> {
        while self.printing {
            let Some(judged) = self.pending.remove(&self.next) else {
                return Ok(());
            };
            match judged {
                Err(cause) => {
                    self.stop = Some((self.next, cause));
                    self.stopped();
                }
                Ok(judged) => self.print(judged, output)?,
            }
        }
        Ok(())
    }

    /// Print one row, and stop when the reader has closed the pipe.
    fn print(&mut self, judged: Judged, output: &mut Output<'_>) -> Result<(), Failure> {
        self.replayed += usize::from(judged.replayed);
        if !output.take(judged)? {
            self.stopped();
        }
        self.next += 1;
        Ok(())
    }

    /// Stop the printing and the scheduling with it.
    const fn stopped(&mut self) {
        self.halted = true;
        self.printing = false;
    }

    /// Give the exit code, or the stop that names the earliest failed record.
    ///
    /// # Errors
    ///
    /// Returns [`Failure::Stopped`] when a record failed, carrying the cause
    /// that sets the exit code.
    fn finished(self, output: &mut Output<'_>) -> Result<ExitCode, Failure> {
        match self.stop {
            Some((place, cause)) => Err(Failure::Stopped {
                at: place + 1,
                finished: place,
                replayed: self.replayed,
                held: output.holds(),
                cause: Box::new(cause),
            }),
            None => {
                output.ended()?;
                Ok(ExitCode::SUCCESS)
            }
        }
    }
}
