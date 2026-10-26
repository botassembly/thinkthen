//! Typed structured answers and detail use the same names on both backends.
use super::*;
use std::collections::BTreeMap;

/// The absence of function detail, represented by an omitted field.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoDetail {}
impl schemars::JsonSchema for NoDetail {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NoDetail".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        false.into()
    }
}

macro_rules! value {
    ($name:ident, $doc:literal, {$($(#[$field_attr:meta])* $field:ident:$ty:ty => $field_doc:literal),+ $(,)?}) => {
        #[doc=$doc]
        #[non_exhaustive]
        #[derive(Debug,Clone,Serialize,Deserialize,schemars::JsonSchema)]
        #[serde(deny_unknown_fields)]
        pub struct $name {$($(#[$field_attr])* pub(crate) $field:$ty,)+}
        impl $name {$ (
            #[doc=$field_doc]
            #[must_use]
            pub const fn $field(&self)->&$ty{&self.$field}
        )+}
    };
}
value!(Found,"A selected span of the original evidence.",{text:String=>"The original selected text.",start:usize=>"The span start in the selected offset unit.",end:usize=>"The span end in the selected offset unit."});
value!(Name,"A recognized name and its typed support probability.",{text:String=>"The original name text.",start:usize=>"The span start.",end:usize=>"The span end.",kind:String=>"The selected kind.",probability:Probability=>"The name support probability."});
value!(RelationName,"One relation endpoint.",{text:String=>"The endpoint text.",start:usize=>"The span start.",end:usize=>"The span end.",kind:String=>"The endpoint kind."});
value!(Relation,"One allowed relation and its probability.",{source:RelationName=>"The source endpoint.",target:RelationName=>"The target endpoint.",relation:String=>"The rule name.",probability:Probability=>"The probability that the relation is stated.",#[serde(default,skip_serializing_if="std::ops::Not::not")]either:bool=>"Whether the rule is unordered."});
value!(PieceDetail,"Boundary probabilities for one original piece.",{text:String=>"The original piece.",start:usize=>"The span start.",end:usize=>"The span end.",tags:BTreeMap<String,Probability> =>"The boundary tag probabilities."});
value!(KindDetail,"Kind probabilities for one proposed name.",{start:usize=>"The span start.",end:usize=>"The span end.",kinds:BTreeMap<String,Probability> =>"The kind probabilities."});
value!(RecognizeDetail,"Complete typed name-recognition detail.",{pieces:Vec<PieceDetail> =>"The original pieces and boundary probabilities.",names:Vec<KindDetail> =>"The proposed names and kind probabilities."});
value!(LabelDetail,"Probability for every named label.",{labels:BTreeMap<String,Probability> =>"The probability for each label."});
value!(LevelDetail,"Probabilities in the question's level order.",{levels:Vec<Probability> =>"The ordered level probabilities."});
value!(PairDetail,"Recognized names and complete probabilities for allowed relation pairs.",{names:Vec<Name> =>"Every recognized name, including names without accepted relations.",pairs:Vec<Relation> =>"The evaluated relation pairs."});

impl From<crate::core::Found> for Found {
    fn from(v: crate::core::Found) -> Self {
        Self {
            text: v.text,
            start: v.start,
            end: v.end,
        }
    }
}
impl From<crate::core::Name> for Name {
    fn from(v: crate::core::Name) -> Self {
        Self {
            text: v.text,
            start: v.start,
            end: v.end,
            kind: v.kind,
            probability: v.p,
        }
    }
}
impl From<crate::core::RelationName> for RelationName {
    fn from(v: crate::core::RelationName) -> Self {
        Self {
            text: v.text,
            start: v.start,
            end: v.end,
            kind: v.kind,
        }
    }
}
impl From<crate::core::Relation> for Relation {
    fn from(v: crate::core::Relation) -> Self {
        Self {
            source: v.source.into(),
            target: v.target.into(),
            relation: v.relation,
            probability: v.p,
            either: v.either,
        }
    }
}
impl From<crate::core::RecognizeDetail> for RecognizeDetail {
    fn from(v: crate::core::RecognizeDetail) -> Self {
        Self {
            pieces: v
                .pieces
                .into_iter()
                .map(|p| PieceDetail {
                    text: p.text,
                    start: p.start,
                    end: p.end,
                    tags: p.tags,
                })
                .collect(),
            names: v
                .names
                .into_iter()
                .map(|p| KindDetail {
                    start: p.start,
                    end: p.end,
                    kinds: p.kinds,
                })
                .collect(),
        }
    }
}
