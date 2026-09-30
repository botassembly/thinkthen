//! The built-in named backends, each with its own key variables (ADR 0114).
//!
//! Both built-ins speak this adapter's wire shape, so the table lives here with
//! the adapter's other vendor words. `core::backend::named` reads it and names
//! no vendor of its own.

/// One built-in backend: its name, its base, its key variables in the order
/// they are read, and its model.
pub(crate) struct BuiltIn {
    pub(crate) name: &'static str,
    pub(crate) base: &'static str,
    pub(crate) keys: &'static [&'static str],
    pub(crate) model: &'static str,
}

/// The built-in backends, in name order, which the unknown-name sentence lists.
pub(crate) const BUILT_INS: [BuiltIn; 2] = [
    BuiltIn {
        name: "liquid",
        base: "https://api.liquid.ai/decisions/v1",
        keys: &["LIQUIDAI_API_KEY", "LIQUID_API_KEY"],
        model: "d1:free",
    },
    BuiltIn {
        name: "typesafe",
        base: super::DEFAULT_BASE,
        keys: &["TYPESAFE_API_KEY"],
        model: super::DEFAULT_MODEL,
    },
];
