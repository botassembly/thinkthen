//! Member-local retention and turns, emitted through the shared display.
use super::{Judged, Mode, Output};
use crate::core::turns;
use crate::failure::Failure;

impl Output<'_> {
    pub(crate) fn take_members(
        &mut self,
        rows: Vec<Judged>,
        index: usize,
    ) -> Result<bool, Failure> {
        if rows.len() == 1
            && !rows
                .first()
                .and_then(|row| row.rank.as_ref())
                .is_some_and(super::rank::RankRow::is_set)
        {
            let row = rows
                .into_iter()
                .next()
                .ok_or(Failure::Defect("a question lost its row"))?;
            return self.take(row);
        }
        let limit = match &self.mode {
            Mode::Ordered { top, .. } => *top,
            Mode::Streaming(_) => return Err(Failure::Defect("a set rank must hold output")),
        };
        if self.members.is_empty() {
            self.members.resize_with(rows.len(), Vec::new);
            self.member_scores.resize_with(rows.len(), Vec::new);
        }
        for (member, row) in rows.into_iter().enumerate() {
            self.check_model(&row)?;
            let value = row
                .order_value
                .ok_or(Failure::Defect("a ranked row carries no probability"))?;
            if row.rank.as_ref().is_some_and(super::rank::RankRow::is_set) {
                self.member_scores
                    .get_mut(member)
                    .ok_or(Failure::Defect("a rank lost its scores"))?
                    .push((index, value));
            }
            let held = self
                .members
                .get_mut(member)
                .ok_or(Failure::Defect("a rank lost its member"))?;
            if limit.is_none() {
                held.push((index, row));
                continue;
            }
            let place = held.partition_point(|(_, earlier)| {
                earlier
                    .order_value
                    .is_some_and(|score| score.total_cmp(&value).is_ge())
            });
            if limit.is_some_and(|limit| place >= limit) {
                continue;
            }
            if limit == Some(held.len()) {
                held.pop();
            }
            held.insert(place, (index, row));
        }
        self.usage.record_done();
        Ok(true)
    }

    pub(super) fn end_members(&mut self) -> Result<(), Failure> {
        let positions = member_positions(&self.member_scores)?;
        let Mode::Ordered { top, writer, .. } = &mut self.mode else {
            return Err(Failure::Defect("a set rank must hold output"));
        };
        for rows in &mut self.members {
            rows.sort_by(|(_, one), (_, other)| {
                other
                    .order_value
                    .unwrap_or_default()
                    .total_cmp(&one.order_value.unwrap_or_default())
            });
        }
        let lists = self
            .members
            .iter()
            .map(|member| member.iter().map(|(index, _)| *index).collect())
            .collect::<Vec<_>>();
        // Preserve ranked identities in lists, then index the same bounded
        // payloads for lookup without a scan for every emitted original.
        for rows in &mut self.members {
            rows.sort_unstable_by_key(|(index, _)| *index);
        }
        for (at, (index, member)) in turns(&lists, *top).into_iter().enumerate() {
            let row = self
                .members
                .get_mut(member)
                .and_then(|rows| {
                    rows.binary_search_by_key(&index, |(at, _)| *at)
                        .ok()
                        .and_then(|place| rows.get_mut(place))
                })
                .map(|(_, row)| row)
                .ok_or(Failure::Defect("a merged rank lost its record"))?;
            row.finish_rank(at, positions.get(&index).map(Vec::as_slice))?;
            if let Some(mismatch) = &row.profile_mismatch {
                mismatch.print_once()?;
            }
            if !self.display.emit(&mut **writer, row)? {
                break;
            }
        }
        Ok(())
    }
}

// Retain numeric scores for full member positions while payloads stay bounded by top.
fn member_positions(
    scores: &[Vec<(usize, f64)>],
) -> Result<std::collections::BTreeMap<usize, Vec<usize>>, Failure> {
    let mut positions = std::collections::BTreeMap::new();
    let width = scores.len();
    for (member, scores) in scores.iter().enumerate() {
        let values = scores.iter().map(|(_, value)| *value).collect::<Vec<_>>();
        for (at, place) in crate::core::ranking(&values, None).into_iter().enumerate() {
            let (index, _) = scores
                .get(place)
                .ok_or(Failure::Defect("a rank lost its score"))?;
            let row = positions
                .entry(*index)
                .or_insert_with(|| vec![usize::MAX; width]);
            *row.get_mut(member)
                .ok_or(Failure::Defect("a rank lost its member position"))? = at;
        }
    }
    for row in positions.values() {
        if row.contains(&usize::MAX) {
            return Err(Failure::Defect("a rank lost its member position"));
        }
    }
    Ok(positions)
}
