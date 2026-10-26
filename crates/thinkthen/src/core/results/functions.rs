//! Each function defines its answer and detail exactly once.
use super::*;
use crate::core::Detail;

macro_rules! implementation {
    ($name:ident,$answer:ty,$detail:ty,$a:ident=>$decode:expr,$d:ident=>$decode_detail:expr) => {
        impl Function for questions::$name {
            type Answer = $answer;
            type Detail = $detail;
            const NAME: FunctionName = FunctionName::$name;
        }
        impl sealed::Sealed for questions::$name {}
        impl Decode for questions::$name {
            fn answer($a: crate::core::Answer) -> Result<$answer, Error> {
                $decode
            }
            fn detail($d: crate::core::Detail) -> Result<Option<$detail>, Error> {
                let $d = Some($d);
                $decode_detail
            }
        }
    };
}
macro_rules! define_function {
    (Decide) => {crate::core::results::functions::implementation!(Decide,bool,NoDetail,a=>match a{crate::core::Answer::Bool(v)=>Ok(v),_=>Err(mismatch())},d=>{let _=d;Ok(None)});};
    (Filter) => {crate::core::results::functions::implementation!(Filter,bool,NoDetail,a=>match a{crate::core::Answer::Bool(v)=>Ok(v),_=>Err(mismatch())},d=>{let _=d;Ok(None)});};
    (Choose) => {crate::core::results::functions::implementation!(Choose,String,LabelDetail,a=>match a{crate::core::Answer::Label(v)=>Ok(v),_=>Err(mismatch())},d=>labels(d));};
    (Tag) => {crate::core::results::functions::implementation!(Tag,Vec<String>,LabelDetail,a=>match a{crate::core::Answer::Labels(v)=>Ok(v),_=>Err(mismatch())},d=>labels(d));};
    (Score) => {crate::core::results::functions::implementation!(Score,f64,LevelDetail,a=>match a{crate::core::Answer::Number(v)=>Ok(v),_=>Err(mismatch())},d=>levels(d));};
    (Rank) => {crate::core::results::functions::implementation!(Rank,std::num::NonZeroUsize,LevelDetail,a=>match a{crate::core::Answer::Position(v)=>std::num::NonZeroUsize::new(v).ok_or_else(mismatch),_=>Err(mismatch())},d=>levels(d));};
    (Find) => {crate::core::results::functions::implementation!(Find,Option<Found>,NoDetail,a=>match a{crate::core::Answer::Found(v)=>Ok(v.map(Into::into)),_=>Err(mismatch())},d=>{let _=d;Ok(None)});};
    (Recognize) => {crate::core::results::functions::implementation!(Recognize,Vec<Name>,RecognizeDetail,a=>match a{crate::core::Answer::Names(v)=>Ok(v.into_iter().map(Into::into).collect()),_=>Err(mismatch())},d=>match d{Some(crate::core::Detail::Recognize(v))=>Ok(Some(v.into())),None=>Ok(None),_=>Err(mismatch())});};
    (Relate) => {crate::core::results::functions::implementation!(Relate,Vec<Relation>,PairDetail,a=>match a{crate::core::Answer::Relations(v)=>Ok(v.into_iter().map(Into::into).collect()),_=>Err(mismatch())},d=>match d{Some(crate::core::Detail::Relate(v))=>Ok(Some(PairDetail{names:v.names.into_iter().map(Into::into).collect(),pairs:v.pairs.into_iter().map(Into::into).collect()})),None=>Ok(None),_=>Err(mismatch())});};
    (Annotate) => {crate::core::results::functions::implementation!(Annotate,std::collections::BTreeMap<String,AnnotationEntry>,NoDetail,a=>{let _=a;Err(mismatch())},d=>{let _=d;Ok(None)});};
}
pub(crate) use define_function;
pub(crate) use implementation;
pub(super) fn labels(value: Option<Detail>) -> Result<Option<LabelDetail>, Error> {
    match value {
        Some(Detail::Labels(labels)) => Ok(Some(LabelDetail { labels })),
        None => Ok(None),
        _ => Err(mismatch()),
    }
}
pub(super) fn levels(value: Option<Detail>) -> Result<Option<LevelDetail>, Error> {
    match value {
        Some(Detail::Levels(levels)) => Ok(Some(LevelDetail { levels })),
        None => Ok(None),
        _ => Err(mismatch()),
    }
}
