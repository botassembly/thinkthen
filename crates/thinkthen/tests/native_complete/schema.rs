//! Validate actual public calls with the same strict schema packaged for hosts.
use super::child;
#[path = "../support/complete_schema.rs"]
mod shared;
use serde::Serialize;
pub(super) use shared::{check, check_batch};

#[cfg(test)]
pub(super) fn call<T: Serialize>(call: &thinkthen::Call<T>, definition: &str) {
    check(&serde_json::to_value(call).unwrap(), definition);
}
