//! Existing native preparation enters a request directly, without JSON execution.
use crate::{
    Error, FindQuestionFile, LoadedQuestion, Question, QuestionSet, RankSet, Recognize,
    RecognizeQuestionFile, RecordChooseQuestion, Relate,
};
use serde::{Deserialize, Serialize};

/// A prepared definition in the existing authored question grammar.
#[derive(Clone, Debug)]
pub enum RequestDefinition {
    /// A primitive question, including a banded decide reading.
    Atomic(LoadedQuestion),
    /// An ordered annotation set.
    Annotate(QuestionSet),
    /// An authored set with independent rank admission retained at decoding.
    DecodedSet {
        /// Ordinary annotation admission.
        annotation: QuestionSet,
        /// Rank admission without rewriting authored controls.
        rank: Option<RankSet>,
    },
    /// An ordered rank set.
    RankSet(RankSet),
    /// A rank question.
    Rank(Question),
    /// A find preparation retaining authored pointers.
    Find(FindQuestionFile),
    /// Recognition retaining authored pointers.
    Recognize(RecognizeQuestionFile),
    /// Direct native recognition preparation.
    Recognition(Recognize),
    /// A relate preparation.
    Relate(Relate),
    /// Choose with a per-record replacement shortlist.
    DynamicChoose(RecordChooseQuestion),
}
impl From<LoadedQuestion> for RequestDefinition {
    fn from(q: LoadedQuestion) -> Self {
        Self::Atomic(q)
    }
}
impl From<Question> for RequestDefinition {
    fn from(q: Question) -> Self {
        match q.kind() {
            crate::QuestionKind::Rank => Self::Rank(q),
            _ => Self::Atomic(LoadedQuestion::Question(q)),
        }
    }
}
impl From<QuestionSet> for RequestDefinition {
    fn from(q: QuestionSet) -> Self {
        Self::Annotate(q)
    }
}
impl From<RankSet> for RequestDefinition {
    fn from(q: RankSet) -> Self {
        Self::RankSet(q)
    }
}
impl From<FindQuestionFile> for RequestDefinition {
    fn from(q: FindQuestionFile) -> Self {
        Self::Find(q)
    }
}
impl From<RecognizeQuestionFile> for RequestDefinition {
    fn from(q: RecognizeQuestionFile) -> Self {
        Self::Recognize(q)
    }
}
impl From<Recognize> for RequestDefinition {
    fn from(q: Recognize) -> Self {
        Self::Recognition(q)
    }
}
impl From<Relate> for RequestDefinition {
    fn from(q: Relate) -> Self {
        Self::Relate(q)
    }
}
impl From<RecordChooseQuestion> for RequestDefinition {
    fn from(q: RecordChooseQuestion) -> Self {
        Self::DynamicChoose(q)
    }
}

impl<'de> Deserialize<'de> for RequestDefinition {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let value = crate::core::Json::deserialize(de)?;
        let text = crate::core::json_line(&value).map_err(serde::de::Error::custom)?;
        Self::parse(&value, &text).map_err(serde::de::Error::custom)
    }
}
impl RequestDefinition {
    pub(crate) fn from_authored_json(text: &str) -> Result<Self, Error> {
        let value = crate::core::Json::parse(text).map_err(Error::refused)?;
        Self::parse(&value, text)
    }
    fn parse(value: &crate::core::Json, text: &str) -> Result<Self, Error> {
        let crate::core::Json::Object(members) = value else {
            return Err(Error::usage("a definition is an authored question object"));
        };
        let has = |key: &str| members.iter().any(|(name, _)| name == key);
        if has("questions") {
            return Ok(Self::DecodedSet {
                annotation: QuestionSet::from_json(text)?,
                rank: RankSet::from_json(text).ok(),
            });
        }
        if has("rank") {
            return Question::rank_from_json(text).map(Self::Rank);
        }
        if has("find") {
            return FindQuestionFile::from_json(text).map(Self::Find);
        }
        if has("recognize") {
            return RecognizeQuestionFile::from_json(text).map(Self::Recognize);
        }
        if has("relate") {
            return Relate::from_records_json(text).map(Self::Relate);
        }
        if has("choose") && !has("options") {
            return RecordChooseQuestion::from_json(text).map(Self::DynamicChoose);
        }
        Question::from_json(text).map(Self::Atomic)
    }
    pub(super) fn json(&self) -> Result<crate::core::Json, Error> {
        let text = match self {
            Self::Atomic(q) => q.to_json()?,
            Self::Rank(q) => q.to_json()?,
            Self::Find(q) => q.question().to_json()?,
            Self::Recognize(q) => return recognition_json(q.question()),
            Self::Recognition(q) => return recognition_json(q),
            Self::Relate(q) => return relate_json(q),
            Self::Annotate(q) => return set_json(&q.0),
            Self::DecodedSet { annotation: q, .. } => return set_json(&q.0),
            Self::RankSet(q) => return set_json(&q.0),
            Self::DynamicChoose(q) => return dynamic_json(q),
        };
        let mut value = crate::core::Json::parse(&text).map_err(Error::refused)?;
        // `none` is a call control rather than authored find grammar.
        if let crate::core::Json::Object(fields) = &mut value {
            if matches!(self, Self::Atomic(LoadedQuestion::Question(q)) if !q.authored_threshold) {
                fields.retain(|(key, _)| key != "threshold");
            }
            if fields.iter().any(|(key, _)| key == "find") {
                fields.retain(|(key, _)| key != "none");
            }
        }
        Ok(value)
    }
}
impl Serialize for RequestDefinition {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.json().map_err(serde::ser::Error::custom)?.serialize(s)
    }
}
fn set_json(set: &crate::core::QuestionSet) -> Result<crate::core::Json, Error> {
    let mut questions = Vec::new();
    for member in set.questions() {
        let kind = match member.question() {
            crate::core::Question::Decide { .. } => super::super::question::Kind::Decide,
            crate::core::Question::Choose { .. } => super::super::question::Kind::Choose,
            crate::core::Question::Tag { .. } => super::super::question::Kind::Tag,
            crate::core::Question::Score { .. } => super::super::question::Kind::Score,
        };
        let q = Question {
            metadata: member.metadata().clone(),
            core: member.question().clone(),
            threshold: member.threshold(),
            authored_threshold: member.threshold().is_some(),
            model: None,
            profile: None,
            batch: None,
            kind,
        };
        questions.push((
            member.name().to_owned(),
            crate::core::Json::parse(&q.to_json()?).map_err(Error::refused)?,
        ));
    }
    let mut fields = vec![
        ("version".to_owned(), crate::core::Json::Number(1.into())),
        ("questions".to_owned(), crate::core::Json::Object(questions)),
    ];
    if let Some(profile) = set.profile() {
        fields.push((
            "profile".to_owned(),
            crate::core::Json::String(profile.as_str().to_owned()),
        ));
    }
    if let Some(batch) = set.batch() {
        fields.push(("batch".to_owned(), batch.clone()));
    }
    Ok(crate::core::Json::Object(fields))
}
#[cfg(test)]
mod schema;

