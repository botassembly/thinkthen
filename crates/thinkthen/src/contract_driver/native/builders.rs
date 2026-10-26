//! Native mutations retain typed leaves and admit complete snapshots once.
use crate::{
    Code, Cut, Error, FunctionName, Pointer, Question, SimpleQuestion, Threshold, questions,
};
use std::collections::BTreeMap;
fn question_error(message: impl Into<String>) -> Error {
    Error::from_contract(Code::UsageQuestion, message)
}
fn name_error(message: impl Into<String>) -> Error {
    Error::from_contract(Code::UsageName, message)
}
/// A native question's unfinished required lists and validated optional values.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct QuestionBuilder {
    function: FunctionName,
    text: String,
    items: Vec<String>,
    rules: Vec<questions::RelationRule>,
    entries: BTreeMap<String, questions::AnnotatableQuestion>,
    threshold: Option<Threshold>,
    evidence: Option<Pointer>,
    top: Option<crate::core::RankLimit>,
}
impl QuestionBuilder {
    /// Start a typed builder; required lists are checked when a snapshot is opened.
    /// # Errors
    /// Rejects invalid wording or text on a function without a wording field.
    pub fn new(function: FunctionName, text: &str) -> Result<Self, Error> {
        match function {
            FunctionName::Recognize | FunctionName::Relate | FunctionName::Annotate => {
                if !text.is_empty() {
                    return Err(question_error(crate::core::diagnostic::requirement(
                        "question",
                        "no wording for this function",
                        "a string (contents redacted)",
                    )));
                }
            }
            _ => {
                questions::Decide::new(text)?;
            }
        }
        Ok(Self {
            function,
            text: text.into(),
            items: Vec::new(),
            rules: Vec::new(),
            entries: BTreeMap::new(),
            threshold: None,
            evidence: None,
            top: None,
        })
    }
    /// Snapshot an admitted question without parsing or dropping required lists.
    #[must_use]
    pub fn snapshot(question: &Question) -> Self {
        let mut draft = Self {
            function: question.function(),
            text: String::new(),
            items: Vec::new(),
            rules: Vec::new(),
            entries: BTreeMap::new(),
            threshold: None,
            evidence: question.evidence_override().cloned(),
            top: None,
        };
        macro_rules! predicate {
            ($q:ident) => {{
                draft.text = $q.inner.text.clone();
                draft.threshold = Some($q.inner.threshold);
            }};
        }
        match question {
            Question::Decide(q) => predicate!(q),
            Question::Filter(q) => predicate!(q),
            Question::Find(q) => predicate!(q),
            Question::Choose(q) => {
                predicate!(q);
                draft.items = q
                    .inner
                    .options
                    .iter()
                    .map(|item| item.label().to_owned())
                    .collect();
            }
            Question::Tag(q) => {
                predicate!(q);
                draft.items = q.inner.labels.clone();
            }
            Question::Score(q) => {
                draft.text = q.inner.text.clone();
                draft.items = q.inner.levels.clone();
            }
            Question::Rank(q) => {
                draft.text = q.inner.text.clone();
                draft.items = q.inner.levels.clone();
                draft.top = q.top;
            }
            Question::Recognize(q) => {
                draft.items = q
                    .inner
                    .kinds
                    .iter()
                    .map(|item| item.label().to_owned())
                    .collect();
                draft.threshold = Some(q.inner.threshold);
            }
            Question::Relate(q) => {
                draft.rules = q.inner.rules.iter().cloned().map(Into::into).collect();
                draft.threshold = Some(q.inner.threshold);
            }
            Question::Annotate(q) => {
                draft.entries = q.questions().clone();
            }
        }
        draft
    }
    /// Read the builder's function without opening it.
    #[must_use]
    pub fn function(&self) -> FunctionName {
        self.function
    }
    /// Add one entry to this function's required domain.
    /// # Errors
    /// Rejects a setter for another domain, a bad leaf or duplicate entry unchanged.
    pub fn add_item(&mut self, domain: FunctionName, text: &str) -> Result<(), Error> {
        let matches = matches!(
            (domain, self.function),
            (FunctionName::Choose, FunctionName::Choose)
                | (FunctionName::Tag, FunctionName::Tag)
                | (
                    FunctionName::Score,
                    FunctionName::Score | FunctionName::Rank
                )
                | (FunctionName::Recognize, FunctionName::Recognize)
        );
        if !matches {
            return Err(question_error(crate::core::diagnostic::requirement(
                "domain",
                "the selected function's domain",
                "a domain for another function",
            )));
        }
        crate::core::ChoiceOption::new(text)?;
        if self.items.iter().any(|item| item == text) {
            return Err(question_error(crate::core::diagnostic::requirement(
                "domain",
                "distinct entries",
                "a repeated entry (contents redacted)",
            )));
        }
        self.items.push(text.into());
        Ok(())
    }
    /// Set a validated numeric threshold for a threshold-taking function.
    /// # Errors
    /// Rejects score, rank and annotation without changing the builder.
    pub fn set_cut(&mut self, value: f64) -> Result<(), Error> {
        let cut = Cut::new(value)?;
        if matches!(
            self.function,
            FunctionName::Score | FunctionName::Rank | FunctionName::Annotate
        ) {
            return Err(Error::from_contract(
                Code::UsageThreshold,
                crate::core::diagnostic::requirement(
                    "threshold",
                    "no threshold for this function",
                    "a numeric cut",
                ),
            ));
        }
        self.threshold = Some(cut.into());
        Ok(())
    }
    /// Set an ordered band only for decide, choose and filter.
    /// # Errors
    /// Returns the shared threshold error and preserves the prior state.
    pub fn set_band(&mut self, low: f64, high: f64) -> Result<(), Error> {
        let threshold = Threshold::band(low, high)?;
        if !matches!(
            self.function,
            FunctionName::Decide | FunctionName::Choose | FunctionName::Filter
        ) {
            return Err(Error::from_contract(
                Code::UsageThreshold,
                crate::core::diagnostic::requirement(
                    "threshold",
                    "a numeric cut for this function",
                    "a low/high band",
                ),
            ));
        }
        self.threshold = Some(threshold);
        Ok(())
    }
    /// Copy one admitted evidence selection.
    /// # Errors
    /// Returns the shared pointer error without changing the builder.
    pub fn set_evidence(&mut self, pointer: &str) -> Result<(), Error> {
        self.evidence = Some(pointer.parse()?);
        Ok(())
    }
    /// Retain at most this many successful ranks.
    /// # Errors
    /// Rejects a non-rank builder or zero top without changing its state.
    pub fn set_top(&mut self, top: usize) -> Result<(), Error> {
        if self.function != FunctionName::Rank {
            return Err(question_error(crate::core::diagnostic::requirement(
                "top",
                "a rank question",
                "another function",
            )));
        }
        self.top = Some(crate::core::RankLimit::new(top)?);
        Ok(())
    }
    /// Append a fully admitted rule while retaining existing rules on error.
    /// # Errors
    /// Returns the shared rule or function admission error.
    pub fn add_rule(&mut self, rule: questions::RelationRule) -> Result<(), Error> {
        if self.function != FunctionName::Relate {
            return Err(question_error(crate::core::diagnostic::requirement(
                "rules",
                "a relate question",
                "another function",
            )));
        }
        let mut rules = self.rules.clone();
        rules.push(rule);
        questions::Relate::new(rules.clone())?;
        self.rules = rules;
        Ok(())
    }
    /// Snapshot one eligible annotation child; never retain a caller's pointer.
    /// # Errors
    /// Rejects rank, annotate, duplicate names and invalid names unchanged.
    pub fn add_entry(&mut self, name: &str, child: Question) -> Result<(), Error> {
        if self.function != FunctionName::Annotate {
            return Err(question_error(crate::core::diagnostic::requirement(
                "entries",
                "an annotate question",
                "another function",
            )));
        }
        if self.entries.contains_key(name) {
            return Err(name_error(crate::core::diagnostic::requirement(
                "names",
                "unique question names",
                "a repeated name (contents redacted)",
            )));
        }
        let child = questions::AnnotatableQuestion::try_from(SimpleQuestion::try_from(child)?)?;
        let mut entries = self.entries.clone();
        entries.insert(name.into(), child);
        questions::Annotate::new(entries.clone())?;
        self.entries = entries;
        Ok(())
    }
    /// Open an immutable snapshot through the canonical constructors.
    /// # Errors
    /// Returns the shared required-list or leaf admission error.
    pub fn build(&self) -> Result<Question, Error> {
        let threshold = self.threshold.unwrap_or_default();
        let cut = threshold.as_cut();
        macro_rules! finish {
            ($variant:ident,$value:expr) => {{
                let value = $value;
                Question::$variant(match &self.evidence {
                    Some(pointer) => value.evidence(pointer.clone()),
                    None => value,
                })
            }};
        }
        Ok(match self.function {
            FunctionName::Decide => finish!(
                Decide,
                questions::Decide::new(&self.text)?.threshold(threshold)
            ),
            FunctionName::Filter => finish!(
                Filter,
                questions::Filter::new(&self.text)?.threshold(threshold)
            ),
            FunctionName::Choose => finish!(
                Choose,
                questions::Choose::new(&self.text, self.items.clone())?.threshold(threshold)
            ),
            FunctionName::Tag => finish!(
                Tag,
                questions::Tag::new(&self.text, self.items.clone())?.threshold(cut?)
            ),
            FunctionName::Score => finish!(
                Score,
                questions::Score::new(&self.text, self.items.clone())?
            ),
            FunctionName::Rank => {
                let mut value = questions::Rank::new(&self.text, self.items.clone())?;
                value.top = self.top;
                finish!(Rank, value)
            }
            FunctionName::Find => finish!(Find, questions::Find::new(&self.text)?.threshold(cut?)),
            FunctionName::Recognize => finish!(
                Recognize,
                questions::Recognize::new(self.items.clone())?.threshold(cut?)
            ),
            FunctionName::Relate => finish!(
                Relate,
                questions::Relate::new(self.rules.clone())?.threshold(cut?)
            ),
            FunctionName::Annotate => {
                finish!(Annotate, questions::Annotate::new(self.entries.clone())?)
            }
        })
    }
}
/// Mutable named snapshots used only by typed native toolbox operations.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct ToolboxBuilder(BTreeMap<String, SimpleQuestion>);
impl ToolboxBuilder {
    /// Insert a snapshot without transferring or retaining the child handle.
    /// # Errors
    /// Rejects duplicate names and annotation while preserving this collection.
    pub fn insert(&mut self, name: &str, question: Question) -> Result<(), Error> {
        if self.0.contains_key(name) {
            return Err(name_error(crate::core::diagnostic::requirement(
                "names",
                "unique question names",
                "a repeated name (contents redacted)",
            )));
        }
        let mut entries = self.0.clone();
        entries.insert(name.into(), question.try_into()?);
        questions::Toolbox::new(entries.clone())?;
        self.0 = entries;
        Ok(())
    }
    /// Admit a canonical toolbox through the ordinary shared grammar.
    /// # Errors
    /// Returns shared question and name errors without executing a question.
    pub fn from_definitions_json(json: &str) -> Result<Self, Error> {
        let toolbox = questions::Toolbox::from_definitions_json(json)?;
        Ok(Self(
            toolbox
                .iter()
                .map(|(name, question)| (name.to_owned(), question.clone()))
                .collect(),
        ))
    }
    /// Load named questions from the command's saved toolbox format.
    /// # Errors
    /// Returns the shared saved question admission error.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        let toolbox = questions::Toolbox::from_json(json)?;
        Ok(Self(
            toolbox
                .iter()
                .map(|(name, question)| (name.to_owned(), question.clone()))
                .collect(),
        ))
    }
    /// Serialize this snapshot in the command's saved toolbox format.
    /// # Errors
    /// Rejects an empty toolbox or a name that cannot be saved.
    pub fn to_json(&self) -> Result<String, Error> {
        questions::Toolbox::new(self.0.clone())?.to_json()
    }
    /// Export admitted flat definitions for native host type hydration.
    /// # Errors
    /// Returns the shared question error for an empty collection.
    pub fn definitions_json(&self) -> Result<String, Error> {
        questions::Toolbox::new(self.0.clone())?.definitions_json()
    }
    /// Select a snapshot of the requested function.
    /// # Errors
    /// Returns the shared name error for an absent name or wrong function.
    pub fn select_as(&self, name: &str, function: FunctionName) -> Result<QuestionBuilder, Error> {
        let question = self.select(name)?;
        if question.function() != function {
            return Err(name_error(crate::core::diagnostic::requirement(
                "name",
                "a question of the requested function",
                "a question of another function",
            )));
        }
        Ok(question)
    }
    /// Select an independent snapshot that outlives the toolbox.
    /// # Errors
    /// Returns a shared name error when the name is absent.
    pub fn select(&self, name: &str) -> Result<QuestionBuilder, Error> {
        self.0
            .get(name)
            .cloned()
            .map(Question::from)
            .map(|question| QuestionBuilder::snapshot(&question))
            .ok_or_else(|| {
                name_error(crate::core::diagnostic::requirement(
                    "name",
                    "an existing named question",
                    "an unknown name (contents redacted)",
                ))
            })
    }
}
