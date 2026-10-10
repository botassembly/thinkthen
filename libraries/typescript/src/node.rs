//! Native engine ownership and usage observations.
#![allow(unsafe_code, reason = "napi emits registration glue")]
#![allow(missing_docs, reason = "napi emits public method glue")]
use napi::Result;
use napi_derive::napi;
use thinkthen::Engine;
/// One engine with its own settings, built by `new tt.Engine(options)`.
#[napi]
#[derive(Debug)]
pub struct NativeEngine {
    pub(crate) engine: Engine,
}

/// An owned live persistence observation, with native fixed advice on failure.
#[napi(object)]
#[derive(Debug)]
pub struct NativeUsageStatus {
    pub state: String,
    pub advice: Option<String>,
}

impl NativeUsageStatus {
    fn of(status: thinkthen::UsagePersistence) -> Result<Self> {
        let state = serde_json::to_value(status)
            .and_then(serde_json::from_value)
            .map_err(|_| napi::Error::from_reason("native usage state could not be converted"))?;
        Ok(Self {
            state,
            advice: status.advice().map(str::to_owned),
        })
    }
}

#[napi]
impl NativeEngine {
    #[napi]
    pub fn usage_persistence(&self) -> Result<NativeUsageStatus> {
        NativeUsageStatus::of(self.engine.usage_persistence())
    }

    #[napi]
    pub fn finish_usage_status(&self) -> Result<NativeUsageStatus> {
        NativeUsageStatus::of(self.engine.finish_usage_status())
    }
}
