# A run stopped by SIGINT prints no stopped-at line

Status: Closed by ticket 0074 after independent review and the integrated local ladder

The [2026-09-23 completion checklist](../planning/mainline-readiness-2026-09-23.md#complete-ticket-0074-next) records the deterministic subprocess, signal-state, failure-path, review, and landing requirements. The accepted branch waits only for remote-main landing.

A run stopped by a bad record prints a line on standard error naming the record and the resume path. A run stopped by Ctrl-C prints nothing.

## Reproduction

A 20-record `filter --cache` run against a local stand-in, interrupted after 2 seconds, 2026-09-21:

    $ timeout -s INT 2 thinkthen filter 'Does this ask for a refund?' \
        --url http://127.0.0.1:8806 --jsonl --field /body --cache resumecache2 --jobs 1 \
        < resume.jsonl
    (7 records printed on standard output, standard error empty, exit 130)

For comparison, a bad record prints:

    thinkthen: stopped at record 1; 0 records finished, 0 records from a recording

## Expected

The same stopped-at line with the resume hint, or a documented reason SIGINT differs. The resume itself works: the interrupted run's entries are all valid, and a rerun sends only the unfinished records.

## How bad it is for a user

Minor. The run is safe to resume and the exit code is 130, but a user who stopped a batch gets no count of what finished.

Found by experiment 218, wave 1, areas 4 and 8.

## Product ruling, 2026-09-22

Catch SIGINT cooperatively, stop starting requests after cancellation is observed, let requests already sent finish within the existing attempt timeout, print completed ordered output and the stopped-at line, then restore the normal shell meaning by re-raising SIGINT for exit 130. A single-document or aggregate request already sent also finishes within its attempt timeout; its completed output is written before SIGINT is re-raised. No new exit code is introduced. Ian can overturn this ruling.
