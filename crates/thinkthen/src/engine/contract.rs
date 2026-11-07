//! Canonical calls drain results independently of an input owner.
use super::*;
use crate::Code;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub(crate) enum RecordEvent<T> {
    Record(T),
    Flush,
}
impl<T> RecordEvent<T> {
    pub(crate) fn map<U>(self, f: impl FnOnce(T) -> U) -> RecordEvent<U> {
        match self {
            Self::Record(value) => RecordEvent::Record(f(value)),
            Self::Flush => RecordEvent::Flush,
        }
    }
}
#[derive(Debug)]
enum Event {
    Queued,
    Answered(usize, Result<Vec<Row>, Error>),
    Producer(Result<(), Error>),
}
fn fault() -> Error {
    Error::from_contract(Code::InternalBug, "the call worker failed")
}
impl Engine {
    #[expect(
        clippy::expect_used,
        reason = "the scope owns every worker and channel until completion"
    )]
    pub(crate) fn call_parallel<F: Function>(
        &self,
        question: &F,
        records: impl Iterator<Item = RecordEvent<Record>> + Send,
        mut sink: impl FnMut(Result<Row, Error>),
        identity: String,
    ) -> Result<(), Error> {
        let url = self.backend.url();
        let (jobs, inbox) = sync_channel(WORKERS);
        let inbox = Mutex::new(inbox);
        let (events, output) = sync_channel(WORKERS);
        let stopped = AtomicBool::new(false);
        let mut failure = None;
        thread::scope(|scope| {
            // Owned here, so leaving the scope's body wakes a producer waiting for a permit.
            let (permit, permits) = sync_channel(REORDER_WINDOW);
            for _ in 0..REORDER_WINDOW {
                permit.send(()).expect("fresh permit queue");
            }
            for _ in 0..WORKERS {
                let (events, inbox, url, stopped) = (events.clone(), &inbox, &url, &stopped);
                scope.spawn(move || {
                    // After a whole-call failure no worker sends a request, but each keeps
                    // taking work, so a producer waiting on a full queue always ends. The
                    // collector alone cancels, after the failure has reached the sink.
                    while let Some((place, chunk)) = next_job(inbox) {
                        let rows = if stopped.load(Ordering::Relaxed) {
                            Ok(Vec::new())
                        } else {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                self.answer(
                                    chunk,
                                    url,
                                    if place == 0 {
                                        BatchStage::First
                                    } else {
                                        BatchStage::Later
                                    },
                                )
                            }))
                            .map_err(|_| fault())
                            .and_then(std::convert::identity)
                        };
                        if rows.is_err() {
                            stopped.store(true, Ordering::Relaxed);
                        }
                        if events.send(Event::Answered(place, rows)).is_err() {
                            break;
                        }
                    }
                });
            }
            let producer = events.clone();
            let url = &url;
            scope.spawn(move || {
                let mut report = super::input_report::InputReport::new(self.batch.get().get());
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut packer = Packer::new(
                        question,
                        self.backend.wire_adapter(),
                        url,
                        self.backend.model(),
                        self.batch.get(),
                        self.backend.limits(),
                        identity,
                    );
                    // Reserve space before asking the input owner for another batch.
                    permits.recv().map_err(|_| fault())?;
                    let mut place = 0;
                    let mut send = |chunk,
                                    report: &mut super::input_report::InputReport|
                     -> Result<(), Error> {
                        self.cancel.complete(Ok(()))?;
                        producer.send(Event::Queued).map_err(|_| fault())?;
                        jobs.send((place, chunk)).map_err(|_| fault())?;
                        report.packed();
                        place += 1;
                        permits.recv().map_err(|_| fault())?;
                        Ok(())
                    };
                    for record in records {
                        self.cancel.complete(Ok(()))?;
                        let chunk = match record {
                            RecordEvent::Record(record) => {
                                report.record();
                                packer.push(record)
                            }
                            RecordEvent::Flush => {
                                report.idle(packer.pending_records());
                                packer.flush()
                            }
                        };
                        if let Some(chunk) = chunk {
                            send(chunk, &mut report)?;
                        }
                        if let Some(chunk) = packer.flush_full() {
                            send(chunk, &mut report)?;
                        }
                    }
                    if let Some(chunk) = packer.finish() {
                        send(chunk, &mut report)?;
                    }
                    Ok(())
                }))
                .unwrap_or_else(|_| Err(fault()));
                report.finish(result.is_ok());
                let _ = producer.send(Event::Producer(result));
            });
            drop(events);
            let collected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.collect(output, &permit, &mut sink, &mut failure);
            }));
            if collected.is_err() {
                failure.get_or_insert_with(fault);
                self.cancel.cancel();
                // Free a producer blocked on either queue, so the scope can join it.
                drop(permit);
                while next_job(&inbox).is_some() {}
            }
        });
        failure.map_or_else(|| self.cancel.complete(Ok(())), Err)
    }
    /// Hand rows to `sink` in input order, returning a permit for each request handed on.
    fn collect(
        &self,
        output: Receiver<Event>,
        permit: &SyncSender<()>,
        sink: &mut impl FnMut(Result<Row, Error>),
        failure: &mut Option<Error>,
    ) {
        let mut emitted = false;
        let mut window = Window {
            rows: VecDeque::new(),
            first: 0,
            sink: |row: Result<Row, Error>| {
                let row = match row {
                    Ok(row) => match row.outcome() {
                        Outcome::Failed { error } if !error.code().is_record() => {
                            Err(error.clone())
                        }
                        _ => Ok(row),
                    },
                    Err(error) => Err(error),
                };
                match row {
                    Ok(row) if !emitted => sink(Ok(row)),
                    Ok(_) => {}
                    Err(error) => {
                        if !emitted {
                            emitted = true;
                            sink(Err(error));
                        }
                        self.cancel.cancel();
                    }
                }
            },
        };
        for event in output {
            match event {
                Event::Queued => window.rows.push_back(None),
                Event::Answered(place, rows) => {
                    let rows = match rows {
                        Ok(rows) => rows.into_iter().map(Ok).collect(),
                        Err(error) => {
                            failure.get_or_insert_with(|| error.clone());
                            (window.sink)(Err(error));
                            Vec::new()
                        }
                    };
                    let before = window.first;
                    window.fill(place, rows);
                    for _ in before..window.first {
                        let _ = permit.send(());
                    }
                }
                Event::Producer(Err(error)) => {
                    failure.get_or_insert_with(|| error.clone());
                    (window.sink)(Err(error));
                }
                Event::Producer(Ok(())) => {}
            }
        }
    }
}
