//! The warm aggregate: first-seen groups, bounded chunks and one deadline.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rusqlite::functions::{Aggregate, Context};
use thinkthen::{LoadedQuestion, QuestionKind};

use super::{deadline, shared};
use crate::question::{question, text};
use crate::{Failure, ffi, guard, worker};

/// Warm sends each group's texts in chunks of this many rows.
const CHUNK: usize = 256;

/// One question's texts in a warm pass.
#[derive(Debug)]
struct Group {
    question: Arc<LoadedQuestion>,
    context: Option<String>,
    seen: HashSet<String>,
    pending: Vec<String>,
}

/// The warm aggregate's state: groups in first-seen order, found by the
/// question argument's text in constant time (R5-18).
#[derive(Debug, Default)]
pub(crate) struct WarmState {
    groups: Vec<Group>,
    index: HashMap<(String, Option<String>), usize>,
    judged: i64,
    deadline: Option<Option<i64>>,
    due: Option<Instant>,
    failed: bool,
}

/// A full chunk of one question's texts, ready to send.
type Flush = (Arc<LoadedQuestion>, Option<String>, Vec<String>);

impl WarmState {
    /// Add one row. An equal pair in the same pass is asked once. A group
    /// that reaches a chunk comes back to be sent.
    fn add(
        &mut self,
        argument: &str,
        context: Option<String>,
        parse: impl FnOnce() -> Result<Arc<LoadedQuestion>, Failure>,
        evidence: String,
    ) -> Result<Option<Flush>, Failure> {
        let key = (argument.to_owned(), context.clone());
        let at = match self.index.get(&key) {
            Some(at) => *at,
            None => {
                self.groups.push(Group {
                    question: parse()?,
                    context,
                    seen: HashSet::new(),
                    pending: Vec::new(),
                });
                self.index.insert(key, self.groups.len() - 1);
                self.groups.len() - 1
            }
        };
        let group = self
            .groups
            .get_mut(at)
            .ok_or_else(|| Failure::defect("a warm group is missing"))?;
        if group.seen.insert(evidence.clone()) {
            group.pending.push(evidence);
        }
        Ok((group.pending.len() >= CHUNK).then(|| {
            (
                Arc::clone(&group.question),
                group.context.clone(),
                std::mem::take(&mut group.pending),
            )
        }))
    }
}

/// Judge one chunk through the shared planner. The send budget counts actual
/// attempts, so a failed chunk returns no aggregate count.
fn flush(
    context: &Context<'_>,
    (held, shared, texts): Flush,
    due: Option<Instant>,
) -> Result<i64, Failure> {
    let count =
        i64::try_from(texts.len()).map_err(|_| Failure::defect("a warm chunk is too long"))?;
    if count == 0 {
        return Ok(0);
    }
    worker::run_until(ffi::handle_of(context), due, move |engine, options| {
        let options = if let Some(shared) = &shared {
            options.context(shared)
        } else {
            options
        };
        // The band stays out of the request, so decide reads what this fills.
        match &*held {
            LoadedQuestion::Question(asked) => engine
                .decide_many_with(asked, texts, options)
                .try_for_each(|row| row.map(drop))?,
            LoadedQuestion::Banded(asked) => engine
                .decide_many_with(asked, texts, options)
                .try_for_each(|row| row.map(drop))?,
        }
        Ok(count)
    })?;
    Ok(count)
}

/// Warm's question: decide only, cut or banded (ticket 0129), any other kind
/// named to `thinkthen_decide`.
fn warm_question(argument: &str) -> Result<Arc<LoadedQuestion>, Failure> {
    let held = question(argument)?;
    match &*held {
        LoadedQuestion::Question(asked) if asked.kind() != QuestionKind::Decide => {
            Err(Failure::usage(
                "thinkthen_warm takes a decide question; ask others with thinkthen_decide",
            ))
        }
        _ => Ok(held),
    }
}

/// `thinkthen_warm(question, text)`: judge every distinct pair once, filling
/// the engine's cache, and answer how many pairs it judged.
#[derive(Debug)]
pub(super) struct Warm;

impl Aggregate<WarmState, i64> for Warm {
    fn init(&self, _: &mut Context<'_>) -> rusqlite::Result<WarmState> {
        Ok(WarmState::default())
    }

    fn step(&self, context: &mut Context<'_>, state: &mut WarmState) -> rusqlite::Result<()> {
        if state.failed {
            return Ok(());
        }
        let result = guard("thinkthen_warm", || {
            let due = deadline(context)?;
            if state.deadline.is_some_and(|first| first != due) {
                return Err(Failure::usage(
                    "thinkthen_warm takes one deadline for the whole group",
                ));
            }
            if state.deadline.is_none() {
                state.due = due
                    .and_then(|millis| u64::try_from(millis).ok())
                    .and_then(|millis| Instant::now().checked_add(Duration::from_millis(millis)));
            }
            state.deadline = Some(due);
            let Some(argument) = text(context.get_raw(0), "the question")? else {
                return Ok(());
            };
            let Some(evidence) = text(context.get_raw(1), "the text")? else {
                return Ok(());
            };
            let shared = shared(context)?;
            if let Some(chunk) =
                state.add(&argument, shared, || warm_question(&argument), evidence)?
            {
                state.judged += flush(context, chunk, state.due)?;
            }
            Ok(())
        });
        if result.is_err() {
            state.failed = true;
        }
        Ok(result?)
    }

    fn finalize(
        &self,
        context: &mut Context<'_>,
        state: Option<WarmState>,
    ) -> rusqlite::Result<i64> {
        Ok(guard("thinkthen_warm", || {
            let Some(mut state) = state else {
                return Ok(0);
            };
            if state.failed {
                return Ok(0);
            }
            for group in std::mem::take(&mut state.groups) {
                state.judged += flush(
                    context,
                    (group.question, group.context, group.pending),
                    state.due,
                )?;
            }
            Ok(state.judged)
        })?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use thinkthen::{LoadedQuestion, Question};

    use super::WarmState;

    /// R5-18: 200,000 rows under 200,000 distinct questions group in under 1 s.
    #[test]
    fn warm_finds_each_group_in_constant_time() {
        let held = Question::decide("Is this a complaint?")
            .map(|builder| Arc::new(LoadedQuestion::Question(builder.cut())));
        let held = held.map_err(|error| error.to_string()).unwrap();
        let mut state = WarmState::default();
        let started = Instant::now();
        for at in 0..200_000 {
            let flushed = state.add(
                &format!("q{at}"),
                None,
                || Ok(Arc::clone(&held)),
                "text".to_owned(),
            );
            assert!(matches!(flushed, Ok(None)));
        }
        assert_eq!(state.groups.len(), 200_000);
        assert!(
            started.elapsed() < Duration::from_secs(1),
            "{:?}",
            started.elapsed()
        );
    }
}
