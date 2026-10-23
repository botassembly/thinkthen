//! A client admits routing and limits once, separately from calls.

use crate::{Cancel, Engine, Error, SdkCall, SdkRequest, SdkSettings};

/// One admitted configuration and its reusable engine.
#[derive(Debug, Clone)]
pub struct Client {
    engine: Engine,
}

impl Client {
    /// Admit client configuration and resolve only its selected credentials.
    ///
    /// # Errors
    /// Returns the library's settings or backend admission error.
    pub fn from_json(json: &str, variable: impl Fn(&str) -> Option<String>) -> Result<Self, Error> {
        let settings = SdkSettings::from_json(json)?;
        Self::from_settings(&settings, variable)
    }

    /// Build a client from already admitted settings.
    ///
    /// # Errors
    /// Returns the selected backend or batch admission error.
    pub fn from_settings(
        settings: &SdkSettings,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            engine: settings.engine(variable)?,
        })
    }

    /// Admit configuration using the caller's named backend table.
    ///
    /// # Errors
    /// Returns the library's settings or backend admission error.
    pub fn from_json_with_backends(
        json: &str,
        backends: &crate::BackendTable,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            engine: SdkSettings::from_json(json)?.engine_with_backends(backends, variable)?,
        })
    }

    /// Admit one call without reading files or resolving client settings again.
    ///
    /// # Errors
    /// Returns the library's question admission error.
    pub fn prepare(&self, request: &SdkRequest, cancel: Cancel) -> Result<SdkCall, Error> {
        request.prepare(&self.engine, cancel)
    }
}
