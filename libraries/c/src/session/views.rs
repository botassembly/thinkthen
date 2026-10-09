//! Primitive transport projection. All nested pointers belong to one packet owner.
#![allow(
    non_camel_case_types,
    missing_docs,
    reason = "immutable C buffers and tagged primitive views"
)]
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use std::any::Any;
use std::ffi::c_char;
use thinkthen::ErrorKind;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_utf8_v1 {
    pub data: *const c_char,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_extension_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub json: thinkthen_complete_utf8_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_extensions_v1 {
    pub data: *const thinkthen_complete_extension_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_entry_v1 {
    pub name: thinkthen_complete_utf8_v1,
    pub value: *const thinkthen_complete_json_v1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_array_v1 {
    pub data: *const *const thinkthen_complete_json_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_object_v1 {
    pub data: *const thinkthen_complete_json_entry_v1,
    pub len: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_json_data_v1 {
    pub boolean: u32,
    pub number: thinkthen_complete_utf8_v1,
    pub string: thinkthen_complete_utf8_v1,
    pub array: thinkthen_complete_json_array_v1,
    pub object: thinkthen_complete_json_object_v1,
}
impl std::fmt::Debug for thinkthen_complete_json_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("JSON payload")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_v1 {
    pub kind: u32,
    pub data: thinkthen_complete_json_data_v1,
}
pub const THINKTHEN_COMPLETE_JSON_NULL_V1: u32 = 1;
pub const THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: u32 = 2;
pub const THINKTHEN_COMPLETE_JSON_NUMBER_V1: u32 = 3;
pub const THINKTHEN_COMPLETE_JSON_STRING_V1: u32 = 4;
pub const THINKTHEN_COMPLETE_JSON_ARRAY_V1: u32 = 5;
pub const THINKTHEN_COMPLETE_JSON_OBJECT_V1: u32 = 6;

#[derive(Default)]
pub(crate) struct Storage {
    allocations: Vec<Box<dyn Any>>,
}
impl std::fmt::Debug for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("owned view backing")
    }
}
impl Storage {
    pub(crate) fn hold<T: 'static>(&mut self, value: T) -> *const T {
        let value = Box::new(value);
        let pointer = std::ptr::from_ref(value.as_ref());
        self.allocations.push(value);
        pointer
    }
    pub(crate) fn slice<T: 'static>(&mut self, values: Vec<T>) -> (*const T, usize) {
        let values = values.into_boxed_slice();
        let len = values.len();
        if len == 0 {
            return (std::ptr::null(), 0);
        }
        let data = values.as_ptr();
        self.allocations.push(Box::new(values));
        (data, len)
    }
    pub(crate) fn text(&mut self, value: &str) -> thinkthen_complete_utf8_v1 {
        let (data, len) = self.slice(value.as_bytes().to_vec());
        thinkthen_complete_utf8_v1 {
            data: data.cast(),
            len,
        }
    }
    pub(crate) fn extensions(
        &mut self,
        node: &Node,
        known: &[&str],
    ) -> Result<thinkthen_complete_extensions_v1, ErrorKind> {
        let mut values = Vec::new();
        for (name, node) in node.object()? {
            if !known.contains(&name.as_str()) {
                values.push(thinkthen_complete_extension_v1 {
                    name: self.text(name),
                    json: self.text(node.raw.get()),
                });
            }
        }
        let (data, len) = self.slice(values);
        Ok(thinkthen_complete_extensions_v1 { data, len })
    }
}

/// Presence belongs to result conversion, independently of scalar conversion.
pub(crate) fn read_presence<T>(
    node: Option<&Node>,
    required: bool,
    empty: T,
    read: impl FnOnce(&Node, &mut Storage) -> Result<T, ErrorKind>,
    store: &mut Storage,
) -> Result<(u32, T), ErrorKind> {
    use super::views_generated::{
        THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, THINKTHEN_COMPLETE_PRESENCE_NULL_V1,
        THINKTHEN_COMPLETE_PRESENCE_VALUE_V1,
    };
    match node {
        None if required => Err(ErrorKind::Defect),
        None => Ok((THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, empty)),
        Some(node) if node.kind() == "null" => Ok((THINKTHEN_COMPLETE_PRESENCE_NULL_V1, empty)),
        Some(node) => Ok((THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, read(node, store)?)),
    }
}

#[derive(Debug)]
enum Kind {
    Null,
    Boolean(bool),
    Number,
    String(String),
    Array(Vec<Node>),
    Object(Vec<(String, Node)>),
}
#[derive(Debug)]
pub(crate) struct Node {
    raw: Box<RawValue>,
    value: Kind,
}
struct Entries(Vec<(String, Box<RawValue>)>);
struct Ordered;
impl<'de> Visitor<'de> for Ordered {
    type Value = Entries;
    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("JSON object")
    }
    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Entries, M::Error> {
        let mut entries = Vec::new();
        while let Some(entry) = map.next_entry()? {
            entries.push(entry);
        }
        Ok(Entries(entries))
    }
}
impl<'de> Deserialize<'de> for Entries {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Ordered)
    }
}
impl Node {
    pub(crate) fn parse(text: &str) -> Result<Self, ErrorKind> {
        let raw = serde_json::from_str(text).map_err(|_| ErrorKind::Defect)?;
        Self::from_raw(raw)
    }
    fn from_raw(raw: Box<RawValue>) -> Result<Self, ErrorKind> {
        let text = raw.get();
        let value = match text.as_bytes().first() {
            Some(b'n') => Kind::Null,
            Some(b't' | b'f') => {
                Kind::Boolean(serde_json::from_str(text).map_err(|_| ErrorKind::Defect)?)
            }
            Some(b'"') => Kind::String(serde_json::from_str(text).map_err(|_| ErrorKind::Defect)?),
            Some(b'[') => {
                let items: Vec<Box<RawValue>> =
                    serde_json::from_str(text).map_err(|_| ErrorKind::Defect)?;
                Kind::Array(
                    items
                        .into_iter()
                        .map(Self::from_raw)
                        .collect::<Result<_, _>>()?,
                )
            }
            Some(b'{') => {
                let Entries(items) = serde_json::from_str(text).map_err(|_| ErrorKind::Defect)?;
                Kind::Object(
                    items
                        .into_iter()
                        .map(|(key, value)| Ok((key, Self::from_raw(value)?)))
                        .collect::<Result<_, ErrorKind>>()?,
                )
            }
            Some(_) => Kind::Number,
            None => return Err(ErrorKind::Defect),
        };
        Ok(Self { raw, value })
    }
    pub(crate) fn kind(&self) -> &'static str {
        match self.value {
            Kind::Null => "null",
            Kind::Boolean(_) => "boolean",
            Kind::Number => "number",
            Kind::String(_) => "string",
            Kind::Array(_) => "array",
            Kind::Object(_) => "object",
        }
    }
    pub(crate) fn text(&self) -> Result<&str, ErrorKind> {
        if let Kind::String(value) = &self.value {
            Ok(value)
        } else {
            Err(ErrorKind::Defect)
        }
    }
    pub(crate) fn boolean(&self) -> Result<u32, ErrorKind> {
        if let Kind::Boolean(value) = self.value {
            Ok(u32::from(value))
        } else {
            Err(ErrorKind::Defect)
        }
    }
    pub(crate) fn number<T: std::str::FromStr>(&self) -> Result<T, ErrorKind> {
        if !matches!(self.value, Kind::Number) {
            return Err(ErrorKind::Defect);
        }
        self.raw.get().parse().map_err(|_| ErrorKind::Defect)
    }
    pub(crate) fn array(&self) -> Result<&[Node], ErrorKind> {
        if let Kind::Array(value) = &self.value {
            Ok(value)
        } else {
            Err(ErrorKind::Defect)
        }
    }
    pub(crate) fn object(&self) -> Result<&[(String, Node)], ErrorKind> {
        if let Kind::Object(value) = &self.value {
            Ok(value)
        } else {
            Err(ErrorKind::Defect)
        }
    }
    pub(crate) fn member(&self, name: &str) -> Option<&Node> {
        self.object()
            .ok()?
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value)
    }
    pub(crate) fn required(&self, name: &str) -> Result<&Node, ErrorKind> {
        self.member(name).ok_or(ErrorKind::Defect)
    }
    pub(crate) fn is_member(&self, name: &str, value: &str) -> bool {
        self.member(name).and_then(|node| node.text().ok()) == Some(value)
    }
}

