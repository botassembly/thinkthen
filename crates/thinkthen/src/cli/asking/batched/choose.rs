//! Resolve each row's question before adding it to the shared batch planner.

use super::{Asks, BatchError, Batcher, Former, Held, Input, Placed};

impl Former {
    /// Queue any earlier closed batch before a later invalid row stops the run.
    pub(super) fn push(&mut self, held: Result<Held, Placed>) {
        let parsed = held.and_then(|held| {
            let question = self
                .asks
                .of(&held.record)
                .map_err(|error| Placed::at(error, held.at))?;
            let record = self
                .reading
                .batch_record(&held.record)
                .map_err(|error| Placed::at(error.into(), held.at))?;
            Ok((record, question, held))
        });
        let (record, question, held) = match parsed {
            Ok(parsed) => parsed,
            Err(error) => {
                self.end();
                self.queue.push_back(Input::Failed(error));
                return;
            }
        };
        if self.batcher.is_none() {
            let created = Batcher::new(
                self.backend.clone(),
                self.profile.clone(),
                question.clone(),
                self.setting,
                self.context.clone(),
            );
            match created {
                Ok(batcher) => self.batcher = Some(batcher),
                Err(error) => {
                    self.queue.push_back(Input::Failed(Placed::at(
                        self.limits.refused(error, true),
                        held.at,
                    )));
                    return;
                }
            }
        }
        self.held.push(held);
        let mut closed = Vec::new();
        let pushed = match (&mut self.batcher, &self.asks) {
            (Some(batcher), Asks::Fixed(_)) => batcher.push(record, &mut closed),
            (Some(batcher), Asks::FromRecord { .. }) => {
                batcher.push_with_question(record, question, &mut closed)
            }
            (None, _) => Err(BatchError::Defect("a record has no batch planner")),
        };
        for batch in closed {
            self.queue_batch(batch);
        }
        if let Err(error) = pushed {
            self.held.clear();
            self.queue.push_back(Input::Failed(Placed::from(
                self.limits.refused(error, false),
            )));
        }
    }
}
