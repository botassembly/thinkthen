//! The order `rank` prints its records in, and the cut `--top` makes.
//!
//! Every record is judged before any of this runs, so `top` saves no request.
//! ADR 0017 puts the sort, the tiebreak, and the slice here, so a library in
//! another language is held to the same cases.

/// Put the places in order of the probability of yes, highest first.
///
/// The sort is stable, so two records whose probabilities are exactly equal
/// keep their input order. `top` names how many places the caller prints, and
/// `None` prints every one.
#[must_use]
pub(crate) fn ranking(of: &[f64], top: Option<usize>) -> Vec<usize> {
    let mut places: Vec<(usize, f64)> = of.iter().copied().enumerate().collect();
    places.sort_by(|(_, one), (_, other)| other.total_cmp(one));
    places
        .into_iter()
        .take(top.unwrap_or(usize::MAX))
        .map(|(place, _)| place)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::ranking;
    use proptest::collection::vec;
    use proptest::option;
    use proptest::{prop_assert, prop_assert_eq, proptest};

    #[test]
    fn the_highest_probability_comes_first_and_an_exact_tie_keeps_input_order() {
        assert_eq!(ranking(&[0.1, 0.9, 0.5], None), [1, 2, 0]);
        assert_eq!(ranking(&[0.4, 0.4, 0.4], None), [0, 1, 2]);
        assert_eq!(ranking(&[0.4, 0.9, 0.4, 0.9], None), [1, 3, 0, 2]);
    }

    #[test]
    fn top_takes_the_first_places_of_the_order_and_never_more_than_there_are() {
        let odds = [0.1, 0.9, 0.5];
        assert_eq!(ranking(&odds, Some(1)), [1]);
        assert_eq!(ranking(&odds, Some(2)), [1, 2]);
        assert_eq!(ranking(&odds, Some(3)), [1, 2, 0]);
        assert_eq!(ranking(&odds, Some(9)), [1, 2, 0]);
        assert_eq!(ranking(&[], Some(3)), [0_usize; 0]);
        assert_eq!(ranking(&[], None), [0_usize; 0]);
    }

    proptest! {
        /// The order is a permutation of the places, or the first `top` of one.
        #[test]
        fn the_order_is_a_permutation_of_the_input_or_a_prefix_of_one(
            odds in vec(0.0_f64..=1.0, 0..12),
            top in option::of(0_usize..16),
        ) {
            let whole = ranking(&odds, None);
            let mut seen = whole.clone();
            seen.sort_unstable();
            prop_assert_eq!(seen, (0..odds.len()).collect::<Vec<_>>());

            let cut = ranking(&odds, top);
            prop_assert_eq!(cut.len(), top.unwrap_or(odds.len()).min(odds.len()));
            prop_assert_eq!(&cut, &whole.get(..cut.len()).expect("a prefix of the order"));
        }

        /// No record sorts above one the model answered higher.
        #[test]
        fn the_probabilities_never_rise_along_the_order(odds in vec(0.0_f64..=1.0, 0..12)) {
            let order = ranking(&odds, None);
            for pair in order.windows(2) {
                let [before, after] = pair else { continue };
                let one = odds.get(*before).copied().expect("a judged record");
                let next = odds.get(*after).copied().expect("a judged record");
                prop_assert!(one >= next);
                // An exact tie keeps input order, so the places rise with it.
                if (one - next).abs() < f64::EPSILON {
                    prop_assert!(before < after);
                }
            }
        }
    }
}
