//! Validate all constituents before normalization, lookup or any paid send.

use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Deserialize;

use super::{Answer, Entries, JSONL};
use crate::core::adapters::built_in;
use crate::core::pack::{self, QuestionKey, State, model_json};
use crate::core::{AnswerOutcome, Json, ModelName, ObservationId, Url, bytes_sha256, hex};
use crate::engine::error::Error;

pub(super) fn invalid() -> Error {
    Error::Entry(
        JSONL.to_owned(),
        "stored constituents or identity are invalid".to_owned(),
    )
}

pub(super) fn legacy_id(
    answer: &Answer,
    state: &str,
    source: Option<&Json>,
) -> Result<ObservationId, Error> {
    let (_, canonical_answer) = canonical_parts(answer, state)?;
    let present = |name| source.is_none_or(|source| source.member(name).is_some());
    let metadata = crate::core::LegacyMetadata {
        answered_by: Some(&answer.answered_by),
        input_tokens: present("input_tokens").then_some(answer.input_tokens),
        output_tokens: present("output_tokens").then_some(answer.output_tokens),
        taken_at: Some(answer.taken_at),
        origin: Some(&answer.origin),
    };
    crate::core::legacy_observation(&answer.key, &canonical_answer, &metadata)
        .map_err(|_| invalid())
}

impl Entries {
    /// Normalization is pure over the fully read snapshot; collisions refuse before mutation.
    pub(super) fn normalized(mut self) -> Result<Self, Error> {
        for original in self.originals.values() {
            original.validate()?;
        }
        for (digest, state) in &self.states {
            canonical_state(digest, state)?;
        }
        let mut normalized = BTreeMap::new();
        for (_, mut answer) in self.answers {
            let state = self.states.get(&answer.state).ok_or_else(invalid)?;
            let legacy = super::fixture::key_of(&answer, state).ok_or_else(invalid)?;
            let version = answer.key_version.unwrap_or(1);
            if !matches!(version, 1 | 2)
                || answer
                    .adapter
                    .as_deref()
                    .is_some_and(|adapter| adapter != built_in::NAME)
            {
                return Err(invalid());
            }
            if version == 1 && legacy.hex() != answer.key {
                return Err(invalid());
            }
            let (question, _) = canonical_parts(&answer, state)?;
            let canonical_state = canonical_state(&answer.state, state)?;
            let url = crate::core::posting_address(&answer.url).map_err(|_| invalid())?;
            let model = model_json(&answer.model).map_err(|_| invalid())?;
            let reported = model_json(&answer.answered_by).map_err(|_| invalid())?;
            let key = QuestionKey::complete(&url, &model, &reported, &canonical_state, &question);
            if version == 1 && answer.observation_id.is_none() {
                answer.observation_id = Some(legacy_id(&answer, state, None)?);
            }
            if version == 2
                && (answer.adapter.as_deref() != Some(built_in::NAME)
                    || answer.observation_id.is_none()
                    || key.hex() != answer.key)
            {
                return Err(invalid());
            }
            answer.key = key.hex();
            answer.url = url.as_str().to_owned();
            answer.key_version = Some(2);
            answer.adapter = Some(built_in::NAME.to_owned());
            if let Some(held) = normalized.insert(answer.key.clone(), answer.clone())
                && held != answer
            {
                return Err(invalid());
            }
        }
        self.answers = normalized;
        Ok(self)
    }
}

pub(super) fn lookup_key(answer: &Answer, state: &str) -> Result<QuestionKey, Error> {
    let (question, _) = canonical_parts(answer, state)?;
    let state = canonical_state(&answer.state, state)?;
    let url = crate::core::posting_address(&answer.url).map_err(|_| invalid())?;
    let model = model_json(&answer.model).map_err(|_| invalid())?;
    Ok(QuestionKey::complete(
        &url, &model, &model, &state, &question,
    ))
}

pub(super) fn canonical_parts(answer: &Answer, state: &str) -> Result<(String, String), Error> {
    ModelName::new(&answer.model).map_err(|_| invalid())?;
    ModelName::reported(answer.answered_by.clone()).map_err(|_| invalid())?;
    Url::new(&answer.url).map_err(|_| invalid())?;
    if !matches!(answer.origin.as_str(), "live" | "converted" | "quoted") {
        return Err(invalid());
    }
    canonical_state(&answer.state, state)?;
    let (question, decoder) = built_in::canonical_question(&answer.question).ok_or_else(invalid)?;
    let outcomes = pack::read(&[decoder], &[Ok(&answer.answer)], &answer.answered_by)
        .map_err(|_| invalid())?;
    let [AnswerOutcome::Answered(decoded)] = outcomes.as_slice() else {
        return Err(invalid());
    };
    let canonical_answer = serde_json::to_string(decoded).map_err(|_| invalid())?;
    Ok((question, canonical_answer))
}

pub(super) fn canonical_state(digest: &str, state: &str) -> Result<String, Error> {
    if bytes_sha256(state.as_bytes()) == digest {
        let state = Json::parse(state).map_err(|_| invalid())?;
        return serde_json::to_string(&state).map_err(|_| invalid());
    }
    if hex(&State::image_sha256(state)) != digest {
        return Err(invalid());
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Image {
        media: crate::core::image::ImageMedia,
        base64: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Stored {
        schema: String,
        text: Json,
        images: Vec<Image>,
    }
    Json::parse(state).map_err(|_| invalid())?;
    let stored: Stored = serde_json::from_str(state).map_err(|_| invalid())?;
    if stored.schema != "thinkthen.image-state/1" {
        return Err(invalid());
    }
    let images = stored
        .images
        .into_iter()
        .map(|image| {
            let bytes = STANDARD.decode(&image.base64).map_err(|_| invalid())?;
            if STANDARD.encode(&bytes) != image.base64 {
                return Err(invalid());
            }
            let (width, height) =
                crate::engine::image::decode(image.media, &bytes).map_err(|_| invalid())?;
            Ok(crate::core::image::Image {
                media: image.media,
                bytes: bytes.into(),
                width,
                height,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    // Reuse the edge's total-byte/count validation and the established core serializer.
    crate::engine::image::validate_set(
        images.iter().map(|image| image.bytes.len()),
        stored.text.as_str().map_or(0, str::len),
    )
    .map_err(|_| invalid())?;
    let restored = crate::core::image::ImageState {
        text: stored.text,
        images: images.into(),
    };
    serde_json::to_string(&restored).map_err(|_| invalid())
}
