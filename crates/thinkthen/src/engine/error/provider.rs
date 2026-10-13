//! Bounded provider prose that passed the transport secrecy boundary.

#[derive(Clone)]
pub(crate) struct ProviderDiagnostic {
    pub(crate) backend: &'static str,
    pub(crate) quota: bool,
    pub(crate) message: Option<String>,
}

impl ProviderDiagnostic {
    pub(crate) fn sentence(&self, base: String) -> String {
        let base = if self.quota {
            format!("the backend {} account is out of credit", self.backend)
        } else {
            base
        };
        match &self.message {
            Some(message) => format!("{base}; provider message: {message}"),
            None => base,
        }
    }
}

impl std::fmt::Debug for ProviderDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderDiagnostic")
            .field("backend", &self.backend)
            .field("quota", &self.quota)
            .field("message", &self.message.as_ref().map(|_| "<withheld>"))
            .finish()
    }
}
