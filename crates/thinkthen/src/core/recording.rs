//! One recorded exchange, filed under the digest of what was sent.

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::core::adapters::built_in;
use crate::core::digest::hex;
use crate::core::text::Url;

/// The schema string a version one recording entry carries.
const SCHEMA: &str = "thinkthen.recording/1";

/// Why a file under a recording folder is not the entry that was asked for.
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
    /// The entry records another exchange, so the file was damaged or edited.
    #[error("the entry records a different exchange, so the file was damaged or hand-edited")]
    Mismatched,
    /// The entry could not be written as JSON.
    ///
    /// The message names no cause, for the reason [`Self::Malformed`] gives.
    #[error("the entry could not be written as JSON")]
    Unwritable,
}

/// The name one exchange is filed under: its SHA-256 in lowercase hex.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Digest(String);

impl Digest {
    /// The name of the file one exchange is recorded in.
    #[must_use]
    pub(crate) fn file_name(&self) -> String {
        format!("{}.json", self.0)
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
            .field(
                "request",
                &format_args!("<{} bytes withheld>", self.request.len()),
            )
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
#[derive(Deserialize, Serialize)]
pub(crate) struct Entry {
    schema: String,
    adapter: String,
    url: String,
    request: Box<RawValue>,
    response: Box<RawValue>,
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

impl Entry {
    /// Record one exchange the backend answered and the adapter read.
    ///
    /// # Errors
    ///
    /// Returns [`EntryError::Malformed`] when either body is not JSON.
    pub(crate) fn of(exchange: &Exchange<'_>, response: &[u8]) -> Result<Self, EntryError> {
        Ok(Self {
            schema: SCHEMA.to_owned(),
            adapter: built_in::NAME.to_owned(),
            url: exchange.url.as_str().to_owned(),
            request: json(exchange.request)?,
            response: json(response)?,
        })
    }

    /// Write the entry as the text one file under a recording folder holds.
    ///
    /// # Errors
    ///
    /// Returns [`EntryError::Unwritable`] when the entry cannot be written as JSON.
    pub(crate) fn written(&self) -> Result<String, EntryError> {
        let mut text = serde_json::to_string_pretty(self).map_err(|_| EntryError::Unwritable)?;
        text.push('\n');
        Ok(text)
    }

    /// Read back the response this file recorded for the exchange being replayed.
    ///
    /// A digest names the file, so a file that holds another exchange was damaged
    /// or edited by hand and is refused rather than answered from.
    ///
    /// # Errors
    ///
    /// Returns [`EntryError`] when the file is not an entry, when it names
    /// another schema, or when it records another exchange.
    pub(crate) fn replayed(bytes: &[u8], exchange: &Exchange<'_>) -> Result<Vec<u8>, EntryError> {
        let entry: Self = serde_json::from_slice(bytes).map_err(place)?;
        if entry.schema != SCHEMA {
            return Err(EntryError::Schema);
        }
        if entry.adapter != built_in::NAME
            || entry.url != exchange.url.as_str()
            || entry.request.get().as_bytes() != exchange.request
        {
            return Err(EntryError::Mismatched);
        }
        Ok(entry.response.get().as_bytes().to_owned())
    }
}

/// Take bytes that are JSON as one value to embed, never as a string.
fn json(bytes: &[u8]) -> Result<Box<RawValue>, EntryError> {
    serde_json::from_slice(bytes).map_err(place)
}

/// Where a JSON reader stopped, with nothing of what it stopped on.
fn place(error: serde_json::Error) -> EntryError {
    EntryError::Malformed(error.line(), error.column())
}

#[cfg(test)]
mod tests {
    use super::{Entry, EntryError, Exchange, SCHEMA};
    use crate::core::adapters::built_in;
    use crate::core::plan::Plan;
    use crate::core::question::Question;
    use crate::core::text::{Evidence, ModelName, QuestionText, Url};

    /// The digest of the `decide-urgent` fixture request against the built-in URL.
    ///
    /// The value is the SHA-256 of `systemone`, a newline, the URL, a newline,
    /// and the request bytes, taken outside this program. Pinning it holds the
    /// file name still, so a recording made today is found tomorrow. Ticket
    /// 0007 removed the adapter type and left the digest untouched.
    const PINNED: &str = "bd370a64f0a785a6ee73bab801eb4e1ae01ebbda8aacc51bc288ef400b2a4a78.json";

    /// One response as a backend sends it, on the one line it crossed the wire on.
    const RESPONSE: &str = concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.92}},"#,
        r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
    );

