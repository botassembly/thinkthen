//! Immutable author snapshots copied exclusively from native metadata getters.
use super::Storage;
use crate::ffi::carriers::{
    InputDeclarationV1, InputPropertiesV1, InputPropertyV1, OptionalU64V1, QuestionAuthorV1,
};
use crate::ffi::values as abi;
use thinkthen::{InputDeclaration, InputPropertyType, QuestionName, WordingVersion};
#[derive(Clone, Default)]
pub(crate) struct Author {
    pub(crate) name: Option<QuestionName>,
    pub(crate) version: Option<WordingVersion>,
    pub(crate) item: Option<InputDeclaration>,
    pub(crate) context: Option<InputDeclaration>,
}
impl Author {
    pub(crate) fn of(
        name: Option<&QuestionName>,
        version: Option<WordingVersion>,
        item: Option<&InputDeclaration>,
        context: Option<&InputDeclaration>,
    ) -> Self {
        Self {
            name: name.cloned(),
            version,
            item: item.cloned(),
            context: context.cloned(),
        }
    }
}
macro_rules! native_author {
    ($q:expr) => {{
        let q = &$q;
        $crate::current::author::Author::of(
            q.name(),
            q.wording_version(),
            q.item_schema(),
            q.context_schema(),
        )
    }};
}
pub(crate) use native_author;
pub(crate) struct AuthorOwner {
    data: Author,
    _storage: Storage,
    pub(crate) view: QuestionAuthorV1,
}
impl Clone for AuthorOwner {
    fn clone(&self) -> Self {
        Self::new(self.data.clone())
    }
}
impl AuthorOwner {
    pub(crate) fn new(data: Author) -> Self {
        let mut storage = Storage::default();
        let view = storage.author(&data);
        Self {
            data,
            _storage: storage,
            view,
        }
    }
}
impl Storage {
    pub(crate) fn declaration(&mut self, value: Option<&InputDeclaration>) -> InputDeclarationV1 {
        match value {
            None => InputDeclarationV1::default(),
            Some(InputDeclaration::String) => InputDeclarationV1 {
                kind: abi::DECLARATION_STRING_V1,
                ..InputDeclarationV1::default()
            },
            Some(InputDeclaration::Object(object)) => {
                let properties = object
                    .properties()
                    .iter()
                    .map(|p| InputPropertyV1 {
                        name: self.string(p.name()),
                        kind: match p.kind() {
                            InputPropertyType::String => abi::PROPERTY_STRING_V1,
                            InputPropertyType::Number => abi::PROPERTY_NUMBER_V1,
                            InputPropertyType::Boolean => abi::PROPERTY_BOOLEAN_V1,
                            InputPropertyType::StringList => abi::PROPERTY_STRING_LIST_V1,
                        },
                    })
                    .collect();
                let (data, len) = self.array(properties);
                InputDeclarationV1 {
                    kind: abi::DECLARATION_OBJECT_V1,
                    properties: InputPropertiesV1 { data, len },
                    required: self.strings(object.required()),
                }
            }
        }
    }
    pub(crate) fn author(&mut self, data: &Author) -> QuestionAuthorV1 {
        QuestionAuthorV1 {
            name: self.optional_string(data.name.as_ref().map(QuestionName::as_str)),
            wording_version: OptionalU64V1 {
                present: i32::from(data.version.is_some()),
                value: data.version.map_or(0, |v| u64::from(v.get())),
            },
            item_schema: self.declaration(data.item.as_ref()),
            context_schema: self.declaration(data.context.as_ref()),
        }
    }
    pub(crate) fn row_author(&mut self, author: &Author, members: Vec<QuestionAuthorV1>) {
        let author = self.author(author);
        self.3.push(author);
        self.4.push(members);
    }
}
