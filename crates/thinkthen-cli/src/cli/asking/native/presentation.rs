//! Command views of complete native atomic results.
use crate::failure::Failure;
use crate::saved_json::{Json, Kind};
use crate::schedule::Outcome;
use crate::{
    CompleteChoice, CompleteDecision, CompleteFilter, CompleteRank, CompleteScore, CompleteTags,
    RawRecord, ResultMetadata,
};
use serde::{Serialize, Serializer};

pub(super) enum Atomic<'a> {
    Decision(&'a CompleteDecision),
    Choice(&'a CompleteChoice),
    Tags(&'a CompleteTags),
    Score(&'a CompleteScore),
    Filter(&'a CompleteFilter),
    Rank(&'a CompleteRank),
}
impl Atomic<'_> {
    pub(super) fn meta(&self) -> ResultMetadata<'_> {
        match self {
            Self::Decision(value) => value.meta(),
            Self::Choice(value) => value.meta(),
            Self::Tags(value) => value.meta(),
            Self::Score(value) => value.meta(),
            Self::Filter(value) => value.meta(),
            Self::Rank(value) => value.meta(),
        }
    }
    pub(super) fn outcome(&self) -> Outcome {
        match self {
            Self::Decision(value) => value.value().into(),
            Self::Choice(value) if value.value().is_none() => Outcome::Unresolved,
            Self::Filter(value) if !value.value() => Outcome::No,
            _ => Outcome::Yes,
        }
    }
    pub(super) fn order_value(&self) -> Option<f64> {
        match self {
            Self::Decision(value) => value.probability(),
            Self::Score(value) => Some(value.value()),
            Self::Filter(value) => value.probability(),
            Self::Rank(value) => value.score().or_else(|| value.probability()),
            _ => None,
        }
    }
    pub(super) fn label(&self) -> Option<&str> {
        match self {
            Self::Choice(value) => value.value(),
            _ => None,
        }
    }
    fn document(&self) -> Result<Json, Failure> {
        let text = match self {
            Self::Decision(value) => value.to_json(),
            Self::Choice(value) => value.to_json(),
            Self::Tags(value) => value.to_json(),
            Self::Score(value) => value.to_json(),
            Self::Filter(value) => value.to_json(),
            Self::Rank(value) => value.to_json(),
        }?;
        Json::parse(&text).map_err(|_| Failure::Defect("a complete result is not JSON"))
    }
    pub(super) fn value(&self) -> Result<Json, Failure> {
        self.document()?
            .member("value")
            .cloned()
            .ok_or(Failure::Defect("a complete result has no value"))
    }
    pub(super) fn details(
        &self,
        input: Option<&RawRecord>,
        index: Option<usize>,
        streams: bool,
    ) -> Result<String, Failure> {
        let document = self.document()?;
        let Kind::Object(fields) = document.kind() else {
            return Err(Failure::Defect("a complete result is not an object"));
        };
        crate::render::json_line(&Details {
            fields,
            input,
            index,
            strip_batch: !streams,
        })
        .map_err(Failure::from)
    }
}

// Omission belongs to the command's output mode. Every retained native field and
// its order pass through unchanged; answer values and identities are never rebuilt.
struct Details<'a> {
    fields: &'a [(String, Json)],
    input: Option<&'a RawRecord>,
    index: Option<usize>,
    strip_batch: bool,
}
impl Serialize for Details<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(None)?;
        for (name, value) in self.fields {
            if name == "input" || name == "index" {
                continue;
            }
            if name == "meta" && self.strip_batch {
                map.serialize_entry(name, &WithoutBatch(value))?;
            } else {
                map.serialize_entry(name, value)?;
            }
            if name == "value" {
                if let Some(input) = self.input {
                    map.serialize_entry("input", input)?;
                }
                if let Some(index) = self.index {
                    map.serialize_entry("index", &index)?;
                }
            }
        }
        map.end()
    }
}
struct WithoutBatch<'a>(&'a Json);
impl Serialize for WithoutBatch<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let Kind::Object(fields) = self.0.kind() else {
            return Err(serde::ser::Error::custom(
                "complete metadata is not an object",
            ));
        };
        let mut map = serializer.serialize_map(None)?;
        for (name, value) in fields {
            if name != "batch_setting" && name != "batch_warning" {
                map.serialize_entry(name, value)?;
            }
        }
        map.end()
    }
}

pub(super) fn without_batch(value: &impl Serialize) -> Result<String, Failure> {
    let text = crate::render::json_line(value)?;
    let document = Json::parse(&text)?;
    let Kind::Object(fields) = document.kind() else {
        return Err(Failure::Defect("a complete result is not an object"));
    };
    struct Document<'a>(&'a [(String, Json)]);
    impl Serialize for Document<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeMap as _;
            let mut map = serializer.serialize_map(None)?;
            for (name, value) in self.0 {
                if name == "meta" {
                    map.serialize_entry(name, &WithoutBatch(value))?;
                } else {
                    map.serialize_entry(name, value)?;
                }
            }
            map.end()
        }
    }
    crate::render::json_line(&Document(fields)).map_err(Failure::from)
}

#[cfg(test)]
mod tests {
    //! Stays: command plan presentation must preserve native counts and withhold authored evidence.
    use super::*;

    #[test]
    fn command_details_preserve_authored_values_and_native_identity_order() {
        let document = Json::parse(r#"{"schema":"thinkthen.result/2","answer_id":"unchanged","value":{"authored":false},"question":{"text":"question-marker"},"answer":{"p":0.75},"meta":{"batch_setting":"max","model":"same-model","batch_warning":{"tuned":2},"requests":["first","second"]}}"#).unwrap();
        let Kind::Object(fields) = document.kind() else {
            panic!("object")
        };
        let original = RawRecord::json(r#"{"z":null,"a":false}"#).unwrap();
        let text = crate::render::json_line(&Details {
            fields,
            input: Some(&original),
            index: Some(3),
            strip_batch: true,
        })
        .unwrap();
        assert_eq!(
            text,
            r#"{"schema":"thinkthen.result/2","answer_id":"unchanged","value":{"authored":false},"input":{"z":null,"a":false},"index":3,"question":{"text":"question-marker"},"answer":{"p":0.75},"meta":{"model":"same-model","requests":["first","second"]}}"#
        );
        let streamed = crate::render::json_line(&Details {
            fields,
            input: None,
            index: None,
            strip_batch: false,
        })
        .unwrap();
        assert_eq!(streamed, crate::render::json_line(&document).unwrap());
        assert!(!format!("{document:?}").contains("question-marker"));
    }
}
