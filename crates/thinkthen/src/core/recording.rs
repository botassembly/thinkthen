//! The digest of one request.

use std::fmt;

use sha2::{Digest as _, Sha256};

use crate::core::adapters::built_in;
use crate::core::digest::hex;
use crate::core::text::{Url, Withheld};

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

#[cfg(test)]
mod tests {
    //! Stays: recording identities and replay bytes require direct private serialization contract checks.
    use super::Exchange;
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

    #[test]
    fn the_digest_of_the_fixture_request_is_pinned() {
        let url = Url::new("https://api.typesafe.ai/v1/systemone").expect("not blank");
        let request = built_in::encode(&plan()).expect("a plan is writable");
        assert_eq!(Exchange::new(&url, &request).digest().as_str(), PINNED);
    }
}
