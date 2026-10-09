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

#[cfg(test)]
mod tests {
    use super::Retained;
    #[test]
    fn member_top_retains_only_union_payloads_and_preserves_full_positions() {
        for limit in 1..=3 {
            let mut retained = Retained::new(Some(limit), 2);
            let mut released = std::collections::BTreeSet::new();
            for (at, (name, weights)) in [("a", [0.7, 1.0]), ("b", [0.6, 0.98]), ("c", [0.5, 0.99])]
                .into_iter()
                .enumerate()
            {
                let dropped = std::cell::RefCell::new(Vec::new());
                retained
                    .take(
                        at,
                        name,
                        &weights,
                        Some(&|ordinal| dropped.borrow_mut().push(ordinal)),
                    )
                    .unwrap();
                released.extend(dropped.into_inner());
                assert!(retained.lists.iter().all(|list| list.len() <= limit));
                assert!(retained.rows.len() <= 2 * limit);
            }
            let (rows, selected, positions) = retained.finish().unwrap();
            assert_eq!(selected.len(), limit);
            assert_eq!(
                selected
                    .iter()
                    .map(|(ordinal, _)| rows[ordinal])
                    .collect::<Vec<_>>(),
                ["a", "b", "c"][..limit]
            );
            assert_eq!(positions[&1], [1, 2]);
            assert_eq!(positions[&2], [2, 1]);
            assert!(rows.keys().all(|ordinal| !released.contains(ordinal)));
        }
    }
}
