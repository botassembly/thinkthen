# A Retry-After wait is not bounded by --timeout

Status: Closed by ticket 0064

A 429 that carries a Retry-After header sets the wait between attempts, and the wait ignores `--timeout`. The `--timeout` help says "Seconds one attempt may take, from connect to the last byte", and a server can hold the run for up to 60 seconds between attempts with no output.

## Reproduction

A loopback stand-in answered 429 with a header, 2026-09-21:

    $ echo '{"mode":"status","status":429,"retry_after":20}' > ctl/mode.json
    $ thinkthen decide 'Is it?' --max-retries 1 --timeout 2 --url http://127.0.0.1:8903 < ev.txt
    thinkthen: the backend answered with status 429: the backend's rate limit was reached
    (exit 4; wall 20.01 s)

Measured again on the rebuilt release binary at main `db19349`:

    Retry-After: 5 with --timeout 2   -> wall 5.006 s
    Retry-After: 70                   -> wall 60.007 s

`specification/backends.md:47` caps the wait at 60 seconds, and the cap holds. The wait still sits outside every bound the user sets, and nothing prints while it runs. With `--max-retries 2` a rate-limited backend can spend three waits of up to 60 seconds each in silence.

## Expected

The `--timeout` help says what it bounds and what it does not. Better, the wait joins the attempt budget, or the run prints one line saying it waits N seconds for the rate limit. A user who typed `--timeout 2` should not watch minutes of silence.

## How bad it is for a user

Minor. The wait is capped and the run stays correct, and a scripted user cannot bound the time while a person cannot see why nothing happens.

Found by experiment 218, wave 1.5, the blind seat.

Ticket 0064 caps header-selected and exponential waits at `--timeout`. A loopback test proves a 30-second header retries after the one-second timeout boundary while the existing unit test retains the independent 60-second header ceiling.
