//! Small input tables grouped by threshold and top limit to bound each test.

use super::*;

#[test]
fn filter_prints_a_subsequence_at_a_low_threshold() -> io::Result<()> {
    filter_subsequence("0.1")
}

#[test]
fn filter_prints_a_subsequence_at_the_default_threshold() -> io::Result<()> {
    filter_subsequence("0.5")
}

#[test]
fn filter_prints_a_subsequence_at_a_high_threshold() -> io::Result<()> {
    filter_subsequence("0.9")
}

fn filter_subsequence(cut: &str) -> io::Result<()> {
    for count in 0..8_usize {
        let input = spread(count);
        let lines: Vec<&str> = input.lines().collect();

        let listener = by_place()?;
        let output = over(
            "filter",
            listener.base(),
            &["--jsonl", "--field", "/body", "--threshold", cut],
            &input,
        )?;
        assert_eq!(code(&output), 0, "filter {count} {cut}: {}", said(&output));
        let kept = printed(&output);
        let kept: Vec<&str> = kept.lines().collect();
        let mut next = lines.iter();
        for line in &kept {
            assert!(
                next.any(|held| held == line),
                "filter {count} {cut} printed a line that is not the next input line"
            );
        }
        assert!(kept.len() <= count, "filter {count} {cut} printed too much");
    }
    Ok(())
}

#[test]
fn rank_prints_a_full_permutation_without_a_top_limit() -> io::Result<()> {
    rank_permutation(None)
}

#[test]
fn rank_top_one_prints_one_distinct_input_record() -> io::Result<()> {
    rank_permutation(Some(1))
}

#[test]
fn rank_top_three_prints_distinct_input_records_up_to_the_limit() -> io::Result<()> {
    rank_permutation(Some(3))
}

#[test]
fn rank_with_an_oversized_top_limit_prints_a_full_permutation() -> io::Result<()> {
    rank_permutation(Some(99))
}

fn rank_permutation(top: Option<usize>) -> io::Result<()> {
    for count in 0..8_usize {
        let input = spread(count);
        let lines: Vec<&str> = input.lines().collect();

        let listener = by_place()?;
        let asked = top.map(|n| n.to_string());
        let mut arguments = vec!["--jsonl", "--field", "/body"];
        if let Some(number) = asked.as_deref() {
            arguments.extend(["--top", number]);
        }
        let output = over("rank", listener.base(), &arguments, &input)?;
        assert_eq!(code(&output), 0, "rank {count} {top:?}: {}", said(&output));
        let ordered = printed(&output);
        let mut ordered: Vec<&str> = ordered.lines().collect();
        assert_eq!(
            ordered.len(),
            top.unwrap_or(count).min(count),
            "rank {count} {top:?} printed the wrong number of records"
        );
        ordered.sort_unstable();
        ordered.dedup();
        assert!(
            ordered.iter().all(|line| lines.contains(line)),
            "rank {count} {top:?} printed a line that was never read"
        );
        assert_eq!(
            ordered.len(),
            top.unwrap_or(count).min(count),
            "rank {count} {top:?} printed one record twice"
        );
    }
    Ok(())
}
