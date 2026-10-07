//! A native complete batch owns its worker; Ruby only pulls the next packet.
use super::{CancelValue, EngineValue, Fault, checked};
use crate::call::complete::stream::Session;
use magnus::{Error, Module, RClass, RModule, Ruby, method};
#[magnus::wrap(class = "ThinkThen::Native::CompleteBatch", free_immediately, size)]
#[derive(Debug)]
struct NativeBatch(Session);
impl NativeBatch {
    fn pull(ruby: &Ruby, rb_self: &Self) -> Result<String, Error> {
        checked(ruby, rb_self.0.advance().map_err(Fault::from))?;
        loop {
            let result = super::complete_poll(ruby, &rb_self.0)?;
            if let Some(packet) = checked(ruby, result.map_err(Fault::from))? {
                return Ok(packet);
            }
        }
    }
    fn close(&self) {
        self.0.close();
    }
    fn cancel(&self) {
        self.0.cancel();
    }
}
fn start(
    ruby: &Ruby,
    engine: &EngineValue,
    request: String,
    deadline: Option<i64>,
    cancel: Option<&CancelValue>,
) -> Result<NativeBatch, Error> {
    let caller = cancel.map(|c| c.0.clone());
    checked(
        ruby,
        Session::start(engine.engine.clone(), request, deadline, caller, None).map_err(Fault::from),
    )
    .map(NativeBatch)
}
pub(super) fn register(ruby: &Ruby, native: RModule, engine: RClass) -> Result<(), Error> {
    let batch = native.define_class("CompleteBatch", ruby.class_object())?;
    batch.define_method("pull", method!(NativeBatch::pull, 0))?;
    batch.define_method("close", method!(NativeBatch::close, 0))?;
    batch.define_method("cancel", method!(NativeBatch::cancel, 0))?;
    engine.define_method("complete_batch", method!(start, 3))?;
    Ok(())
}
