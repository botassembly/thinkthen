//! Member-local winners retain payloads; full scalar scores retain member positions.
use crate::public::Error;
use std::collections::BTreeMap;

pub(super) struct Retained<T> {
    top: Option<usize>,
    scores: Vec<Vec<f64>>,
    lists: Vec<Vec<usize>>,
    rows: BTreeMap<usize, T>,
}
impl<T> Retained<T> {
    pub(super) fn new(top: Option<usize>, width: usize) -> Self {
        Self {
            top,
            scores: vec![Vec::new(); width],
            lists: vec![Vec::new(); width],
            rows: BTreeMap::new(),
        }
    }
    pub(super) fn take(
        &mut self,
        at: usize,
        row: T,
        weights: &[f64],
        release: Option<&dyn Fn(usize)>,
    ) -> Result<(), Error> {
        if at != self.scores.first().map_or(0, Vec::len) {
            return Err(Error::defect("set rank lost native record order"));
        }
        if weights.len() != self.lists.len() {
            return Err(Error::defect("set rank lost a member"));
        }
        let mut evicted = Vec::new();
        for ((scores, list), value) in self.scores.iter_mut().zip(&mut self.lists).zip(weights) {
            let value = *value;
            let place = list.partition_point(|earlier| {
                scores.get(*earlier).is_some_and(|score| *score >= value)
            });
            scores.push(value);
            if self.top.is_some_and(|top| place >= top) {
                continue;
            }
            if self.top == Some(list.len())
                && let Some(old) = list.pop()
            {
                evicted.push(old);
            }
            list.insert(place, at);
        }
        if self.lists.iter().any(|list| list.contains(&at)) {
            self.rows.insert(at, row);
        } else if let Some(release) = release {
            release(at);
        }
        for old in evicted {
            if !self.lists.iter().any(|list| list.contains(&old))
                && self.rows.remove(&old).is_some()
                && let Some(release) = release
            {
                release(old);
            }
        }
        Ok(())
    }
    #[expect(
        clippy::type_complexity,
        reason = "selection retains full occurrence indices and member positions"
    )]
    pub(super) fn finish(
        self,
    ) -> Result<
        (
            BTreeMap<usize, T>,
            Vec<(usize, usize)>,
            BTreeMap<usize, Vec<usize>>,
        ),
        Error,
    > {
        let mut positions = BTreeMap::new();
        for (member, scores) in self.scores.iter().enumerate() {
            for (position, ordinal) in crate::core::ranking(scores, None).into_iter().enumerate() {
                let row = positions
                    .entry(ordinal)
                    .or_insert_with(|| vec![0; self.scores.len()]);
                *row.get_mut(member).ok_or_else(super::super::wrong)? = position;
            }
        }
        Ok((
            self.rows,
            crate::core::turns(&self.lists, self.top),
            positions,
        ))
    }
}
