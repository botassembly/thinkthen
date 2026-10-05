//! Independent literal turns and attribution, including uneven saved lists.
use super::turns;

#[test]
fn rank_set_depths_consume_duplicates_and_preserve_member_attribution() {
    let cases = [
        (
            vec![vec![0, 1, 2], vec![0, 2, 1]],
            vec![(0, 0), (1, 0), (2, 1)],
        ),
        (
            vec![vec![1, 2, 0], vec![2, 0, 1]],
            vec![(1, 0), (2, 1), (0, 1)],
        ),
        (
            vec![vec![0, 1, 2], vec![0, 2, 1], vec![3, 2, 1]],
            vec![(0, 0), (3, 2), (1, 0), (2, 1)],
        ),
        (
            vec![vec![], vec![2, 1], vec![2, 3, 1]],
            vec![(2, 1), (1, 1), (3, 2)],
        ),
    ];
    for (lists, expected) in cases {
        assert_eq!(turns(&lists, None), expected);
    }
    assert_eq!(turns(&[vec![0, 1], vec![0, 2]], Some(0)), []);
    assert_eq!(
        turns(&[vec![0, 1, 2], vec![0, 2, 1]], Some(2)),
        [(0, 0), (1, 0)]
    );
}
