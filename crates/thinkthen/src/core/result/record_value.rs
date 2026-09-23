//! One parsed record beside the bare value it produced.

use serde::Serialize;

use crate::core::records::Record;

/// The default row for a value command over a record stream.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct RecordValue<T> {
    input: Record,
    value: T,
}

impl<T> RecordValue<T> {
    /// Keep one parsed record beside its bare answer.
    #[must_use]
    pub(crate) const fn new(input: Record, value: T) -> Self {
        Self { input, value }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::RecordValue;
    use crate::core::{Framing, Reading, Value, json_line};

    const CASES: &str = include_str!("../../../../../conformance/record-values.json");

    #[derive(Deserialize)]
    struct Document {
        schema: String,
        cases: Vec<Case>,
    }

    #[derive(Deserialize)]
    struct Case {
        id: String,
        input: String,
        value: FixtureValue,
        expect: String,
    }

    #[derive(Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum FixtureValue {
        YesNo { held: Option<bool> },
        Choice { held: Option<String> },
        Tag { held: Vec<String> },
        Score { held: f64 },
    }

    impl FixtureValue {
        fn into_value(self) -> Value {
            match self {
                Self::YesNo { held } => Value::YesNo(held),
                Self::Choice { held } => Value::Choice(held),
                Self::Tag { held } => Value::Tag(held),
                Self::Score { held } => Value::Score(held),
            }
        }
    }

    #[test]
    fn shared_typed_cases_cross_the_production_serializer() {
        let document: Document = serde_json::from_str(CASES).expect("shared record-value cases");
        assert_eq!(document.schema, "thinkthen.record-value-conformance/1");
        let reading = Reading::new(Framing::Jsonl, Vec::new()).expect("JSONL framing");
        for case in document.cases {
            let record = reading.record(case.input.as_bytes()).expect("typed record");
            let actual = json_line(&RecordValue::new(record, case.value.into_value()))
                .expect("record value serializes");
            assert_eq!(actual, case.expect, "{}", case.id);
        }
    }
}