fn encoded<T: Serialize>(value: &T) -> Result<crate::core::Json, Error> {
    let text = crate::core::json_line(value)
        .map_err(|_| Error::defect("definition could not be written"))?;
    crate::core::Json::parse(&text).map_err(Error::refused)
}
fn recognition_json(q: &Recognize) -> Result<crate::core::Json, Error> {
    use crate::core::Json;
    let Json::Object(document) = encoded(&q.0)? else {
        return Err(Error::defect("recognition definition lost its object"));
    };
    let mut fields = vec![("version".to_owned(), Json::Number(1.into()))];
    let mut recognize = Vec::new();
    for (key, value) in document {
        match key.as_str() {
            "verb" => {}
            "relation_threshold" => {}
            "threshold" | "profile" => fields.push((key, value)),
            _ => recognize.push((key, value)),
        }
    }
    if q.0.authored_relation_threshold {
        fields.push((
            "relation_threshold".to_owned(),
            encoded(&q.0.relation_threshold)?,
        ));
    }
    if q.0.authored_relations && q.0.relations.is_empty() {
        recognize.push(("relations".to_owned(), Json::Array(Vec::new())));
    }
    fields.push(("recognize".to_owned(), Json::Object(recognize)));
    if let Some(model) = &q.0.model {
        fields.push(("model".to_owned(), Json::String(model.as_str().to_owned())));
    }
    if !q.0.on.is_empty() {
        fields.push(("on".to_owned(), encoded(&q.0.on)?));
    }
    if let Json::Object(metadata) = encoded(&q.0.metadata)? {
        fields.extend(metadata);
    }
    Ok(Json::Object(fields))
}
fn relate_json(q: &Relate) -> Result<crate::core::Json, Error> {
    use crate::core::Json;
    let fields = vec![
        (
            "name".to_owned(),
            Json::String(q.0.name_field().as_str().to_owned()),
        ),
        (
            "kind".to_owned(),
            Json::String(q.0.kind_field().as_str().to_owned()),
        ),
    ];
    let relate = vec![
        ("fields".to_owned(), Json::Object(fields)),
        ("relations".to_owned(), encoded(&q.0.relations)?),
    ];
    let mut result = vec![
        ("version".to_owned(), Json::Number(1.into())),
        ("relate".to_owned(), Json::Object(relate)),
        ("threshold".to_owned(), encoded(&q.0.threshold)?),
    ];
    if let Some(model) = &q.0.model {
        result.push(("model".to_owned(), Json::String(model.as_str().to_owned())));
    }
    if let Some(profile) = &q.0.profile {
        result.push((
            "profile".to_owned(),
            Json::String(profile.as_str().to_owned()),
        ));
    }
    if let Json::Object(metadata) = encoded(&q.0.metadata)? {
        result.extend(metadata);
    }
    Ok(Json::Object(result))
}

fn dynamic_json(q: &RecordChooseQuestion) -> Result<crate::core::Json, Error> {
    use crate::core::Json;
    let mut fields = vec![("choose".to_owned(), q.text().0.clone())];
    if q.authored_threshold
        && let Some(rule) = q.threshold
    {
        fields.push(("threshold".into(), encoded(&rule)?));
    }
    if let Some(model) = &q.model {
        fields.push(("model".into(), Json::String(model.as_str().to_owned())));
    }
    if let Some(profile) = &q.profile {
        fields.push(("profile".into(), Json::String(profile.as_str().to_owned())));
    }
    if let Some(batch) = &q.batch {
        fields.push(("batch".into(), batch.clone()));
    }
    if !q.metadata.reading.on.is_empty() {
        fields.push(("on".into(), encoded(&q.metadata.reading.on)?));
    }
    if let Json::Object(metadata) = encoded(&q.metadata)? {
        fields.extend(metadata);
    }
    Ok(Json::Object(fields))
}
