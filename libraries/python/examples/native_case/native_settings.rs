//! Fixture settings use only public native builders and an explicit owned environment.
use serde_json::{Map, Value};
use thinkthen::{Engine, EngineBuilder, Error, ErrorKind};
fn invalid() -> Error {
    Error::new(ErrorKind::Usage, "unsupported engine setting")
}
pub(super) fn engine(settings: Map<String, Value>) -> Result<Engine, Error> {
    let mut b = EngineBuilder::from_env()?;
    for (key, value) in settings {
        let text = || value.as_str().ok_or_else(invalid);
        let number = || value.as_u64().ok_or_else(invalid);
        b = match key.as_str() {
            "base_url" => b.base_url(text()?)?,
            "backend" => b.backend(text()?)?,
            "model" => b.model(text()?)?,
            "cache" if value == false => b.no_cache(),
            "cache" if value.is_null() => b,
            "cache" => b.cache_at(text()?)?,
            "record" if value.is_null() => b,
            "record" => b.record(text()?)?,
            "replay" => b.replay(text()?)?,
            "profile" => b.profile(text()?)?,
            "refresh_cache" => b.refresh_cache(value.as_bool().ok_or_else(invalid)?),
            "max_requests" => {
                b.max_requests(Some(usize::try_from(number()?).map_err(|_| invalid())?))?
            }
            "max_requests_total" => b.max_requests_total(Some(number()?)),
            "max_request_bytes" => {
                b.max_request_bytes(usize::try_from(number()?).map_err(|_| invalid())?)?
            }
            "throttle" => b.throttle(u8::try_from(number()?).map_err(|_| invalid())?)?,
            "timeout" => b.timeout(std::time::Duration::from_secs(number()?))?,
            "max_retries" => b.max_retries(u32::try_from(number()?).map_err(|_| invalid())?),
            "batch" if value == "max" => b.batch(thinkthen::BatchSetting::Max),
            "batch" => b.batch(thinkthen::BatchSetting::Records(
                std::num::NonZeroUsize::new(usize::try_from(number()?).map_err(|_| invalid())?)
                    .ok_or_else(invalid)?,
            )),
            _ => return Err(invalid()),
        };
    }
    b.build()
}
