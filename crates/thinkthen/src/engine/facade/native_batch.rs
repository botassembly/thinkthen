//! Bounded native work over the process width and ordered worker admission.

use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::workers;

use super::Engine;

impl Engine {
    pub(crate) fn ask_batches_recoverable<W, R, E>(
        &self,
        cancel: &Cancel<'_>,
        items: Vec<W>,
        work: &(impl Fn(W) -> Result<R, Error> + Sync),
        terminal: impl Fn(&R) -> bool,
        each: impl FnMut(R) -> Result<(), E>,
    ) -> Result<(), E>
    where
        W: Send,
        R: Send,
        E: From<Error>,
    {
        let width = self.state(cancel)?.width;
        workers::ordered_until(width, items, cancel, work, terminal, each)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier, mpsc};
    use std::time::Duration;

    use crate::engine::Cancel;
    use crate::engine::error::Error;
    use crate::engine::workers;

    #[test]
    fn later_fatal_result_halts_admission_while_first_worker_is_held() {
        let both_started = Arc::new(Barrier::new(2));
        let release_first = Arc::new(Barrier::new(2));
        let (fatal_sent, fatal_seen) = mpsc::channel();
        let (third_sent, third_seen) = mpsc::channel();
        let started = AtomicUsize::new(0);
        let work = |item| {
            started.fetch_add(1, Ordering::SeqCst);
            match item {
                0 => {
                    both_started.wait();
                    release_first.wait();
                    Ok(0)
                }
                1 => {
                    both_started.wait();
                    fatal_sent.send(()).expect("fatal worker notified");
                    Err(Error::Defect("fatal second request"))
                }
                2 => {
                    third_sent.send(()).expect("third worker notified");
                    Ok(2)
                }
                _ => unreachable!("three planned requests"),
            }
        };
        std::thread::scope(|scope| {
            let release_gate = Arc::clone(&release_first);
            let release = scope.spawn(move || {
                fatal_seen
                    .recv_timeout(Duration::from_secs(2))
                    .expect("second worker completed");
                let third_started = third_seen.recv_timeout(Duration::from_millis(100)).is_ok();
                release_gate.wait();
                third_started
            });
            let mut delivered = Vec::new();
            let result = workers::ordered(2, vec![0, 1, 2], &Cancel::default(), &work, |item| {
                delivered.push(item);
                Ok::<(), Error>(())
            });
            assert!(matches!(result, Err(Error::Defect("fatal second request"))));
            assert_eq!(delivered, [0], "joined first result remains ordered");
            assert!(
                !release.join().expect("release helper"),
                "third work was admitted"
            );
        });
        assert_eq!(started.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn terminal_denial_joins_later_fatal_and_never_admits_third_work() {
        let release_fatal = Arc::new(Barrier::new(2));
        let (denial_sent, denial_seen) = mpsc::channel();
        let (third_sent, third_seen) = mpsc::channel();
        let (started, finished) = (AtomicUsize::new(0), AtomicUsize::new(0));
        let work = |item| {
            started.fetch_add(1, Ordering::SeqCst);
            match item {
                0 => {
                    finished.fetch_add(1, Ordering::SeqCst);
                    Ok((0, true))
                }
                1 => {
                    release_fatal.wait();
                    finished.fetch_add(1, Ordering::SeqCst);
                    Err(Error::Defect("fatal after denied admission"))
                }
                2 => {
                    third_sent.send(()).expect("third worker notified");
                    finished.fetch_add(1, Ordering::SeqCst);
                    Ok((2, false))
                }
                _ => unreachable!("three planned requests"),
            }
        };
        std::thread::scope(|scope| {
            let release_gate = Arc::clone(&release_fatal);
            let release = scope.spawn(move || {
                denial_seen
                    .recv_timeout(Duration::from_secs(2))
                    .expect("terminal denial delivered");
                let third_started = third_seen.recv_timeout(Duration::from_millis(100)).is_ok();
                release_gate.wait();
                third_started
            });
            let mut delivered = Vec::new();
            let result = workers::ordered_until(
                2,
                vec![0, 1, 2],
                &Cancel::default(),
                &work,
                |outcome| outcome.1,
                |outcome| {
                    delivered.push(outcome.0);
                    denial_sent.send(()).expect("denial observed");
                    Ok::<(), Error>(())
                },
            );
            assert!(matches!(
                result,
                Err(Error::Defect("fatal after denied admission"))
            ));
            assert_eq!(delivered, [0]);
            assert!(
                !release.join().expect("release helper"),
                "third work admitted"
            );
        });
        assert_eq!(started.load(Ordering::SeqCst), 2);
        assert_eq!(finished.load(Ordering::SeqCst), 2, "both workers joined");
    }
}
