//! Valid tag paths with least-member-piece membership support.

const N: usize = 5;
const NEG: f64 = f64::NEG_INFINITY;

fn starts(tag: usize) -> bool {
    matches!(tag, 0 | 3 | 4)
}
fn ends(tag: usize) -> bool {
    matches!(tag, 2..=4)
}
fn follows(left: usize, right: usize) -> bool {
    if matches!(left, 0 | 1) {
        matches!(right, 1 | 2)
    } else {
        starts(right)
    }
}

/// A decoded name's inclusive piece bounds and least member-piece membership.
pub(super) struct Span {
    pub(super) first: usize,
    pub(super) last: usize,
    pub(super) p: f64,
}

#[expect(
    clippy::indexing_slicing,
    reason = "dynamic-programming indices stay within allocated tag tables"
)]
pub(super) fn decode(probabilities: &[[f64; N]]) -> Vec<Span> {
    let n = probabilities.len();
    if n == 0 {
        return Vec::new();
    }
    let weights: Vec<[f64; N]> = probabilities
        .iter()
        .map(|row| row.map(|p| p.max(0.000001).ln()))
        .collect();
    let mut best = [NEG; N];
    let mut parents = vec![[0; N]; n];
    for tag in 0..N {
        if starts(tag) {
            best[tag] = weights[0][tag];
        }
    }
    for i in 1..n {
        let mut next = [NEG; N];
        for tag in 0..N {
            let mut parent = 0;
            for (prev, score) in best.iter().enumerate() {
                if !follows(prev, tag) {
                    continue;
                }
                let candidate = score + weights[i][tag];
                if candidate > next[tag] {
                    next[tag] = candidate;
                    parent = prev;
                }
            }
            parents[i][tag] = parent;
        }
        best = next;
    }
    let mut tag = (0..N)
        .filter(|&t| ends(t))
        .reduce(|a, b| if best[a] >= best[b] { a } else { b })
        .unwrap_or(4);
    let mut path = vec![0; n];
    for i in (0..n).rev() {
        path[i] = tag;
        tag = parents[i][tag];
    }
    let mut spans = Vec::new();
    let mut first = 0;
    for (i, &tag) in path.iter().enumerate() {
        if tag == 0 {
            first = i;
        }
        if tag == 3 {
            first = i;
        }
        if matches!(tag, 2 | 3) {
            let p = probabilities[first..=i]
                .iter()
                .map(|row| 1.0 - row[4])
                .fold(1.0, f64::min);
            spans.push(Span { first, last: i, p });
        }
    }
    spans
}
