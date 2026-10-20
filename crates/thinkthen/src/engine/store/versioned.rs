//! Validate all constituents before normalization, lookup or any paid send.

use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Deserialize;

use super::{Answer, Entries, JSONL};
use crate::core::adapters::ApiType;
use crate::core::pack::{self, QuestionKey, State, model_json};
use crate::core::{AnswerOutcome, Json, ModelName, Url, bytes_sha256, hex};
use crate::engine::error::Error;

pub(super) fn invalid() -> Error {
    Error::Entry(
        JSONL.to_owned(),
        "stored constituents or identity are invalid".to_owned(),
    )
}

/// The common validator also reads SQLite snapshots; retain their actual source.
pub(super) fn sqlite_error(error: Error) -> Error {
    match error {
        Error::Entry(_, why) => Error::Entry(super::SQLITE.to_owned(), why),
        Error::StoreVersion(_) => Error::StoreVersion(super::SQLITE.to_owned()),
        other => other,
    }
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
            let key = complete_key(&answer, state, &answer.answered_by)?;
            let url = crate::core::posting_address(&answer.url).map_err(|_| invalid())?;
            if answer
                .adapter
                .as_deref()
                .and_then(ApiType::from_name)
                .is_none()
                || answer.observation_id.is_none()
                || key.hex() != answer.key
            {
                return Err(invalid());
            }
            answer.url = url.as_str().to_owned();
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
    complete_key(answer, state, &answer.model)
}

pub(super) fn complete_key(
    answer: &Answer,
    state: &str,
    reported: &str,
) -> Result<QuestionKey, Error> {
    let (question, _) = canonical_parts(answer, state)?;
    // State identity, already validated above, distinguishes explicit images;
    // ordinary JSON with an image-looking shape remains ordinary evidence.
    let images = bytes_sha256(state.as_bytes()) != answer.state;
    let state = canonical_state(&answer.state, state)?;
    let url = crate::core::posting_address(&answer.url).map_err(|_| invalid())?;
    let model = model_json(&answer.model).map_err(|_| invalid())?;
    let reported = model_json(reported).map_err(|_| invalid())?;
    let constructor = if images {
        QuestionKey::complete_image
    } else {
        QuestionKey::complete
    };
    let api = adapter(answer)?;
    Ok(if api == ApiType::Primary {
        constructor(&url, &model, &reported, &state, &question)
    } else {
        if images {
            return Err(invalid());
        }
        QuestionKey::complete_for(api, &url, &model, &reported, &state, &question)
    })
}

pub(super) fn canonical_parts(answer: &Answer, state: &str) -> Result<(String, String), Error> {
    ModelName::new(&answer.model).map_err(|_| invalid())?;
    ModelName::reported(answer.answered_by.clone()).map_err(|_| invalid())?;
    Url::new(&answer.url).map_err(|_| invalid())?;
    if !matches!(
        answer.origin.as_str(),
        "live" | "converted" | "quoted" | "fixture"
    ) {
        return Err(invalid());
    }
    canonical_state(&answer.state, state)?;
    let api = adapter(answer)?;
    let (question, decoder) = api
        .canonical_question(&answer.question)
        .ok_or_else(invalid)?;
    let outcomes = pack::read_for(api, &[decoder], &[Ok(&answer.answer)], &answer.answered_by)
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

fn adapter(answer: &Answer) -> Result<ApiType, Error> {
    answer
        .adapter
        .as_deref()
        .and_then(ApiType::from_name)
        .ok_or_else(invalid)
}
