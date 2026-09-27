use super::{
    BUILT_IN, Closed, LimitCase, SONG, backend, check, decide, loopback, profile, sized, text,
};

#[test]
fn the_ceiling_closes_batches_at_every_address_and_a_profile_lowers_it() {
    use Closed::{End, Limit};
    let built_in = || backend(BUILT_IN, "jev-latest");
    let cases: [LimitCase; 7] = [
        (
            built_in(),
            None,
            decide(SONG),
            sized(20_000),
            vec![(3, End)],
        ),
        (
            built_in(),
            None,
            decide(SONG),
            sized(40_000),
            vec![(2, Limit), (1, End)],
        ),
        (
            built_in(),
            None,
            decide(SONG),
            vec![text(&"a".repeat(100_000))],
            vec![(1, End)],
        ),
        (
            built_in(),
            profile(r#""max_request_bytes":200000"#),
            decide(SONG),
            sized(40_000),
            vec![(2, Limit), (1, End)],
        ),
        (
            loopback(),
            None,
            decide(SONG),
            sized(40_000),
            vec![(2, Limit), (1, End)],
        ),
        (
            loopback().with_request_size(200_000),
            None,
            decide(SONG),
            sized(40_000),
            vec![(3, End)],
        ),
        (
            loopback().with_request_size(200_000),
            profile(r#""max_request_bytes":50000"#),
            decide(SONG),
            sized(40_000),
            vec![(1, Limit), (1, Limit), (1, End)],
        ),
    ];
    check(cases);
}
