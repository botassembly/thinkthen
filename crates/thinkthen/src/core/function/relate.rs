//! Relations stated by the evidence, after shared name recognition.

use crate::core::{
    Answer, Ask, AskAnswer, ChoiceOption, Detail, Error, ErrorKind, Function, Name, Outcome,
    Recognize, Threshold, Verdict,
};
use serde::Deserialize;

/// An ordered, typed relation rule.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RelationRule {
    name: String,
    source: String,
    target: String,
    reads: String,
    either: bool,
}

/// Strict toolbox representation of a relation rule.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawRule {
    name: String,
    source: String,
    target: String,
    #[serde(default, deserialize_with = "read_words")]
    reads: Option<String>,
    #[serde(default)]
    either: bool,
}
fn read_words<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    String::deserialize(deserializer).map(Some)
}
impl RawRule {
    pub(crate) fn validate(self) -> Result<RelationRule, Error> {
        let rule = RelationRule::new(self.name, self.source, self.target)?.either(self.either);
        match self.reads {
            Some(reads) => rule.reads(reads),
            None => Ok(rule),
        }
    }
}

fn label(text: String) -> Result<String, Error> {
    ChoiceOption::new(text.clone())?;
    if text.contains(['*', '=', ':']) || text.chars().any(char::is_control) {
        return Err(Error::new(
            ErrorKind::Question,
            "relation names and kinds must be concrete text without separators or controls",
        ));
    }
    Ok(text)
}
impl RelationRule {
    /// Construct a directed rule with wording derived from its name.
    ///
    /// # Errors
    /// Returns usage.question for blank or malformed names and kinds.
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Result<Self, Error> {
        let name = label(name.into())?;
        Ok(Self {
            reads: name.replace('_', " "),
            name,
            source: label(source.into())?,
            target: label(target.into())?,
            either: false,
        })
    }
    /// Parse NAME=SOURCE:TARGET, with both concrete kinds required.
    ///
    /// # Errors
    /// Returns usage.question for malformed rules.
    pub fn parse(text: &str) -> Result<Self, Error> {
        let (name, sides) = text.split_once('=').ok_or_else(|| {
            Error::new(
                ErrorKind::Question,
                "a relation rule needs NAME=SOURCE:TARGET",
            )
        })?;
        let (source, target) = sides.split_once(':').ok_or_else(|| {
            Error::new(
                ErrorKind::Question,
                "a relation rule needs NAME=SOURCE:TARGET",
            )
        })?;
        Self::new(name, source, target)
    }
    /// Treat this rule as an unordered relation.
    #[must_use]
    pub fn either(mut self, either: bool) -> Self {
        self.either = either;
        self
    }
    /// Set the verb wording used in the pair question.
    ///
    /// # Errors
    /// Returns usage.question for blank wording.
    pub fn reads(mut self, reads: impl Into<String>) -> Result<Self, Error> {
        let reads = reads.into();
        if reads.trim().is_empty() {
            return Err(Error::new(ErrorKind::Question, "relation wording is blank"));
        }
        self.reads = reads;
        Ok(self)
    }
}

