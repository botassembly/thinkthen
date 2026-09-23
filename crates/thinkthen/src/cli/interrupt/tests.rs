use super::{ACTIONS, Action, Routing, StartError, State};
use crate::cli::edge::Environment;
use crate::cli::failure::{Failure, report};
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock, mpsc};

fn routing(cleanup: impl FnOnce() -> Result<(), ()> + Send + 'static) -> Routing {
    Routing::Test(Box::new(cleanup))
}

fn observed_activation(
    state: &State,
    result: Result<(), ()>,
) -> (super::Guard<'_>, mpsc::Receiver<(bool, bool)>) {
    let armed = Arc::clone(&state.default_armed);
    let active = Arc::clone(&state.active);
    let (send, observed) = mpsc::channel();
    let mut environment = Environment::default();
    let guard = state
        .activate_with(&mut environment, || {
            Ok(routing(move || {
                send.send((armed.load(Ordering::SeqCst), active.load(Ordering::SeqCst)))
                    .expect("observe");
                result
            }))
        })
        .expect("active");
    (guard, observed)
}

fn defect(result: Result<super::Guard<'_>, Failure>, message: &str) {
    let Err(failure) = result else {
        panic!("operation succeeded");
    };
    let mut written = Vec::new();
    assert_eq!(report(&failure, &mut written), ExitCode::from(70));
    assert_eq!(
        written,
        format!("thinkthen: defect: {message}\n").as_bytes()
    );
}

fn released(state: &State, observed: mpsc::Receiver<(bool, bool)>) {
    assert_eq!(observed.recv().expect("cleanup"), (true, true));
    assert!(!state.active.load(Ordering::SeqCst));
}

#[test]
fn actions_close_the_finalize_race_and_arm_after_delivery() {
    assert_eq!(ACTIONS[0], Action::ConditionalDefault);
    assert_eq!(ACTIONS[1], Action::Cancel);
    assert_eq!(ACTIONS[2], Action::ConditionalDefault);
    assert_eq!(ACTIONS[3], Action::ArmDefault);
    for arm_between in [false, true] {
        let state = State::install(|_, _| Ok(()), |_| Ok(()));
        state.default_armed.store(false, Ordering::SeqCst);
        let first = state.default_armed.load(Ordering::SeqCst);
        if arm_between {
            state.default_armed.store(true, Ordering::SeqCst);
        }
        state.cancel.fire();
        let second = state.default_armed.load(Ordering::SeqCst);
        state.default_armed.store(true, Ordering::SeqCst);
        assert_eq!([first, second], [false, arm_between]);
        assert!(state.cancel.fired());
        assert!(state.default_armed.load(Ordering::SeqCst));
    }
}

#[test]
fn activation_excludes_a_loser_and_reuses_the_reset_process_token() {
    let state = State::install(|_, _| Ok(()), |_| Ok(()));
    let mut first = Environment::default();
    let fresh = first.cancel().flag();
    let owner = state
        .activate_with(&mut first, || Ok(routing(|| Ok(()))))
        .expect("owner");
    assert!(!Arc::ptr_eq(&fresh, &first.cancel().flag()));
    assert!(!state.default_armed.load(Ordering::SeqCst));
    state.cancel.fire();
    let mut loser = Environment::default();
    let loser_token = loser.cancel().flag();
    defect(
        state.activate_with(&mut loser, || panic!("loser routed")),
        "another command entry is already active",
    );
    assert!(state.cancel.fired());
    assert!(Arc::ptr_eq(&loser_token, &loser.cancel().flag()));
    drop(owner);
    let mut second = Environment::default();
    let guard = state
        .activate_with(&mut second, || {
            assert!(!state.cancel.fired(), "stale cancellation reached routing");
            Ok(routing(|| Ok(())))
        })
        .expect("second");
    assert!(Arc::ptr_eq(&first.cancel().flag(), &second.cancel().flag()));
    assert!(matches!(guard.finish(ExitCode::SUCCESS), Ok(code) if code == ExitCode::SUCCESS));
}

static EMULATIONS: AtomicUsize = AtomicUsize::new(0);

fn emulation_returns(state: &State) -> Result<(), ()> {
    assert!(state.default_armed.load(Ordering::SeqCst));
    assert!(state.active.load(Ordering::SeqCst));
    EMULATIONS.fetch_add(1, Ordering::SeqCst);
    Err(())
}

