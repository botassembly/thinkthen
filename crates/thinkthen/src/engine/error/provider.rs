//! Known provider error fields shared by every error surface.

#[derive(Clone)]
pub(crate) struct ProviderDiagnostic {
    pub(crate) backend: &'static str,
    pub(crate) quota: bool,
    pub(crate) ollama: bool,
    pub(crate) error_type: Option<String>,
    pub(crate) code: Option<String>,
}

impl ProviderDiagnostic {
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

impl std::fmt::Debug for ProviderDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderDiagnostic")
            .field("backend", &self.backend)
            .field("quota", &self.quota)
            .field("ollama", &self.ollama)
            .field("type", &self.error_type)
            .field("code", &self.code)
            .finish()
    }
}
