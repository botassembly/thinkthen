//! Register the public Ruby native values.
use super::{
    CancelValue, CompletionValue, EngineValue, Error, QuestionValue, Ruby, SetValue, complete,
    default_engine, file_plan, file_question, function, method, new_engine, question, set_file,
    set_json,
};
use magnus::prelude::*;
/// Register the public value classes and the private `ThinkThen::Native`
/// functions. `lib/thinkthen.rb` defines the error classes and the verbs.
#[magnus::init(name = "thinkthen")]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("ThinkThen")?;
    let cancel = module.define_class("Cancel", ruby.class_object())?;
    cancel.define_singleton_method("new", function!(CancelValue::default, 0))?;
    cancel.define_method("cancel", method!(CancelValue::cancel, 0))?;
    cancel.define_method("cancelled?", method!(CancelValue::is_cancelled, 0))?;
    let question_class = module.define_class("Question", ruby.class_object())?;
    question_class.define_method("json", method!(QuestionValue::json, 0))?;
    let set_class = module.define_class("QuestionSet", ruby.class_object())?;
    set_class.define_method("names", method!(SetValue::names, 0))?;
    let native = module.define_module("Native")?;
    let completion = native.define_class("Completion", ruby.class_object())?;
    completion.define_method("done?", method!(CompletionValue::done, 0))?;
    completion.define_method("result", method!(CompletionValue::result, 1))?;
    let engine = native.define_class("Engine", ruby.class_object())?;
    engine.define_method("call", method!(EngineValue::call, 8))?;
    engine.define_method("usage", method!(EngineValue::usage, 0))?;
    engine.define_method("plan", method!(EngineValue::plan, 4))?;
    native.define_module_function("default_engine", function!(default_engine, 0))?;
    native.define_module_function("engine", function!(new_engine, 1))?;
    native.define_module_function("question", function!(question, 1))?;
    native.define_module_function("question_file", function!(file_question, 1))?;
    native.define_module_function("plan_file", function!(file_plan, 2))?;
    native.define_module_function("set_json", function!(set_json, 1))?;
    native.define_module_function("set_file", function!(set_file, 1))?;
    complete::register(ruby, native, engine)?;
    Ok(())
}
