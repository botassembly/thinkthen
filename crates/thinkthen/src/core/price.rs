//! Caller-selected exact prices and whole-call estimated cost arithmetic.

/// Micro-USD per million reported tokens, bounded by USD 1,000,000.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Prices {
    input: u64,
    output: u64,
}

impl std::fmt::Debug for Prices {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Prices(<withheld>)")
    }
}

impl Prices {
    pub(crate) fn parse(input: &str, output: &str) -> Option<Self> {
        Some(Self {
            input: decimal_micro(input)?,
            output: decimal_micro(output)?,
        })
    }

    /// Round the combined rational amount once, to the nearest micro-USD.
    pub(crate) fn estimate(self, input: u64, output: u64) -> Option<String> {
        let numerator = u128::from(input)
            .checked_mul(u128::from(self.input))?
            .checked_add(u128::from(output).checked_mul(u128::from(self.output))?)?;
        let rounded = numerator.checked_add(500_000)? / 1_000_000;
        Some(format!(
            "{}.{:06}",
            rounded / 1_000_000,
            rounded % 1_000_000
        ))
    }
}

fn decimal_micro(value: &str) -> Option<u64> {
    let (whole, fraction) = match value.split_once('.') {
        Some((whole, fraction)) if (1..=6).contains(&fraction.len()) => (whole, Some(fraction)),
        Some(_) => return None,
        None => (value, None),
    };
    if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let whole = whole.trim_start_matches('0');
    let whole = if whole.is_empty() { "0" } else { whole };
    let whole = whole.parse::<u64>().ok()?;
    if whole > 1_000_000 {
        return None;
    }
    let fractional = match fraction {
        Some(digits) if digits.bytes().all(|byte| byte.is_ascii_digit()) => {
            let scale = 10_u64.pow(u32::try_from(6 - digits.len()).ok()?);
            digits.parse::<u64>().ok()?.checked_mul(scale)?
        }
        Some(_) => return None,
        None => 0,
    };
    let micro = whole.checked_mul(1_000_000)?.checked_add(fractional)?;
    (micro <= 1_000_000_000_000).then_some(micro)
}

#[cfg(test)]
mod tests {
    use super::Prices;

    #[test]
    fn accepted_decimal_edges_and_combined_rounding() {
        let cases = [
            ("0", "0.000000", 1, 1, "0.000000"),
            ("0.5", "0", 1, 0, "0.000001"),
            ("0.25", "0.25", 1, 1, "0.000001"),
            ("0000001.20", "2", 1_000_000, 1_000_000, "3.200000"),
            ("1000000", "0", 1, 0, "1.000000"),
        ];
        for (input, output, input_tokens, output_tokens, expected) in cases {
            let prices = Prices::parse(input, output).unwrap();
            assert_eq!(
                prices.estimate(input_tokens, output_tokens).as_deref(),
                Some(expected)
            );
        }
    }

    #[test]
    fn refused_decimal_spellings() {
        for refused in [
            "",
            ".1",
            "1.",
            " 1",
            "+1",
            "-1",
            "1e0",
            "1.0000000",
            "1.2.3",
            "1000000.000001",
            "18446744073709551616",
            "１",
        ] {
            assert!(Prices::parse(refused, "0").is_none(), "{refused}");
        }
    }
}
