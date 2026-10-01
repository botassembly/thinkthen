//! One retry gate and one pacer per posting address. `engine::limits` holds
//! the one set this process's engines share.

use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use crate::core::MAX_PER_MINUTE;
use crate::engine::Cancel;
use crate::engine::error::Error;

#[derive(Debug, Default)]
pub(crate) struct Gates {
    closed: Mutex<HashMap<String, Gate>>,
    changed: Condvar,
    /// The next start each paced address allows.
    slots: Mutex<HashMap<String, Instant>>,
}

/// Read `THINKTHEN_REQUESTS_PER_MINUTE`: a whole number from 1 to 60,000.
pub(crate) fn per_minute(text: Option<&str>) -> Result<Option<NonZeroU32>, &'static str> {
    text.map(|text| {
        text.parse::<u32>()
            .ok()
            .filter(|rate| {
                (1..=MAX_PER_MINUTE).contains(rate) && text.bytes().all(|b| b.is_ascii_digit())
            })
            .and_then(NonZeroU32::new)
            .ok_or("THINKTHEN_REQUESTS_PER_MINUTE takes a whole number from 1 to 60000")
    })
    .transpose()
}

/// The spacing between starts at this rate. No rate, no spacing: no address
/// has a default (ruling 14, ticket 0343).
pub(crate) fn interval(rate: Option<NonZeroU32>) -> Option<Duration> {
    rate.map(|rate| Duration::from_secs(60) / rate.get())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Opening {
    At(Instant),
    Never,
}

impl Opening {
    fn after(wait: Duration) -> Self {
        Instant::now()
            .checked_add(wait)
            .map_or(Self::Never, Self::At)
    }

    fn passed(self, now: Instant) -> bool {
        matches!(self, Self::At(at) if at <= now)
    }

    fn remaining(self, now: Instant) -> Duration {
        match self {
            Self::At(at) => at.saturating_duration_since(now),
            Self::Never => Duration::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Gate {
    until: Opening,
    /// The latest wait a server asked for, and the status that asked it.
    floor: Option<(Opening, u16)>,
}

impl Gate {
    /// The status of a server floor more than `longest` from `now`. A request
    /// that would wait longer fails with it unsent (ticket 0367).
    fn past(&self, now: Instant, longest: Duration) -> Option<u16> {
        let (floor, status) = self.floor?;
        let bound = now.checked_add(longest).map_or(Opening::Never, Opening::At);
        (floor > bound).then_some(status)
    }

    fn floor_passed(&self, now: Instant) -> bool {
        self.floor.is_none_or(|(floor, _)| floor.passed(now))
    }
}

impl Gates {
    fn lock(&self) -> MutexGuard<'_, HashMap<String, Gate>> {
        self.closed.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Take the next start slot for `url` and wait for it, holding no send place.
    /// A slot past the call's deadline fails at once and reserves nothing. A slot
    /// abandoned by a later stop stays spent, which only slows later starts.
    pub(crate) fn pace(
        &self,
        url: &str,
        every: Duration,
        cancel: &Cancel<'_>,
    ) -> Result<(), Error> {
        let budget = cancel.stop_or_remaining()?;
        let now = Instant::now();
        let slot = {
            let mut slots = self.slots.lock().unwrap_or_else(PoisonError::into_inner);
            let slot = slots
                .get(url)
                .copied()
                .filter(|at| *at > now)
                .unwrap_or(now);
            if let (Some(budget), Some(passed)) = (budget, cancel.passed())
                && slot - now > budget
            {
                return Err(passed);
            }
            slots.insert(url.to_owned(), slot + every);
            slot
        };
        if slot > now {
            cancel.observed_block();
        }
        cancel.wait(slot - now).map_or(Ok(()), Err)
    }

    /// Keep the later opening when two requests receive a retried status.
    /// `server_floor` is the status when a server header asked for the wait.
    pub(crate) fn close(&self, url: &str, wait: Duration, server_floor: Option<u16>) {
        let mut closed = self.lock();
        let opening = Opening::after(wait);
        let gate = closed.entry(url.to_owned()).or_insert(Gate {
            until: opening,
            floor: None,
        });
        gate.until = gate.until.max(opening);
        if let Some(status) = server_floor {
            gate.floor = Some(
                gate.floor
                    .map_or((opening, status), |floor| floor.max((opening, status))),
            );
        }
        self.changed.notify_all();
    }

    pub(crate) fn may_send(&self, url: &str, cap: Instant) -> bool {
        let closed = self.lock();
        let now = Instant::now();
        closed
            .get(url)
            .is_none_or(|gate| gate.until.passed(now) || (cap <= now && gate.floor_passed(now)))
    }

    /// Wait without a send permit. Return false once this attempt's wait cap
    /// passes. A server floor more than `longest`, the longest retry wait,
    /// away fails at once with its status.
    pub(crate) fn wait_open(
        &self,
        url: &str,
        (cap, longest): (Instant, Duration),
        cancel: &Cancel<'_>,
    ) -> Result<bool, Error> {
        let mut closed = self.lock();
        let mut observed = false;
        loop {
            let budget = cancel.stop_or_remaining()?;
            let now = Instant::now();
            let Some(gate) = closed.get(url).copied() else {
                return Ok(true);
            };
            if gate.until.passed(now) {
                return Ok(true);
            }
            if let Some(status) = gate.past(now, longest) {
                return Err(Error::Status(status));
            }
            if cap <= now && gate.floor_passed(now) {
                return Ok(false);
            }
            if !observed {
                observed = true;
                cancel.observed_block();
            }
            let poll = gate
                .until
                .remaining(now)
                .min(
                    gate.floor
                        .filter(|(floor, _)| !floor.passed(now))
                        .map_or(Duration::MAX, |(floor, _)| floor.remaining(now)),
                )
                .min(Cancel::poll())
                .min(budget.unwrap_or(Duration::MAX));
            debug_assert!(!poll.is_zero(), "a closed gate needs a positive wait");
            closed = self
                .changed
                .wait_timeout(closed, poll)
                .map_or_else(|poisoned| poisoned.into_inner().0, |(closed, _)| closed);
        }
    }
}

#[cfg(test)]
#[test]
fn an_expired_server_floor_cannot_spin_under_a_later_unheaded_gate() {
    let gates = Gates::default();
    let now = Instant::now();
    gates.lock().insert(
        "local".into(),
        Gate {
            until: Opening::At(now + Duration::from_secs(1)),
            floor: Some((Opening::At(now), 429)),
        },
    );
    assert!(
        !gates
            .wait_open(
                "local",
                (now + Duration::from_millis(20), Duration::from_secs(1)),
                &Cancel::default()
            )
            .unwrap()
    );
}

/// Main held every request to an address until a server floor passed,
/// however far off, and an overflowed floor never passed. A floor within the
/// longest retry wait still holds a request; one past it, or an overflowed
/// one, fails it at once with the floor's status (ticket 0367).
#[cfg(test)]
#[test]
fn a_server_floor_past_the_longest_wait_fails_at_once_with_its_status() {
    let longest = Duration::from_millis(100);
    let cases = [
        (Some(Duration::from_millis(60)), 429, Ok(true)),
        (Some(Duration::from_secs(10)), 429, Err(429)),
        (None, 503, Err(503)),
    ];
    for (wait, status, expected) in cases {
        let gates = Gates::default();
        let now = Instant::now();
        let floor = wait.map_or(Opening::Never, |wait| Opening::At(now + wait));
        gates.lock().insert(
            "local".into(),
            Gate {
                until: floor,
                floor: Some((floor, status)),
            },
        );
        let opened = gates.wait_open("local", (now + longest, longest), &Cancel::default());
        let waited = now.elapsed();
        match expected {
            Ok(open) => {
                assert_eq!(opened.unwrap(), open, "{wait:?}");
                assert!(waited >= Duration::from_millis(60), "{wait:?}: {waited:?}");
            }
            Err(code) => {
                assert!(
                    matches!(opened, Err(Error::Status(got)) if got == code),
                    "{wait:?}"
                );
                assert!(waited < longest, "{wait:?}: waited {waited:?}");
            }
        }
    }
}

#[cfg(test)]
#[test]
fn paced_starts_from_many_threads_keep_one_interval_apart() {
    let (gates, every) = (Gates::default(), Duration::from_millis(40));
    let take = || {
        gates.pace("local", every, &Cancel::default()).unwrap();
        Instant::now()
    };
    let begun = Instant::now();
    let mut starts: Vec<Instant> = std::thread::scope(|scope| {
        let threads: Vec<_> = (0..3)
            .map(|_| scope.spawn(|| (0..3).map(|_| take()).collect::<Vec<_>>()))
            .collect();
        threads
            .into_iter()
            .flat_map(|thread| thread.join().unwrap())
            .collect()
    });
    starts.sort();
    for (index, start) in (0u32..).zip(&starts) {
        assert!(*start >= begun + every * index, "start {index} came early");
    }
    let (blocked, signals) = std::sync::mpsc::channel();
    gates
        .pace("other", every, &Cancel::observed(blocked))
        .unwrap();
    assert!(signals.try_recv().is_err(), "another address waited");
}

#[cfg(test)]
#[test]
fn a_cancel_ends_a_paced_wait() {
    let gates = Gates::default();
    let every = Duration::from_secs(60);
    gates.pace("local", every, &Cancel::default()).unwrap();
    let (blocked, signals) = std::sync::mpsc::channel();
    let cancel = Cancel::observed(blocked);
    std::thread::scope(|scope| {
        let waiter = scope.spawn(|| gates.pace("local", every, &cancel));
        signals.recv().unwrap();
        cancel.fire();
        assert!(matches!(waiter.join().unwrap(), Err(Error::Cancelled)));
    });
}

#[cfg(test)]
#[test]
fn a_slot_past_the_deadline_fails_at_once_and_reserves_nothing() {
    let (gates, every) = (Gates::default(), Duration::from_secs(60));
    gates.pace("local", every, &Cancel::default()).unwrap();
    let next = gates.slots.lock().unwrap()["local"];
    let short = Cancel::default().with_deadline(crate::engine::Deadline::after(every / 2));
    for _ in 0..5 {
        assert!(matches!(
            gates.pace("local", every, &short),
            Err(Error::Deadline(_))
        ));
    }
    assert_eq!(gates.slots.lock().unwrap()["local"], next);
}

#[cfg(test)]
#[test]
fn the_rate_variable_takes_whole_numbers_from_1_to_60000() {
    for (text, rate) in [("1", 1), ("60000", 60_000)] {
        assert_eq!(per_minute(Some(text)).unwrap(), NonZeroU32::new(rate));
    }
    assert_eq!(per_minute(None).unwrap(), None);
    for text in ["0", "60001", "+5", " 5", "1.5", ""] {
        assert!(per_minute(Some(text)).is_err(), "{text:?} was accepted");
    }
    assert_eq!(interval(None), None, "no address has a default rate");
    assert_eq!(
        interval(NonZeroU32::new(600)),
        Some(Duration::from_millis(100))
    );
}
