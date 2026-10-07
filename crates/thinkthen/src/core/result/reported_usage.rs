//! Independently reported counts; missing dimensions never become observed zero.

use serde::Serialize;

use super::{Usage, share};

/// Token counts reported independently by an accepted response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "completeUsage"))]
pub struct ReportedUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_tokens: Option<u64>,
}

impl ReportedUsage {
    pub(crate) const fn new(input_tokens: Option<u64>, output_tokens: Option<u64>) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }

    /// The reported input count, including an explicitly reported zero.
    #[must_use]
    pub const fn input_tokens(self) -> Option<u64> {
        self.input_tokens
    }

    /// The reported output count, absent when the response omitted it.
    #[must_use]
    pub const fn output_tokens(self) -> Option<u64> {
        self.output_tokens
    }

    pub(crate) fn complete(self) -> Option<Usage> {
        self.input_tokens
            .zip(self.output_tokens)
            .map(|(input, output)| Usage::new(input, output))
    }

    pub(crate) fn share(self, records: usize, position: usize) -> Self {
        Self::new(
            self.input_tokens
                .map(|count| share(count, records, position)),
            self.output_tokens
                .map(|count| share(count, records, position)),
        )
    }
}

impl ReportedUsage {
    pub(crate) const fn from_complete(usage: Usage) -> Self {
        let (input, output) = usage.token_counts();
        Self::new(Some(input), Some(output))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Count {
    sum: Option<u64>,
    missing: bool,
    overflow: bool,
}

impl Count {
    fn add(&mut self, value: Option<u64>) {
        match (self.sum, value) {
            (_, None) => self.missing = true,
            (None, Some(value)) => self.sum = Some(value),
            (Some(sum), Some(value)) => match sum.checked_add(value) {
                Some(sum) => self.sum = Some(sum),
                None => self.overflow = true,
            },
        }
    }

    fn total(self) -> Result<Option<u64>, ()> {
        if self.missing {
            Ok(None)
        } else if self.overflow {
            Err(())
        } else {
            Ok(self.sum)
        }
    }
}

/// Summed observations, with unknown and overflow handled per dimension.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ReportedSum {
    input: Count,
    output: Count,
    seen: bool,
}

impl ReportedSum {
    pub(crate) fn add(&mut self, usage: Option<ReportedUsage>) {
        self.seen = true;
        self.input.add(usage.and_then(ReportedUsage::input_tokens));
        self.output
            .add(usage.and_then(ReportedUsage::output_tokens));
    }

    pub(crate) fn total(self) -> Result<Option<ReportedUsage>, ()> {
        if !self.seen {
            return Ok(None);
        }
        let usage = ReportedUsage::new(self.input.total()?, self.output.total()?);
        Ok((usage.input_tokens.is_some() || usage.output_tokens.is_some()).then_some(usage))
    }
}

#[cfg(test)]
mod tests {
    use super::{ReportedSum, ReportedUsage};

    #[test]
    fn partial_counts_sum_independently_and_never_invent_missing_observations() {
        type Case = (
            &'static [Option<ReportedUsage>],
            Result<Option<ReportedUsage>, ()>,
        );
        const INPUT: Option<ReportedUsage> = Some(ReportedUsage::new(Some(887), None));
        const FULL: Option<ReportedUsage> = Some(ReportedUsage::new(Some(13), Some(4)));
        const OUTPUT: Option<ReportedUsage> = Some(ReportedUsage::new(None, Some(0)));
        const MOST: Option<ReportedUsage> = Some(ReportedUsage::new(Some(u64::MAX), Some(0)));
        let cases: &[Case] = &[
            (&[], Ok(None)),
            (&[INPUT], Ok(INPUT)),
            (
                &[INPUT, FULL],
                Ok(Some(ReportedUsage::new(Some(900), None))),
            ),
            (&[FULL, OUTPUT], Ok(Some(ReportedUsage::new(None, Some(4))))),
            (&[INPUT, OUTPUT], Ok(None)),
            (&[INPUT, None], Ok(None)),
            (&[MOST, FULL], Err(())),
            (&[MOST, FULL, None], Ok(None)),
        ];
        for (observations, expected) in cases {
            let mut sum = ReportedSum::default();
            for observation in *observations {
                sum.add(*observation);
            }
            assert_eq!(&sum.total(), expected);
        }
        assert_eq!(
            INPUT.unwrap().share(2, 0),
            ReportedUsage::new(Some(444), None)
        );
        assert_eq!(
            INPUT.unwrap().share(2, 1),
            ReportedUsage::new(Some(443), None)
        );
    }
}
