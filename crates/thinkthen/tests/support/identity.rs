//! Fixed loopback reply and quoted request shared by native and command correlation cases.
pub(crate) const REPLY: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
// The quote form is fixed by specification/records.md, independently of headers.
pub(crate) const BODY: &[u8] = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#;
