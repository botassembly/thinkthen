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

use thinkthen_core::Outcome;

use crate::edge;
use crate::failure::Failure;

/// How many requests are in flight when `--jobs` names no number.
///
/// ADR 0010 fixes it. The vendor's own example code uses 4 to 12 workers and
/// says the public endpoint limits concurrency above about eight, so 4 is safe
/// everywhere and a measured run can raise it.
const DEFAULT_JOBS: usize = 4;

/// One record's place and the bytes it holds, as the queue carries them.
type Work = (usize, Vec<u8>);

/// One record's place and the answer it earned.
type Answered = (usize, Result<Judged, Failure>);

/// What one worker calls on one record's bytes.
type Asking<'a> = dyn Fn(&[u8]) -> Result<Judged, Failure> + Sync + 'a;

/// One record's answer, as the line it prints and what the run counts.
pub(crate) struct Judged {
    /// The line standard output takes, or nothing when the view prints none.
    pub(crate) printed: Option<String>,
    /// What the answer earns a run over one document.
    pub(crate) outcome: Outcome,
    /// True when a recording answered rather than a backend.
    pub(crate) replayed: bool,
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
            .finish()
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
pub(crate) fn over_records(
    row: &Asking<'_>,
    chunks: &mut dyn Iterator<Item = Result<Vec<u8>, Failure>>,
    jobs: usize,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let (hand, queue) = mpsc::sync_channel::<Work>(jobs);
    let queue = Mutex::new(queue);
    let (give, gathered) = mpsc::channel::<Answered>();
    thread::scope(|scope| {
        for _ in 0..jobs {
            let queue = &queue;
            let give = give.clone();
            scope.spawn(move || work(queue, &give, row));
        }
        drop(give);
        let mut run = Run::new();
        loop {
            run.dispatch(chunks, &hand, jobs);
            run.drain(writer)?;
            if run.in_flight == 0 {
                break;
            }
            let Ok((place, judged)) = gathered.recv() else {
                // Every worker went away with records still out. Nothing will
                // answer them, and a short file must not read as a whole one.
                return Err(Failure::Defect("a worker ended with records in flight"));
            };
            run.in_flight -= 1;
            run.halted |= judged.is_err();
            run.pending.insert(place, judged);
        }
        drop(hand);
        run.finished()
    })
}

/// Take one record at a time off the queue and answer it, until the queue ends.
///
/// The lock is held over the wait, so one worker takes one record and the next
/// worker takes the next. A worker that finds the queue closed is done.
fn work(queue: &Mutex<Receiver<Work>>, give: &Sender<Answered>, row: &Asking<'_>) {
    while let Ok(Ok((place, bytes))) = queue.lock().map(|taken| taken.recv()) {
        if give.send((place, row(&bytes))).is_err() {
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
            exhausted: false,
            halted: false,
            printing: true,
            stop: None,
        }
    }

    /// Send records until the workers hold `jobs` of them or the input ends.
    ///
    /// A record the reader could not hand over takes its own place in the
    /// order, so it stops the run where it sits rather than where it was read.
    fn dispatch(
        &mut self,
        chunks: &mut dyn Iterator<Item = Result<Vec<u8>, Failure>>,
        hand: &SyncSender<Work>,
        jobs: usize,
    ) {
        while !self.halted && !self.exhausted && self.in_flight < jobs {
            match chunks.next() {
                None => self.exhausted = true,
                Some(Err(error)) => self.refuse(error),
                Some(Ok(bytes)) => self.send(bytes, hand),
            }
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
    fn send(&mut self, bytes: Vec<u8>, hand: &SyncSender<Work>) {
        if hand.send((self.dispatched, bytes)).is_err() {
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
    fn drain(&mut self, writer: &mut dyn Write) -> Result<(), Failure> {
        while self.printing {
            let Some(judged) = self.pending.remove(&self.next) else {
                return Ok(());
            };
            match judged {
                Err(cause) => {
                    self.stop = Some((self.next, cause));
                    self.stopped();
                }
                Ok(judged) => self.print(judged, writer)?,
            }
        }
        Ok(())
    }

    /// Print one row, and stop when the reader has closed the pipe.
    fn print(&mut self, judged: Judged, writer: &mut dyn Write) -> Result<(), Failure> {
        self.replayed += usize::from(judged.replayed);
        let open = match judged.printed {
            Some(line) => edge::write_line(writer, &line)?,
            None => true,
        };
        if !open {
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
    fn finished(self) -> Result<ExitCode, Failure> {
        match self.stop {
            Some((place, cause)) => Err(Failure::Stopped {
                at: place + 1,
                finished: place,
                replayed: self.replayed,
                cause: Box::new(cause),
            }),
            None => Ok(ExitCode::SUCCESS),
        }
    }
}
