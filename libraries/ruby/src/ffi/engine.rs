//! Convert explicit engine settings on the Ruby thread.
use super::{EngineValue, Settings, batch_of, checked, guarded};
use magnus::prelude::*;
use magnus::{Error, RHash, Ruby, TryConvert};
pub(super) fn new_engine(ruby: &Ruby, options: RHash) -> Result<EngineValue, Error> {
    fn read<T: TryConvert>(ruby: &Ruby, options: RHash, key: &str) -> Result<Option<T>, Error> {
        options
            .get(ruby.to_symbol(key))
            .filter(|value| !value.is_nil())
            .map(T::try_convert)
            .transpose()
    }
    let settings = Settings {
        backend: read(ruby, options, "backend")?,
        base_url: read(ruby, options, "base_url")?,
        model: read(ruby, options, "model")?,
        throttle: read(ruby, options, "throttle")?,
        max_requests: read(ruby, options, "max_requests")?,
        max_request_bytes: read(ruby, options, "max_request_bytes")?,
        cache_at: read(ruby, options, "cache_at")?,
        no_cache: read(ruby, options, "no_cache")?.unwrap_or(false),
        refresh_cache: read(ruby, options, "refresh_cache")?.unwrap_or(false),
        timeout: read(ruby, options, "timeout")?,
        max_retries: read(ruby, options, "max_retries")?,
        record: read(ruby, options, "record")?,
        replay: read(ruby, options, "replay")?,
        profile: read(ruby, options, "profile")?,
        batch: options
            .get(ruby.to_symbol("batch"))
            .map(|value| batch_of(ruby, value))
            .transpose()?
            .flatten(),
        max_requests_total: read(ruby, options, "max_requests_total")?,
    };
    checked(ruby, guarded(|| settings.build())).map(|engine| EngineValue { engine })
}
