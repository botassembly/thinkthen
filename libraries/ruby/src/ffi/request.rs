//! Named calls enter the native owner once; Ruby owns only its pull callback.
use super::{CancelValue, EngineValue, Fault, checked};
use magnus::{Error, RClass, RModule, Ruby, Value, function, method, prelude::*};
use thinkthen::{Request, RequestArguments};
fn client(ruby: &Ruby) -> Result<RClass, Error> {
    let module: RModule = ruby.class_object().const_get("ThinkThen")?;
    module.const_get("Client")
}
fn build(ruby: &Ruby, verb: &str, question: Value, input: Value, options: Value) -> Result<Request, Error> {
    let parts: (String, String, String) = client(ruby)?.funcall("components", (question, input, options))?;
    checked(ruby, Request::named_from_json(verb, &parts.0, &parts.1, &parts.2).map_err(Fault::from))
}
fn request_engine(ruby: &Ruby, text: String) -> Result<EngineValue, Error> {
    checked(ruby, thinkthen::EngineBuilder::from_settings_json(&text)
        .and_then(thinkthen::EngineBuilder::build).map_err(Fault::from))
        .map(|engine| EngineValue { engine })
}
fn call(ruby: &Ruby, engine: &EngineValue, verb: String, parts: (Value, Value, Value), token: &CancelValue, producer: Value) -> Result<Value, Error> {
    let RequestArguments { question, input, options } = build(ruby, &verb, parts.0, parts.1, parts.2)?.call.arguments;
    let result = super::session_wait::call(ruby, &engine.engine, &verb,
        RequestArguments { question, input, options }, &token.0, producer)?;
    let result = checked(ruby, result.map_err(Fault::from))?;
    let tree = checked(ruby, result.value_tree().map_err(Fault::from))?;
    super::native_result::plain(ruby, &tree)
}
fn plan(ruby: &Ruby, engine: &EngineValue, verb: String, question: Value, input: Value, controls: (Value, bool)) -> Result<Value, Error> {
    let request = checked(ruby, build(ruby, &verb, question, input, controls.0)?.admit().map_err(Fault::from))?;
    let planned = checked(ruby, engine.engine.plan_request(&request, thinkthen::RequestEnvironment::default()).map_err(Fault::from))?;
    let tree = super::tree::tree(&planned).map_err(|_| Error::new(ruby.exception_runtime_error(), "native preview could not be read"))?;
    super::native_result::convert(ruby, "completeplan", &tree)
}
pub(super) fn register(_ruby: &Ruby, native: RModule, engine: RClass) -> Result<(), Error> {
    native.define_module_function("request_engine", function!(request_engine, 1))?;
    engine.define_method("call", method!(call, 4))?;
    engine.define_method("plan", method!(plan, 4))?;
    Ok(())
}
