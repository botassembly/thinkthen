//! Exact caller-selected prices on an engine builder.

use super::EngineBuilder;
use crate::core::Prices;
use crate::public::error::Error;

impl EngineBuilder {
    /// Replace the pair of USD prices per million reported input/output tokens.
    /// Prices are decimal strings with at most six fractional digits.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] without changing this builder when either price
    /// is malformed or exceeds USD 1,000,000 per million tokens.
    pub fn prices_usd_per_million(mut self, input: &str, output: &str) -> Result<Self, Error> {
        self.prices = Some(Prices::parse(input, output).ok_or_else(|| {
            Error::usage("prices are two decimal strings from 0 through 1000000 with at most six fractional digits")
        })?);
        Ok(self)
    }
}
