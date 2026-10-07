//! The retained member rows and merged top prefix protect the memory boundary.
use super::{Judged, Output};
use crate::core::Outcome;
use crate::engine::usage::Counters;

fn row(name: &str, probability: f64) -> Judged {
    Judged {
        model: None,
        printed: Some(name.to_owned()),
        position: None,
        outcome: Outcome::Yes,
        replayed: false,
        order_value: Some(probability),
        rank: None,
        partial_failure: false,
        profile_mismatch: None,
    }
}

#[test]
fn rank_set_retains_m_times_k_candidates_and_counts_every_original() {
    for (limit, expected) in [
        (1, b"a\n".as_slice()),
        (2, b"a\nb\n".as_slice()),
        (3, b"a\nb\nc\n".as_slice()),
    ] {
        let mut bytes = Vec::new();
        let usage = Counters::new(None);
        let mut output = Output::ordered(&mut bytes, Some(limit), &usage);
        for (index, (name, first, second)) in [("a", 0.7, 1.0), ("b", 0.6, 0.98), ("c", 0.5, 0.99)]
            .into_iter()
            .enumerate()
        {
            output
                .take_members(vec![row(name, first), row(name, second)], index)
                .expect("members admitted");
            assert_eq!(output.members.len(), 2);
            assert!(output.members.iter().all(|member| member.len() <= limit));
            assert!(output.members.iter().map(Vec::len).sum::<usize>() <= 2 * limit);
        }
        output.ended().expect("all members completed");
        assert_eq!(bytes, expected);
        assert_eq!(usage.run_snapshot().records, 3);
    }
}
