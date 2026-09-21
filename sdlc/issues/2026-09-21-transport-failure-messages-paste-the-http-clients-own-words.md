# Transport failure messages paste the HTTP client's own words

Status: Open

A backend that is too slow, a backend that never answers, and a connection that dies all print one identical message, and the message ends in the HTTP client library's own error-kind string. A stranger cannot tell the three apart, and no message names the `--timeout` or `--max-retries` levers.

## Reproduction

Commands from a scratch folder against a local stand-in, 2026-09-21:

    $ # a reply that arrives too late
    $ timeout 20 thinkthen decide 'Does this convey urgency?' --timeout 3 --max-retries 0 < evidence.txt
    thinkthen: the backend could not be reached: timeout: global
    exit 4, 3.04s, 1 send

    $ # a reply that never comes
    $ timeout 20 thinkthen decide 'Does this convey urgency?' --timeout 2 --max-retries 0 < evidence.txt
    thinkthen: the backend could not be reached: timeout: global
    exit 4, 2.01s, 1 send

    $ # a connection that dies
    $ timeout 20 thinkthen decide 'Does this convey urgency?' --timeout 5 --max-retries 0 < evidence.txt
    thinkthen: the backend could not be reached: timeout: global
    exit 4, 5.07s, 1 send

`timeout: global` is the HTTP client's name for its own timeout kind. The OS-error forms are better: `io: Connection refused (os error 111)` and `io: failed to lookup address information: Name or service not known` name the actual event.

## Expected

`specification/backends.md` sets the bar for statuses, and the experiment brief sets the same bar for transport: each message tells a stranger what happened and what to do next. A message that names the event, the seconds that elapsed, and the `--timeout` lever meets it. No message needs to name an address or echo a body.

## How bad it is for a user

Minor. The exit code and the event class are right, and the wording is the gap.

Found by experiment 218, wave 1, areas 3 and 7.
