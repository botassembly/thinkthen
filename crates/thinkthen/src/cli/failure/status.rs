//! Safe fixed messages for backend status codes.

const PHRASES: [(u16, &str); 8] = [
    (
        400,
        "the backend refused the request; check --model and the request size",
    ),
    (401, "the key was refused"),
    (402, "the account has no credit"),
    (403, "the key may not use this model or address"),
    (404, "nothing answers at this address"),
    (
        422,
        "the backend refused the request as malformed or too large",
    ),
    (429, "the backend's rate limit was reached"),
    (
        500,
        "the backend failed after the allowed attempts; try again later or change --max-retries",
    ),
];

/// Name the status and its fixed phrase without carrying a response body.
pub(super) fn said(status: u16) -> String {
    let answered = format!("the backend answered with status {status}");
    match PHRASES.iter().find(|(code, _)| *code == status) {
        Some((_, phrase)) => format!("{answered}: {phrase}"),
        None => answered,
    }
}
