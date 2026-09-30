//! Existing distinct command behavior, one test per verb so they spread over
//! the runner.

use super::*;

#[test]
fn filter_prints_a_subsequence_of_its_input() -> io::Result<()> {
    for count in 0..8_usize {
        let input = spread(count);
        let lines: Vec<&str> = input.lines().collect();

        for cut in ["0.1", "0.5", "0.9"] {
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
    }
    Ok(())
}

#[test]
fn rank_prints_a_permutation_of_its_input() -> io::Result<()> {
    for count in 0..8_usize {
        let input = spread(count);
        let lines: Vec<&str> = input.lines().collect();

        for top in [None, Some(1_usize), Some(3), Some(99)] {
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
    }
    Ok(())
}
