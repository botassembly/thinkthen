//! Admission batches use the existing pure packer before cache lookup or dispatch.
use super::super::{Asker, Failed};
use crate::core::pack::{Ask, Entry, Packer};
use std::collections::VecDeque;
use std::sync::Arc;

pub(super) struct Admitted<A: Asker> {
    pub(super) input: A::Input,
    pub(super) asks: Vec<Ask>,
}
type Admission<A> = Result<Vec<Admitted<A>>, Failed<<A as Asker>::Error>>;
pub(super) struct Staging<A: Asker> {
    packer: Packer<usize>,
    inputs: VecDeque<Admitted<A>>,
    first: usize,
    most: usize,
}
impl<A: Asker> Staging<A> {
    pub(super) fn new(packer: &Packer<(Ask, usize)>) -> Self {
        Self {
            packer: packer.fresh(),
            inputs: VecDeque::new(),
            first: 0,
            most: packer.inputs_limit(),
        }
    }
    pub(super) fn len(&self) -> usize {
        self.inputs.len()
    }
    pub(super) fn is_open(&self) -> bool {
        !self.inputs.is_empty()
    }
    pub(super) fn add(&mut self, input: A::Input, asks: Vec<Ask>, label: usize) -> Admission<A> {
        let place = self.first + self.inputs.len();
        let entries = asks
            .iter()
            .map(|ask| Entry {
                state: ask.state.clone(),
                question: Arc::clone(&ask.question),
                options: super::options(ask),
                item: place,
            })
            .collect();
        let mut closed = Vec::new();
        self.packer
            .add(entries, &mut closed)
            .map_err(|error| Failed::Pack { error, at: label })?;
        self.inputs.push_back(Admitted { input, asks });
        if self.inputs.len() >= self.most {
            return Ok(self.flush());
        }
        if closed.is_empty() {
            return Ok(Vec::new());
        }
        // An input split across requests is admitted as a whole. A request
        // still holding any of that input keeps it in the current stage.
        let through = self.packer.first_open_item().copied().unwrap_or(place + 1);
        Ok(self.take(through.saturating_sub(self.first)))
    }
    pub(super) fn flush(&mut self) -> Vec<Admitted<A>> {
        self.packer.close();
        self.take(self.inputs.len())
    }
    pub(super) fn discard(&mut self) {
        self.packer.close();
        self.first += self.inputs.len();
        self.inputs.clear();
    }
    fn take(&mut self, count: usize) -> Vec<Admitted<A>> {
        let count = count.min(self.inputs.len());
        self.first += count;
        self.inputs.drain(..count).collect()
    }
}
