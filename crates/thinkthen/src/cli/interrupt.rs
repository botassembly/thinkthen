use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use crate::cli::edge::Environment;
use crate::cli::failure::Failure;
use crate::engine::Cancel;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    ConditionalDefault,
    Cancel,
    ArmDefault,
}

const ACTIONS: [Action; 4] = [
    Action::ConditionalDefault,
    Action::Cancel,
    Action::ConditionalDefault,
    Action::ArmDefault,
];

struct State {
    cancel: Cancel,
    default_armed: Arc<AtomicBool>,
    active: Arc<AtomicBool>,
    installed: bool,
    emulate: fn(&State) -> Result<(), ()>,
}

impl State {
    fn install(
        mut register: impl FnMut(Action, &Self) -> Result<(), ()>,
        emulate: fn(&Self) -> Result<(), ()>,
    ) -> Self {
        let mut state = Self {
            cancel: Cancel::default(),
            default_armed: Arc::new(AtomicBool::new(true)),
            active: Arc::new(AtomicBool::new(false)),
            installed: false,
            emulate,
        };
        for action in ACTIONS {
            if register(action, &state).is_err() {
                return state;
            }
        }
        state.installed = true;
        state
    }

    fn activate_with<'a>(
        &'a self,
        environment: &mut Environment,
        route: impl FnOnce() -> Result<Routing, StartError>,
    ) -> Result<Guard<'a>, Failure> {
        if !self.installed {
            return Err(Failure::Defect("SIGINT handling could not be installed"));
        }
        self.active
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .map_err(|_| Failure::Defect("another command entry is already active"))?;
        self.cancel.reset();
        environment.cancel = self.cancel.clone();
        let routing = route().map_err(|error| {
            self.active.store(false, Ordering::SeqCst);
            error.failure()
        })?;
        self.default_armed.store(false, Ordering::SeqCst);
        Ok(Guard {
            state: self,
            routing: Some(routing),
        })
    }

    fn activate(&'static self, environment: &mut Environment) -> Result<Guard<'static>, Failure> {
        let acknowledgment = environment.sigint_ack.as_deref().map(PathBuf::from);
        self.activate_with(environment, || {
            Routing::start(self.cancel.clone(), acknowledgment)
        })
    }
}

pub(super) struct Guard<'a> {
    state: &'a State,
    routing: Option<Routing>,
}

