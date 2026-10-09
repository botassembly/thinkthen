//! Typed boundary proposals preserve decoded spans without assigning a kind.
use super::{Recognized, RecognizedEntity, Relation};
use crate::core;

/// One unclassified proposal; offsets count Unicode scalar values.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundaryProposal(pub(crate) core::BoundaryProposal);
impl BoundaryProposal {
    /// Original text, including decoded punctuation.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.0.text
    }
    /// First Unicode scalar position.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.0.start
    }
    /// Position after the final Unicode scalar.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.0.end
    }
    /// Scalar span length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.0.length
    }
    /// P(span) over valid paths, rounded to four decimal places.
    #[must_use]
    pub const fn probability(&self) -> f64 {
        self.0.probability
    }
}

/// The selected recognition value; proposals carry no entity kind or strength.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RecognitionValue<'a> {
    /// Classified entities and any declared relations.
    Whole {
        /// Settled classified names.
        entities: &'a [RecognizedEntity],
        /// Asked relation results, absent without rules.
        relations: Option<&'a [Relation]>,
    },
    /// Decoded unclassified proposals after the probability cut.
    BoundaryOnly(&'a [BoundaryProposal]),
}
impl Recognized {
    /// The resolved execution mode.
    #[must_use]
    pub const fn mode(&self) -> crate::RecognitionMode {
        if self.proposals.is_some() {
            crate::RecognitionMode::BoundaryOnly
        } else {
            crate::RecognitionMode::Whole
        }
    }
    /// Boundary proposals, absent for whole recognition.
    #[must_use]
    pub fn proposals(&self) -> Option<&[BoundaryProposal]> {
        self.proposals.as_deref()
    }
    /// Borrow the selected typed value without decoding JSON.
    #[must_use]
    pub fn value(&self) -> RecognitionValue<'_> {
        match &self.proposals {
            Some(proposals) => RecognitionValue::BoundaryOnly(proposals),
            None => RecognitionValue::Whole {
                entities: &self.entities,
                relations: self.relations.as_deref(),
            },
        }
    }
}
