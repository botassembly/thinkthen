//! Immutable author snapshots copied exclusively from native metadata getters.
use super::Storage;
use crate::ffi::carriers::{
    InputDeclarationView, InputProperties, InputPropertyView, QuestionAuthorInfo,
};
use crate::ffi::values as abi;
use thinkthen::{InputDeclaration, InputPropertyType, QuestionName};
#[derive(Default)]
pub(crate) struct Author {
    pub(crate) name: Option<QuestionName>,
    pub(crate) item: Option<InputDeclaration>,
    pub(crate) context: Option<InputDeclaration>,
}
impl Author {
    pub(crate) fn of(
        name: Option<&QuestionName>,
        item: Option<&InputDeclaration>,
        context: Option<&InputDeclaration>,
    ) -> Self {
        Self {
            name: name.cloned(),
            item: item.cloned(),
            context: context.cloned(),
        }
    }
}
macro_rules! native_author {
    ($q:expr) => {{
        let q = &$q;
        $crate::current::author::Author::of(q.name(), q.item_schema(), q.context_schema())
    }};
}
pub(crate) use native_author;
pub(crate) struct AuthorOwner {
    _storage: Storage,
    pub(crate) view: QuestionAuthorInfo,
}
impl AuthorOwner {
    pub(crate) fn new(data: Author) -> Self {
        let mut storage = Storage::default();
        let view = storage.author(&data);
        Self {
            _storage: storage,
            view,
        }
    }
}
impl Storage {
    pub(crate) fn declaration(&mut self, value: Option<&InputDeclaration>) -> InputDeclarationView {
        match value {
            None => InputDeclarationView::default(),
            Some(InputDeclaration::String) => InputDeclarationView {
                kind: abi::DECLARATION_STRING,
                ..InputDeclarationView::default()
            },
            Some(InputDeclaration::Object(object)) => {
                let properties = object
                    .properties()
                    .iter()
                    .map(|p| InputPropertyView {
                        name: self.string(p.name()),
                        kind: match p.kind() {
                            InputPropertyType::String => abi::PROPERTY_STRING,
                            InputPropertyType::Number => abi::PROPERTY_NUMBER,
                            InputPropertyType::Boolean => abi::PROPERTY_BOOLEAN,
                            InputPropertyType::StringList => abi::PROPERTY_STRING_LIST,
                        },
                    })
                    .collect();
                let (data, len) = self.slice(properties);
                InputDeclarationView {
                    kind: abi::DECLARATION_OBJECT,
                    properties: InputProperties { data, len },
                    required: self.strings(object.required()),
                }
            }
        }
    }
    pub(crate) fn author(&mut self, data: &Author) -> QuestionAuthorInfo {
        QuestionAuthorInfo {
            name: self.optional_string(data.name.as_ref().map(QuestionName::as_str)),
            item_schema: self.declaration(data.item.as_ref()),
            context_schema: self.declaration(data.context.as_ref()),
        }
    }
}
