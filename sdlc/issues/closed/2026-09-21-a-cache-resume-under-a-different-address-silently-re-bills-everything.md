# A cache resume under a different address silently re-bills everything

Status: Addressed by ticket 0065

A recording entry's name is the SHA-256 of the adapter, the URL, and the request bytes (`specification/recording.md`, "An entry"). The address is inside the digest, and nothing tells the user. A user who resumes a stopped run without the identical `THINKTHEN_BASE_URL` or `--url` misses every cached answer, sends every record to the default vendor address, and sees a clean exit 0 with no warning.

An experiment run fell into this trap on 2026-09-21 and paid for it. One rerun in a fresh shell without `THINKTHEN_BASE_URL` exported, with the machine's ambient key present, re-sent 20 records to `https://api.typesafe.ai/v1/systemone`. The recording folder holds the proof:

    $ grep -h '"url"' resumecache/*.json | sort | uniq -c
    6  "url": "http://127.0.0.1:8806/systemone"
    20 "url": "https://api.typesafe.ai/v1/systemone"

The 20 paid calls reported 5,630 input tokens and 420 output tokens. The 6 stand-in entries were useless to that run. The spend bypassed the pre-declared live ledger, because the launcher never ran. The 20 entries stay uncommitted in the experiment's scratch folder, because a recording commits its evidence. No key material appears in any entry.

The orchestrator verified the mechanism independently. Three records cached under one loopback address, then the same command with `--url` naming a second loopback address:

    $ thinkthen filter 'Does this ask for a refund?' --jsonl --field /body \
        --url http://127.0.0.1:8892 --cache cache < three.jsonl
    {"body":"refund please"}
    {"body":"hello there"}
    {"body":"money back now"}
    exit 0, and the second stand-in saw 3 fresh requests

The run printed all three records, exited 0, and said nothing about the cold cache or the new address. A stopped 100,000-record run resumed this way re-bills the whole run.

The `--replay` path at least fails, but its message names only the digest:

    $ THINKTHEN_BASE_URL=http://127.0.0.1:8803 thinkthen decide '...' --quiet \
        --replay demos/27-test-with-no-network/recording/ < .../report.txt
    thinkthen: the replay folder holds no entry named `6bb436e2...e1e9.json`
    exit 5

A stranger cannot learn from that sentence that the address is part of the entry's name. The same recording replays cleanly under the address it was recorded with.

## What is needed

At minimum, a warning when a cache or record folder answers none of its requests because the address changed. `--details` carries `meta.url`, but nothing on the miss path names the address a run is about to send everything to. The replay miss message can say that the digest covers the address, so an address change reads as an address change and not as a missing file.

## How bad it is for a user

Major. Silent money loss on a resume, with an exit code that reports success.

Found by experiment 218, wave 1, areas 8 and 9.
