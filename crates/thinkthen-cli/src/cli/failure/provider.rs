//! Command wording around admitted public provider identifiers.

#[derive(Clone)]
pub(crate) struct Diagnostic {
    pub(crate) backend: &'static str,
    pub(crate) quota: bool,
    pub(crate) ollama: bool,
    pub(crate) error_type: Option<String>,
    pub(crate) code: Option<String>,
}

impl Diagnostic {
    pub(crate) fn sentence(&self, base: String) -> String {
        let base = if self.quota {
            format!("the backend {} account is out of credit", self.backend)
        } else {
            base
        };
        let base = if self.ollama {
            format!("{base}; this address looks like Ollama; try --backend ollama")
        } else {
            base
        };
        let mut sentence = base;
        for (field, value) in [("type", &self.error_type), ("code", &self.code)] {
            if let Some(value) = value {
                use std::fmt::Write as _;
                let _ = write!(sentence, "; provider {field}: {value}");
            }
        }
        sentence
    }
}

impl std::fmt::Debug for Diagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Diagnostic")
            .field("backend", &self.backend)
            .field("quota", &self.quota)
            .field("ollama", &self.ollama)
            .field("type", &self.error_type)
            .field("code", &self.code)
            .finish()
    }
}

impl From<thinkthen::ProviderFailure<'_>> for Diagnostic {
    fn from(provider: thinkthen::ProviderFailure<'_>) -> Self {
        Self {
            backend: provider.backend,
            quota: provider.quota,
            ollama: provider.alternate_backend_hint,
            error_type: provider.error_type.map(str::to_owned),
            code: provider.code.map(str::to_owned),
        }
    }
}
