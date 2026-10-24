//! Shared SDK admission. Host bridges pass JSON once, then use typed calls.
//!
//! Client settings are admitted separately and retained by a client. A call
//! carries only function, tagged question, options and initial records. Libraries
//! take question definitions from memory; filenames belong to the command.

use std::collections::BTreeMap;
use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;

use crate::{
    Annotate, BackendTable, Batch, Cancel, Engine, Error, ErrorKind, Pointer, Rank, RankLimit,
    Record, Row, Threshold, Toolbox, ToolboxFunction, ToolboxQuestion,
};

/// Immutable settings captured when a host creates its client.
#[derive(Debug, Clone, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SdkSettings {
    mode: Option<Mode>,
    key_source: Option<String>,
    adapter: Option<String>,
    url: Option<String>,
    model: Option<String>,
    #[schemars(with = "Option<f64>")]
    batch: Option<serde_json::Number>,
}

impl SdkSettings {
    /// Admit a host's JSON configuration once.
    ///
    /// # Errors
    /// Returns a library usage error for malformed settings.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json)
            .map_err(|error| Error::new(ErrorKind::Settings, error.to_string()))
    }

    /// Select the effective backend and model without building a client.
    ///
    /// # Errors
    /// Returns the named backend admission error without reading credentials.
    pub fn backend_selection<'a>(
        &'a self,
        table: &'a BackendTable,
    ) -> Result<(&'a str, &'a str), Error> {
        table.selection(
            self.adapter
                .as_deref()
                .unwrap_or(crate::backend::DEFAULT_BACKEND),
            self.model.as_deref(),
        )
    }

    /// Create the engine whose connections the client retains.
    ///
    /// # Errors
    /// Returns the existing backend or batch admission error.
    pub fn engine(&self, variable: impl Fn(&str) -> Option<String>) -> Result<Engine, Error> {
        self.engine_with_backends(&BackendTable::default(), variable)
    }

    /// Create the engine using a caller's named backend definitions.
    ///
    /// # Errors
    /// Returns the selected backend or batch admission error.
    pub fn engine_with_backends(
        &self,
        table: &BackendTable,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Engine, Error> {
        if matches!(self.mode, Some(Mode::Proxy)) {
            return Err(Error::new(ErrorKind::Mode, "proxy mode is not available"));
        }
        let backend = table.resolve_key(
            self.adapter
                .as_deref()
                .unwrap_or(crate::backend::DEFAULT_BACKEND),
            self.url.as_deref(),
            self.model.as_deref(),
            self.key_source.as_deref(),
            &variable,
        )?;
        let batch = Batch::override_from(
            self.batch.as_ref().map(ToString::to_string).as_deref(),
            variable,
        )?;
        let batch = batch.unwrap_or_default();
        Ok(Engine::new(backend).batch(batch))
    }
}

#[derive(Debug, Clone, Copy, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
enum Mode {
    Embedded,
    Proxy,
}