// Schema derives emit metadata maps in sibling impls, scoped to result values.
#[allow(
    clippy::disallowed_types,
    reason = "schema derivation builds metadata; evidence remains typed"
)]
mod output {
    use crate::core::{Name, Probability};
    use serde::Serialize;
    /// A relation endpoint in the original text.
    #[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
    pub struct RelationName {
        /// Original name text.
        pub text: String,
        /// Inclusive Unicode scalar offset.
        pub start: usize,
        /// Exclusive Unicode scalar offset.
        pub end: usize,
        /// Recognized kind.
        pub kind: String,
    }
    impl From<&Name> for RelationName {
        fn from(name: &Name) -> Self {
            Self {
                text: name.text.clone(),
                start: name.start,
                end: name.end,
                kind: name.kind.clone(),
            }
        }
    }
    /// One allowed pair and its probability of being stated in the text.
    #[derive(Debug, Clone, PartialEq, Serialize, schemars::JsonSchema)]
    pub struct Relation {
        /// The first endpoint, or directed source.
        pub source: RelationName,
        /// The second endpoint, or directed target.
        pub target: RelationName,
        /// The rule name.
        pub relation: String,
        /// Probability of yes for this pair.
        pub p: Probability,
        /// Present only for unordered rules.
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        pub either: bool,
    }
}
pub use output::{Relation, RelationName};
/// Relations stated in a text, using shared recognition rounds.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Relate {
    rules: Vec<RelationRule>,
    #[serde(skip)]
    recognize: Recognize,
    threshold: Threshold,
}
impl Relate {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Relations {
            rules: self
                .rules
                .iter()
                .map(|rule| super::identity::RuleDefinition {
                    either: rule.either,
                    name: &rule.name,
                    reads: &rule.reads,
                    source: &rule.source,
                    target: &rule.target,
                })
                .collect(),
            threshold: self.threshold.canonical(),
        }
    }

    /// Construct a question from nonempty, distinctly named rules.
    ///
    /// # Errors
    /// Returns usage.question for empty or duplicate rules or invalid kinds.
    pub fn new(rules: Vec<RelationRule>) -> Result<Self, Error> {
        if rules.is_empty()
            || rules.iter().enumerate().any(|(i, rule)| {
                rules
                    .iter()
                    .take(i)
                    .any(|earlier| earlier.name == rule.name)
            })
        {
            return Err(Error::new(
                ErrorKind::Question,
                "relate needs nonempty, distinctly named rules",
            ));
        }
        let mut kinds = Vec::new();
        for rule in &rules {
            for kind in [&rule.source, &rule.target] {
                if !kinds.contains(kind) {
                    kinds.push(kind.clone());
                }
            }
        }
        Ok(Self {
            recognize: Recognize::new(kinds)?,
            rules,
            threshold: Threshold::default(),
        })
    }
    /// Set the relation probability cut.
    ///
    /// # Errors
    /// Returns usage.threshold for a band.
    pub fn threshold(mut self, threshold: Threshold) -> Result<Self, Error> {
        self.threshold = threshold.require_cut()?;
        Ok(self)
    }

    fn recognized(
        &self,
        text: &str,
        answers: &[Result<AskAnswer, Error>],
    ) -> Result<(usize, Vec<Name>), Error> {
        let boundary = self.recognize.initial(text).len();
        let (first, remaining) = answers.split_at(boundary);
        let kind_count = self.recognize.kinds(text, first)?.len();
        let count = boundary + kind_count;
        let (kinds, _) = remaining.split_at(kind_count);
        let (names, _) = self.recognize.finish(text, first, kinds)?;
        Ok((count, names))
    }

    fn pairs(&self, names: &[Name]) -> Vec<(RelationName, RelationName, &RelationRule)> {
        let mut unique: Vec<&Name> = Vec::new();
        for name in names {
            if !unique
                .iter()
                .any(|held| held.text == name.text && held.kind == name.kind)
            {
                unique.push(name);
            }
        }
        let mut pairs = Vec::new();
        for rule in &self.rules {
            for (i, source) in unique.iter().enumerate() {
                for (j, target) in unique.iter().enumerate() {
                    let forward = source.kind == rule.source && target.kind == rule.target;
                    let reverse = source.kind == rule.target && target.kind == rule.source;
                    if if rule.either {
                        i < j && (forward || reverse)
                    } else {
                        i != j && forward
                    } {
                        pairs.push((
                            RelationName::from(*source),
                            RelationName::from(*target),
                            rule,
                        ));
                    }
                }
            }
        }
        pairs
    }
    fn finish(&self, text: &str, answers: &[Result<AskAnswer, Error>]) -> Result<Outcome, Error> {
        let (count, names) = self.recognized(text, answers)?;
        let pairs = self.pairs(&names);
        let (_, probabilities) = answers.split_at(count);
        let mut detail = Vec::new();
        for ((source, target, rule), answer) in pairs.into_iter().zip(probabilities) {
            let p = answer.as_ref().map_err(Clone::clone)?.yes();
            detail.push(Relation {
                source,
                target,
                relation: rule.name.clone(),
                p,
                either: rule.either,
            });
        }
        let kept = detail
            .iter()
            .filter(|pair| self.threshold.verdict(pair.p) == Verdict::Yes)
            .cloned()
            .collect();
        Ok(Outcome::Answered {
            answer: Answer::Relations(kept),
            p: None,
            detail: Some(Detail::Relate(detail)),
        })
    }
}
impl Function for Relate {
    const NAME: &'static str = "relate";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        self.recognize.try_asks_record(record, limits)
    }
    #[expect(
        clippy::expect_used,
        reason = "typed endpoints contain only serializable strings and offsets"
    )]
    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        let evidence = record.evidence();
        if answers.len() == question.recognize.initial(evidence).len() {
            return Function::followup_record(context.for_function(&question.recognize), limits);
        }
        let (count, names) = question.recognized(evidence, answers)?;
        if answers.len() > count {
            return Ok(Vec::new());
        }
        let state: std::sync::Arc<str> = evidence.into();
        Ok(question.pairs(&names).into_iter().map(|(source, target, rule)| {
            let one = serde_json::to_string(&source).expect("an endpoint serializes");
            let two = serde_json::to_string(&target).expect("an endpoint serializes");
            let reads = &rule.reads;
            let question = if rule.either { format!("Does the text itself state that {one} {reads} {two}, or that {two} {reads} {one}?") } else { format!("Does the text itself state that {one} {reads} {two}?") };
            Ask::YesNo { instructions: super::quoted(evidence, &question), yes: None, no: None }.with_state(state.clone())
        }).collect())
    }
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        let evidence = record.evidence();
        question
            .finish(evidence, answers)
            .unwrap_or_else(|error| Outcome::Failed { error })
    }
}
