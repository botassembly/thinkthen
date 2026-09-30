//! `plan`: the requests a call would send, previewed through the public
//! `Engine::plan_with` on the Ruby thread (ticket 0291). It reads no key and
//! no cache and sends nothing, so it needs no worker.

use super::*;
use thinkthen::CallOptions;

impl EngineValue {
    /// The result schema's `plan` object for one question and its records.
    pub(super) fn plan(
        ruby: &Ruby,
        rb_self: &Self,
        subject: Value,
        records: Vec<String>,
        batch: Option<Value>,
        context: Option<String>,
    ) -> Result<Value, Error> {
        let question = question_of(subject)?;
        let mut options = CallOptions::new();
        if let Some(setting) = batch
            .map(|value| batch_of(ruby, value))
            .transpose()?
            .flatten()
        {
            options = options.batch(setting);
        }
        if let Some(text) = context.as_deref() {
            options = options.context(text);
        }
        let estimate = guarded(|| {
            Ok(match &question {
                LoadedQuestion::Question(asked) => {
                    rb_self.engine.plan_with(asked, records, options)?
                }
                LoadedQuestion::Banded(asked) => {
                    rb_self.engine.plan_with(asked, records, options)?
                }
            })
        });
        ruby_json(ruby, &checked(ruby, estimate)?)
    }
}