/// A request parsed at the host boundary, before question validation.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "contract::Contract")]
#[schemars(with = "contract::Contract")]
pub struct SdkRequest {
    function: FunctionName,
    question: QuestionInput,
    options: Options,
    records: Vec<Value>,
    loaded_question: Option<ToolboxQuestion>,
    loaded_toolbox: Option<Toolbox>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
enum QuestionInput {
    Inline {
        value: Value,
    },
    Toolbox {
        value: BTreeMap<String, Value>,
        name: Option<String>,
    },
    Reference {
        name: String,
    },
}

impl QuestionInput {
    fn value(&self) -> Value {
        match self {
            Self::Inline { value } => value.clone(),
            Self::Toolbox { value, .. } => Value::Object(value.clone().into_iter().collect()),
            Self::Reference { .. } => Value::Null,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
enum FunctionName {
    Decide,
    Choose,
    Tag,
    Score,
    Filter,
    Rank,
    Find,
    Annotate,
    Recognize,
    Relate,
}

impl FunctionName {
    fn parse(value: &str) -> Result<Self, Error> {
        use serde::de::IntoDeserializer;
        Self::deserialize(value.into_deserializer())
            .map_err(|error: serde::de::value::Error| invalid(error.to_string()))
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Tag => "tag",
            Self::Score => "score",
            Self::Filter => "filter",
            Self::Rank => "rank",
            Self::Find => "find",
            Self::Annotate => "annotate",
            Self::Recognize => "recognize",
            Self::Relate => "relate",
        }
    }
}

#[derive(Debug, Default)]
struct Options {
    on: Option<String>,
    threshold: Option<Value>,
    options: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    levels: Option<Vec<String>>,
    top: Option<RankLimit>,
}

#[derive(Debug)]
enum Question {
    Ordinary(ToolboxFunction),
    Annotate(Annotate),
    Rank(Rank),
}

/// An admitted question with a client engine and one operation's cancellation.
#[derive(Debug)]
pub struct SdkCall {
    engine: Engine,
    question: Question,
    on: Pointer,
    annotation_on: BTreeMap<String, Pointer>,
    top: Option<RankLimit>,
}

impl SdkRequest {
    /// Use an already admitted named question without reparsing its toolbox.
    ///
    /// # Errors
    /// Returns a usage error for malformed host options.
    pub fn from_question(
        function: &str,
        question: ToolboxQuestion,
        options_json: &str,
    ) -> Result<Self, Error> {
        question
            .ensure_function(function)
            .map_err(Error::into_argument)?;
        Ok(Self {
            function: FunctionName::parse(function)?,
            question: QuestionInput::Inline { value: Value::Null },
            options: contract::options(FunctionName::parse(function)?, options_json)?,
            records: Vec::new(),
            loaded_question: Some(question),
            loaded_toolbox: None,
        })
    }

    /// Use an already admitted toolbox for annotation without reparsing it.
    ///
    /// # Errors
    /// Returns a usage error for malformed host options.
    pub fn from_toolbox(toolbox: Toolbox, options_json: &str) -> Result<Self, Error> {
        Ok(Self {
            function: FunctionName::Annotate,
            question: QuestionInput::Inline { value: Value::Null },
            options: contract::options(FunctionName::Annotate, options_json)?,
            loaded_question: None,
            records: Vec::new(),
            loaded_toolbox: Some(toolbox),
        })
    }

    /// Parse a JSON request without reading the process environment.
    ///
    /// # Errors
    /// Returns a library usage error for malformed request fields.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str::<contract::Contract>(json)
            .map_err(|error| invalid(error.to_string()))?
            .try_into()
    }

    /// Initial records carried by this contract call, in input order.
    #[must_use]
    pub fn records(&self) -> &[Value] {
        &self.records
    }

    /// Execute a complete contract call using its settings and records.
    ///
    /// Settings read only the variables supplied by the caller. Records preserve
    /// strings as text and every other JSON value as structured evidence.
    ///
    /// # Errors
    /// Returns the library's settings, question or execution error.
    pub fn execute(
        &self,
        client: &crate::Client,
        cancel: Cancel,
        mut sink: impl FnMut(Row),
    ) -> Result<(), Error> {
        let call = client.prepare(self, cancel)?;
        call.run(
            self.records.iter().map(|value| (call.record(value), ())),
            |(), row| sink(row),
        )
    }

    /// Validate a question using the library's existing admission functions.
    ///
    /// # Errors
    /// Returns the existing question, threshold, pointer or toolbox error.
    pub fn prepare(&self, engine: &Engine, cancel: Cancel) -> Result<SdkCall, Error> {
        self.prepare_argument(engine, cancel)
            .map_err(Error::into_argument)
    }

    fn prepare_argument(&self, engine: &Engine, cancel: Cancel) -> Result<SdkCall, Error> {
        if let QuestionInput::Reference { name } = &self.question {
            return Err(Error::new(
                ErrorKind::Reference,
                format!("question reference `{name}` requires proxy mode"),
            ));
        }
        let override_on = self
            .options
            .on
            .as_deref()
            .map(Pointer::from_str)
            .transpose()?;
        let mut on = override_on.clone().unwrap_or_default();
        if self.function == FunctionName::Annotate {
            let read;
            let toolbox = if let Some(toolbox) = &self.loaded_toolbox {
                toolbox
            } else {
                read = Toolbox::from_bytes(self.question.value().to_string().as_bytes())?;
                &read
            };
            let annotation_on = toolbox
                .questions()
                .map(|(name, q)| {
                    (
                        name.to_owned(),
                        override_on
                            .clone()
                            .or_else(|| q.on().cloned())
                            .unwrap_or_default(),
                    )
                })
                .collect();
            let mut question = Annotate::new(toolbox);
            if let Some(on) = &override_on {
                question = question.on(on);
            }
            let question = Question::Annotate(question);
            return Ok(SdkCall {
                engine: engine.clone().cancel(cancel),
                question,
                on: Pointer::default(),
                annotation_on,
                top: self.options.top,
            });
        }
        let loaded = match &self.loaded_question {
            Some(question) => Some(question.clone()),
            None => match &self.question {
                QuestionInput::Toolbox { value, name } => {
                    let toolbox = Toolbox::from_bytes(
                        Value::Object(value.clone().into_iter().collect())
                            .to_string()
                            .as_bytes(),
                    )?;
                    let question = toolbox.question(name.as_deref())?.clone();
                    question.ensure_function(self.function.as_str())?;
                    Some(question)
                }
                QuestionInput::Reference { .. } | QuestionInput::Inline { .. } => None,
            },
        };
        let function = if let Some(question) = loaded {
            on = override_on
                .or_else(|| question.on().cloned())
                .unwrap_or_default();
            self.options.override_vectors(question.function().clone())?
        } else {
            let mut fields = serde_json::Map::new();
            let kind = if self.function == FunctionName::Rank {
                if self.options.levels.is_some() {
                    "score"
                } else {
                    "decide"
                }
            } else {
                self.function.as_str()
            };
            fields.insert(kind.to_owned(), self.question.value());
            if let Some(v) = &self.options.options {
                fields.insert(
                    "options".into(),
                    serde_json::to_value(v).map_err(|e| invalid(e.to_string()))?,
                );
            }
            if let Some(v) = &self.options.labels {
                fields.insert(
                    "labels".into(),
                    serde_json::to_value(v).map_err(|e| invalid(e.to_string()))?,
                );
            }
            if let Some(v) = &self.options.levels {
                fields.insert(
                    "levels".into(),
                    serde_json::to_value(v).map_err(|e| invalid(e.to_string()))?,
                );
            }
            let toolbox = Toolbox::from_bytes(Value::Object(fields).to_string().as_bytes())?;
            toolbox.question(None)?.function().clone()
        };
        let function = self.options.apply(function)?;
        let engine = engine.clone().cancel(cancel);
        let question = if self.function == FunctionName::Rank {
            Question::Rank(match function {
                ToolboxFunction::Decide(q) => Rank::Decide(q),
                ToolboxFunction::Score(q) => Rank::Score(q),
                _ => return Err(invalid("rank needs a decide or score question")),
            })
        } else {
            Question::Ordinary(function)
        };
        Ok(SdkCall {
            engine,
            question,
            on,
            annotation_on: BTreeMap::new(),
            top: self.options.top,
        })
    }
}

impl Options {
    fn override_vectors(&self, function: ToolboxFunction) -> Result<ToolboxFunction, Error> {
        Ok(match function {
            ToolboxFunction::Choose(q) => {
                ToolboxFunction::Choose(if let Some(options) = &self.options {
                    q.options(
                        options
                            .iter()
                            .cloned()
                            .map(crate::ChoiceOption::new)
                            .collect::<Result<_, _>>()?,
                    )?
                } else {
                    q
                })
            }
            ToolboxFunction::Tag(q) => ToolboxFunction::Tag(if let Some(labels) = &self.labels {
                q.labels(labels.clone())?
            } else {
                q
            }),
            ToolboxFunction::Score(q) => {
                ToolboxFunction::Score(if let Some(levels) = &self.levels {
                    q.levels(levels.clone())?
                } else {
                    q
                })
            }
            other => other,
        })
    }

