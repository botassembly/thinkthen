//! Mechanical serde visitor for auxiliary native structs, never a JSON encoding.
use serde::{Serialize, ser};
use thinkthen::RequestValueTree as Tree;
use std::collections::BTreeMap;
#[derive(Debug)]
pub(super) struct Failure;
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("native value cannot be represented") }
}
impl std::error::Error for Failure {}
impl ser::Error for Failure {
    fn custom<T: std::fmt::Display>(_: T) -> Self { Self }
}
pub(super) fn tree<T: Serialize + ?Sized>(value: &T) -> Result<Tree, Failure> { value.serialize(Encoder) }
struct Encoder;
struct Sequence(Vec<Tree>);
struct Mapping { values: BTreeMap<String, Tree>, key: Option<String> }
impl ser::Serializer for Encoder {
    type Ok = Tree;
    type Error = Failure;
    type SerializeSeq = Sequence;
    type SerializeTuple = Sequence;
    type SerializeTupleStruct = Sequence;
    type SerializeTupleVariant = ser::Impossible<Tree, Failure>;
    type SerializeMap = Mapping;
    type SerializeStruct = Mapping;
    type SerializeStructVariant = ser::Impossible<Tree, Failure>;
    fn serialize_i8(self, value: i8) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_i16(self, value: i16) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_i32(self, value: i32) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_i64(self, value: i64) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_i128(self, value: i128) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_u8(self, value: u8) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_u16(self, value: u16) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_u32(self, value: u32) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_u64(self, value: u64) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_u128(self, value: u128) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_f32(self, value: f32) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_f64(self, value: f64) -> Result<Tree, Failure> { Ok(Tree::Number(value.to_string())) }
    fn serialize_bool(self, value: bool) -> Result<Tree, Failure> { Ok(Tree::Bool(value)) }
    fn serialize_str(self, value: &str) -> Result<Tree, Failure> { Ok(Tree::String(value.into())) }
    fn serialize_char(self, value: char) -> Result<Tree, Failure> { self.serialize_str(&value.to_string()) }
    fn serialize_bytes(self, value: &[u8]) -> Result<Tree, Failure> { Ok(Tree::List(value.iter().map(|v| Tree::Number(v.to_string())).collect())) }
    fn serialize_none(self) -> Result<Tree, Failure> { Ok(Tree::Null) }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Tree, Failure> { tree(value) }
    fn serialize_unit(self) -> Result<Tree, Failure> { Ok(Tree::Null) }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Tree, Failure> { Ok(Tree::Null) }
    fn serialize_unit_variant(self, _: &'static str, _: u32, value: &'static str) -> Result<Tree, Failure> { Ok(Tree::String(value.into())) }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, value: &T) -> Result<Tree, Failure> { tree(value) }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(self, _: &'static str, _: u32, name: &'static str, value: &T) -> Result<Tree, Failure> { Ok(Tree::Map(BTreeMap::from([(name.into(), tree(value)?)]))) }
    fn serialize_seq(self, _: Option<usize>) -> Result<Sequence, Failure> { Ok(Sequence(Vec::new())) }
    fn serialize_tuple(self, _: usize) -> Result<Sequence, Failure> { Ok(Sequence(Vec::new())) }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Sequence, Failure> { Ok(Sequence(Vec::new())) }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Failure> { Err(Failure) }
    fn serialize_map(self, _: Option<usize>) -> Result<Mapping, Failure> { Ok(Mapping { values: BTreeMap::new(), key: None }) }
    fn serialize_struct(self, name: &'static str, _: usize) -> Result<Mapping, Failure> {
        if name == "$serde_json::private::RawValue" { return Err(Failure); }
        self.serialize_map(None)
    }
    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Failure> { Err(Failure) }
}
impl ser::SerializeSeq for Sequence {
    type Ok = Tree;
    type Error = Failure;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Failure> { self.0.push(tree(value)?); Ok(()) }
    fn end(self) -> Result<Tree, Failure> { Ok(Tree::List(self.0)) }
}
impl ser::SerializeTuple for Sequence {
    type Ok = Tree;
    type Error = Failure;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Failure> { self.0.push(tree(value)?); Ok(()) }
    fn end(self) -> Result<Tree, Failure> { Ok(Tree::List(self.0)) }
}
impl ser::SerializeTupleStruct for Sequence {
    type Ok = Tree;
    type Error = Failure;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Failure> { self.0.push(tree(value)?); Ok(()) }
    fn end(self) -> Result<Tree, Failure> { Ok(Tree::List(self.0)) }
}
impl ser::SerializeMap for Mapping {
    type Ok = Tree;
    type Error = Failure;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Failure> {
        let Tree::String(key) = tree(value)? else { return Err(Failure); };
        self.key = Some(key); Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Failure> {
        self.values.insert(self.key.take().ok_or(Failure)?, tree(value)?); Ok(())
    }
    fn end(self) -> Result<Tree, Failure> { Ok(Tree::Map(self.values)) }
}
impl ser::SerializeStruct for Mapping {
    type Ok = Tree;
    type Error = Failure;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Failure> {
        self.values.insert(key.into(), tree(value)?); Ok(())
    }
    fn end(self) -> Result<Tree, Failure> { Ok(Tree::Map(self.values)) }
}
