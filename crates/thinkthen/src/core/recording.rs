//! The digest of one request, and the old request-level entry `cache
//! convert` reads, by ADR 0111 section 9.

use std::fmt;

use serde::Deserialize;
use serde_json::value::RawValue;
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::core::adapters::built_in;
use crate::core::digest::hex;
use crate::core::text::{Url, Withheld};

mod convert;
pub(crate) use convert::{Converting, convert};

/// The schema string a version one recording entry carries.
const SCHEMA: &str = "thinkthen.recording/1";

/// Why a file under a folder is not an old entry this version reads.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum EntryError {
    /// The bytes are not a recording entry.
    ///
    /// The message names the place and never the text. An entry is written
    /// around the evidence that was sent, and a JSON reader quotes the text it
    /// stopped on, so quoting the reader would print the evidence.
    #[error("the file is not a recording entry: the JSON at line {0} column {1} is not one")]
    Malformed(usize, usize),
    /// The entry was written by a version that names another schema.
    ///
    /// The message names only this version's trusted fixed schema. It never
    /// repeats the schema field from the entry: that field is unbounded text
    /// from a file and can hold a control byte or private evidence.
    #[error(
        "the entry names a schema this version does not read, \
             and this version reads `{SCHEMA}`"
    )]
    Schema,
    /// The entry names an address this version refuses.
    #[error("the entry records a different exchange, so the file was damaged or hand-edited")]
    Mismatched,
}

/// The name one exchange is filed under: its SHA-256 in lowercase hex.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Digest(String);

impl Digest {
    /// A digest spelled as 64 lowercase hex figures, such as a question key.
    #[must_use]
    pub(crate) const fn named(hex: String) -> Self {
        Self(hex)
    }

    /// Read the lowercase hexadecimal digest without its file suffix.
    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

/// What one exchange is: where it goes and what it carries.
pub(crate) struct Exchange<'a> {
    url: &'a Url,
    request: &'a [u8],
}

impl fmt::Debug for Exchange<'_> {
    /// Show what an exchange addresses and never the evidence its request holds.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Exchange")
            .field("url", &self.url)
            .field("request", &Withheld(self.request.len()))
            .finish()
    }
}

impl<'a> Exchange<'a> {
    /// Name the exchange one request body makes to one backend.
    #[must_use]
    pub(crate) const fn new(url: &'a Url, request: &'a [u8]) -> Self {
        Self { url, request }
    }

    /// Take the digest of the adapter's name, the URL, and the request bytes.
    ///
    /// The adapter writes the same plan to the same bytes every time, so the
    /// same command reaches the same entry. The adapter's name leads the
    /// digest, as it has since ticket 0004, so an entry recorded then is found
    /// now.
    #[must_use]
    pub(crate) fn digest(&self) -> Digest {
        let mut hasher = Sha256::new();
        hasher.update(built_in::NAME.as_bytes());
        hasher.update(b"\n");
        hasher.update(self.url.as_str().as_bytes());
        hasher.update(b"\n");
        hasher.update(self.request);
        Digest(hex(&hasher.finalize()))
    }
}

/// One recorded exchange, in the five fields a recording file holds.
#[derive(Deserialize)]
pub(crate) struct Entry {
    schema: String,
    adapter: String,
    url: String,
    request: Box<RawValue>,
    response: Box<RawValue>,
    /// Slice 1 of ADR 0111 rewrote this exchange into the quoted form.
    #[serde(default)]
    quoted: bool,
}

impl fmt::Debug for Entry {
    /// Name the entry and show nothing of it.
    ///
    /// Every field an entry holds was read out of a file, so no field of it
    /// reaches a line a person or a log will read.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Entry(<withheld>)")
    }
}

/// Where a JSON reader stopped, with nothing of what it stopped on.
fn place(error: serde_json::Error) -> EntryError {
    EntryError::Malformed(error.line(), error.column())
}


#[cfg(test)]
mod tests {
    use super::{Converting, EntryError, Exchange, SCHEMA, convert};
    use crate::core::adapters::built_in;
    use crate::core::plan::Plan;
    use crate::core::question::Question;
    use crate::core::text::{Evidence, ModelName, QuestionText, Url};

    /// The digest of the `decide-urgent` fixture request against the built-in URL.
    ///
    /// The value is the SHA-256 of `systemone`, a newline, the URL, a newline,
    /// and the request bytes, taken outside this program. Dry runs and
    /// attempt observations still name a request by it.
    const PINNED: &str = "bd370a64f0a785a6ee73bab801eb4e1ae01ebbda8aacc51bc288ef400b2a4a78";

    fn plan() -> Plan {
        Plan::authored(
            Evidence::new("Help! My payouts have been failing for 3 days.").expect("not blank"),
            ModelName::new("jev-latest").expect("not blank"),
            vec![Question::Decide {
                text: QuestionText::new("Does this convey urgency?").expect("not blank"),
                yes: None,
                no: None,
            }],
        )
        .expect("a plan of one question")
    }

    /// Why `convert` refused an entry as unreadable.
    fn unreadable(entry: &str) -> EntryError {
        match convert(entry.as_bytes(), false) {
            Err(Converting::Unreadable(error)) => error,
            other => panic!("an unreadable entry, not {other:?}"),
        }
    }

    #[test]
    fn the_digest_of_the_fixture_request_is_pinned() {
        let url = Url::new("https://api.typesafe.ai/v1/systemone").expect("not blank");
        let request = built_in::encode(&plan()).expect("a plan is writable");
        assert_eq!(Exchange::new(&url, &request).digest().as_str(), PINNED);
    }

    /// A file in a converted folder is untrusted, and no message repeats a field.
    ///
    /// Every field of an entry comes out of a file. A file that parses and
    /// names another schema would otherwise print whatever that field held:
    /// unbounded text, control bytes, and whatever the evidence was.
    #[test]
    fn no_refusal_of_a_parseable_entry_repeats_a_field_it_read() {
        // The escape is written the way JSON writes it, so the file is a file a
        // reader takes and the field carries a control byte all the same.
        let hostile = r"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5";
        let entry = format!(
            "{{\"schema\":\"{hostile}\",\"adapter\":\"systemone\",\
             \"url\":\"{hostile}\",\"request\":{{}},\"response\":{{}}}}"
        );

        let error = unreadable(&entry);

        assert_eq!(error, EntryError::Schema);
        assert_eq!(
            error.to_string(),
            "the entry names a schema this version does not read, \
             and this version reads `thinkthen.recording/1`"
        );
        assert!(!format!("{error:?}").contains("PWNED"), "{error:?}");
    }

    /// A damaged entry holds the evidence it recorded, and no message quotes it.
    ///
    /// A JSON reader names the text it stopped on. The entry is a file this
    /// tool wrote around the evidence, so that text is the evidence, and the
    /// message says where the file broke and never what it held.
    #[test]
    fn no_refusal_of_a_damaged_entry_quotes_the_evidence_inside_it() {
        let evidence = "marker-evidence-7b3ac5";
        let damaged = format!(
            "{{\"schema\":\"{SCHEMA}\",\"adapter\":\"systemone\",\
             \"request\":{{\"evidence\":\"{evidence}\"}} \"response\":{{}}}}"
        );

        let error = unreadable(&damaged);

        assert!(!error.to_string().contains(evidence), "{error}");
        assert_eq!(
            error.to_string(),
            "the file is not a recording entry: the JSON at line 1 column 105 is not one"
        );
    }
}