    fn apply(&self, function: ToolboxFunction) -> Result<ToolboxFunction, Error> {
        let threshold = self
            .threshold
            .as_ref()
            .map(|value| {
                value
                    .as_str()
                    .map_or_else(|| value.to_string(), str::to_owned)
                    .parse::<Threshold>()
            })
            .transpose()?;
        Ok(match function {
            ToolboxFunction::Decide(q) => {
                ToolboxFunction::Decide(threshold.map_or(q.clone(), |t| q.threshold(t)))
            }
            ToolboxFunction::Filter(q) => {
                ToolboxFunction::Filter(threshold.map_or(q.clone(), |t| q.threshold(t)))
            }
            ToolboxFunction::Choose(q) => {
                ToolboxFunction::Choose(threshold.map_or(q.clone(), |t| q.threshold(t)))
            }
            ToolboxFunction::Tag(q) => ToolboxFunction::Tag(if let Some(t) = threshold {
                q.threshold(t)?
            } else {
                q
            }),
            ToolboxFunction::Score(q) => ToolboxFunction::Score(q),
            ToolboxFunction::Find(q) => ToolboxFunction::Find(if let Some(t) = threshold {
                q.threshold(t)?
            } else {
                q
            }),
            ToolboxFunction::Recognize(q) => {
                ToolboxFunction::Recognize(if let Some(t) = threshold {
                    q.threshold(t)?
                } else {
                    q
                })
            }
            ToolboxFunction::Relate(q) => ToolboxFunction::Relate(if let Some(t) = threshold {
                q.threshold(t)?
            } else {
                q
            }),
        })
    }
}

impl SdkCall {
    /// Turn a host value into a record, preserving text versus JSON provenance.
    #[must_use]
    pub fn record(&self, value: &Value) -> Record {
        match value.as_str() {
            Some(text) => Record::text(text),
            None => Record::json(&value.to_string(), &self.on),
        }
    }

