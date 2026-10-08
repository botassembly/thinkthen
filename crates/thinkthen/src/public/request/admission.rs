//! Pure complete-header admission precedes selectors, readers and engines.
use super::{
    Request, RequestDefinition, RequestFunction as Function, RequestImage, RequestInput,
    RequestOptions, RequestQuestion,
};
use crate::{Error, LoadedQuestion, Question, QuestionKind};

/// A validated request header; execution resolves explicit source authority afterward.
#[derive(Clone, Debug)]
pub struct AdmittedRequest {
    pub(super) request: Request,
    pub(super) definition: Option<RequestDefinition>,
}
impl AdmittedRequest {
    /// Borrow the admitted typed header.
    #[must_use]
    pub const fn request(&self) -> &Request {
        &self.request
    }
    /// Resolve an explicit question selector after header admission.
    /// # Errors
    /// Saved selector failures are Local; definitions retain Usage admission.
    pub fn resolve_question(&self) -> Result<RequestDefinition, Error> {
        if let Some(q) = &self.definition {
            return Ok(q.clone());
        }
        let question = &self.request.call.arguments().question;
        let text = match question {
            RequestQuestion::File { path } => crate::read_question_file(path)
                .map_err(|_| Error::local("the question file could not be read"))?,
            RequestQuestion::Name { name } => crate::public::named_question::named_text(name)?,
            RequestQuestion::Reference { reference } => {
                match crate::public::named_question::Reference::resolve(reference)? {
                    crate::public::named_question::Reference::Name(name) => {
                        crate::public::named_question::named_text(&name)?
                    }
                    crate::public::named_question::Reference::Path(path) => {
                        crate::read_question_file(&path)
                            .map_err(|_| Error::local("the question file could not be read"))?
                    }
                }
            }
            _ => {
                return Err(Error::defect(
                    "admitted inline question lost its definition",
                ));
            }
        };
        let value: RequestDefinition = serde_json::from_str(&text)
            .map_err(|_| Error::local("the saved question is invalid"))?;
        admit_definition(self.request.call.function(), value)
            .map_err(|e| Error::local(e.detail().message()))
    }
}
pub(super) fn admit(request: Request) -> Result<AdmittedRequest, Error> {
    let function = request.call.function();
    let args = request.call.arguments();
    admit_options(function, &args.input, &args.options)?;
    admit_input(function, &args.input, &args.options)?;
    let definition = match &args.question {
        RequestQuestion::Definition { value } => Some(admit_definition(function, value.clone())?),
        RequestQuestion::Text { text } => Some(text_question(function, text)?),
        RequestQuestion::File { path } => {
            path_header(path)?;
            None
        }
        RequestQuestion::Name { name } => {
            crate::QuestionName::new(name)?;
            None
        }
        RequestQuestion::Reference { reference } => {
            if reference.is_empty() || !reference.starts_with('@') {
                return Err(Error::usage(
                    "an explicit reference starts with @ and names a question",
                ));
            }
            None
        }
    };
    let admitted = AdmittedRequest {
        request,
        definition,
    };
    if let Some(definition) = &admitted.definition {
        admitted.admit_inline(definition)?;
    }
    Ok(admitted)
}
fn text_question(function: Function, text: &str) -> Result<RequestDefinition, Error> {
    match function {
        Function::Decide | Function::Filter => Ok(Question::decide(text)?.cut().into()),
        Function::Rank => Ok(Question::rank(text)?.into()),
        Function::Find => Ok(RequestDefinition::Atomic(LoadedQuestion::Question(
            Question::find(text)?,
        ))),
        _ => Err(Error::usage(
            "this function requires an authored or native definition",
        )),
    }
}
fn admit_definition(
    function: Function,
    definition: RequestDefinition,
) -> Result<RequestDefinition, Error> {
    let definition = match definition {
        RequestDefinition::Atomic(LoadedQuestion::Question(mut q))
            if function == Function::Rank =>
        {
            if q.authored_threshold
                || !matches!(
                    q.kind(),
                    QuestionKind::Decide | QuestionKind::Score | QuestionKind::Rank
                )
            {
                return Err(Error::usage(
                    "rank takes a decide or score question without an authored threshold",
                ));
            }
            q.threshold = None;
            if q.kind() == QuestionKind::Decide {
                q.kind = crate::public::NativeQuestionKind::Rank;
            }
            RequestDefinition::Rank(q)
        }
        RequestDefinition::DecodedSet { annotation, rank } => match function {
            Function::Annotate => RequestDefinition::Annotate(annotation),
            Function::Rank => RequestDefinition::RankSet(rank.ok_or_else(|| {
                Error::usage("rank requires decide members without authored cuts or pointers")
            })?),
            _ => {
                return Err(Error::usage(
                    "question kind does not match the requested function",
                ));
            }
        },
        other => other,
    };
    let valid = match &definition {
        RequestDefinition::Atomic(q) => match function {
            Function::Decide => q.kind() == QuestionKind::Decide,
            Function::Filter => {
                q.kind() == QuestionKind::Decide && matches!(q, LoadedQuestion::Question(_))
            }
            Function::Choose => q.kind() == QuestionKind::Choose,
            Function::Tag => q.kind() == QuestionKind::Tag,
            Function::Score => q.kind() == QuestionKind::Score,
            Function::Find => q.kind() == QuestionKind::Find,
            _ => false,
        },
        RequestDefinition::Rank(_) | RequestDefinition::RankSet(_) => function == Function::Rank,
        RequestDefinition::Find(_) => function == Function::Find,
        RequestDefinition::Annotate(_) => function == Function::Annotate,
        RequestDefinition::Recognize(_) | RequestDefinition::Recognition(_) => {
            function == Function::Recognize
        }
        RequestDefinition::Relate(_) => function == Function::Relate,
        RequestDefinition::DynamicChoose(_) => function == Function::Choose,
        RequestDefinition::DecodedSet { .. } => false,
    };
    if !valid {
        return Err(Error::usage(
            "question kind does not match the requested function",
        ));
    }
    Ok(definition)
}
fn admit_options(
    function: Function,
    input: &RequestInput,
    options: &RequestOptions,
) -> Result<(), Error> {
    if let Some(model) = &options.model {
        crate::core::ModelName::new(model).map_err(|_| Error::usage("invalid model name"))?;
    }
    if let Some(rule) = &options.threshold {
        if !matches!(
            function,
            Function::Decide | Function::Choose | Function::Tag | Function::Filter
        ) {
            return Err(Error::usage("this function does not accept this threshold"));
        }
        let rule = rule.native()?;
        if function != Function::Decide && !rule.is_cut() {
            return Err(Error::usage("this function does not accept this threshold"));
        }
    }
    if (options.examples.is_some() || options.examples_field.is_some())
        && function != Function::Recognize
    {
        return Err(Error::usage("examples apply only to recognize"));
    }
    if options.options_field.is_some() && function != Function::Choose {
        return Err(Error::usage("options_field applies only to choose"));
    }
    if options.none && function != Function::Find {
        return Err(Error::usage("none applies only to find"));
    }
    if options.top.is_some() && function != Function::Rank {
        return Err(Error::usage("top applies only to rank"));
    }
    if options.files_only
        && (function != Function::Filter || !matches!(input, RequestInput::Source { .. }))
    {
        return Err(Error::usage("files_only requires filter source input"));
    }
    if options.details
        && !matches!(
            function,
            Function::Decide | Function::Choose | Function::Score | Function::Tag
        )
    {
        return Err(Error::usage("details applies only to primitive judgments"));
    }
    if let Some(batch) = &options.batch {
        batch.native()?;
    }
    if let Some(ms) = options.deadline_ms {
        crate::CallOptions::new().deadline_ms(ms)?;
    }
    for pointer in options.field.iter().flatten().chain(
        [
            &options.context_field,
            &options.options_field,
            &options.examples_field,
        ]
        .into_iter()
        .flatten(),
    ) {
        crate::core::Pointer::new(pointer).map_err(|_| Error::usage("invalid field pointer"))?;
    }
    Ok(())
}
fn admit_input(
    function: Function,
    input: &RequestInput,
    options: &RequestOptions,
) -> Result<(), Error> {
    match input {
        RequestInput::Text { images, .. } | RequestInput::Json { images, .. } => {
            if matches!(function, Function::Rank | Function::Find | Function::Relate) {
                return Err(Error::usage("this function requires a complete input set"));
            }
            image_headers(function, images, options)?;
        }
        RequestInput::Records { items }
        | RequestInput::Units { items }
        | RequestInput::Entities { items } => {
            if matches!(input, RequestInput::Units { .. }) && function != Function::Find {
                return Err(Error::usage("units applies only to find"));
            }
            if matches!(input, RequestInput::Entities { .. }) && function != Function::Relate {
                return Err(Error::usage("entities applies only to relate"));
            }
            for item in items {
                admit_item(function, item, options)?;
            }
        }
        RequestInput::Source { source } => {
            if source.paths.is_empty() {
                return Err(Error::usage("source requires at least one path"));
            }
            for path in &source.paths {
                path_header(path)?;
            }
            crate::InputReaderOptions {
                reading: source.reading,
                media: source.media,
            }
            .validate()?;
            if source.media == crate::ReaderMedia::Image {
                if !function.images() {
                    return Err(Error::usage(format!(
                        "{} accepts text only; images are unsupported",
                        function.name()
                    )));
                }
                admit_image_projection(options)?;
            }
        }
        RequestInput::Feed {
            name,
            reading,
            images,
            ..
        } => {
            if name.is_empty() {
                return Err(Error::usage("a feed requires a name"));
            }
            reading.validate()?;
            if !images.is_empty()
                && (reading.window.is_some() || reading.unit != crate::SourceUnit::Line)
            {
                return Err(Error::usage(
                    "image attachments cannot accompany located unit controls",
                ));
            }
            image_headers(function, images, options)?;
        }
    }
    Ok(())
}
fn admit_image_projection(options: &RequestOptions) -> Result<(), Error> {
    if options
        .field
        .as_ref()
        .is_some_and(|fields| !fields.is_empty())
        || options.context_field.is_some()
        || options.options_field.is_some()
    {
        return Err(Error::usage("image input cannot accompany field pointers"));
    }
    Ok(())
}
fn image_headers(
    function: Function,
    images: &[RequestImage],
    _options: &RequestOptions,
) -> Result<(), Error> {
    if images.is_empty() {
        return Ok(());
    }
    if !function.images() {
        return Err(Error::usage(format!(
            "{} accepts text only; images are unsupported",
            function.name()
        )));
    }
    if images.len() > crate::MAX_IMAGES {
        return Err(Error::usage("image evidence requires 1 to 8 images"));
    }
    for image in images {
        match image {
            RequestImage::File { path, .. } => path_header(path)?,
            RequestImage::Bytes { bytes, .. } if bytes.len() > crate::MAX_IMAGE_BYTES => {
                return Err(Error::usage("image exceeds the compressed byte SDK limit"));
            }
            RequestImage::Bytes { media, bytes } => {
                crate::ImageInput::new(*media, bytes.clone())?;
            }
        }
    }
    Ok(())
}
fn path_header(path: &std::path::Path) -> Result<(), Error> {
    if path.as_os_str().is_empty() {
        Err(Error::usage("explicit file paths must not be empty"))
    } else {
        Ok(())
    }
}

pub(super) fn admit_item(
    function: Function,
    item: &super::RequestItem,
    options: &RequestOptions,
) -> Result<(), Error> {
    if item.original.is_none() && item.images.is_empty() {
        return Err(Error::usage("an item requires original evidence or images"));
    }
    image_headers(function, &item.images, options)?;
    if item.original.is_none() {
        admit_image_projection(options)?;
    }
    if (item.context.is_some() && options.context_field.is_some())
        || (item.options.is_some() && options.options_field.is_some())
    {
        return Err(Error::usage(
            "explicit item context/options conflict with projection pointers",
        ));
    }
    if item.examples.is_some() && function != Function::Recognize {
        return Err(Error::usage("item examples apply only to recognize"));
    }
    if item.examples.is_some() && options.examples_field.is_some() {
        return Err(Error::usage("item examples conflict with examples_field"));
    }
    if item.options.is_some() && function != Function::Choose {
        return Err(Error::usage("item options apply only to choose"));
    }
    Ok(())
}
