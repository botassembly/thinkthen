//! The built-in named backends, each with its own key variables (ADR 0114).
//!
//! Every built-in speaks this adapter's wire shape, so the table lives here with
//! the adapter's other vendor words. `core::backend::named` reads it and names
//! no vendor of its own.

use crate::core::plan::Descriptions;

/// One built-in backend: its name, its base, its key variables in the order
/// they are read, its model, and how its descriptions travel (ADR 0115).
pub(crate) struct BuiltIn {
    pub(crate) name: &'static str,
    pub(crate) path: &'static str,
    pub(crate) base: &'static str,
    pub(crate) keys: &'static [&'static str],
    pub(crate) model: &'static str,
    pub(crate) descriptions: Descriptions,
}

/// The built-in backends, in name order, which the unknown-name sentence lists.
pub(crate) const BUILT_INS: [BuiltIn; 5] = [
    BuiltIn {
        name: "liquid",
        path: super::ENDPOINT_PATH,
        base: "https://api.liquid.ai/decisions/v1",
        keys: &["LIQUIDAI_API_KEY", "LIQUID_API_KEY"],
        model: "d1:free",
        descriptions: Descriptions::Authored,
    },
    BuiltIn {
        name: "ollama",
        path: super::ENDPOINT_PATH,
        base: "http://localhost:11434/v1",
        keys: &["OLLAMA_API_KEY"],
        model: "nimble",
        // Ollama refuses an object description with status 400. This row's
        // text form is the workaround, owned by the debt issue
        // `sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md`.
        descriptions: Descriptions::Text,
    },
    BuiltIn {
        name: "openrouter",
        path: super::ENDPOINT_PATH,
        base: "https://openrouter.ai/api/v1",
        keys: &["OPENROUTER_API_KEY"],
        model: "typesafe/jev-1.13",
        descriptions: Descriptions::BothSides,
    },
    BuiltIn {
        name: "perplexity",
        path: "decisions",
        base: "https://api.perplexity.ai/v1",
        keys: &["PERPLEXITY_API_KEY"],
        model: "pplx-decider-v1-27b",
        descriptions: Descriptions::Authored,
    },
    BuiltIn {
        name: "typesafe",
        path: super::ENDPOINT_PATH,
        base: super::DEFAULT_BASE,
        keys: &["TYPESAFE_API_KEY"],
        model: super::DEFAULT_MODEL,
        descriptions: Descriptions::Authored,
    },
];
