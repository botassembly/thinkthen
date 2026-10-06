//! Author metadata is carried separately from the behavioral question.
use super::{
    BandedQuestion, Choice, ChooseQuestion, Error, InputDeclaration, Question, QuestionName,
    TagQuestion, WordingVersion,
};
use crate::core::declaration::QuestionMetadata;

macro_rules! accessors {
    ($($field:tt)+) => {
        /// Optional exact author name; never inferred from wording or a path.
        #[must_use]
        pub fn name(&self) -> Option<&QuestionName> { self.$($field)+.name.as_ref() }
        /// Optional author wording version, with no default.
        #[must_use]
        pub const fn wording_version(&self) -> Option<WordingVersion> { self.$($field)+.wording_version }
        /// Optional declaration of the selected typed item.
        #[must_use]
        pub fn item_schema(&self) -> Option<&InputDeclaration> { self.$($field)+.item_schema.as_ref() }
        /// Optional declaration of explicit per-item context only.
        #[must_use]
        pub fn context_schema(&self) -> Option<&InputDeclaration> { self.$($field)+.context_schema.as_ref() }
        /// Attach a validated author name, outside semantic identity.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_name(mut self, name: QuestionName) -> Result<Self, Error> {
            super::question::once(&mut self.$($field)+.name, name, "the name")?; Ok(self)
        }
        /// Attach a validated wording version, outside semantic identity.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_wording_version(mut self, version: WordingVersion) -> Result<Self, Error> {
            super::question::once(&mut self.$($field)+.wording_version, version, "the wording version")?; Ok(self)
        }
        /// Declare the typed selected item without projecting or coercing it.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_item_schema(mut self, schema: InputDeclaration) -> Result<Self, Error> {
            super::question::once(&mut self.$($field)+.item_schema, schema, "item_schema")?; Ok(self)
        }
        /// Declare explicit per-item context; shared context remains plain text.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_context_schema(mut self, schema: InputDeclaration) -> Result<Self, Error> {
            super::question::once(&mut self.$($field)+.context_schema, schema, "context_schema")?; Ok(self)
        }
    };
}
pub(super) use accessors;
impl Question {
    accessors!(metadata);
}

macro_rules! wrappers {
    () => {
        /// Optional exact author name.
        #[must_use]
        pub fn name(&self) -> Option<&QuestionName> {
            self.0.name()
        }
        /// Optional author wording version.
        #[must_use]
        pub fn wording_version(&self) -> Option<WordingVersion> {
            self.0.wording_version()
        }
        /// Optional typed item declaration.
        #[must_use]
        pub fn item_schema(&self) -> Option<&InputDeclaration> {
            self.0.item_schema()
        }
        /// Optional explicit per-item context declaration.
        #[must_use]
        pub fn context_schema(&self) -> Option<&InputDeclaration> {
            self.0.context_schema()
        }
        /// Attach a validated author name.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_name(mut self, value: QuestionName) -> Result<Self, Error> {
            self.0 = self.0.with_name(value)?;
            Ok(self)
        }
        /// Attach a validated wording version.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_wording_version(mut self, value: WordingVersion) -> Result<Self, Error> {
            self.0 = self.0.with_wording_version(value)?;
            Ok(self)
        }
        /// Attach an item declaration.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_item_schema(mut self, value: InputDeclaration) -> Result<Self, Error> {
            self.0 = self.0.with_item_schema(value)?;
            Ok(self)
        }
        /// Attach a per-item context declaration.
        /// # Errors
        /// Returns Usage when already supplied.
        pub fn with_context_schema(mut self, value: InputDeclaration) -> Result<Self, Error> {
            self.0 = self.0.with_context_schema(value)?;
            Ok(self)
        }
    };
}
impl BandedQuestion {
    wrappers!();
}
impl<C: Choice> ChooseQuestion<C> {
    wrappers!();
}
impl<C: Choice> TagQuestion<C> {
    wrappers!();
}

impl QuestionMetadata {
    pub(crate) fn validate_item(&self, input: &super::QuestionInput) -> Result<(), Error> {
        let Some(schema) = &self.item_schema else {
            return Ok(());
        };
        let valid = match input {
            super::QuestionInput::Text(text) => {
                schema.accepts(&crate::core::Json::String(text.clone()))
            }
            super::QuestionInput::Record(record) => schema.accepts(&record.value),
            super::QuestionInput::Images(images) => images
                .text()
                .is_some_and(|text| schema.accepts(&crate::core::Json::String(text.to_owned()))),
        };
        if valid {
            Ok(())
        } else {
            Err(Error::usage("the item does not match item_schema"))
        }
    }
    pub(crate) fn validate_context(
        &self,
        context: Option<&super::RecordContext>,
    ) -> Result<(), Error> {
        if let Some(context) = context {
            context.validate(self.context_schema.as_ref())?;
        }
        Ok(())
    }
}

impl super::Recognize {
    accessors!(0.metadata);
}
impl super::Relate {
    accessors!(0.metadata);
}
impl super::RecordChooseQuestion {
    accessors!(metadata);
}

impl super::LoadedQuestion {
    fn declared(&self) -> &QuestionMetadata {
        match self {
            Self::Question(question) => &question.metadata,
            Self::Banded(question) => &question.0.metadata,
        }
    }
    /// Optional author name, including a banded loaded question.
    #[must_use]
    pub fn name(&self) -> Option<&QuestionName> {
        self.declared().name.as_ref()
    }
    /// Optional author wording version.
    #[must_use]
    pub fn wording_version(&self) -> Option<WordingVersion> {
        self.declared().wording_version
    }
    /// Optional item declaration.
    #[must_use]
    pub fn item_schema(&self) -> Option<&InputDeclaration> {
        self.declared().item_schema.as_ref()
    }
    /// Optional explicit per-item context declaration.
    #[must_use]
    pub fn context_schema(&self) -> Option<&InputDeclaration> {
        self.declared().context_schema.as_ref()
    }
}
