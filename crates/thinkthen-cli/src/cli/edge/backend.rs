//! Select one route through the captured public builder, then capture its key.
use super::Environment;
use crate::failure::Failure;
use crate::{EngineBuilder, ResolvedSettings};

impl Environment {
    pub(crate) fn resolve(
        &self,
        backend: Option<&str>,
        url: Option<&str>,
        model: Option<&str>,
    ) -> Result<ResolvedSettings, Failure> {
        self.resolve_guarded(backend, url, model, None)
    }

    /// Check's explicit-address refusal precedes selected credentials and the
    /// explicit model validator, as its original command admission did.
    pub(crate) fn resolve_for_check(
        &self,
        backend: Option<&str>,
        url: Option<&str>,
        model: Option<&str>,
        no_address: &'static str,
    ) -> Result<ResolvedSettings, Failure> {
        self.resolve_guarded(backend, url, model, Some(no_address))
    }

    fn resolve_guarded(
        &self,
        backend: Option<&str>,
        url: Option<&str>,
        model: Option<&str>,
        no_address: Option<&'static str>,
    ) -> Result<ResolvedSettings, Failure> {
        if let Some(resolved) = self.resolved.get() {
            return Ok(resolved.clone());
        }
        let environment_tier = backend.is_none() && url.is_none();
        let (backend, url) = if environment_tier {
            (self.backend.as_deref(), self.base_url.as_deref())
        } else {
            (backend, url)
        };
        self.update_builder(|mut builder| {
            if let Some(backend) = backend {
                builder = builder.backend(backend)?;
            }
            if let Some(url) = url {
                builder = builder.base_url(url)?;
            }
            if let Some(message) = no_address
                && builder.resolved_settings()?.url_source == "built_in"
            {
                return Err(crate::Error::new(crate::ErrorKind::Usage, message));
            }
            if let Some(model) = model {
                builder = builder.model(model)?;
            }
            // Selection runs host guards before reading any selected key variable.
            let route = builder.resolved_settings()?;
            self.key.capture(&route);
            if let Some(key) = self.key.value() {
                builder = builder.api_key(key)?;
            }
            Ok(builder)
        })?;
        let mut resolved = self.inspect_builder(EngineBuilder::resolved_settings)?;
        if environment_tier && resolved.url_source == "command_line" {
            resolved.url_source = "environment";
        }
        if self.named_cache() {
            resolved.cache_enabled_source = "environment";
            resolved.cache_path_source = "environment";
        }
        let _settled = self.resolved.set(resolved.clone());
        Ok(resolved)
    }

    pub(crate) fn inspect_builder<T>(
        &self,
        inspect: impl FnOnce(&EngineBuilder) -> Result<T, crate::Error>,
    ) -> Result<T, Failure> {
        let held = self
            .builder
            .lock()
            .map_err(|_| Failure::Defect("command settings lock failed"))?;
        let builder = held
            .as_ref()
            .ok_or(Failure::Defect("command settings were already consumed"))?;
        inspect(builder).map_err(Failure::from)
    }

    pub(crate) fn named(&self) -> Option<&str> {
        self.resolved
            .get()
            .and_then(|settings| settings.backend_name.as_deref())
    }
}

#[cfg(test)]
mod tests {
    //! Stays: command-owned route selection must preserve backend and address precedence.
    use super::*;
    #[test]
    fn backend_check_requires_an_explicit_address_before_explicit_model_validation() {
        let environment = Environment::default();
        let failure = environment
            .resolve_for_check(None, None, Some("bad\nmodel"), "named address required")
            .unwrap_err();
        assert_eq!(
            crate::failure::sentence(&failure),
            (2, "named address required".to_owned())
        );
    }
}
