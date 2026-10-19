//! Command-owned selection handles over read-only native ordered JSON.
use serde::{Serialize, Serializer};
use serde_json::Number;
use std::{fmt, sync::OnceLock};
use thinkthen::{Error, ParsedJson, QuestionContent};

/// A saved selection owns its shared parse, so selected answers outlive input lists.
/// Containers create child handles only when inspected; they never copy a parsed tree.
pub(crate) struct Json {
    source: Option<ParsedJson>,
    path: Vec<Step>,
    kind: OnceLock<Kind>,
}
#[derive(Clone)]
enum Step { Field(String), Element(usize) }

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Kind {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

pub(crate) static NULL: Json = Json { source: None, path: Vec::new(), kind: OnceLock::new() };

impl Json {
    pub(crate) fn parse(text: &str) -> Result<Self, Error> {
        Ok(Self { source: Some(ParsedJson::from_json(text)?), path: Vec::new(), kind: OnceLock::new() })
    }
    pub(crate) fn made(kind: Kind) -> Self {
        Self { source: None, path: Vec::new(), kind: OnceLock::from(kind) }
    }
    fn content(&self) -> Option<QuestionContent<'_>> {
        let mut held = self.source.as_ref()?.content();
        for step in &self.path {
            held = match step {
                Step::Field(name) => held.member(name)?,
                Step::Element(at) => held.element(*at)?,
            };
        }
        Some(held)
    }
    fn child(&self, step: Step) -> Self {
        let mut path = self.path.clone();
        path.push(step);
        Self { source: self.source.clone(), path, kind: OnceLock::new() }
    }
    pub(crate) fn kind(&self) -> &Kind {
        self.kind.get_or_init(|| {
            let Some(held) = self.content() else { return Kind::Null };
            if let Some(fields) = held.fields() {
                return Kind::Object(fields.map(|(name, _)| (name.to_owned(), self.child(Step::Field(name.to_owned())))).collect());
            }
            if let Some(elements) = held.elements() {
                return Kind::Array(elements.enumerate().map(|(at, _)| self.child(Step::Element(at))).collect());
            }
            if let Some(text) = held.text() { return Kind::String(text.to_owned()) }
            if let Some(number) = held.number() { return Kind::Number(number.clone()) }
            if let Some(boolean) = held.boolean() { return Kind::Bool(boolean) }
            Kind::Null
        })
    }
    pub(crate) fn member(&self, name: &str) -> Option<&Self> {
        let Kind::Object(fields) = self.kind() else { return None };
        fields.iter().find_map(|(key, value)| (key == name).then_some(value))
    }
    pub(crate) fn element(&self, at: usize) -> Option<&Self> {
        let Kind::Array(elements) = self.kind() else { return None };
        elements.get(at)
    }
    pub(crate) fn as_str(&self) -> Option<&str> {
        let Kind::String(text) = self.kind() else { return None };
        Some(text)
    }
    pub(crate) fn is_null(&self) -> bool { matches!(self.kind(), Kind::Null) }
}
impl Clone for Json {
    fn clone(&self) -> Self {
        if self.source.is_some() {
            Self { source: self.source.clone(), path: self.path.clone(), kind: OnceLock::new() }
        } else {
            Self::made(self.kind().clone())
        }
    }
}
impl PartialEq for Json {
    fn eq(&self, other: &Self) -> bool { self.kind() == other.kind() }
}
impl fmt::Debug for Json {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { formatter.write_str("SavedJson(<withheld>)") }
}
impl Serialize for Json {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(content) = self.content() { return content.serialize(serializer) }
        match self.kind() {
            Kind::Null => serializer.serialize_none(),
            Kind::Bool(value) => value.serialize(serializer),
            Kind::Number(value) => value.serialize(serializer),
            Kind::String(value) => value.serialize(serializer),
            Kind::Array(value) => value.serialize(serializer),
            Kind::Object(value) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(value.len()))?;
                for (name, held) in value { map.serialize_entry(name, held)?; }
                map.end()
            }
        }
    }
}

/// Command-owned selector, admitted by the public native pointer grammar.
#[derive(Debug)]
pub(crate) struct Pointer(String);
impl Pointer {
    pub(crate) fn as_str(&self) -> &str { &self.0 }
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, Error> {
        let text = text.into();
        thinkthen::validate_pointer(&text)?;
        Ok(Self(text))
    }
    pub(crate) fn resolve<'a>(&self, value: &'a Json) -> Option<&'a Json> {
        // Native resolution owns index and traversal admission. The command only
        // locates an owning handle for the already-admitted borrowed selection.
        value.content()?.pointer(&self.0).ok()??;
        let mut found = value;
        for raw in self.0.split('/').skip(1) {
            let name = raw.replace("~1", "/").replace("~0", "~");
            found = match found.kind() {
                Kind::Object(_) => found.member(&name)?,
                Kind::Array(_) => found.element(name.parse().ok()?)?,
                _ => return None,
            };
        }
        Some(found)
    }
}
