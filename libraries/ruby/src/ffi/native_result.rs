//! Convert the native value tree without serializing across the Ruby boundary.
use magnus::{Error, RModule, Ruby, Value, prelude::*};
use thinkthen::RequestValueTree as Tree;

pub(super) fn plain(ruby: &Ruby, value: &Tree) -> Result<Value, Error> {
    let result = match value {
        Tree::Absent | Tree::Null => ruby.qnil().as_value(),
        Tree::Bool(value) => ruby.into_value(*value),
        Tree::String(value) => ruby.into_value(value.clone()),
        Tree::Number(value) => {
            let method = if value.contains(['.', 'e', 'E']) { "Float" } else { "Integer" };
            ruby.module_kernel().funcall(method, (value.as_str(),))?
        }
        Tree::List(values) => {
            let array = ruby.ary_new();
            for value in values { array.push(plain(ruby, value)?)?; }
            array.as_value()
        }
        Tree::Map(values) => {
            let hash = ruby.hash_new();
            for (key, value) in values {
                if !matches!(value, Tree::Absent) { hash.aset(key.as_str(), plain(ruby, value)?)?; }
            }
            hash.as_value()
        }
    };
    result.funcall("freeze", ())
}
pub(super) fn convert(ruby: &Ruby, kind: &str, tree: &Tree) -> Result<Value, Error> {
    results(ruby)?.funcall("convert", (kind, plain(ruby, tree)?))
}
fn results(ruby: &Ruby) -> Result<RModule, Error> {
    let module: RModule = ruby.class_object().const_get("ThinkThen")?;
    module.const_get("Results")
}
