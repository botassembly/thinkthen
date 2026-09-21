# Exit 6 and partial failure are told two ways

Status: Open

Two specification pages tell the story of a failed question inside a good reply in opposite words, and the code already follows one of them.

- `specification/records.md:107`: "Codes 6, 7, and 8 stay reserved and no command uses them."
- `specification/channels.md:58`: "6 | `annotate` completed its input, but one or more logical questions failed"
- `specification/annotate.md:47`: "A backend failure is never `null`, because a failure ends the run."
- `specification/annotate.md:70`: "0 when the run finished with every question answered, 6 when it finished after one or more logical questions failed"
- `specification/backends.md:75`: "It preserves those failures only when another logical question in the reply is valid."
- `specification/result.md:113`: "A failed bare value is `{"failed":{"kind":"backend","cause":CAUSE}}`."
- `specification/recording.md:57`: a recorded partial reply replays "the same good answers, failed markers, failure count, and exit 6".

The code follows the channels story. `crates/thinkthen/src/annotate.rs:344` sets `partial_failure: failed_questions > 0`.

## Reproduction

Verified on the rebuilt release binary at main `db19349`, 2026-09-21. A question set with one `decide` question and one `choose` question, against a stand-in whose choice distribution sums to 0.5:

    $ cat > mixed.json <<'EOF'
    {"version":1,"questions":{"good":{"decide":"Is this a complaint?"},"bad":{"choose":"Which team?","options":["a","b"]}}}
    EOF
    $ printf 'my refund is late and I am angry\n' > msg.txt
    $ echo '{"mode":"badsum"}' > ctl/mode.json
    $ THINKTHEN_API_KEY=canary-qa218 thinkthen annotate mixed.json --url http://127.0.0.1:8941 < msg.txt
    {"good":true,"bad":{"failed":{"kind":"backend","cause":"invalid_distribution"}}}
    (exit 6)

The good answer survived and the failed question carried its marker. A reader of `records.md:107` learns that exit 6 cannot happen. Wave 1.5 first met this on a nine-hour-old binary, where the whole reply was refused at exit 4; the rebuild confirmed the current code.

## Expected

Two page edits and no code change. `records.md:107` stops calling 6 unused and names it as `annotate`'s partial-failure code. `annotate.md:47` moves to the preserved-failure story. The "never `null`" caution stays and gets sharper: a failure is a marker, never `null`, and a reply with no valid answer refuses the run at exit 4.

## How bad it is for a user

Major. A caller who reads `records.md` believes partial failure is impossible and writes no branch for exit 6, and `annotate` returns it.

Found by experiment 218, wave 1.5, the spec cross-reader seat.