    /// The file one recorded exchange is written into, field by field.
    const FILE: &str = concat!(
        "{\n",
        "  \"schema\": \"thinkthen.recording/1\",\n",
        "  \"adapter\": \"systemone\",\n",
        "  \"url\": \"https://api.typesafe.ai/v1/systemone\",\n",
        r#"  "request": {"state":"Help! My payouts have been failing for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"Does this convey urgency?"}}},"#,
        "\n",
        r#"  "response": {"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":312,"output_tokens":48}}"#,
        "\n}\n",
    );

    fn url() -> Url {
        Url::new("https://api.typesafe.ai/v1/systemone").expect("not blank")
    }

    fn plan() -> Plan {
        Plan::new(
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

    fn request() -> Vec<u8> {
        built_in::encode(&plan()).expect("a plan is writable")
    }

    #[test]
    fn the_digest_of_the_fixture_request_is_the_name_the_entry_keeps() {
        let url = url();
        let request = request();
        let exchange = Exchange::new(&url, &request);
        assert_eq!(exchange.digest().file_name(), PINNED);
    }

    #[test]
    fn an_entry_carries_the_two_bodies_and_nothing_a_request_header_held() {
        let url = url();
        let request = request();
        let exchange = Exchange::new(&url, &request);
        let entry = Entry::of(&exchange, RESPONSE.as_bytes()).expect("both bodies are JSON");
        let written = entry.written().expect("an entry is writable");

        assert_eq!(written, FILE);
        for shown in [
            "authorization",
            "Authorization",
            "Bearer",
            "THINKTHEN_API_KEY",
        ] {
            assert!(!written.contains(shown), "{written}");
        }

        let replayed =
            Entry::replayed(written.as_bytes(), &exchange).expect("the entry answers its own url");
        let reply = built_in::decode(&plan(), &replayed).expect("a systemone response");
        assert_eq!(reply.model().as_str(), "jev-latest");
    }

    #[test]
    fn an_entry_that_records_another_exchange_is_refused() {
        let url = url();
        let request = request();
        let exchange = Exchange::new(&url, &request);
        let written = Entry::of(&exchange, RESPONSE.as_bytes())
            .expect("both bodies are JSON")
            .written()
            .expect("an entry is writable");

        let elsewhere = Url::new("http://127.0.0.1:1/v1").expect("not blank");
        assert_eq!(
            Entry::replayed(written.as_bytes(), &Exchange::new(&elsewhere, &request)),
            Err(EntryError::Mismatched)
        );
        let other = br#"{"state":"something else"}"#;
        assert_eq!(
            Entry::replayed(written.as_bytes(), &Exchange::new(&url, other)),
            Err(EntryError::Mismatched)
        );

        let stale = written.replace("thinkthen.recording/1", "thinkthen.recording/0");
        assert_eq!(
            Entry::replayed(stale.as_bytes(), &exchange),
            Err(EntryError::Schema)
        );
        assert!(matches!(
            Entry::replayed(b"not an entry at all", &exchange),
            Err(EntryError::Malformed(..))
        ));
    }

    /// A file in a recording folder is untrusted, and no message repeats a field.
    ///
    /// Every field of an entry comes out of a file. A file that parses and
    /// names another schema would otherwise print whatever that field held:
    /// unbounded text, control bytes, and whatever the evidence was.
    #[test]
    fn no_refusal_of_a_parseable_entry_repeats_a_field_it_read() {
        let url = Url::new("http://127.0.0.1:9/v1/systemone").expect("an address");
        let exchange = Exchange::new(&url, br#"{"asked":1}"#);
        // The escape is written the way JSON writes it, so the file is a file a
        // reader takes and the field carries a control byte all the same.
        let hostile = r"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5";
        let entry = format!(
            "{{\"schema\":\"{hostile}\",\"adapter\":\"systemone\",\
             \"url\":\"{hostile}\",\"request\":{{}},\"response\":{{}}}}"
        );

        let error = Entry::replayed(entry.as_bytes(), &exchange)
            .expect_err("an entry naming another schema is refused");

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
        let url = Url::new("http://127.0.0.1:9/v1/systemone").expect("an address");
        let exchange = Exchange::new(&url, br#"{"asked":1}"#);
        let evidence = "marker-evidence-7b3ac5";
        let damaged = format!(
            "{{\"schema\":\"{SCHEMA}\",\"adapter\":\"systemone\",\
             \"request\":{{\"evidence\":\"{evidence}\"}} \"response\":{{}}}}"
        );

        let error =
            Entry::replayed(damaged.as_bytes(), &exchange).expect_err("a damaged entry is refused");

        assert!(!error.to_string().contains(evidence), "{error}");
        assert_eq!(
            error.to_string(),
            "the file is not a recording entry: the JSON at line 1 column 105 is not one"
        );
    }
}
