//! Why a send budget or estimated input admission refused a live attempt.

/// Why a process send budget refused a live attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendBudgetDenial {
    /// No attempt for this call was sent.
    BeforeFirstSend,
    /// An earlier attempt in this call was sent, but another request was refused.
    BeforeAdditionalSend,
    /// A retry was refused after this backend status was received.
    BeforeRetry {
        /// The backend status whose retry would cross the process total.
        last_status: u16,
    },
}

/// Why estimated input admission refused one final encoded body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstimatedInputDenial {
    /// Before this call's first request.
    InitialRequest {
        /// The selected finite limit.
        limit: u64,
    },
    /// Before a later ordinary or split request.
    AdditionalRequest {
        /// The selected finite limit.
        limit: u64,
    },
    /// Before a same-exchange retry.
    Retry {
        /// The selected finite limit.
        limit: u64,
        /// The prior retryable status.
        last_status: u16,
    },
}

impl std::fmt::Display for EstimatedInputDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (limit, ending) = match *self {
            Self::InitialRequest { limit } => {
                (limit, "before this call's first request".to_owned())
            }
            Self::AdditionalRequest { limit } => {
                (limit, "before another request in this call".to_owned())
            }
            Self::Retry { limit, last_status } => {
                (limit, format!("before retrying status {last_status}"))
            }
        };
        write!(
            formatter,
            "max_estimated_input_tokens_total={limit} (encoded-body-bytes-908-v1) would be exceeded {ending}"
        )
    }
}
