//! Content-only question roles reuse native grammar after the host reads references.
use super::usage;
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use thinkthen::{
    Error, FindQuestionFile, LoadedQuestion, Question, QuestionSet, RankSet, Recognize,
    RecognizeQuestionFile, RecordChooseQuestion, RecordReading, Relate,
};

#[derive(Debug)]
pub enum Prepared {
    Atomic(LoadedQuestion),
    Dynamic(RecordChooseQuestion),
    Filter(Question),
    Rank(Question),
    RankSet(RankSet),
    Find(Question, RecordReading),
    Annotate(QuestionSet),
    Recognize(Recognize, RecordReading),
    Relate(Relate),
}
impl Prepared {
    pub fn parse(verb: &str, source: &str) -> Result<Self, Error> {
        if source.starts_with('@') {
            return Err(usage(
                "the question reference was not read by this database",
            ));
        }
        Self::content(verb, source)
    }
    /// Parse only supplied content, never an embedded file reference.
    pub fn content(verb: &str, source: &str) -> Result<Self, Error> {
        match verb {
            "annotate" => QuestionSet::from_json(source).map(Self::Annotate),
            "rank" => {
                let fields: BTreeMap<String, Box<RawValue>> = serde_json::from_str(source)
                    .map_err(|_| usage("rank requires saved decide, score or question set JSON"))?;
                if fields.contains_key("questions") {
                    RankSet::from_json(source).map(Self::RankSet)
                } else {
                    Question::rank_from_json(source).map(Self::Rank)
                }
            }
            "find" => FindQuestionFile::from_json(source).map(|q| {
                let (q, r) = q.into_parts();
                Self::Find(q, r)
            }),
            "recognize" => RecognizeQuestionFile::from_json(source).map(|q| {
                let (q, r) = q.into_parts();
                Self::Recognize(q, r)
            }),
            "relate" => Relate::from_records_json(source).map(Self::Relate),
            "choose" => {
                let fields: BTreeMap<String, Box<RawValue>> = serde_json::from_str(source)
                    .map_err(|_| usage("choose requires question JSON"))?;
                if fields.contains_key("options") {
                    Question::from_json(source).map(Self::Atomic)
                } else {
                    RecordChooseQuestion::from_json(source).map(Self::Dynamic)
                }
            }
            "filter" => match Question::from_json(source)? {
                LoadedQuestion::Question(q) => Ok(Self::Filter(q)),
                _ => Err(usage("filter requires a decide question")),
            },
            "decide" | "score" | "tag" => Question::from_json(source).map(Self::Atomic),
            _ => Err(usage("unknown SQL complete function")),
        }
    }
    pub fn configured(mut self, verb: &str, settings: &thinkthen::Settings) -> Result<Self, Error> {
        let expected = match verb {
            "decide" | "filter" => Some(thinkthen::QuestionKind::Decide),
            "choose" => Some(thinkthen::QuestionKind::Choose),
            "score" => Some(thinkthen::QuestionKind::Score),
            "tag" => Some(thinkthen::QuestionKind::Tag),
            _ => None,
        };
        match &mut self {
            Self::Atomic(q) => match q {
                LoadedQuestion::Question(q) if Some(q.kind()) != expected => {
                    return Err(usage("the question uses another function"));
                }
                LoadedQuestion::Banded(_) if verb != "decide" => {
                    return Err(usage("this function takes no band"));
                }
                _ => {}
            },
            Self::Filter(q) if Some(q.kind()) != expected => {
                return Err(usage("filter requires decide"));
            }
            Self::Find(q, _) if settings.none() == Some(true) => {
                *q = q.clone().offering_none()?;
            }
            _ => {}
        }
        let check = match verb {
            "decide" | "filter" => Some(thinkthen::For::Decide),
            "choose" => Some(thinkthen::For::Choose),
            "score" => Some(thinkthen::For::Score),
            "tag" => Some(thinkthen::For::Tag),
            "find" => Some(thinkthen::For::Find),
            "rank" => Some(thinkthen::For::Rank),
            _ => Some(thinkthen::For::Rank),
        };
        if let Some(check) = check {
            settings.check(check).map_err(|e| usage(&e.to_string()))?;
        }
        if let Some(model) = settings.model() {
            match &mut self {
                Self::Rank(q) | Self::Find(q, _) => *q = q.clone().with_model(model)?,
                Self::Recognize(_, _) | Self::Relate(_) | Self::Annotate(_) | Self::RankSet(_) => {
                    return Err(usage(
                        "aggregate model selection uses the host engine settings",
                    ));
                }
                _ => {}
            }
        }
        Ok(self)
    }
    pub fn reading(&self) -> Option<&RecordReading> {
        match self {
            Self::Find(_, r) | Self::Recognize(_, r) => Some(r),
            _ => None,
        }
    }
}
