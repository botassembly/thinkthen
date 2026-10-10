//! The interpreter boundary; native values retain no Ruby objects.
use crate::Fault;
use magnus::{
    Error, Exception, ExceptionClass, RModule, Ruby, Value, function, method, prelude::*,
};
mod native_result;
mod request;
mod results_generated;
#[magnus::wrap(class = "ThinkThen::Cancel", free_immediately, size)]
#[derive(Debug, Default)]
struct CancelValue(thinkthen::CancelToken);
impl CancelValue {
    fn cancel(&self) {
        self.0.cancel();
    }
    fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }
}
#[magnus::wrap(class = "ThinkThen::Native::Engine", free_immediately, size)]
#[derive(Debug)]
struct EngineValue {
    engine: thinkthen::Engine,
}
impl EngineValue {
    fn usage(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let value = serde_json::to_value(this.engine.usage()).map_err(|_| {
            Error::new(
                ruby.exception_runtime_error(),
                "native usage could not be read",
            )
        })?;
        native_result::plain(ruby, &value)
    }
    fn usage_persistence(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        status(ruby, this.engine.usage_persistence())
    }
    fn finish_usage_status(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        status(ruby, this.engine.finish_usage_status())
    }
}
fn status(ruby: &Ruby, state: thinkthen::UsagePersistence) -> Result<Value, Error> {
    let name = serde_json::to_value(state).map_err(|_| {
        Error::new(
            ruby.exception_runtime_error(),
            "native usage state could not be read",
        )
    })?;
    Ok(ruby.into_value((name.as_str().unwrap_or("failed"), state.advice())))
}
fn checked<T>(ruby: &Ruby, value: Result<T, Fault>) -> Result<T, Error> {
    value.map_err(|fault| raise(ruby, fault))
}
fn raise(ruby: &Ruby, fault: Fault) -> Error {
    let build = || -> Result<Error, Error> {
        let module: RModule = ruby.class_object().const_get("ThinkThen")?;
        let class: ExceptionClass = module.const_get(format!(
            "{}Error",
            match fault.kind {
                thinkthen::ErrorKind::Usage => "Usage",
                thinkthen::ErrorKind::Backend => "Backend",
                thinkthen::ErrorKind::Deadline => "Deadline",
                thinkthen::ErrorKind::Local => "Local",
                thinkthen::ErrorKind::Cancelled => "Cancelled",
                thinkthen::ErrorKind::Defect => "Defect",
            }
        ))?;
        let error: Value = class.funcall(
            "new",
            (fault.message.as_str(), fault.kind.name(), fault.retryable),
        )?;
        if let Some(facts) = fault.facts {
            let json = serde_json::to_value(facts).map_err(|_| {
                Error::new(
                    ruby.exception_runtime_error(),
                    "native facts could not be read",
                )
            })?;
            error.funcall::<_, _, Value>(
                "instance_variable_set",
                (
                    "@facts",
                    native_result::convert(ruby, "completeFacts", &json)?,
                ),
            )?;
        }
        Exception::from_value(error)
            .map(Error::from)
            .ok_or_else(|| {
                Error::new(
                    ruby.exception_runtime_error(),
                    "native failure could not be raised",
                )
            })
    };
    build().unwrap_or_else(|error| error)
}
#[magnus::init(name = "thinkthen")]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("ThinkThen")?;
    let cancel = module.define_class("Cancel", ruby.class_object())?;
    cancel.define_singleton_method("new", function!(CancelValue::default, 0))?;
    cancel.define_method("cancel", method!(CancelValue::cancel, 0))?;
    cancel.define_method("cancelled?", method!(CancelValue::is_cancelled, 0))?;
    let native = module.define_module("Native")?;
    let engine = native.define_class("Engine", ruby.class_object())?;
    engine.define_method("usage", method!(EngineValue::usage, 0))?;
    engine.define_method(
        "usage_persistence",
        method!(EngineValue::usage_persistence, 0),
    )?;
    engine.define_method(
        "finish_usage_status",
        method!(EngineValue::finish_usage_status, 0),
    )?;
    native_result::register(ruby, native)?;
    request::register(ruby, native, engine)
}
