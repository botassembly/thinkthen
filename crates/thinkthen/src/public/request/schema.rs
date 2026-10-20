//! The committed canonical request schema, shared by transport discovery.

/// The complete canonical request schema used by native admission and discovery.
/// This is the same generated document the library's schema contract checks.
#[must_use]
pub const fn request_schema() -> &'static str {
    include_str!("request.schema.json")
}
