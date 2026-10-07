//! Packaged canonical schema for the additive native complete-call contract.
/// The strict schema for actual complete calls and safe complete errors.
///
/// Result/1 and the released generic door retain their compatibility definitions.
/// The document is generated from the same concrete serializers as the native
/// result schema and included in the crate for consumers such as MCP.
#[must_use]
pub const fn complete_call_schema() -> &'static str {
    include_str!("complete.schema.json")
}
