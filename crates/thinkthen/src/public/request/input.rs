//! Ordered originals and image descriptors keep authority outside core.
use super::present;
use crate::{ImageMedia, RawRecord, ReaderMedia, ReaderOptions, RecordContext, RecordOptions};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Explicit source paths and physical-unit controls.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RequestSource {
    /// Caller paths, retaining order and duplicates.
    pub paths: Vec<PathBuf>,
    /// Native physical-unit settings.
    #[serde(default, with = "reader")]
    #[cfg_attr(test, schemars(with = "RequestReader"))]
    pub reading: ReaderOptions,
    /// Explicit logical framing; omission retains physical text/image reading.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "RequestFraming"))]
    pub framing: Option<RequestFraming>,
    /// Whole-file media; text is the default.
    #[serde(default)]
    pub media: ReaderMedia,
}
/// Evidence framing for a caller-supplied bounded feed.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum RequestFraming {
    /// One whole document.
    #[default]
    Document,
    /// One literal text record per line.
    Lines,
    /// One original JSON value per line.
    Jsonl,
    /// One record per CSV row.
    Csv,
    /// One record per TSV row.
    Tsv,
}
/// Closed evidence selectors; feed contains no runtime reader.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum RequestInput {
    /// One literal text item.
    Text {
        /// Original text.
        text: String,
        /// Ordered attachments.
        #[serde(default)]
        images: Vec<RequestImage>,
    },
    /// One authored JSON item, including JSON null.
    Json {
        /// Original ordered JSON.
        #[serde(with = "original_json")]
        #[cfg_attr(test, schemars(with = "serde_json::Value"))]
        value: RawRecord,
        /// Ordered attachments.
        #[serde(default)]
        images: Vec<RequestImage>,
    },
    /// Ordered closed per-item descriptors.
    Records {
        /// All supplied item descriptors.
        items: Vec<RequestItem>,
    },
    /// Ordered find candidates.
    Units {
        /// Complete unit set.
        items: Vec<RequestItem>,
    },
    /// Ordered relate entity originals.
    Entities {
        /// Complete entity set.
        items: Vec<RequestItem>,
    },
    /// Ordered explicit filesystem source.
    Source {
        /// Native source selection.
        source: RequestSource,
    },
    /// A feed supplied separately by the execution environment.
    Feed {
        /// Caller-assigned feed name.
        name: String,
        /// Native framing selection.
        #[serde(default)]
        framing: RequestFraming,
        /// Physical reading controls for located feeds.
        #[serde(default, with = "reader")]
        #[cfg_attr(test, schemars(with = "RequestReader"))]
        reading: ReaderOptions,
        /// Optional ordered attachments to each item.
        #[serde(default)]
        images: Vec<RequestImage>,
    },
}
/// Optional original; omission admits an images-only item.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum RequestOriginal {
    /// Literal text never reinterpreted as JSON.
    Text {
        /// Exact text.
        text: String,
    },
    /// Ordered arbitrary JSON evidence.
    Json {
        /// Original value.
        #[serde(with = "original_json")]
        #[cfg_attr(test, schemars(with = "serde_json::Value"))]
        value: RawRecord,
    },
}
/// A closed descriptor with distinct absence and explicit empty context.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RequestItem {
    /// Absent only for image-only evidence.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "RequestOriginal"))]
    pub original: Option<RequestOriginal>,
    /// Explicit per-item context; null refuses.
    #[serde(
        default,
        deserialize_with = "context",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "ContextSchema"))]
    pub context: Option<RecordContext>,
    /// Whole replacement ordered shortlist; null refuses.
    #[serde(
        default,
        deserialize_with = "options",
        serialize_with = "write_options",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "Vec<OptionSchema>"))]
    pub options: Option<RecordOptions>,
    /// Unconfirmed recognition proposals using exact Unicode scalar piece edges.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "Vec<crate::RecognitionSeedSpan>"))]
    pub seed_spans: Option<Vec<crate::RecognitionSeedSpan>>,
    /// Optional recognition boundary examples; an empty list clears the fallback.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "Vec<crate::RecognitionExample>"))]
    pub examples: Option<Vec<crate::RecognitionExample>>,
    /// Ordered attachments, retaining duplicates.
    #[serde(default)]
    pub images: Vec<RequestImage>,
}
/// Closed attachment descriptors; bytes stay native until serialized.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum RequestImage {
    /// Explicit attachment path and optional media assertion.
    File {
        /// User-selected image path.
        path: PathBuf,
        /// Optional declared JPEG/PNG MIME.
        #[serde(
            default,
            deserialize_with = "present",
            skip_serializing_if = "Option::is_none"
        )]
        #[cfg_attr(test, schemars(with = "ImageMedia"))]
        media: Option<ImageMedia>,
    },
    /// Original bytes represented as canonical base64 on the wire.
    Bytes {
        /// JPEG/PNG MIME.
        media: ImageMedia,
        /// Exact bytes.
        #[serde(with = "image_bytes")]
        #[cfg_attr(test, schemars(with = "String"))]
        bytes: Vec<u8>,
    },
}
impl std::fmt::Debug for RequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestInput(<withheld>)")
    }
}
impl std::fmt::Debug for RequestImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestImage(<withheld>)")
    }
}
pub(super) mod original_json {
    use super::{Deserialize, RawRecord, Serialize};
    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
        de: D,
    ) -> Result<RawRecord, D::Error> {
        let value = crate::core::Json::deserialize(de)?;
        let text = crate::core::json_line(&value).map_err(serde::de::Error::custom)?;
        RawRecord::json(&text).map_err(serde::de::Error::custom)
    }
    pub(super) fn serialize<S: serde::Serializer>(
        value: &RawRecord,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        value.serialize(s)
    }
}
mod image_bytes {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use serde::Deserialize;
    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
        de: D,
    ) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(de)?;
        let bytes = STANDARD
            .decode(&encoded)
            .map_err(|_| serde::de::Error::custom("image bytes require canonical base64"))?;
        if STANDARD.encode(&bytes) != encoded {
            return Err(serde::de::Error::custom(
                "image bytes require canonical base64",
            ));
        }
        Ok(bytes)
    }
    pub(super) fn serialize<S: serde::Serializer>(value: &[u8], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&STANDARD.encode(value))
    }
}
fn context<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Option<RecordContext>, D::Error> {
    let value = crate::core::Json::deserialize(de)?;
    let context = match value {
        crate::core::Json::String(text) => RecordContext::Text(text),
        crate::core::Json::Object(_) => {
            let text = crate::core::json_line(&value).map_err(serde::de::Error::custom)?;
            let raw = RawRecord::json(&text).map_err(serde::de::Error::custom)?;
            RecordContext::Object(
                crate::ObjectContext::new(&raw).map_err(serde::de::Error::custom)?,
            )
        }
        _ => {
            return Err(serde::de::Error::custom(
                "per-item context is text or an object",
            ));
        }
    };
    Ok(Some(context))
}
#[derive(Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "OptionSchema"))]
#[serde(deny_unknown_fields)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
struct OptionSchema<T = Box<serde_json::value::RawValue>> {
    name: String,
    #[serde(
        default,
        deserialize_with = "description",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "serde_json::Value"))]
    description: Option<T>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
