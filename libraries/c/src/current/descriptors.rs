//! Owned authored descriptors; native code alone validates question semantics.
use super::{Choice, Content, QuestionHandle, Storage};
use crate::failures::Failure;
use crate::ffi::carriers::{
    ChoiceV1, ChoicesV1, OptionalContentV1, OptionalDoubleV1, OptionalSizeV1, QuestionMemberV1,
    QuestionMembersV1, QuestionViewV1, RelationV1, RelationsV1, RuleV1, StringsV1,
};
#[derive(Clone)]
pub(crate) struct Relation {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) reads: Option<String>,
    pub(crate) either: bool,
    pub(crate) single: bool,
}
#[derive(Clone)]
pub(crate) struct QuestionData {
    pub(crate) kind: u32,
    pub(crate) text: Option<Content>,
    pub(crate) yes: Option<Content>,
    pub(crate) no: Option<Content>,
    pub(crate) choices: Vec<Choice>,
    pub(crate) threshold: RuleV1,
    pub(crate) relation_threshold: RuleV1,
    pub(crate) model: Option<String>,
    pub(crate) profile: Option<String>,
    pub(crate) batch: Option<usize>,
    pub(crate) batch_max: bool,
    pub(crate) none: bool,
    pub(crate) on: Vec<String>,
    pub(crate) members: Vec<(String, QuestionHandle)>,
    pub(crate) kinds: Vec<Choice>,
    pub(crate) relations: Vec<Relation>,
    pub(crate) name_pointer: Option<String>,
    pub(crate) kind_pointer: Option<String>,
}
impl Storage {
    pub(crate) fn optional_content(&mut self, value: Option<&Content>) -> OptionalContentV1 {
        value.map_or_else(OptionalContentV1::default, |content| OptionalContentV1 {
            present: 1,
            value: self.content(content),
        })
    }
    pub(crate) fn choices(&mut self, choices: &[Choice]) -> ChoicesV1 {
        let choices = choices
            .iter()
            .map(|choice| ChoiceV1 {
                name: self.string(&choice.name),
                description: self.optional_content(choice.description.as_ref()),
                weight: OptionalDoubleV1 {
                    present: i32::from(choice.weight.is_some()),
                    value: choice.weight.unwrap_or(0.0),
                },
            })
            .collect();
        let (data, len) = self.array(choices);
        ChoicesV1 { data, len }
    }
    pub(crate) fn question(&mut self, q: &QuestionData) -> Result<QuestionViewV1, Failure> {
        let on = q.on.iter().map(|name| self.string(name)).collect();
        let (data, len) = self.array(on);
        let on = StringsV1 { data, len };
        let members = q
            .members
            .iter()
            .map(|(name, member)| {
                let description = member.descriptor.as_ref().ok_or_else(|| {
                    Failure::defect("native loaded question descriptor access is unavailable")
                })?;
                let view = self.question(description)?;
                let (question, _) = self.array(vec![view]);
                Ok(QuestionMemberV1 {
                    name: self.string(name),
                    question,
                })
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        let (data, len) = self.array(members);
        let members = QuestionMembersV1 { data, len };
        let relations = q
            .relations
            .iter()
            .map(|r| RelationV1 {
                name: self.string(&r.name),
                source: self.string(&r.source),
                target: self.string(&r.target),
                reads: self.optional_string(r.reads.as_deref()),
                either: i32::from(r.either),
                single: i32::from(r.single),
            })
            .collect();
        let (data, len) = self.array(relations);
        Ok(QuestionViewV1 {
            kind: q.kind,
            text: q
                .text
                .as_ref()
                .map_or_else(Default::default, |text| self.content(text)),
            yes: self.optional_content(q.yes.as_ref()),
            no: self.optional_content(q.no.as_ref()),
            choices: self.choices(&q.choices),
            threshold: q.threshold,
            relation_threshold: q.relation_threshold,
            model: self.optional_string(q.model.as_deref()),
            profile: self.optional_string(q.profile.as_deref()),
            batch: OptionalSizeV1 {
                present: i32::from(q.batch.is_some()),
                value: q.batch.unwrap_or(0),
            },
            batch_max: i32::from(q.batch_max),
            none: i32::from(q.none),
            on,
            members,
            kinds: self.choices(&q.kinds),
            relations: RelationsV1 { data, len },
            name_pointer: self.optional_string(q.name_pointer.as_deref()),
            kind_pointer: self.optional_string(q.kind_pointer.as_deref()),
        })
    }
    pub(crate) fn images(
        &mut self,
        images: &[super::ImageHandle],
    ) -> crate::ffi::carriers::OptionalImageViewsV1 {
        if images.is_empty() {
            return Default::default();
        }
        let views = images
            .iter()
            .map(|image| {
                let mut view = image.view();
                view.filename = self.optional_string(image.filename.as_deref());
                // Cloning retains the native immutable Arc bytes independently of source/image handles.
                self.0.push(Box::new(image.native.clone()));
                view
            })
            .collect();
        let (data, len) = self.array(views);
        crate::ffi::carriers::OptionalImageViewsV1 {
            present: 1,
            value: crate::ffi::carriers::ImageViewsV1 { data, len },
        }
    }
    pub(crate) fn position(
        &mut self,
        position: Option<&super::Position>,
    ) -> crate::ffi::carriers::OptionalLocationV1 {
        position.map_or_else(Default::default, |position| {
            crate::ffi::carriers::OptionalLocationV1 {
                present: 1,
                value: crate::ffi::carriers::LocationV1 {
                    file: self.optional_string(Some(&position.file)),
                    first_line: OptionalSizeV1 {
                        present: i32::from(position.first_line.is_some()),
                        value: position.first_line.unwrap_or(0),
                    },
                    last_line: OptionalSizeV1 {
                        present: i32::from(position.last_line.is_some()),
                        value: position.last_line.unwrap_or(0),
                    },
                },
            }
        })
    }
}
