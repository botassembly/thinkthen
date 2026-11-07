//! Answer meanings and descriptions reach the engine through its own question grammar.
#![allow(
    unsafe_code,
    reason = "typed exports borrow foreign builders and counted fields"
)]
use crate::{
    HostError, QuestionHandle,
    exports::{THINKTHEN_OK, operation},
    memory::{live_mut, text},
};
use serde_json::{Map, Value};
use thinkthen::{Error, FunctionName, Question, native};

/// What a yes and a no mean, and each option's description, kept until admission.
#[derive(Debug, Default, Clone)]
pub(crate) struct Meanings {
    yes: Option<String>,
    no: Option<String>,
    descriptions: Vec<(String, String)>,
}
impl Meanings {
    fn is_empty(&self) -> bool {
        self.yes.is_none() && self.no.is_none() && self.descriptions.is_empty()
    }
    /// Admit the question again with these fields, so the engine refuses any
    /// that do not fit: a meaning on another function or an unknown option name.
    pub(crate) fn apply(&self, question: Question) -> Result<Question, Error> {
        if self.is_empty() {
            return Ok(question);
        }
        let function = question.function();
        let mut value = serde_json::to_value(&question).map_err(|_| native::internal_error())?;
        let object = value.as_object_mut().ok_or_else(native::internal_error)?;
        for (field, meaning) in [("true", &self.yes), ("false", &self.no)] {
            if let Some(meaning) = meaning {
                object.insert(field.into(), Value::String(meaning.clone()));
            }
        }
        let field = match function {
            FunctionName::Choose => Some("choices"),
            FunctionName::Tag => Some("labels"),
            FunctionName::Score | FunctionName::Rank => Some("levels"),
            _ => None,
        };
        let mut remaining = Map::new();
        for (name, description) in &self.descriptions {
            let option = field
                .and_then(|field| object.get_mut(field))
                .and_then(Value::as_array_mut)
                .and_then(|options| {
                    options.iter_mut().find(|option| {
                        option
                            .as_str()
                            .or_else(|| option.get("name").and_then(Value::as_str))
                            == Some(name.as_str())
                    })
                });
            match option {
                Some(option) => {
                    *option = serde_json::json!({"name": name, "description": description});
                }
                // The engine's unknown-field rule refuses names that match no option.
                None => {
                    remaining.insert(name.clone(), Value::String(description.clone()));
                }
            }
        }
        if !remaining.is_empty() {
            object.insert("descriptions".into(), Value::Object(remaining));
        }
        Question::from_json(&value.to_string())
    }
}

// Check a change against a complete question now; an unfinished one is checked when it runs.
unsafe fn change(
    question: *mut QuestionHandle,
    error: *mut HostError,
    edit: impl FnOnce(&mut Meanings) -> Result<(), Error>,
) -> i32 {
    /* SAFETY: one thread drives the builder and lends the error slot until return. */
    unsafe {
        operation(error, || {
            let handle = live_mut(question)?;
            let mut next = handle.meanings.clone();
            edit(&mut next)?;
            if let Ok(complete) = handle.builder.build() {
                next.apply(complete)?;
            }
            handle.meanings = next;
            Ok(THINKTHEN_OK)
        })
    }
}

/// Say what a yes answer means, for a decide or filter question.
///
/// The meaning is sent with the question so the model reads it, and setting it again replaces
/// it. Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not decide or filter, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_yes_means(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends the builder and counted text until return. */
    unsafe {
        change(question, error, |meanings| {
            meanings.yes = Some(text(data, len)?.to_owned());
            Ok(())
        })
    }
}

/// Say what a no answer means, for a decide or filter question.
///
/// Setting it again replaces it. Returns THINKTHEN_OK, or a negative code with the question
/// unchanged: -THINKTHEN_CODE_USAGE_QUESTION when the question is not decide or filter, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_no_means(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends the builder and counted text until return. */
    unsafe {
        change(question, error, |meanings| {
            meanings.no = Some(text(data, len)?.to_owned());
            Ok(())
        })
    }
}

/// Describe one choice, label or level by name, for choose, tag, score and rank.
///
/// Add the option first; describing the same name again replaces its description. Returns
/// THINKTHEN_OK, or a negative code with the question unchanged: -THINKTHEN_CODE_USAGE_QUESTION
/// when name is not one of the question's options or the function takes no descriptions, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. A question still missing its options
/// is checked when it runs instead.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. name points to name_len readable bytes, or to a NUL-
/// terminated string when name_len is THINKTHEN_NUL_TERMINATED; it is copied before return.
/// description points to len readable bytes, or to a NUL-terminated string when len is
/// THINKTHEN_NUL_TERMINATED; it is copied before return. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_description(
    question: *mut QuestionHandle,
    name: *const u8,
    name_len: usize,
    description: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends the builder and both counted texts until return. */
    unsafe {
        change(question, error, |meanings| {
            let name = text(name, name_len)?.to_owned();
            let description = text(description, len)?.to_owned();
            meanings.descriptions.retain(|(known, _)| *known != name);
            meanings.descriptions.push((name, description));
            Ok(())
        })
    }
}
