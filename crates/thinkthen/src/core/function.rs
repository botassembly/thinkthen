//! The functions. Each turns evidence into asks and reads the answers into a row.

pub(crate) mod annotate;
pub(crate) mod choose;
pub(crate) mod decide;
pub(crate) mod filter;
pub(crate) mod find;
pub(crate) mod identity;
pub(crate) mod rank;
pub(crate) mod recognize;
pub(crate) mod relate;
pub(crate) mod score;
pub(crate) mod tag;

/// `text` quoted as a JSON string, then `question`, as every function asks it.
#[expect(clippy::expect_used, reason = "a string always serializes")]
fn quoted(text: &str, question: &str) -> String {
    let text = serde_json::to_string(text).expect("a string serializes");
    format!("The text is {text}. {question}")
}

#[expect(
    clippy::expect_used,
    reason = "admitted built-in question fields serialize infallibly"
)]
pub(crate) fn identity(function: &str, question: &impl serde::Serialize) -> String {
    use sha2::{Digest, Sha256};
    let bytes = serde_json::to_vec(&(function, question)).expect("admitted question serializes");
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn stub_key(identity: &str, evidence: &str, round: usize, position: usize) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hash = Sha256::new();
    for field in [
        identity.as_bytes(),
        evidence.as_bytes(),
        &(round as u64).to_be_bytes(),
        &(position as u64).to_be_bytes(),
    ] {
        hash.update((field.len() as u64).to_be_bytes());
        hash.update(field);
    }
    hash.finalize().into()
}

/// A question that prepares rounds and reads the answers for one record.
pub trait Function: Sync + Sized {
    /// The caller-visible function name.
    const NAME: &'static str;

    /// Content identity independent of backend, model and internal prompt wording.
    #[must_use]
    fn identity(&self) -> String {
        self.identity_as(Self::NAME)
    }

    /// Content identity with the caller's operation, such as filtering.
    /// Implementations hash caller-visible wording, options and thresholds;
    /// routing and internal prompt templates must not enter the identity.
    #[must_use]
    fn identity_as(&self, function: &str) -> String;

    /// Deterministic caller-test key for one ask in this question's local round.
    /// Composite questions delegate to the member's own content and evidence.
    #[must_use]
    fn stub_key(
        context: FunctionAnswers<'_, Self>,
        identity: &str,
        _limits: crate::core::Limits,
        position: usize,
    ) -> [u8; 32] {
        stub_key(
            identity,
            context.record().evidence(),
            context.answers().len(),
            position,
        )
    }

    /// Prepare this admitted record's initial asks for the selected limits.
    ///
    /// # Errors
    /// Returns this record's preparation error.
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<crate::core::Ask>, crate::core::Error>;

    /// Prepare a dependent round using the original question and matched answers.
    ///
    /// # Errors
    /// Returns this record's preparation or answer error.
    fn followup_record(
        context: FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<crate::core::Ask>, crate::core::Error> {
        let _ = (context, limits);
        Ok(Vec::new())
    }

    /// Read this question's matched record and complete answer history.
    fn read_record(
        context: FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> crate::core::Outcome;
}

/// An admitted record and its answers, bound to the question that asked them.
///
/// The packer constructs this context after matching every reply to its ask.
/// Callers cannot construct it, replace its question or inject answer vectors.
///
/// ```compile_fail
/// use thinkthen::core::{AdmittedRecord, Decide, FunctionAnswers, Record};
/// let question = Decide::new("Yes?").unwrap();
/// let record = AdmittedRecord::new(Record::text("evidence")).unwrap();
/// let forged = FunctionAnswers::new(&question, &record, &[]);
/// ```
#[derive(Debug)]
pub struct FunctionAnswers<'a, F: Function> {
    question: &'a F,
    record: &'a crate::core::AdmittedRecord,
    answers: &'a [Result<crate::core::AskAnswer, crate::core::Error>],
}

impl<'a, F: Function> FunctionAnswers<'a, F> {
    pub(crate) const fn new(
        question: &'a F,
        record: &'a crate::core::AdmittedRecord,
        answers: &'a [Result<crate::core::AskAnswer, crate::core::Error>],
    ) -> Self {
        Self {
            question,
            record,
            answers,
        }
    }

    /// The original question, with the configuration used to prepare the asks.
    #[must_use]
    pub const fn question(&self) -> &'a F {
        self.question
    }

    /// The admitted record that supplied the evidence.
    #[must_use]
    pub const fn record(&self) -> &'a crate::core::AdmittedRecord {
        self.record
    }

    /// One decoded answer or error for every ask, in round order.
    pub const fn answers(&self) -> &'a [Result<crate::core::AskAnswer, crate::core::Error>] {
        self.answers
    }

    pub(crate) const fn for_function<G: Function>(
        &self,
        question: &'a G,
    ) -> FunctionAnswers<'a, G> {
        FunctionAnswers::new(question, self.record, self.answers)
    }
}

/// Read the sole answer delivered for a single-ask question.
#[expect(
    clippy::expect_used,
    reason = "the matched history contains the question's sole answer"
)]
fn single(
    answers: &[Result<crate::core::AskAnswer, crate::core::Error>],
) -> Result<&crate::core::AskAnswer, crate::core::Error> {
    answers
        .first()
        .expect("one ask")
        .as_ref()
        .map_err(Clone::clone)
}
