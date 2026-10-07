//! Constant-space monotonic union of actual HTTP/body-read scopes.
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub(super) struct Timeline(Mutex<Clock>);
#[derive(Debug, Default)]
struct Clock {
    active: usize,
    since: Option<Instant>,
    elapsed: Option<Duration>,
    invalid: bool,
}
impl Clock {
    fn enter(&mut self, now: Instant) {
        if self.active == 0 {
            self.since = Some(now);
        }
        match self.active.checked_add(1) {
            Some(active) => self.active = active,
            None => self.invalid = true,
        }
    }
    fn leave(&mut self, now: Instant) {
        let Some(active) = self.active.checked_sub(1) else {
            self.invalid = true;
            return;
        };
        self.active = active;
        if active == 0 {
            self.elapsed = self.since.take().and_then(|since| {
                self.elapsed
                    .unwrap_or_default()
                    .checked_add(now.checked_duration_since(since)?)
            });
            self.invalid |= self.elapsed.is_none();
        }
    }
    fn total(&self, now: Instant) -> Option<Duration> {
        if self.invalid {
            return None;
        }
        let open = self.since.map_or(Some(Duration::ZERO), |since| {
            now.checked_duration_since(since)
        })?;
        self.elapsed.unwrap_or_default().checked_add(open)
    }
}
impl Timeline {
    pub(super) fn enter(&self) -> Interval<'_> {
        if let Ok(mut clock) = self.0.lock() {
            clock.enter(Instant::now());
        }
        Interval(self)
    }
    pub(super) fn total(&self) -> Option<Duration> {
        self.0.lock().ok()?.total(Instant::now())
    }
}
/// Leaving through return or unwind closes the same scope.
#[derive(Debug)]
pub(crate) struct Interval<'a>(&'a Timeline);
impl Drop for Interval<'_> {
    fn drop(&mut self) {
        if let Ok(mut clock) = self.0.0.lock() {
            clock.leave(Instant::now());
        }
    }
}
impl super::Counters {
    pub(crate) fn http_interval(&self) -> Interval<'_> {
        self.shared.http.enter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    type Case = (&'static [(bool, u64)], u64);
    fn measure(base: Instant, events: &[(bool, u64)]) -> Option<Duration> {
        let mut clock = Clock::default();
        for (enter, at) in events {
            let now = base + Duration::from_millis(*at);
            if *enter {
                clock.enter(now);
            } else {
                clock.leave(now);
            }
        }
        clock.total(base + Duration::from_millis(300))
    }
    #[test]
    fn http_scopes_sum_disjoint_intervals_and_count_overlaps_once() {
        let base = Instant::now();
        let cases: &[Case] = &[
            (&[], 0),
            (&[(true, 10), (false, 110), (true, 120), (false, 220)], 200),
            (&[(true, 10), (true, 30), (false, 110), (false, 130)], 120),
            (
                &[
                    (true, 10),
                    (true, 10),
                    (true, 20),
                    (false, 30),
                    (false, 40),
                    (false, 110),
                ],
                100,
            ),
        ];
        for (events, expected) in cases {
            assert_eq!(
                measure(base, events),
                Some(Duration::from_millis(*expected))
            );
        }
    }
}
