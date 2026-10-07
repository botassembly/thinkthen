//! Canonical view pieces backed only by values the native API actually exposes.
use super::{Content, Storage};
use crate::ffi::carriers::{
    DecideValueDataV1, DecideValueV1, EdgeV1, EdgesV1, EndpointV1, EntitiesV1, EntityEdgeV1,
    EntityEdgesV1, EntityV1, MemberValueDataV1, MemberValueV1, ObservedProbabilitiesDataV1,
    ObservedProbabilitiesV1, OptionalEntityEdgesV1, ProbabilitiesV1, ProbabilityV1,
    RecognizeValueV1, StringsV1,
};
use thinkthen::{Answer, Judgment, NamedProbability, Probabilities, Recognized, RecognizedEntity};
impl Storage {
    pub(crate) fn decision(&mut self, answer: Answer, authored: Option<&Content>) -> DecideValueV1 {
        let mut data = DecideValueDataV1::default();
        let kind = match answer {
            Answer::Unsure => 0,
            Answer::Yes | Answer::No => {
                if let Some(authored) = authored {
                    data.authored = self.content(authored);
                    2
                } else {
                    data.boolean = i32::from(answer == Answer::Yes);
                    1
                }
            }
        };
        DecideValueV1 { kind, data }
    }
    pub(crate) fn strings(&mut self, strings: &[String]) -> StringsV1 {
        let strings = strings.iter().map(|string| self.string(string)).collect();
        let (data, len) = self.array(strings);
        StringsV1 { data, len }
    }
    pub(crate) fn member_value(
        &mut self,
        value: &Judgment,
        authored: Option<&Content>,
    ) -> MemberValueV1 {
        let mut data = MemberValueDataV1::default();
        let kind = match value {
            Judgment::Decision(answer) => {
                data.decide = self.decision(*answer, authored);
                1
            }
            Judgment::Choice(choice) => {
                data.choose = self.optional_string(choice.as_deref());
                2
            }
            Judgment::Tags(tags) => {
                data.tag = self.strings(tags);
                3
            }
            Judgment::Score(score) => {
                data.score = *score;
                4
            }
        };
        MemberValueV1 { kind, data }
    }
    pub(crate) fn named_probabilities(&mut self, named: &[NamedProbability]) -> ProbabilitiesV1 {
        let named = named
            .iter()
            .map(|p| ProbabilityV1 {
                name: self.string(p.name()),
                probability: p.probability(),
            })
            .collect();
        let (data, len) = self.array(named);
        ProbabilitiesV1 { data, len }
    }
    pub(crate) fn observed_probabilities(
        &mut self,
        probabilities: &Probabilities,
    ) -> ObservedProbabilitiesV1 {
        let mut data = ObservedProbabilitiesDataV1::default();
        let kind = match probabilities {
            Probabilities::YesNo { yes } => {
                data.yes = *yes;
                1
            }
            Probabilities::Named(named) => {
                data.named = self.named_probabilities(named);
                2
            }
        };
        ObservedProbabilitiesV1 { kind, data }
    }
    pub(crate) fn entity(&mut self, entity: &RecognizedEntity) -> EntityV1 {
        EntityV1 {
            text: self.string(entity.text()),
            start: entity.start(),
            end: entity.end(),
            length: entity.length(),
            kind: self.string(entity.kind()),
            strength: entity.strength(),
        }
    }
    pub(crate) fn recognition_value(&mut self, recognized: &Recognized) -> RecognizeValueV1 {
        let entities = recognized
            .entities()
            .iter()
            .map(|e| self.entity(e))
            .collect();
        let (data, len) = self.array(entities);
        let relations = recognized
            .relations()
            .map(|edges| {
                let edges = edges
                    .iter()
                    .map(|edge| EntityEdgeV1 {
                        relation: self.string(edge.relation()),
                        source: self.entity(edge.source()),
                        target: self.entity(edge.target()),
                        probability: edge.probability(),
                        either: i32::from(edge.either()),
                    })
                    .collect();
                let (data, len) = self.array(edges);
                OptionalEntityEdgesV1 {
                    present: 1,
                    value: EntityEdgesV1 { data, len },
                }
            })
            .unwrap_or_default();
        RecognizeValueV1 {
            entities: EntitiesV1 { data, len },
            relations,
        }
    }
    pub(crate) fn edges(&mut self, edges: &[thinkthen::Edge]) -> EdgesV1 {
        let edges = edges
            .iter()
            .map(|edge| EdgeV1 {
                relation: self.string(edge.relation()),
                source: EndpointV1 {
                    name: self.string(edge.source().name()),
                    kind: self.string(edge.source().kind()),
                },
                target: EndpointV1 {
                    name: self.string(edge.target().name()),
                    kind: self.string(edge.target().kind()),
                },
                probability: edge.probability(),
                either: i32::from(edge.either()),
            })
            .collect();
        let (data, len) = self.array(edges);
        EdgesV1 { data, len }
    }
}
