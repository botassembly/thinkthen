# A corrupt recording entry bills every retry and never repairs

Status: Open

A recording folder that holds one truncated entry becomes stuck. The same documented command sends a fresh request, pays for it, then refuses the write and leaves the entry broken. Every retry pays again, and the only repair today is deleting the file by hand.

## Reproduction

Verified on the rebuilt release binary at main `db19349`, 2026-09-21:

    $ THINKTHEN_API_KEY=canary-qa218 thinkthen decide 'Is it refundable?' --record r6 --url http://127.0.0.1:8942 < one.txt
    (exit 0; the stand-in request log holds 6 requests)
    $ f=$(ls r6/*.json | head -1)
    $ head -c 40 "$f" > /tmp/trunc; cp /tmp/trunc "$f"
    $ THINKTHEN_API_KEY=canary-qa218 thinkthen decide 'Is it refundable?' --record r6 --url http://127.0.0.1:8942 < one.txt
    thinkthen: the entry `043bd07231cdd9f3699276c35b4c585ce24bc0bbdae81d0e2a285eba348d579a.json` was refused: the file is not a recording entry: the JSON at line 3 column 1 is not one
    (exit 5; the request log gained one request, 6 to 7; the entry is still 40 bytes)

The `--cache` path refuses a corrupt entry before sending anything: 0 new requests and exit 5. The `--record` path writes after the call, so the money is spent before the refusal.

The same write-after-call order produced a wave-1 finding on another face: `2026-09-21-a-write-failure-after-a-good-exchange-discards-the-paid-answer.md` records a read-only folder that fails after the request. This page adds the stuck-folder face.

## Expected

The run refuses before the request when the folder already holds a corrupt entry for that digest, the way `--cache` does. Failing that, a successful call overwrites the broken entry and heals the folder. The stopped-run line says a request was sent and its answer was discarded.

## How bad it is for a user

Major. A crash, a full disk, or a hand edit leaves a folder that no documented command can repair, and every attempt to repair it costs money.

Found by experiment 218, wave 1.5, the blind seat.
