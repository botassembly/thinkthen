//! Run a whole toolbox through the shared engine.

use thinkthen::{Annotate, Cancel, Error, Pointer, Toolbox};

use super::AnnotateArgs;

pub(super) fn run(mut args: AnnotateArgs, cancel: Cancel) -> Result<u8, Error> {
    let toolbox = Toolbox::from_bytes(args.toolbox.as_bytes()).map_err(Error::into_argument)?;
    let on = args
        .common
        .on
        .as_deref()
        .map(str::parse::<Pointer>)
        .transpose()?;
    let mut question = Annotate::new(&toolbox);
    if let Some(on) = &on {
        question = question.on(on);
    }
    // Keep the whole JSON record so each question can select its own evidence.
    args.common.on = None;
    super::run(args.common, &question, cancel)
}