impl Guard<'_> {
    fn finalize(&mut self) -> (bool, Result<(), ()>) {
        self.state.default_armed.store(true, Ordering::SeqCst);
        let cancelled = self.state.cancel.fired();
        if cancelled {
            let _returned = (self.state.emulate)(self.state);
        }
        let restored = self.routing.take().map_or(Ok(()), Routing::cleanup);
        self.state.active.store(false, Ordering::SeqCst);
        (cancelled, restored)
    }

    pub(super) fn finish(mut self, code: ExitCode) -> Result<ExitCode, Failure> {
        let (cancelled, restored) = self.finalize();
        if cancelled {
            Ok(ExitCode::from(130))
        } else if restored.is_err() {
            Err(Failure::Defect("SIGINT routing could not be restored"))
        } else {
            Ok(code)
        }
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        if self.routing.is_some() {
            let _cleanup = self.finalize();
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StartError {
    Activation,
    Restoration,
}

impl StartError {
    const fn failure(self) -> Failure {
        match self {
            Self::Activation => Failure::Defect("SIGINT routing could not be activated"),
            Self::Restoration => Failure::Defect("SIGINT routing could not be restored"),
        }
    }
}

enum Routing {
    #[cfg(not(unix))]
    Idle,
    #[cfg(unix)]
    Unix(UnixRouting),
    #[cfg(test)]
    Test(Box<dyn FnOnce() -> Result<(), ()> + Send>),
}

impl Routing {
    fn start(cancel: Cancel, acknowledgment: Option<PathBuf>) -> Result<Self, StartError> {
        #[cfg(unix)]
        {
            UnixRouting::start(cancel, acknowledgment).map(Self::Unix)
        }
        #[cfg(not(unix))]
        {
            Ok(Self::Idle)
        }
    }

    fn cleanup(self) -> Result<(), ()> {
        match self {
            #[cfg(not(unix))]
            Self::Idle => Ok(()),
            #[cfg(unix)]
            Self::Unix(unix) => unix.cleanup(),
            #[cfg(test)]
            Self::Test(cleanup) => cleanup(),
        }
    }
}

#[cfg(unix)]
struct UnixRouting {
    stop: std::sync::mpsc::Sender<()>,
    carrier: std::thread::JoinHandle<Result<(), ()>>,
    original: nix::sys::signal::SigSet,
    restore_fails: bool,
}

#[cfg(unix)]
impl UnixRouting {
    fn start(cancel: Cancel, path: Option<PathBuf>) -> Result<Self, StartError> {
        Self::start_with(cancel, path, [false; 4])
    }

    fn start_with(
        cancel: Cancel,
        path: Option<PathBuf>,
        failures: [bool; 4],
    ) -> Result<Self, StartError> {
        use std::sync::mpsc;

        let [block_fails, spawn_fails, readiness_fails, restore_fails] = failures;
        let signal = sigint_set();
        if block_fails {
            return Err(StartError::Activation);
        }
        let original = signal
            .thread_swap_mask(nix::sys::signal::SigmaskHow::SIG_BLOCK)
            .map_err(|_error| StartError::Activation)?;
        let (stop, stopped) = mpsc::channel();
        let (ready_send, ready) = mpsc::sync_channel(1);
        let acknowledgment = Acknowledgment::new(path);
        let carrier_acknowledgment = acknowledgment.clone();
        if spawn_fails {
            return Err(start_error(&original, restore_fails));
        }
        let carrier = match std::thread::Builder::new()
            .name("thinkthen-sigint".to_owned())
            .spawn(move || {
                carrier(
                    signal,
                    stopped,
                    ready_send,
                    cancel,
                    carrier_acknowledgment,
                    readiness_fails,
                )
            }) {
            Ok(carrier) => carrier,
            Err(_error) => return Err(start_error(&original, restore_fails)),
        };
        let routing = Self {
            stop,
            carrier,
            original,
            restore_fails,
        };
        match ready.recv() {
            Ok(Ok(())) => Ok(routing),
            Ok(Err(())) | Err(_) => Err(routing.abort()),
        }
    }

    fn abort(self) -> StartError {
        let _stopping = self.stop.send(());
        let _joined = self.carrier.join();
        start_error(&self.original, self.restore_fails)
    }

    fn cleanup(self) -> Result<(), ()> {
        let stopped = self.stop.send(()).map_err(|_error| ());
        let joined = self
            .carrier
            .join()
            .map_err(|_panic| ())
            .and_then(|result| result);
        let restored = restore(&self.original, self.restore_fails);
        stopped.and(joined).and(restored)
    }
}

#[cfg(unix)]
fn start_error(original: &nix::sys::signal::SigSet, restore_fails: bool) -> StartError {
    if restore(original, restore_fails).is_err() {
        StartError::Restoration
    } else {
        StartError::Activation
    }
}

#[cfg(unix)]
fn restore(original: &nix::sys::signal::SigSet, injected_failure: bool) -> Result<(), ()> {
    let restored = original.thread_set_mask().map_err(|_error| ());
    if injected_failure { Err(()) } else { restored }
}

#[cfg(unix)]
#[derive(Clone)]
struct Acknowledgment {
    path: Option<PathBuf>,
    failed: Arc<AtomicBool>,
}

#[cfg(unix)]
impl Acknowledgment {
    fn new(path: Option<PathBuf>) -> Self {
        Self {
            path,
            failed: Arc::new(AtomicBool::new(false)),
        }
    }

    fn write(&self) {
        let Some(path) = &self.path else {
            return;
        };
        let result = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .and_then(|mut file| std::io::Write::write_all(&mut file, b"1"));
        if result.is_err() {
            self.failed.store(true, Ordering::Release);
        }
    }

    #[cfg(test)]
    fn failed(&self) -> bool {
        self.failed.load(Ordering::Acquire)
    }
}

#[cfg(unix)]
fn sigint_set() -> nix::sys::signal::SigSet {
    let mut signal = nix::sys::signal::SigSet::empty();
    signal.add(nix::sys::signal::Signal::SIGINT);
    signal
}

#[cfg(unix)]
fn carrier(
    signal: nix::sys::signal::SigSet,
    stopped: std::sync::mpsc::Receiver<()>,
    ready: std::sync::mpsc::SyncSender<Result<(), ()>>,
    cancel: Cancel,
    acknowledgment: Acknowledgment,
    readiness_fails: bool,
) -> Result<(), ()> {
    use std::sync::mpsc::RecvTimeoutError;

    let unblocked = signal.thread_unblock().map_err(|_error| ());
    let announced = if readiness_fails { Err(()) } else { unblocked };
    ready.send(announced).map_err(|_error| ())?;
    unblocked?;
    let mut acknowledged = false;
    loop {
        if cancel.fired() && !acknowledged {
            acknowledgment.write();
            acknowledged = true;
        }
        match stopped.recv_timeout(Cancel::poll()) {
            Ok(()) => {
                if cancel.fired() && !acknowledged {
                    acknowledgment.write();
                }
                break;
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Err(()),
        }
    }
    signal.thread_block().map_err(|_error| ())
}

static STATE: OnceLock<State> = OnceLock::new();

fn process_state() -> &'static State {
    STATE.get_or_init(|| State::install(register, emulate))
}

fn register(action: Action, state: &State) -> Result<(), ()> {
    use signal_hook::consts::signal::SIGINT;
    let registered = match action {
        Action::ConditionalDefault => signal_hook::flag::register_conditional_default(
            SIGINT,
            Arc::clone(&state.default_armed),
        ),
        Action::Cancel => signal_hook::flag::register(SIGINT, state.cancel.flag()),
        Action::ArmDefault => signal_hook::flag::register(SIGINT, Arc::clone(&state.default_armed)),
    };
    registered.map(|_id| ()).map_err(|_error| ())
}

fn emulate(_state: &State) -> Result<(), ()> {
    signal_hook::low_level::emulate_default_handler(signal_hook::consts::signal::SIGINT)
        .map_err(|_error| ())
}

pub(super) fn activate(environment: &mut Environment) -> Result<Guard<'static>, Failure> {
    process_state().activate(environment)
}

#[cfg(test)]
mod tests;
