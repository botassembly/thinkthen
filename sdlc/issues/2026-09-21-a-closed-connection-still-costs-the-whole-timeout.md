# A closed connection still costs the whole timeout

Status: Open

A backend that accepts a connection and then closes it without a response is only noticed when the deadline fires. The attempt runs to the full `--timeout`, the failure reads as a timeout, and every dead peer costs the whole deadline per attempt.

## Reproduction

A local stand-in closes the connection without responding. Commands from a scratch folder, 2026-09-21:

    $ /usr/bin/time -f "wall=%es" thinkthen decide 'Q?' --timeout 2 --max-retries 0 --input msg.txt
    thinkthen: the backend could not be reached: timeout: global
    wall=2.06s

With the default `--timeout 30`, a 20-second guard killed the process while it still waited. A second run at `--timeout 5` also ran to 5.07 seconds.

The peer's close is a fact the client can see at once. The wait is bounded by the deadline, so this is slow rather than a hang, and the retry rule still applies.

## Expected

`specification/backends.md` says `--timeout` "covers one attempt from connect to the last byte." Nothing in the page forbids noticing a close early. A peer that hung up is a transport failure the client can report in milliseconds.

One caveat from the tester: the close was produced by a Python stand-in's close-without-response. A mid-reply cut on a cleaner peer is the stronger case for the same finding.

## How bad it is for a user

Minor. A dead backend costs the user `--timeout` seconds per attempt with a message that names the deadline and not the close.

Found by experiment 218, wave 1, areas 3 and 7.