#[test]
fn cleanup_unwind_emulation_and_route_errors_release_after_arming() {
    EMULATIONS.store(0, Ordering::SeqCst);
    let state = State::install(|_, _| Ok(()), emulation_returns);
    let (guard, observed) = observed_activation(&state, Ok(()));
    state.cancel.fire();
    assert!(matches!(guard.finish(ExitCode::SUCCESS), Ok(code) if code == ExitCode::from(130)));
    released(&state, observed);

    let (guard, observed) = observed_activation(&state, Err(()));
    let failure = guard.finish(ExitCode::SUCCESS).expect_err("restore fails");
    defect(Err(failure), "SIGINT routing could not be restored");
    released(&state, observed);

    let (guard, observed) = observed_activation(&state, Ok(()));
    state.cancel.fire();
    drop(guard);
    assert_eq!(EMULATIONS.load(Ordering::SeqCst), 2);
    released(&state, observed);

    for error in [StartError::Activation, StartError::Restoration] {
        let message = match error {
            StartError::Activation => "SIGINT routing could not be activated",
            StartError::Restoration => "SIGINT routing could not be restored",
        };
        let mut environment = Environment::default();
        defect(
            state.activate_with(&mut environment, || Err(error)),
            message,
        );
        assert!(!state.active.load(Ordering::SeqCst));
    }
}

