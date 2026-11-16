//! Typed label domains project string wire rows without losing retained metadata.
use super::*;
use questions::{Choice, Choose, Rank, Score, Tag};
macro_rules! domain {
    ($name:ident, $answer:ty, $detail:ty, $read:ty, $owned:ty, $take:ident=>$into:expr, $value:ident => $view:expr, $labels:ident => $map:expr, $format:expr) => {
        impl<C: Choice> Function for questions::$name<C> {
            type Answer = $answer;
            type Owned = $owned;
            fn take($take: Self::Answer) -> Self::Owned {
                $into
            }
            type Detail = $detail;
            type Read<'a> = $read;
            const NAME: FunctionName = FunctionName::$name;
            fn read($value: &Self::Answer) -> Self::Read<'_> {
                $view
            }
            fn format(
                answer: &Self::Answer,
                out: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                ($format)(answer, out)
            }
        }
        impl<C: Choice> sealed::Sealed for questions::$name<C> {
            fn project(row: AnyRow, _: Projection) -> Result<Row<Self>, Error> {
                match row {
                    AnyRow::$name(row) => row.map_answer(|$labels| $map),
                    _ => Err(mismatch()),
                }
            }
            fn entry(
                entry: &AnnotationEntry,
                _: Projection,
            ) -> Result<Outcome<<Self as Function>::Answer, <Self as Function>::Detail>, Error>
            {
                let value = entry
                    .outcome_any()
                    .downcast_ref::<Outcome<<questions::$name as Function>::Answer, $detail>>()
                    .cloned()
                    .ok_or_else(mismatch)?;
                value.map_answer(|$labels| $map)
            }
        }
    };
}
// The concrete detail type is supplied directly; referring to the default instantiation
// while implementing it would make associated-type resolution recursive.

domain!(Choose, C, LabelDetail, C::Read<'a>, C, owned=>owned, value=>value.read(), labels=>C::from_label(labels),
    |answer: &C, out: &mut std::fmt::Formatter<'_>| std::write!(out, "{}", super::answer::Leaf(answer.label())));
domain!(Tag, Vec<C>, LabelDetail, &'a [C], Vec<C>, owned=>owned, value=>value.as_slice(), labels=>labels.into_iter().map(C::from_label).collect(),
|answer: &Vec<C>, out: &mut std::fmt::Formatter<'_>| {
    for (index, member) in answer.iter().enumerate() {
        if index != 0 { out.write_str("\t")?; }
        std::write!(out, "{}", super::answer::Leaf(member.label()))?;
    }
    Ok(())
});
domain!(Score, f64, LevelDetail, f64, f64, owned=>owned, value=>*value, labels=>Ok(labels),
    |answer: &f64, out: &mut std::fmt::Formatter<'_>| std::fmt::Display::fmt(answer,out));
domain!(Rank, std::num::NonZeroUsize, LevelDetail, usize, usize, owned=>owned.get(), value=>value.get(), labels=>Ok(labels),
    |answer: &std::num::NonZeroUsize, out: &mut std::fmt::Formatter<'_>| std::fmt::Display::fmt(answer,out));
macro_rules! decode {
    ($name:ident, $value:ident => $answer:expr, $detail:ident => $details:expr) => {
        impl<C: Choice> Decode for $name<C> {
            fn answer($value: crate::core::Answer) -> Result<Self::Answer, Error> {
                $answer
            }
            fn detail($detail: crate::core::Detail) -> Result<Option<Self::Detail>, Error> {
                $details
            }
        }
    };
}
decode!(Choose, value=>match value { crate::core::Answer::Label(label)=>C::from_label(label), _=>Err(mismatch()) }, detail=>labels(Some(detail)));
decode!(Tag, value=>match value { crate::core::Answer::Labels(labels)=>labels.into_iter().map(C::from_label).collect(), _=>Err(mismatch()) }, detail=>labels(Some(detail)));
decode!(Score, value=>match value { crate::core::Answer::Number(value)=>Ok(value), _=>Err(mismatch()) }, detail=>levels(Some(detail)));
decode!(Rank, value=>match value { crate::core::Answer::Position(value)=>std::num::NonZeroUsize::new(value).ok_or_else(mismatch), _=>Err(mismatch()) }, detail=>levels(Some(detail)));
