# 0003: The first live call

Made by hand by the steering agent on 2026-09-19, after ticket 0003 landed, from the main checkout at the landed commit.

## What ran

`--plan` printed the six fields, named the key variable, held no key, and showed the evidence in the request. Exit code 0.

Four live commands followed against the built-in `jev` profile with `--key-env`: a clear yes, a clear no, a vague message, and a pipe closed early by `head`.

## What came back

Every call ended with `thinkthen: the backend answered with status 402` and exit code 4. The tool did not retry, because 402 is outside the retried list. A direct request to the same address with the same key returned a billing error: the organization has no available credits. The request shape was accepted far enough to reach billing.

No tokens were spent.

## What it proved and what it did not

- The path from the shell to the vendor works: TLS, the address, the bearer header, the failure mapping, and the exit code.
- The message gave the status and no body, as the specification says.
- No judgment has come back from a live backend yet. That waits on credits, which only Ian can add.

## What it changes

A bare status code tells a user too little for the common failures. `specification/backends.md` now gives a fixed phrase for 401, 402, 403, 404, 422, and 429. The phrases are fixed text, so the rule against printing a response body still holds. Ticket 0004 carries the change.
