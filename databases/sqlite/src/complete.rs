//! Named complete SQL doors preserve native evaluation and error ownership.
use crate::question;
use thinkthen::Error;
pub(crate) mod selector;

pub(crate) fn prepare(
    verb: &str,
    source: &str,
    settings: &thinkthen::Settings,
) -> Result<crate::complete_native::Prepared, Error> {
    if !source.starts_with('@') {
        return crate::complete_native::prepare(verb, source, settings);
    }
    let reference = if let Some(name) = source.strip_prefix("@@") {
        thinkthen::QuestionFileReference::named(name)?
    } else {
        thinkthen::QuestionFileReference::reference(source)?
    };
    let text = question::resolved_file(source, &reference)
        .map_err(|failure| Error::new(failure.kind, failure.message))?;
    let parsed = crate::complete_native::settings::parse_file(verb, &text, &reference)?;
    crate::complete_native::settings::prepare_file(verb, &text, settings, parsed)
}
