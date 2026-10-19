//! Safe fixed messages for backend status codes.

/// The phrase every retried server status ends with once its attempts run out.
const RETRIED_OUT: &str =
    "the backend failed after the allowed attempts; try again later or change --max-retries";

const PHRASES: [(u16, &str); 19] = [
    (
        302,
        "the redirect was not followed; use the final --url directly",
    ),
    (400, "the backend refused the request"),
    (401, "the key was refused"),
    (402, "the account has no credit"),
    (403, "the key may not use this model or address"),
    (404, "nothing answers at this address"),
    (413, "the backend refused the request as too large"),
    (
        422,
        "the backend refused the request as malformed or too large",
    ),
    (
        429,
        "the backend's rate limit was reached after the allowed attempts; try again later or change --max-retries",
    ),
    (500, RETRIED_OUT),
    (502, RETRIED_OUT),
    (503, RETRIED_OUT),
    (504, RETRIED_OUT),
    (520, RETRIED_OUT),
    (521, RETRIED_OUT),
    (522, RETRIED_OUT),
    (523, RETRIED_OUT),
    (524, RETRIED_OUT),
    (529, RETRIED_OUT),
];

/// Remedies selected where execution knows how the failing input is grouped.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SizeAdvice {
    Text,
    Batches,
    Candidates,
    Backend,
}

impl SizeAdvice {
    const fn sentence(self) -> &'static str {
        match self {
            Self::Text => "shorten the text",
            Self::Batches => "shorten the text or set a lower --max-request-bytes",
            Self::Candidates => "shorten the text or use fewer candidates",
            Self::Backend => "use a backend with a larger input limit",
        }
    }
}

pub(super) fn too_large(tokens: bool, advice: SizeAdvice) -> String {
    let reason = if tokens {
        "the backend answered with status 400 (max_tokens_exceeded): the request has more input tokens than the backend takes".to_owned()
    } else {
        said(413)
    };
    format!("{reason}; {}", advice.sentence())
}

/// Name the status and its fixed phrase without carrying a response body.
pub(super) fn said(status: u16) -> String {
    let answered = format!("the backend answered with status {status}");
    match PHRASES.iter().find(|(code, _)| *code == status) {
        Some((_, phrase)) => format!("{answered}: {phrase}"),
        None => answered,
    }
}
