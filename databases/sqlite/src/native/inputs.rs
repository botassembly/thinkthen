//! SQLite retains native BLOB input alongside shared descriptor inputs.
use crate::complete_native;
use thinkthen::{Error, RecordReading, RequestItem, RequestSource};

type Records<'a> = Box<dyn Iterator<Item = Result<thinkthen::RecordInput<thinkthen::QuestionInput>, Error>> + 'a>;

pub(super) struct Inputs {
    descriptors: complete_native::Inputs,
    native_record: Option<RequestItem>,
    source: String,
}
impl std::fmt::Debug for Inputs {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Inputs(<withheld>)")
    }
}
impl std::ops::Deref for Inputs {
    type Target = complete_native::Inputs;
    fn deref(&self) -> &Self::Target { &self.descriptors }
}
impl std::ops::DerefMut for Inputs {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.descriptors }
}
impl Inputs {
    pub(super) fn parse_request(source: &str) -> Result<Self, Error> {
        Ok(Self { descriptors: complete_native::Inputs::parse_request(source)?, native_record: None, source: source.to_owned() })
    }
    pub(super) fn from_record(record: RequestItem) -> Result<Self, Error> {
        let mut inputs = Self::parse_request(r#"{"records":[]}"#)?;
        inputs.native_record = Some(record);
        Ok(inputs)
    }
    pub(super) fn image_inputs(&self) -> Result<bool, Error> {
        self.native_record.as_ref().map_or_else(|| self.descriptors.image_inputs(), |record| Ok(!record.images.is_empty()))
    }
    pub(super) fn deferred_records(&self, reading: Option<&RecordReading>) -> Records<'_> {
        let Some(record) = &self.native_record else { return self.descriptors.deferred_records(reading); };
        let reading = reading.cloned();
        Box::new(std::iter::once_with(move || {
            let default = RecordReading::new(&[], None, None)?;
            record.compose_record(reading.as_ref().unwrap_or(&default))
        }))
    }
    pub(super) fn request_source(&self) -> Result<Option<RequestSource>, Error> {
        let fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> = serde_json::from_str(&self.source).map_err(|_| complete_native::defect())?;
        let Some(files) = fields.get("files") else { return Ok(None); };
        let files: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> = serde_json::from_str(files.get()).map_err(|_| complete_native::defect())?;
        let options = files.get("options").map(|options| serde_json::from_str::<thinkthen::InputReaderOptions>(options.get()).map_err(|_| complete_native::usage("invalid native reader options"))).transpose()?.unwrap_or_default();
        Ok(Some(RequestSource { paths: self.file_paths()?.into_iter().map(Into::into).collect(), reading: options.reading, media: options.media, framing: self.framing }))
    }
}
