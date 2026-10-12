//! Immediate failures own native diagnostics in a separate calling-thread slot.
use crate::ffi::values as abi;
use std::cell::RefCell;
use std::ffi::c_char;
use thinkthen::{Error, ErrorKind, contained};

thread_local! {
    static LAST: RefCell<Option<crate::failures::Last>> = const { RefCell::new(None) };
}

pub(crate) enum Failure {
    Kind(ErrorKind),
    Native(Error),
}
impl From<ErrorKind> for Failure {
    fn from(kind: ErrorKind) -> Self {
        Self::Kind(kind)
    }
}

fn fail(failure: Failure) -> i32 {
    let kind = match &failure {
        Failure::Kind(kind) => *kind,
        Failure::Native(error) => error.kind(),
    };
    let (code, message) = match kind {
        ErrorKind::Usage => (abi::THINKTHEN_EUSAGE, c"invalid session arguments or input"),
        ErrorKind::Backend => (
            abi::THINKTHEN_EBACKEND,
            c"session backend refused the operation",
        ),
        ErrorKind::Deadline => (abi::THINKTHEN_EDEADLINE, c"session deadline expired"),
        ErrorKind::Local => (abi::THINKTHEN_ELOCAL, c"session local operation failed"),
        ErrorKind::Cancelled => (abi::THINKTHEN_ECANCELLED, c"session operation cancelled"),
        ErrorKind::Defect => (
            abi::THINKTHEN_EDEFECT,
            c"defect: a session operation failed",
        ),
    };
    let owned = match failure {
        Failure::Kind(_) => Error::new(kind, message.to_string_lossy()).into(),
        Failure::Native(error) => error.into(),
    };
    let _ = LAST.try_with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            *slot = Some(crate::failures::Last::of(owned));
        }
    });
    code
}

pub(crate) fn call(body: impl FnOnce() -> Result<(), ErrorKind>) -> i32 {
    native_call(|| body().map_err(Failure::Kind))
}

pub(crate) fn native_call(body: impl FnOnce() -> Result<(), Failure>) -> i32 {
    guard(abi::THINKTHEN_EDEFECT, || match body() {
        Ok(()) => abi::THINKTHEN_OK,
        Err(kind) => fail(kind),
    })
}

pub(crate) fn guard<T>(fallback: T, body: impl FnOnce() -> T) -> T {
    contained(body).unwrap_or_else(|| {
        fail(Failure::Kind(ErrorKind::Defect));
        fallback
    })
}

fn read<T>(fallback: T, inspect: impl Fn(&crate::failures::Last) -> T) -> T {
    LAST.try_with(|slot| {
        slot.try_borrow()
            .ok()
            .and_then(|held| held.as_ref().map(inspect))
    })
    .ok()
    .flatten()
    .unwrap_or(fallback)
}
pub(crate) fn message() -> *const c_char {
    read(
        c"no session failure yet".as_ptr(),
        crate::failures::Last::message,
    )
}
pub(crate) fn retryable() -> i32 {
    read(0, crate::failures::Last::retryable)
}
pub(crate) fn facts() -> *const c_char {
    read(std::ptr::null(), crate::failures::Last::facts)
}

#[cfg(test)]
mod tests {
    #[test]
    fn immediate_native_diagnostics_keep_retry_and_owned_facts() {
        let backend = conformance_backend::Backend::start().expect("loopback");
        let engine = thinkthen::Engine::builder()
            .base_url(&format!("{}/arm/503/v1", backend.origin()))
            .expect("base")
            .api_key("fake-session-key")
            .expect("key")
            .max_retries(0)
            .no_cache()
            .build()
            .expect("engine");
        let question = thinkthen::Question::decide("Is it?")
            .expect("question")
            .cut();
        let error = engine.decide(&question, "text").expect_err("backend fails");
        assert!(error.retryable());
        assert_eq!(
            super::native_call(|| Err(super::Failure::Native(error))),
            crate::ffi::values::THINKTHEN_EBACKEND
        );
        drop(engine);
        assert_eq!(super::retryable(), 1);
        let owned = super::read(None, |last| Some(last.snapshot())).expect("owned failure");
        assert_eq!(owned.facts.expect("facts").requests_sent(), 1);
        let message = super::message();
        let facts = super::facts();
        assert!(!facts.is_null());
        let json = super::read(None, |last| {
            last.snapshot()
                .facts
                .map(|facts| crate::failures::facts_json(&facts))
        })
        .expect("owned JSON");
        assert_eq!(super::call(|| Ok(())), crate::ffi::values::THINKTHEN_OK);
        let constructor = crate::failures::built(|| crate::settings::build("{}"));
        assert!(constructor.is_some());
        drop(constructor);
        assert!(crate::failures::built(|| crate::settings::build("{\"bad\":1}")).is_none());
        assert_eq!(super::message(), message);
        assert_eq!(super::facts(), facts);
        assert_eq!(super::retryable(), 1);
        assert_eq!(
            super::read(None, |last| {
                last.snapshot()
                    .facts
                    .map(|facts| crate::failures::facts_json(&facts))
            }),
            Some(json)
        );
        assert_eq!(crate::failures::retryable(None), 0);
        assert_eq!(
            super::call(|| Err(thinkthen::ErrorKind::Usage)),
            crate::ffi::values::THINKTHEN_EUSAGE
        );
        assert_eq!(super::retryable(), 0);
        assert!(super::facts().is_null());
    }
}
