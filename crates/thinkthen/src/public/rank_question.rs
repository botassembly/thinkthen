//! Native described and saved rank questions use the existing shared resolver.

use std::path::Path;

use super::question::{Kind, text_of};
use super::{Description, Error, Question};
use crate::core::{Cutting, QuestionFile, Typed, Verb, resolve};

impl Question {
    /// Rank by a described yes/no criterion, without an authored cut or band.
    ///
    /// # Errors
    /// Returns a usage error for blank text or a meaning outside the decide grammar.
    pub fn rank_described(
        text: &str,
        yes: Option<Description>,
        no: Option<Description>,
    ) -> Result<Self, Error> {
        Ok(Self::yes_no(
            text_of(text)?,
            yes.as_ref().map(Description::meaning).transpose()?,
            no.as_ref().map(Description::meaning).transpose()?,
            None,
            Kind::Rank,
        ))
    }

    /// Admit a saved described decide or score question for ranking.
    ///
    /// # Errors
    /// Returns a usage error for authored thresholds, another verb or invalid evidence pointers.
    pub fn rank_from_json(text: &str) -> Result<Self, Error> {
        let (file, batch) = QuestionFile::parse_top(text).map_err(Error::refused)?;
        if !matches!(file.verb(), Verb::Decide | Verb::Score) {
            return Err(Error::usage("rank takes a decide or score question"));
        }
        let typed = Typed {
            cutting: Cutting::NoRule,
            ..Typed::default()
        };
        let resolved = resolve(file.verb(), None, Some(&file), &typed).map_err(Error::refused)?;
        Ok(Self {
            metadata: file.metadata.clone(),
            core: resolved
                .question()
                .cloned()
                .ok_or_else(|| Error::defect("a rank question file resolved no question"))?,
            threshold: None,
            authored_threshold: false,
            model: (!resolved.sources().model_is_default()).then(|| resolved.model().clone()),
            profile: resolved.profile().cloned(),
            batch,
            kind: if file.verb() == Verb::Score {
                Kind::Score
            } else {
                Kind::Rank
            },
        })
    }

    /// Load a rank-specific saved decide or score question through the capped reader.
    ///
    /// # Errors
    /// Returns a local error if the file cannot be read or admitted.
    pub fn load_rank(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = super::question_file::load_text(path.as_ref(), "question file")?;
        Self::rank_from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
}
