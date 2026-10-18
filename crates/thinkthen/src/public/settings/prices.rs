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
        self.prices = Some(parse(input, output)?);
        self.setting_prices = (None, None);
        Ok(self)
    }

    pub(super) fn apply_setting_prices(mut self, input: Option<&str>, output: Option<&str>) -> Result<Self, Error> {
        if let Some(input) = input {
            parse(input, "0")?;
            self.setting_prices.0 = Some(input.to_owned());
        }
        if let Some(output) = output {
            parse("0", output)?;
            self.setting_prices.1 = Some(output.to_owned());
        }
        if let (Some(input), Some(output)) = (&self.setting_prices.0, &self.setting_prices.1) {
            self.prices = Some(parse(input, output)?);
        }
        Ok(self)
    }

    pub(super) fn check_setting_prices(&self) -> Result<(), Error> {
        match (&self.setting_prices.0, &self.setting_prices.1) {
            (Some(_), Some(_)) | (None, None) => Ok(()),
            _ => Err(Error::usage(
                "settings prices require both input and output fields",
            )),
        }
    }


}

fn parse(input: &str, output: &str) -> Result<Prices, Error> {
    Prices::parse(input, output).ok_or_else(|| {
        Error::usage("prices are two decimal strings from 0 through 1000000 with at most six fractional digits")
    })
}
