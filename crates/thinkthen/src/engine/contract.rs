//! Canonical calls drain results independently of an input owner.
use super::*;
use crate::Code;

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
    pub(super) fn call_parallel<F: Function>(
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
        let (permit, permits) = sync_channel(REORDER_WINDOW);
        for _ in 0..REORDER_WINDOW {
            permit.send(()).expect("fresh permit queue");
        }
        let mut failure = None;
        thread::scope(|scope| {
            for _ in 0..WORKERS {
                let (events, inbox, url) = (events.clone(), &inbox, &url);
                scope.spawn(move || {
                    while let Some((place, chunk)) = next_job(inbox) {
                        let rows = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            self.answer(chunk, url)
                        }))
                        .map_err(|_| fault());
                        if events.send(Event::Answered(place, rows)).is_err() {
                            break;
                        }
                    }
                });
            }
            let producer = events.clone();
            let url = &url;
            scope.spawn(move || {
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
                    let mut send = |chunk| -> Result<(), Error> {
                        self.cancel.complete(Ok(()))?;
                        producer.send(Event::Queued).map_err(|_| fault())?;
                        jobs.send((place, chunk)).map_err(|_| fault())?;
                        place += 1;
                        permits.recv().map_err(|_| fault())?;
                        Ok(())
                    };
                    for record in records {
                        self.cancel.complete(Ok(()))?;
                        let chunk = match record {
                            RecordEvent::Record(record) => packer.push(record),
                            RecordEvent::Flush => packer.flush(),
                        };
                        if let Some(chunk) = chunk {
                            send(chunk)?;
                        }
                        if let Some(chunk) = packer.flush_full() {
                            send(chunk)?;
                        }
                    }
                    if let Some(chunk) = packer.finish() {
                        send(chunk)?;
                    }
                    Ok(())
                }))
                .unwrap_or_else(|_| Err(fault()));
                let _ = producer.send(Event::Producer(result));
            });
            drop(events);
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
                        Ok(row) => sink(Ok(row)),
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
        });
        failure.map_or_else(|| self.cancel.complete(Ok(())), Err)
    }
}
