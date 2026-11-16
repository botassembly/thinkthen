//! Each function defines its answer and detail exactly once.
use super::*;
use crate::core::Detail;

macro_rules! implementation {
    ($name:ident,$answer:ty,$detail:ty,$read:ty,$borrow:ident=>$view:expr,$a:ident=>$decode:expr,$d:ident=>$decode_detail:expr) => {
        impl Function for questions::$name {
            type Answer = $answer;
            type Owned = $answer;
            fn take(answer: Self::Answer) -> Self::Owned {
                answer
            }
            type Detail = $detail;
            type Read<'a> = $read;
            fn read($borrow: &Self::Answer) -> Self::Read<'_> {
                $view
            }
            fn format(
                answer: &Self::Answer,
                out: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                sealed::Text::text(answer, out)
            }
            fn answer_failure(answer: &Self::Answer) -> Option<&Error> {
                sealed::Failures::failure(answer)
            }
            const NAME: FunctionName = FunctionName::$name;
        }
        impl sealed::Sealed for questions::$name {
            fn project(row: AnyRow, _: Projection) -> Result<Row<Self>, Error> {
                match row {
                    AnyRow::$name(row) => Ok(row),
                    _ => Err(mismatch()),
                }
            }
        }
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
    (Decide) => {crate::core::results::functions::implementation!(Decide,bool,NoDetail,bool,v=>*v,a=>match a{crate::core::Answer::Bool(v)=>Ok(v),_=>Err(mismatch())},d=>{let _=d;Ok(None)});};
    (Filter) => {crate::core::results::functions::implementation!(Filter,bool,NoDetail,bool,v=>*v,a=>match a{crate::core::Answer::Bool(v)=>Ok(v),_=>Err(mismatch())},d=>{let _=d;Ok(None)});};
    (Choose) => {};
    (Tag) => {};
    (Score) => {};
    (Rank) => {};
    (Find) => {crate::core::results::functions::implementation!(Find,Option<Span>,NoDetail,Option<&'a Span>,v=>v.as_ref(),a=>match a{crate::core::Answer::Span(v)=>Ok(v.map(Into::into)),_=>Err(mismatch())},d=>{let _=d;Ok(None)});};
    (Recognize) => {crate::core::results::functions::implementation!(Recognize,Vec<Name>,RecognizeDetail,&'a [Name],v=>v.as_slice(),a=>match a{crate::core::Answer::Names(v)=>Ok(v.into_iter().map(Into::into).collect()),_=>Err(mismatch())},d=>match d{Some(crate::core::Detail::Recognize(v))=>Ok(Some(v.into())),None=>Ok(None),_=>Err(mismatch())});};
    (Relate) => {crate::core::results::functions::implementation!(Relate,Vec<Relation>,PairDetail,&'a [Relation],v=>v.as_slice(),a=>match a{crate::core::Answer::Relations(v)=>Ok(v.into_iter().map(Into::into).collect()),_=>Err(mismatch())},d=>match d{Some(crate::core::Detail::Relate(v))=>Ok(Some(PairDetail{names:v.names.into_iter().map(Into::into).collect(),pairs:v.pairs.into_iter().map(Into::into).collect()})),None=>Ok(None),_=>Err(mismatch())});};
    (Annotate) => {crate::core::results::functions::implementation!(Annotate,Annotations,NoDetail,&'a Annotations,v=>v,a=>{let _=a;Err(mismatch())},d=>{let _=d;Ok(None)});};
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

macro_rules! single {
    (Choose) => {
        impl<C: questions::Choice> SingleFunction for questions::Choose<C> {}
    };
    (Tag) => {
        impl<C: questions::Choice> SingleFunction for questions::Tag<C> {}
    };
    (Score) => {
        impl<C: questions::Choice> SingleFunction for questions::Score<C> {}
    };
    ($name:ident) => {
        impl SingleFunction for questions::$name {}
    };
}
pub(crate) use single;
