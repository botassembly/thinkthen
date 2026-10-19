//! Append retained native annotation values to the command's original object.
use crate::{CompleteAnnotated, RawRecord};
use crate::failure::Failure;
use crate::saved_json::{Json, Kind};
use serde::{Serialize, Serializer};

pub(super) fn line(result: &CompleteAnnotated, original: &RawRecord, streams: bool, details: bool) -> Result<String, Failure> {
    if details { return result.to_json().map_err(Failure::from); }
    let values = Json::parse(&result.value_json()?)?;
    let object = original.content().and_then(|content| content.fields()).is_some();
    if streams && !object {
        #[derive(Serialize)]
        struct RecordValue<'a> { input: &'a RawRecord, value: &'a Json }
        return crate::render::json_line(&RecordValue { input: original, value: &values }).map_err(Failure::from);
    }
    let original = original.content();
    struct Appended<'a> { original: Option<crate::QuestionContent<'a>>, values: &'a Json }
    impl Serialize for Appended<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeMap as _;
            let mut map = serializer.serialize_map(None)?;
            if let Some(fields) = self.original.and_then(|content| content.fields()) {
                for (name, value) in fields { map.serialize_entry(name, &value)?; }
            }
            let Kind::Object(values) = self.values.kind() else {
                return Err(serde::ser::Error::custom("native annotation values are not an object"));
            };
            for (name, value) in values { map.serialize_entry(name, value)?; }
            map.end()
        }
    }
    crate::render::json_line(&Appended { original, values: &values }).map_err(Failure::from)
}
