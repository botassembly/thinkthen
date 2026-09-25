# A huge deadline prints hundreds of digits

Status: Open

Filed on 2026-09-25 from the Python surface port (ticket 0105). No key was used, and no request left the machine.

## What happens

`CallOptions::deadline_seconds` refuses a budget past 4,294,967,295 seconds. Its sentence formats the raw `f64` with `{value}`. Rust's `Display` for `f64` prints every integer digit, so a deadline of `1e300` gives a sentence with a 301-digit number:

    a deadline of 1000000000000000052504760255204420248704468581108159154915854115111802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115467196834763571264 seconds is not -1, 0, or a positive budget of at most 4294967295 seconds

The source is `crates/thinkthen/src/public/options.rs`, in `deadline_seconds`. Any surface that passes a user's deadline through prints the same sentence.

## What would fix it

Print the value in a short form when it is past the cap, such as `{value:e}` (`1e300`), or name only the cap. Keep NaN and infinity as they read now.

## Tests to update when fixed

- `libraries/python/tests/test_inputs.py`, `test_deadlines_follow_adr_0041`: it pins the full sentence as `f"a deadline of {10**300} seconds {budget}"`. Change it to the new short form.
- The engine's own deadline tests, if any pin the long form.
