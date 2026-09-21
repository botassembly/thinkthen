# Printed speed and cost numbers name no measuring record

Status: Open

The repository's own rule is that a number on a page names the record that measured it. Two printed numbers break the rule.

## README's "over 300 ms"

    $ grep -n "300 ms" README.md
    27: ... One measured call took over 300 ms, and a shell tool adds a process start ...
    $ grep -rn "300 ms" probes/ sdlc/records/ | wc -l
    0

The sentence says "one measured call" and no probe or record holds the measurement. The same sentence appears in `sdlc/planning/ten-use-cases.md` line 14 with no source either. The number cannot be rerun without a paid call, which makes the missing record the only check it has.

## Slide 12's cost figures

    $ grep -o "3,000 lines of a novel judged for 3.6 cents.\|0.042 per million input tokens" \
        decks/2026-09-21-thinkthen-semantic-commands/slides/12-cost/slide.html
    3,000 lines of a novel judged for 3.6 cents.
    0.042 per million input tokens, on Jev 1.13.

The slide's 3.6 cents and the per-million price are vendor-price arithmetic over measured token counts, and the line that prints them names no record. `deck.md` line 110 does it right: its 9.65 s binding number names `experiments/205`. The fix for slide 12 is the same pattern, on the marketing side.

## What was confirmed

Slide 13's request arithmetic was confirmed against a stand-in: one record with one question is one request, one record with two questions (two `tag` labels, two `annotate` questions) is still one request, and 1,000 records make exactly 1,000 requests. Command-local numbers also reran clean: process start about 1.6 ms per invocation, and a warm 100-record cache pass answers in 0.006 s with zero requests against 0.065 s cold.

## How bad it is for a user

Minor. The numbers may well be true; a reader who checks them finds nothing behind two of them, and the repository rule exists so that never happens.

Found by experiment 218, wave 1, area 6.
