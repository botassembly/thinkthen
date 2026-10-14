//! Additive scalar input interfaces on the existing scheduler and result carriers.

use crate::public::engine::only;
use crate::public::question::Kind;
use crate::public::{Call, CallOptions, DetailQuestion, Details, Engine, Error, QuestionInput};

impl Engine {
    /// Full details for explicit input with controls.
    /// # Errors
    /// Refuses unsupported images before transport.
    pub(crate) fn details_input_with<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<Details>, Error> {
        let question = question.question();
        only(
            question,
            &[
                Kind::Decide,
                Kind::Banded,
                Kind::Choose,
                Kind::Tag,
                Kind::Score,
            ],
            "details",
        )?;
        let function =
            crate::RequestFunction::for_definition(&question.clone().into()).input_function();
        self.atomic_complete(function, question, input, options)?
            .try_map(|canonical| Details::of_complete::<()>(&canonical, None))
    }
}