struct RequestReader {
    #[serde(default)]
    unit: crate::SourceUnit,
    #[serde(default, deserialize_with = "present")]
    #[cfg_attr(test, schemars(with = "usize"))]
    window: Option<usize>,
}
mod reader {
    use super::{Deserialize, ReaderOptions, RequestReader, Serialize};
    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
        de: D,
    ) -> Result<ReaderOptions, D::Error> {
        let value = RequestReader::deserialize(de)?;
        Ok(ReaderOptions {
            unit: value.unit,
            window: value.window,
        })
    }
    pub(super) fn serialize<S: serde::Serializer>(
        value: &ReaderOptions,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        value.serialize(s)
    }
}
#[cfg(test)]
#[derive(schemars::JsonSchema)]
#[serde(untagged)]
#[expect(dead_code, reason = "deserialization schema for typed ordered context")]
enum ContextSchema {
    Text(String),
    Object(std::collections::BTreeMap<String, serde_json::Value>),
}
// Canonical wire decoding retains its surrounding JSON recursion budget.
// Compatibility descriptors retain their separate raw-description budget.
struct WireDescription(Box<serde_json::value::RawValue>);
impl<'de> Deserialize<'de> for WireDescription {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let value = crate::core::Json::deserialize(de)?;
        let text = crate::core::json_line(&value).map_err(serde::de::Error::custom)?;
        serde_json::value::RawValue::from_string(text)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}
fn description<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    de: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(de).map(Some)
}
fn options<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Option<RecordOptions>, D::Error> {
    let options = Vec::<OptionSchema<WireDescription>>::deserialize(de)?
        .into_iter()
        .map(|option| OptionSchema {
            name: option.name,
            description: option.description.map(|value| value.0),
        })
        .collect();
    record_options(options)
        .map(Some)
        .map_err(serde::de::Error::custom)
}
fn record_options(options: Vec<OptionSchema>) -> Result<RecordOptions, crate::Error> {
    RecordOptions::new(
        options
            .into_iter()
            .map(|option| {
                Ok(crate::RecordOption {
                    name: option.name,
                    description: option
                        .description
                        .map(|value| crate::Description::from_json(value.get()))
                        .transpose()?,
                })
            })
            .collect::<Result<Vec<_>, crate::Error>>()?,
    )
}
fn options_descriptor(source: &str) -> Result<RecordOptions, crate::Error> {
    let options = serde_json::from_str(source)
        .map_err(|_| crate::Error::usage("complete call requires a typed request"))?;
    record_options(options)
}
fn write_options<S: serde::Serializer>(
    value: &Option<RecordOptions>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => value
            .options()
            .iter()
            .map(|option| {
                Ok(OptionSchema {
                    name: option.name.clone(),
                    description: option
                        .description
                        .as_ref()
                        .map(|value| {
                            serde_json::value::RawValue::from_string(value.as_json().to_owned())
                        })
                        .transpose()
                        .map_err(serde::ser::Error::custom)?,
                })
            })
            .collect::<Result<Vec<_>, S::Error>>()?
            .serialize(s),
        None => s.serialize_none(),
    }
}

impl std::fmt::Debug for RequestSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestSource(<withheld>)")
    }
}
impl std::fmt::Debug for RequestOriginal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestOriginal(<withheld>)")
    }
}
impl std::fmt::Debug for RequestItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestItem(<withheld>)")
    }
}

impl RequestItem {
    /// Replace the shortlist from closed named descriptors, retaining description order.
    /// Descriptions retain explicit null separately from omission.
    /// # Errors
    /// Returns Usage for malformed descriptors or invalid names and descriptions.
    pub fn with_options_descriptor(mut self, source: &str) -> Result<Self, crate::Error> {
        self.options = Some(options_descriptor(source)?);
        Ok(self)
    }

    /// Decode a complete-call record descriptor without reading files.
    /// This retains the existing text, document and JSON envelope grammar.
    /// # Errors
    /// Returns the existing Usage diagnostics for malformed descriptors.
    pub fn from_record_descriptor(source: &str) -> Result<Self, crate::Error> {
        super::transport::record_descriptor(source)
    }
}
