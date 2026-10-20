//! Release the GVL for the native call; reacquire it only to pull Ruby input.
#![allow(unsafe_code, reason = "Ruby requires C callbacks outside and inside its GVL")]
#![deny(unsafe_op_in_unsafe_fn)]
use magnus::{Error, Ruby, Value, prelude::*};
use std::ffi::c_void;
use thinkthen::{Call, CancelToken, RequestArguments, RequestValue};
struct Pull {
    producer: Value,
    item: Option<Result<Option<thinkthen::RequestItem>, Error>>,
}
unsafe extern "C" fn pull(data: *mut c_void) -> *mut c_void {
    // SAFETY: the native caller retains Pull while this callback holds the GVL.
    let state = unsafe { &mut *data.cast::<Pull>() };
    // SAFETY: rb_thread_call_with_gvl owns the VM lock on this thread.
    let ruby = unsafe { Ruby::get_unchecked() };
    state.item = Some((|| {
        let text: Option<String> = state.producer.funcall("native_next", ())?;
        text.map(|text| thinkthen::RequestItem::from_json(&text)
            .map_err(|error| super::raise(&ruby, error.into()))).transpose()
    })());
    std::ptr::null_mut()
}
impl Iterator for Pull {
    type Item = Result<thinkthen::RequestItem, thinkthen::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let data = std::ptr::from_mut(self).cast::<c_void>();
        // SAFETY: native execution is outside the GVL; pull protects Ruby errors.
        unsafe { rb_sys::rb_thread_call_with_gvl(Some(pull), data); }
        match self.item.take()? {
            Ok(Some(item)) => Some(Ok(item)),
            Ok(None) => None,
            Err(error) => {
                self.item = Some(Err(error));
                Some(Err(thinkthen::Error::new(thinkthen::ErrorKind::Local, "input producer failed")))
            }
        }
    }
}
struct Run<'a> {
    engine: &'a thinkthen::Engine,
    verb: &'a str,
    arguments: Option<RequestArguments>,
    cancel: &'a CancelToken,
    producer: Option<Pull>,
    result: Option<Result<Call<RequestValue>, thinkthen::Error>>,
}
unsafe extern "C" fn run(data: *mut c_void) -> *mut c_void {
    // SAFETY: call retains Run and all borrowed native owners until return.
    let state = unsafe { &mut *data.cast::<Run<'_>>() };
    if let Some(arguments) = state.arguments.take() {
        let feed = state.producer.as_mut().map(|producer| thinkthen::RequestFeed::new("ruby", producer));
        state.result = Some(state.engine.call(state.verb, arguments, thinkthen::RequestEnvironment {
            controls: thinkthen::CallOptions::new().cancel(state.cancel).surface(thinkthen::Surface::Ruby),
            feed,
        }));
    }
    std::ptr::null_mut()
}
unsafe extern "C" fn unblock(data: *mut c_void) {
    // SAFETY: the same stack owner remains live during the unblock callback.
    unsafe { &*data.cast::<Run<'_>>() }.cancel.cancel();
}
pub(super) fn call(_ruby: &Ruby, engine: &thinkthen::Engine, verb: &str, arguments: RequestArguments, cancel: &CancelToken, producer: Value) -> Result<Result<Call<RequestValue>, thinkthen::Error>, Error> {
    let mut state = Run { engine, verb, arguments: Some(arguments), cancel,
        producer: (!producer.is_nil()).then_some(Pull { producer, item: None }), result: None };
    let data = std::ptr::from_mut(&mut state).cast::<c_void>();
    magnus::rb_sys::protect(|| {
        // SAFETY: callbacks retain stack owners and do not unwind through Ruby.
        unsafe { rb_sys::rb_thread_call_without_gvl(Some(run), data, Some(unblock), data); }
        0
    })?;
    if let Some(Err(error)) = state.producer.and_then(|producer| producer.item) { return Err(error); }
    state.result.ok_or_else(|| Error::new(_ruby.exception_runtime_error(), "native call did not return"))
}
