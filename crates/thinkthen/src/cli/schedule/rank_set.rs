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
        if rows.len() == 1 {
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
        }
        for (member, row) in rows.into_iter().enumerate() {
            self.check_model(&row)?;
            let value = row
                .order_value
                .ok_or(Failure::Defect("a ranked row carries no probability"))?;
            let held = self
                .members
                .get_mut(member)
                .ok_or(Failure::Defect("a rank lost its member"))?;
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
        let Mode::Ordered { top, writer, .. } = &mut self.mode else {
            return Err(Failure::Defect("a set rank must hold output"));
        };
        let lists = self
            .members
            .iter()
            .map(|member| member.iter().map(|(index, _)| *index).collect())
            .collect::<Vec<_>>();
        for (index, member) in turns(&lists, *top) {
            let row = self
                .members
                .get(member)
                .and_then(|rows| rows.iter().find(|(at, _)| *at == index))
                .map(|(_, row)| row)
                .ok_or(Failure::Defect("a merged rank lost its record"))?;
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
