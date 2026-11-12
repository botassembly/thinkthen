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
/// What the producer owns for one call.
struct Input<'a, F: Function> {
    question: Arc<F>,
    url: &'a str,
    identity: String,
    jobs: SyncSender<(usize, Chunk<F>)>,
    permits: Receiver<()>,
}
impl Engine {
    /// Answer `records` on this client's kept threads, handing rows to `sink` on the
    /// caller's thread. The work those threads are given owns what it uses.
    #[expect(clippy::expect_used, reason = "the permit queue is new and unshared")]
    pub(crate) fn call_parallel<F: Function>(
        &self,
        question: Arc<F>,
        records: impl Iterator<Item = RecordEvent<Record>> + Send + 'static,
        mut sink: impl FnMut(Result<Row, Error>),
        identity: String,
    ) -> Result<(), Error> {
        let engine = Arc::new(self.clone());
        let url: Arc<str> = self.backend.url().into();
        let (jobs, inbox) = sync_channel(WORKERS);
        let inbox = Arc::new(Mutex::new(inbox));
        let (events, output) = sync_channel(WORKERS);
        let stopped = Arc::new(AtomicBool::new(false));
        let (permit, permits) = sync_channel(REORDER_WINDOW);
        for _ in 0..REORDER_WINDOW {
            permit.send(()).expect("fresh permit queue");
        }
        // Nothing is sent: each thread returns its `busy`, which is dropped when the
        // thread is free again, and `free` disconnects after the last of them.
        let (busy, free) = sync_channel::<()>(0);
        let started = (0..WORKERS)
            .try_for_each(|_| {
                let (engine, inbox, url) = (engine.clone(), inbox.clone(), url.clone());
                let (stopped, events, busy) = (stopped.clone(), events.clone(), busy.clone());
                self.threads.run(move || {
                    engine.work(&inbox, &url, &stopped, &events);
                    busy
                })
            })
            .and_then(|()| {
                let (engine, url) = (engine.clone(), url.clone());
                let (events, busy) = (events.clone(), busy.clone());
                self.threads.run(move || {
                    let input = Input {
                        question,
                        url: &url,
                        identity,
                        jobs,
                        permits,
                    };
                    engine.produce(input, records, &events);
                    busy
                })
            });
        drop((events, busy));
        let mut failure = None;
        let collected = started.is_ok()
            && std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.collect(output, &permit, &mut sink, &mut failure);
            }))
            .is_ok();
        if !collected {
            failure.get_or_insert_with(fault);
            self.cancel.cancel();
            // Free a producer blocked on either queue, so it can end.
            drop(permit);
            while next_job(&inbox).is_some() {}
        }
        // The next call finds every thread of this one free.
        let _ = free.recv();
        failure.map_or_else(|| self.cancel.complete(Ok(())), Err)
    }
    /// Send requests until no more will come.
    fn work<F: Function>(
        &self,
        inbox: &Mutex<Receiver<(usize, Chunk<F>)>>,
        url: &str,
        stopped: &AtomicBool,
        events: &SyncSender<Event>,
    ) {
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
    }
    /// Pack the input owner's records into requests for the workers.
    fn produce<F: Function>(
        &self,
        input: Input<'_, F>,
        records: impl Iterator<Item = RecordEvent<Record>>,
        events: &SyncSender<Event>,
    ) {
        let Input {
            question,
            url,
            identity,
            jobs,
            permits,
        } = input;
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
            let mut send =
                |chunk, report: &mut super::input_report::InputReport| -> Result<(), Error> {
                    self.cancel.complete(Ok(()))?;
                    events.send(Event::Queued).map_err(|_| fault())?;
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
        let _ = events.send(Event::Producer(result));
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