pub(crate) fn read_json(
    node: &Node,
    store: &mut Storage,
) -> Result<*const thinkthen_complete_json_v1, ErrorKind> {
    let (kind, data) = match &node.value {
        Kind::Null => (
            THINKTHEN_COMPLETE_JSON_NULL_V1,
            thinkthen_complete_json_data_v1 { boolean: 0 },
        ),
        Kind::Boolean(value) => (
            THINKTHEN_COMPLETE_JSON_BOOLEAN_V1,
            thinkthen_complete_json_data_v1 {
                boolean: u32::from(*value),
            },
        ),
        Kind::Number => (
            THINKTHEN_COMPLETE_JSON_NUMBER_V1,
            thinkthen_complete_json_data_v1 {
                number: store.text(node.raw.get()),
            },
        ),
        Kind::String(value) => (
            THINKTHEN_COMPLETE_JSON_STRING_V1,
            thinkthen_complete_json_data_v1 {
                string: store.text(value),
            },
        ),
        Kind::Array(values) => {
            let mut copied = Vec::new();
            for value in values {
                copied.push(read_json(value, store)?);
            }
            let (data, len) = store.slice(copied);
            (
                THINKTHEN_COMPLETE_JSON_ARRAY_V1,
                thinkthen_complete_json_data_v1 {
                    array: thinkthen_complete_json_array_v1 { data, len },
                },
            )
        }
        Kind::Object(values) => {
            let mut copied = Vec::new();
            for (name, value) in values {
                copied.push(thinkthen_complete_json_entry_v1 {
                    name: store.text(name),
                    value: read_json(value, store)?,
                });
            }
            let (data, len) = store.slice(copied);
            (
                THINKTHEN_COMPLETE_JSON_OBJECT_V1,
                thinkthen_complete_json_data_v1 {
                    object: thinkthen_complete_json_object_v1 { data, len },
                },
            )
        }
    };
    Ok(store.hold(thinkthen_complete_json_v1 { kind, data }))
}

#[derive(Debug)]
pub(crate) struct Projection {
    _storage: Storage,
    pub(crate) root: *const super::views_generated::thinkthen_complete_session_packet_v1,
}
impl Projection {
    pub(crate) fn new(text: &str) -> Result<Self, ErrorKind> {
        let node = Node::parse(text)?;
        let mut storage = Storage::default();
        let packet =
            super::views_generated::read_thinkthen_complete_session_packet_v1(&node, &mut storage)?;
        let root = storage.hold(packet);
        Ok(Self {
            _storage: storage,
            root,
        })
    }
}

#[cfg(test)]
#[path = "../ffi/session/views/ffi.rs"]
mod tests;
