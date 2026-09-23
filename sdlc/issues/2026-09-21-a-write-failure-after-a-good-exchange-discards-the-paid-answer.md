# A write failure after a good exchange discards the paid answer

Status: Closed by ticket 0061. A writing mode opens its private temporary entry before key lookup or a request.

`--record` into a folder the process cannot write makes the backend request first and fails at the write. The bill pays for an answer the run throws away.

## Reproduction

A read-only recording folder, a local stand-in, 2026-09-21:

    $ chmod -w ro-rec
    $ printf 'Nothing works since the update.\n' | thinkthen decide 'Does this convey urgency?' \
        --url http://127.0.0.1:8806 --record ro-rec
    thinkthen: the recording folder could not be read or written: Permission denied (os error 13)
    exit 5, nothing on standard output, 1 backend request made and its answer discarded

The refusal message is clear and the failure is loud. Both are right. The cost is the wasted request.

## Expected

Checking the folder before the first request closes it. A sentence in the manual saying the request happens before the write also closes it, more cheaply.

## How bad it is for a user

Minor. A user with a misconfigured folder pays once per record to learn nothing.

Found by experiment 218, wave 1, area 8.
