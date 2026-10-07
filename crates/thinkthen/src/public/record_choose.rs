//! A choose reading whose whole candidate list comes from each record.
use crate::core::{self, Cutting, QuestionFile, Typed, Verb};
use crate::public::question::{Kind, cut, model_of, text_of};
use crate::public::{
    Call, CallOptions, CompleteChoice, CompleteRecord, Engine, Error, InputEvidence, Question,
    QuestionContent, RecordInput, RecordOptions, ResolvedThreshold,
};
use std::fmt;
use std::path::Path;

/// A concrete choose reading awaiting an explicit whole shortlist per record.
/// It is not an executable primitive until those candidates have been admitted.
#[derive(Clone)]
pub struct RecordChooseQuestion {
    pub(super) metadata: core::declaration::QuestionMetadata,
    text: core::QuestionText,
    threshold: Option<core::Threshold>,
    pub(super) model: Option<core::ModelName>,
    profile: Option<core::ProfileName>,
    pub(super) batch: Option<core::Json>,
}
impl Question {
    /// Prepare choose with no fixed candidates; every input supplies its own list.
    /// # Errors
    /// Refuses blank question text.
    pub fn choose_records(text: &str) -> Result<RecordChooseQuestion, Error> {
        Ok(RecordChooseQuestion {
            metadata: core::declaration::QuestionMetadata::default(),
            text: text_of(text)?,
            threshold: None,
            model: None,
            profile: None,
            batch: None,
        })
    }
}
impl RecordChooseQuestion {
    /// Admit ordinary saved choose text, rules, model, profile and batch.
    /// Record candidates replace the entire saved list, when one was authored.
    /// # Errors
    /// Refuses another verb or invalid question grammar. Authored pointers use native record selection.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let (file, batch) = QuestionFile::parse_top(text).map_err(Error::refused)?;
        let typed = Typed {
            options_from_record: true,
            cutting: Cutting::AsTheVerbAllows,
            ..Typed::default()
        };
        let resolved =
            core::resolve(Verb::Choose, None, Some(&file), &typed).map_err(Error::refused)?;
        Ok(Self {
            metadata: file.metadata.clone(),
            text: resolved.text().clone(),
            threshold: resolved.threshold(),
            model: (!resolved.sources().model_is_default()).then(|| resolved.model().clone()),
            profile: resolved.profile().cloned(),
            batch,
        })
    }
    /// Load an ordinary choose file with the shared capped question-file reader.
    /// # Errors
    /// Local errors retain the normal question-file boundary and withhold its contents.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = crate::public::question_file::load_text(path.as_ref(), "question file")?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
    /// Require the leading candidate to reach this cut.
    /// # Errors
    /// Refuses a cut outside the existing choose range.
    pub fn cut_at(mut self, value: f64) -> Result<Self, Error> {
        self.threshold = Some(cut(value)?);
        Ok(self)
    }
    /// Select one explicit model without changing the engine's endpoint or key.
    /// # Errors
    /// Refuses a blank or already named model.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        model_of(&mut self.model, value)?;
        Ok(self)
    }
    /// Replace a saved or previously selected model with an explicit caller model.
    /// The endpoint, credentials and remaining question preparation stay intact.
    /// # Errors
    /// Refuses an invalid or blank model.
    pub fn with_model_override(mut self, value: &str) -> Result<Self, Error> {
        self.model = None;
        model_of(&mut self.model, value)?;
        Ok(self)
    }
    /// Use evidence already selected by native record composition; retain all other preparation.
    pub(crate) fn without_authored_on(mut self) -> Self {
        self.metadata.reading.on.clear();
        self
    }
    /// Exact authored question content, including structured saved questions.
    #[must_use]
    pub fn text(&self) -> QuestionContent<'_> {
        QuestionContent(self.text.as_json())
    }
    /// Actual resolved cut, absent when none was selected.
    #[must_use]
    pub fn threshold(&self) -> Option<ResolvedThreshold> {
        self.threshold.map(ResolvedThreshold::of)
    }
    pub(super) fn with_options(&self, options: &RecordOptions) -> Result<Question, Error> {
        Ok(Question {
            metadata: self.metadata.clone(),
            core: core::Question::Choose {
                text: self.text.clone(),
                options: options.labels()?,
            },
            threshold: self.threshold,
            model: self.model.clone(),
            profile: self.profile.clone(),
            batch: self.batch.clone(),
            kind: Kind::Choose,
        })
    }
}
impl fmt::Debug for RecordChooseQuestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordChooseQuestion")
            .field("threshold", &self.threshold)
            .finish_non_exhaustive()
    }
}
impl Engine {
    /// Execute choose using a mandatory whole ordered shortlist for every original.
    /// # Errors
    /// A missing/invalid later list refuses the entire eager set before sending.
    #[allow(
        clippy::type_complexity,
        reason = "Each original retains its concrete complete choice"
    )]
    pub fn choose_dynamic_records_complete_with<I, T>(
        &self,
        question: &RecordChooseQuestion,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        crate::public::bulk::selected_batch_file(question.batch.as_ref(), &options, self.batch)?;
        let records = self.within_limit(records)?.collect::<Vec<_>>();
        for (at, record) in records.iter().enumerate() {
            if record.options.is_none() {
                return Err(
                    Error::usage("record choose requires candidates on every original")
                        .at_record(at),
                );
            }
        }
        let Some(first) = records.first() else {
            let stop = crate::public::options::Stop::begin(options)?.with_prices(self.prices);
            return stop.run_call(0, |_| Ok(Vec::new()));
        };
        let options_for_first = first
            .options
            .as_ref()
            .ok_or_else(|| Error::defect("admitted record choose lost its candidates"))?;
        self.choose_records_complete_with(
            &question.with_options(options_for_first)?,
            records,
            options,
        )
    }
}
