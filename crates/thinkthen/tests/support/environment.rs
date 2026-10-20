//! Shared synthetic response for environment and usage cases.
/// The one answer every listener gives: a yes at 0.92.
pub(crate) const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);
pub(crate) const EVIDENCE: &str = "Refund me please.";
