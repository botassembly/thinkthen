//! Safe fixed messages for backend status codes.

/// The phrase every retried server status ends with once its attempts run out.
const RETRIED_OUT: &str =
    "the backend failed after the allowed attempts; try again later or change --max-retries";

const PHRASES: [(u16, &str); 14] = [
    (
        302,
        "the redirect was not followed; use the final --url directly",
    ),
    (
        400,
        "the backend refused the request; check --model and the request size",
    ),
    (401, "the key was refused"),
    (402, "the account has no credit"),
    (403, "the key may not use this model or address"),
    (404, "nothing answers at this address"),
    (
        413,
        "the backend refused the request as too large; shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile",
    ),
    (
        422,
        "the backend refused the request as malformed or too large",
    ),
    (429, "the backend's rate limit was reached"),
    (500, RETRIED_OUT),
    (502, RETRIED_OUT),
    (503, RETRIED_OUT),
    (504, RETRIED_OUT),
    (529, RETRIED_OUT),
];

/// Status 400 whose body named `max_tokens_exceeded`, the one reason printed.
pub(super) const TOKEN_LIMIT: &str = "the backend answered with status 400 (max_tokens_exceeded): the request has more input tokens than the backend takes; shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile";

/// Name the status and its fixed phrase without carrying a response body.
pub(super) fn said(status: u16) -> String {
    let answered = format!("the backend answered with status {status}");
    match PHRASES.iter().find(|(code, _)| *code == status) {
        Some((_, phrase)) => format!("{answered}: {phrase}"),
        None => answered,
    }
}