#[cfg(unix)]
mod unix {
    use super::*;
    use crate::cli::interrupt::{Acknowledgment, UnixRouting, carrier, sigint_set};
    use nix::sys::signal::{SigSet, Signal};
    use std::fs;
    use std::io::{BufRead, BufReader, Write as _};
    use std::os::unix::{fs::symlink, process::ExitStatusExt as _};
    use std::path::PathBuf;
    use std::process::{Child, ChildStdout, Command, Stdio};
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "thinkthen-{name}-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            let _removed = fs::remove_file(&path);
            Self(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _removed = fs::remove_file(&self.0);
        }
    }

    fn start_failure(flags: [bool; 4], before: &SigSet) -> StartError {
        let error = UnixRouting::start_with(crate::engine::Cancel::default(), None, flags)
            .err()
            .expect("injected activation fails");
        assert_eq!(SigSet::thread_get_mask().expect("restored"), *before);
        error
    }

    fn refused(path: &Scratch, unchanged: &[u8]) {
        let acknowledgment = Acknowledgment::new(Some(path.0.clone()));
        acknowledgment.write();
        assert!(acknowledgment.failed());
        assert_eq!(fs::read(&path.0).expect("unchanged"), unchanged);
    }

    #[test]
    fn carrier_masks_workers_and_injected_failures_restore_and_join() {
        let before = SigSet::thread_get_mask().expect("mask");
        let routing = UnixRouting::start(crate::engine::Cancel::default(), None).expect("routing");
        assert!(
            SigSet::thread_get_mask()
                .expect("blocked")
                .contains(Signal::SIGINT)
        );
        assert!(
            std::thread::spawn(|| SigSet::thread_get_mask()
                .expect("worker")
                .contains(Signal::SIGINT))
            .join()
            .expect("joined")
        );
        routing.cleanup().expect("cleanup");
        assert_eq!(SigSet::thread_get_mask().expect("restored"), before);
        for (flags, expected) in [
            ([true, false, false, false], StartError::Activation),
            ([false, true, false, false], StartError::Activation),
            ([false, false, true, false], StartError::Activation),
            ([false, true, false, true], StartError::Restoration),
            ([false, false, true, true], StartError::Restoration),
        ] {
            assert_eq!(start_failure(flags, &before), expected);
        }
        let routing = UnixRouting::start_with(
            crate::engine::Cancel::default(),
            None,
            [false, false, false, true],
        )
        .expect("activation");
        assert!(routing.cleanup().is_err());
        assert_eq!(SigSet::thread_get_mask().expect("restored"), before);
    }

    #[test]
    fn acknowledgment_precedes_queued_stop_and_refuses_existing_paths() {
        let created = Scratch::new("ack");
        let acknowledgment = Acknowledgment::new(Some(created.0.clone()));
        let cancel = crate::engine::Cancel::default();
        cancel.fire();
        let signal = sigint_set();
        let original = signal
            .thread_swap_mask(nix::sys::signal::SigmaskHow::SIG_BLOCK)
            .expect("block");
        let (stop, stopped) = mpsc::channel();
        stop.send(()).expect("stop");
        let (ready_send, ready) = mpsc::sync_channel(1);
        let thread = std::thread::spawn(move || {
            carrier(signal, stopped, ready_send, cancel, acknowledgment, false)
        });
        assert_eq!(ready.recv().expect("ready"), Ok(()));
        assert_eq!(thread.join().expect("join"), Ok(()));
        original.thread_set_mask().expect("restore");
        assert_eq!(fs::read(&created.0).expect("byte"), b"1");

        let existing = Scratch::new("existing");
        fs::write(&existing.0, b"keep").expect("fixture");
        refused(&existing, b"keep");
        let target = Scratch::new("target");
        let link = Scratch::new("link");
        fs::write(&target.0, b"target").expect("target");
        symlink(&target.0, &link.0).expect("link");
        refused(&link, b"target");
    }

    fn sigint(process: u32) {
        assert!(
            Command::new("kill")
                .args(["-INT", &process.to_string()])
                .status()
                .expect("kill")
                .success()
        );
    }

    fn child(variable: &str, value: &str) -> (Child, BufReader<ChildStdout>) {
        let mut child = Command::new(std::env::current_exe().expect("test binary"))
            .args([
                "--exact",
                "cli::interrupt::tests::unix::sigint_child",
                "--ignored",
                "--nocapture",
            ])
            .env(variable, value)
            .stdout(Stdio::piped())
            .spawn()
            .expect("child");
        let output = BufReader::new(child.stdout.take().expect("output"));
        (child, output)
    }

    fn await_line(output: &mut impl BufRead, expected: &str) {
        assert!(
            output
                .by_ref()
                .lines()
                .any(|line| matches!(line, Ok(line) if line == expected))
        );
    }

    fn install_prefix(failed_at: usize) -> State {
        let mut position = 0;
        State::install(
            |action, state| {
                if position == failed_at {
                    return Err(());
                }
                position += 1;
                super::super::register(action, state)
            },
            |_| Ok(()),
        )
    }

    fn announce(text: &str) {
        let mut output = std::io::stdout().lock();
        writeln!(output, "{text}").expect("write");
        output.flush().expect("flush");
    }

    #[test]
    #[allow(clippy::excessive_nesting, reason = "synchronized subprocess signals")]
    fn partial_prefixes_and_an_armed_follow_up_sigint_use_the_default() {
        for prefix in 0..4 {
            let (mut child, mut output) = child("THINKTHEN_SIGINT_PREFIX", &prefix.to_string());
            await_line(&mut output, "ready");
            sigint(child.id());
            assert_eq!(
                child.wait().expect("exit").signal(),
                Some(Signal::SIGINT as i32)
            );
        }
        let (mut child, mut output) = child("THINKTHEN_SIGINT_CHILD", "1");
        await_line(&mut output, "ready");
        sigint(child.id());
        await_line(&mut output, "armed");
        sigint(child.id());
        assert_eq!(
            child.wait().expect("exit").signal(),
            Some(signal_hook::consts::signal::SIGINT)
        );
    }

    #[test]
    #[ignore = "subprocess harness"]
    fn sigint_child() {
        if let Ok(prefix) = std::env::var("THINKTHEN_SIGINT_PREFIX") {
            let failed_at = prefix.parse::<usize>().expect("prefix");
            let cell = OnceLock::new();
            let _state = cell.get_or_init(|| install_prefix(failed_at));
            for _ in 0..2 {
                let same = cell.get_or_init(|| panic!("initialization repeated"));
                let mut environment = Environment::default();
                defect(
                    same.activate_with(&mut environment, || panic!("routing started")),
                    "SIGINT handling could not be installed",
                );
            }
            announce("ready");
            loop {
                std::thread::park();
            }
        }
        if std::env::var_os("THINKTHEN_SIGINT_CHILD").is_none() {
            return;
        }
        let state = State::install(super::super::register, super::super::emulate);
        let mut environment = Environment::default();
        let _guard = state
            .activate_with(&mut environment, || {
                Routing::start(crate::engine::Cancel::default(), None)
            })
            .expect("active");
        announce("ready");
        let deadline = Instant::now() + Duration::from_secs(5);
        while !state.default_armed.load(Ordering::SeqCst) {
            assert!(Instant::now() < deadline, "SIGINT was not handled");
            std::thread::yield_now();
        }
        announce("armed");
        loop {
            std::thread::park();
        }
    }
}
