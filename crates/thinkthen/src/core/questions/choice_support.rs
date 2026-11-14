//! Public implementation support for hygienic choices macro expansion.
#[doc(hidden)]
pub use schemars;
#[doc(hidden)]
pub use serde;
/// Build the schema for the macro's declared text labels.
#[doc(hidden)]
pub fn enum_schema(labels: &[&str]) -> schemars::Schema {
    schemars::json_schema!({"type":"string", "enum":labels})
}
/// The stable refusal for a label outside an enum's declared domain.
#[doc(hidden)]
pub fn unknown_label() -> crate::core::Error {
    crate::core::Error::from_contract(
        crate::core::Code::UsageQuestion,
        "the answer label is not a member of the enum's declared domain",
    )
}