    /// Admit structured evidence with this call's selected pointer.
    #[must_use]
    pub fn json_record(&self, json: &str) -> Record {
        Record::json(json, &self.on)
    }

    /// Read the admitted evidence for translating host string offsets.
    ///
    /// # Errors
    /// Returns the record's existing evidence error.
    pub fn evidence(
        &self,
        record: &Record,
        annotation_name: Option<&str>,
    ) -> Result<String, Error> {
        record.evidence_at(
            annotation_name
                .and_then(|name| self.annotation_on.get(name))
                .unwrap_or(&Pointer::default()),
        )
    }

    /// Whether this operation returns named annotation outcomes.
    #[must_use]
    pub const fn is_annotate(&self) -> bool {
        matches!(self.question, Question::Annotate(_))
    }

    /// The admitted evidence pointer for ordinary structured records.
    #[must_use]
    pub const fn evidence_pointer(&self) -> &Pointer {
        &self.on
    }

    /// Names of the admitted annotation outcomes, in their existing order.
    pub fn annotation_names(&self) -> impl Iterator<Item = &str> {
        self.annotation_on.keys().map(String::as_str)
    }

    /// Whether this operation selects original records with `passes`.
    #[must_use]
    pub const fn is_filter(&self) -> bool {
        matches!(
            self.question,
            Question::Ordinary(ToolboxFunction::Filter(_))
        )
    }

    /// Execute one operation; the sink receives original values alongside rows.
    ///
    /// # Errors
    /// Returns the first failed row's error, or `local.cancelled` on cancellation.
    pub fn run<T>(
        &self,
        records: impl IntoIterator<Item = (Record, T)>,
        mut sink: impl FnMut(T, Row),
    ) -> Result<(), Error> {
        enum Ordered<'a> {
            Ordinary(&'a ToolboxFunction),
            Annotate(&'a Annotate),
        }
        let question = match &self.question {
            Question::Rank(question) => {
                return self.engine.rank(question, records, self.top, sink);
            }
            Question::Ordinary(ToolboxFunction::Filter(question)) => {
                return self.engine.filter_rows(question, records, sink);
            }
            Question::Ordinary(question) => Ordered::Ordinary(question),
            Question::Annotate(question) => Ordered::Annotate(question),
        };
        let originals = std::cell::RefCell::new(std::collections::VecDeque::new());
        let records = records.into_iter().map(|(record, original)| {
            originals.borrow_mut().push_back(original);
            record
        });
        let sink = |row| {
            if let Some(original) = originals.borrow_mut().pop_front() {
                sink(original, row);
            }
        };
        match question {
            Ordered::Ordinary(q) => self.engine.call(q, records, sink),
            Ordered::Annotate(q) => self.engine.annotate(q, records, sink),
        }
    }
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Question, message)
}

mod contract;
mod definition;
pub use definition::QuestionDefinition;
