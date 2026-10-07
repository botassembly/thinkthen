//! Question roles and explicit references reuse the native loaders and grammar.
use super::usage;
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use thinkthen::{
    Error, FindQuestionFile, LoadedQuestion, Question, QuestionSet, RankSet, Recognize,
    RecognizeQuestionFile, RecordChooseQuestion, RecordReading, Relate,
};

#[derive(Debug)]
pub(crate) enum Prepared {
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
    pub(crate) fn parse(verb: &str, source: &str) -> Result<Self, Error> {
        if let Some(name) = source.strip_prefix("@@") {
            return Self::named(verb, name);
        }
        if source.starts_with('@') {
            return Self::reference(verb, source);
        }
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
    fn named(verb: &str, source: &str) -> Result<Self, Error> {
        match verb {
            "annotate" => QuestionSet::load_named(source).map(Self::Annotate),
            "rank" => match RankSet::load_named(source) {
                Ok(set) => Ok(Self::RankSet(set)),
                Err(_) => Question::load_rank_named(source).map(Self::Rank),
            },
            "find" => FindQuestionFile::load_named(source).map(|q| {
                let (q, r) = q.into_parts();
                Self::Find(q, r)
            }),
            "recognize" => RecognizeQuestionFile::load_named(source).map(|q| {
                let (q, r) = q.into_parts();
                Self::Recognize(q, r)
            }),
            "relate" => Relate::load_records_named(source).map(Self::Relate),
            "choose" => match Question::load_named(source) {
                Ok(q) => Ok(Self::Atomic(q)),
                Err(_) => RecordChooseQuestion::load_named(source).map(Self::Dynamic),
            },
            "filter" => match Question::load_named(source)? {
                LoadedQuestion::Question(q) => Ok(Self::Filter(q)),
                _ => Err(usage("filter requires decide")),
            },
            "decide" | "score" | "tag" => Question::load_named(source).map(Self::Atomic),
            _ => Err(usage("unknown SQL complete function")),
        }
    }
    pub(crate) fn configured(
        mut self,
        verb: &str,
        settings: &thinkthen::Settings,
    ) -> Result<Self, Error> {
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
            _ => None,
        };
        if let Some(check) = check {
            settings.check(check).map_err(|e| usage(&e.to_string()))?;
        }
        Ok(self)
    }
    fn reference(verb: &str, source: &str) -> Result<Self, Error> {
        match verb {
            "annotate" => QuestionSet::load_reference(source).map(Self::Annotate),
            "rank" => match RankSet::load_reference(source) {
                Ok(set) => Ok(Self::RankSet(set)),
                Err(_) => Question::load_rank_reference(source).map(Self::Rank),
            },
            "find" => FindQuestionFile::load_reference(source).map(|q| {
                let (q, r) = q.into_parts();
                Self::Find(q, r)
            }),
            "recognize" => RecognizeQuestionFile::load_reference(source).map(|q| {
                let (q, r) = q.into_parts();
                Self::Recognize(q, r)
            }),
            "relate" => Relate::load_records_reference(source).map(Self::Relate),
            "choose" => match Question::load_reference(source) {
                Ok(q) => Ok(Self::Atomic(q)),
                Err(_) => RecordChooseQuestion::load_reference(source).map(Self::Dynamic),
            },
            "filter" => match Question::load_reference(source)? {
                LoadedQuestion::Question(q) => Ok(Self::Filter(q)),
                _ => Err(usage("filter requires a decide question")),
            },
            "decide" | "score" | "tag" => Question::load_reference(source).map(Self::Atomic),
            _ => Err(usage("unknown SQL complete function")),
        }
    }
    pub(crate) fn reading(&self) -> Option<&RecordReading> {
        match self {
            Self::Find(_, r) | Self::Recognize(_, r) => Some(r),
            _ => None,
        }
    }
}
