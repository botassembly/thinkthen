//! Closed optional image declaration, independent of transport selection.

use crate::core::adapters::built_in::images::local::ProfileId;
use crate::core::json::Json;
use crate::core::text::ModelName;

use super::ProfileError;

/// An operator declaration of a measured setup and its explicitly chosen alias.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ImageProfile {
    pub(crate) id: ProfileId,
    model_alias: ModelName,
}

impl ImageProfile {
    pub(super) fn parse(value: &Json) -> Result<Self, ProfileError> {
        let Json::Object(members) = value else {
            return Err(ProfileError::ImageProfile);
        };
        if members.len() != 2
            || members
                .iter()
                .any(|(key, _)| !matches!(key.as_str(), "id" | "model_alias"))
        {
            return Err(ProfileError::ImageProfile);
        }
        let id = value
            .member("id")
            .and_then(Json::as_str)
            .and_then(ProfileId::parse)
            .ok_or(ProfileError::ImageProfile)?;
        let alias = value
            .member("model_alias")
            .and_then(Json::as_str)
            .ok_or(ProfileError::ImageProfile)?;
        let model_alias = ModelName::new(alias).map_err(|_| ProfileError::ImageProfile)?;
        if model_alias.as_str() != alias {
            return Err(ProfileError::ImageProfile);
        }
        Ok(Self { id, model_alias })
    }

    pub(crate) fn matches(&self, model: &str) -> bool {
        self.model_alias.as_str() == model
    }
}

impl std::fmt::Debug for ImageProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageProfile")
            .field("id", &self.id)
            .field("model_alias", &"<withheld>")
            .finish()
    }
}
