//! Retain one resolved public recognition declaration before reading evidence.
use crate::args::RecognizeArguments;
use crate::failure::Failure;

pub(super) fn settle(
    arguments: &RecognizeArguments,
    mut admitted: crate::AdmittedRequest,
) -> Result<(crate::Recognize, crate::AdmittedRequest), Failure> {
    let file = arguments.kinds.first().filter(|kind| kind.starts_with('@'));
    if let Some(reference) = file {
        if arguments.kinds.len() != 1
            || !arguments.described.is_empty()
            || !arguments.relations.is_empty()
        {
            return Err(Failure::Recognize(
                crate::failure::recognize::Error::Config {
                    file: true,
                    error: "a recognize question file is one closed version-one object".to_owned(),
                },
            ));
        }
        let question = crate::RecognizeQuestionFile::load_reference(reference)?;
        admitted =
            admitted.with_resolved_definition(crate::RequestDefinition::Recognize(question))?;
    }
    let question = match admitted.resolve_question()? {
        crate::RequestDefinition::Recognition(question) => question,
        crate::RequestDefinition::Recognize(file) => file.into_parts().0,
        _ => return Err(Failure::Defect("recognize lost its definition")),
    };
    Ok((question, admitted))
}
