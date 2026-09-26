# The PostgreSQL check's `has` passes any needle that holds a newline

Status: open. Filed 2026-09-26 by Quick Fix qf-review-273's review.

## Problem

`has` in `databases/postgresql/check.sh` runs `grep -qF -- "$2"`. GNU grep reads a needle with a newline as several patterns, one per line. A needle that starts or ends with a newline holds an empty pattern, and an empty pattern matches every line. A needle with two lines passes when either line appears anywhere.

Five checks build such a needle with `printf`:

- `dev_zero_refuses_fast` and `a_panic_is_an_error` look for `\n1`. Each passes on any output.
- `a_timed_out_batch_leaves_the_session_working` looks for `\nt`. It passes on any output.
- `a_changed_limit_rebuilds` looks for `3\n`. It passes on any output.
- `the_total_holds_across_rows` looks for `t` then the refusal on the next line. It passes when either line appears alone.

So these steps do not prove the answer they name. Each step's other checks, such as `bcount`, still hold.

## Direction

Make `has` match the whole needle as one string, such as with a bash `[[ $1 == *"$2"* ]]` test, and run the check once to see which steps then fail. Or pin each answer line with `same "$(tail -n1 <<<"$out")" t`, as `throttle_setting_range` does.
