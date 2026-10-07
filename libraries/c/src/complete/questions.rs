//! Readable questions retain native authored content and resolved reading rules.
use crate::current::Storage;
use crate::failures::Failure;
use crate::ffi::carriers::{
    ChoiceV1, ChoicesV1, ContentV1, OptionalContentV1, OptionalQuestionV1, OptionalRuleV1,
    OptionalSizeV1, QuestionViewV1, RelationV1, RelationsV1, RuleV1, StringsV1,
};
use thinkthen::{QuestionContent, ResolvedOption, ResolvedQuestion, ResolvedThreshold};
pub(super) fn rule(value: Option<ResolvedThreshold>) -> RuleV1 {
    match value {
        None => RuleV1 {
            kind: 1,
            ..RuleV1::default()
        },
        Some(ResolvedThreshold::Cut(low)) => RuleV1 {
            kind: 2,
            low,
            high: 0.0,
        },
        Some(ResolvedThreshold::Band { low, high }) => RuleV1 { kind: 3, low, high },
    }
}
pub(super) fn threshold(value: Option<ResolvedThreshold>) -> OptionalRuleV1 {
    OptionalRuleV1 {
        present: 1,
        value: rule(value),
    }
}
impl Storage {
    pub(super) fn native_content(
        &mut self,
        value: QuestionContent<'_>,
    ) -> Result<ContentV1, Failure> {
        Ok(match value.text() {
            Some(text) => ContentV1 {
                kind: 1,
                data: self.string(text),
            },
            None => ContentV1 {
                kind: 2,
                data: self.string(&value.to_json()?),
            },
        })
    }
    pub(super) fn optional_native_content(
        &mut self,
        value: Option<QuestionContent<'_>>,
    ) -> Result<OptionalContentV1, Failure> {
        value
            .map(|value| {
                Ok(OptionalContentV1 {
                    present: 1,
                    value: self.native_content(value)?,
                })
            })
            .transpose()
            .map(Option::unwrap_or_default)
    }
    pub(super) fn native_choices<'a>(
        &mut self,
        values: impl Iterator<Item = ResolvedOption<'a>>,
    ) -> Result<ChoicesV1, Failure> {
        let values = values
            .map(|v| {
                Ok(ChoiceV1 {
                    name: self.string(v.name()),
                    description: self.optional_native_content(v.description())?,
                    ..ChoiceV1::default()
                })
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        let (data, len) = self.array(values);
        Ok(ChoicesV1 { data, len })
    }
    pub(super) fn native_on<'a>(&mut self, names: impl Iterator<Item = &'a str>) -> StringsV1 {
        let names = names.map(|name| self.string(name)).collect();
        let (data, len) = self.array(names);
        StringsV1 { data, len }
    }
    pub(super) fn resolved_question(
        &mut self,
        question: ResolvedQuestion<'_>,
        cut: Option<ResolvedThreshold>,
    ) -> Result<QuestionViewV1, Failure> {
        let batch = question.batch();
        Ok(QuestionViewV1 {
            model: self.optional_string(question.model()),
            profile: self.optional_string(question.profile()),
            on: self.native_on(question.on()),
            batch: OptionalSizeV1 {
                present: i32::from(batch.is_some()),
                value: match batch {
                    Some(thinkthen::BatchSetting::Records(count)) => count.get(),
                    _ => 0,
                },
            },
            batch_max: i32::from(matches!(batch, Some(thinkthen::BatchSetting::Max))),
            kind: match question.kind() {
                thinkthen::QuestionKind::Decide => 1,
                thinkthen::QuestionKind::Choose => 2,
                thinkthen::QuestionKind::Tag => 3,
                thinkthen::QuestionKind::Score => 4,
                thinkthen::QuestionKind::Rank => 6,
                thinkthen::QuestionKind::Find => 7,
            },
            text: self.native_content(question.text())?,
            yes: self.optional_native_content(question.yes())?,
            no: self.optional_native_content(question.no())?,
            choices: self.native_choices(question.options())?,
            threshold: rule(cut),
            ..QuestionViewV1::default()
        })
    }
    pub(super) fn primitive(
        &mut self,
        question: ResolvedQuestion<'_>,
        cut: Option<ResolvedThreshold>,
    ) -> Result<OptionalQuestionV1, Failure> {
        Ok(OptionalQuestionV1 {
            present: 1,
            value: self.resolved_question(question, cut)?,
        })
    }
    pub(super) fn native_relations<'a>(
        &mut self,
        values: impl Iterator<Item = thinkthen::ResolvedRelationRule<'a>>,
    ) -> RelationsV1 {
        let values = values
            .map(|r| RelationV1 {
                name: self.string(r.name()),
                source: self.string(r.source()),
                target: self.string(r.target()),
                reads: self.optional_string(Some(r.reads())),
                either: i32::from(r.either()),
                single: i32::from(r.single()),
            })
            .collect();
        let (data, len) = self.array(values);
        RelationsV1 { data, len }
    }
}
