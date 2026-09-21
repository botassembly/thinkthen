# The candidate-6 flow goes red on its designed outcome

Status: Open

The marketing use cases print a `find --none | decide` pipe. The page promises that when nothing fits, "`find` prints nothing, exits 3, and `decide` never runs." A pipe stage always starts. `decide` runs, reads an empty pipe, and exits 2, and under `set -o pipefail` the whole flow reports 2. The designed "open a new incident" outcome reads as an error pipeline.

## Reproduction

Commands as the use-case page prints them, against a local stand-in in `pick_last` mode so `none` wins, 2026-09-21:

    $ cat > incidents-open.txt <<'TXT'
    INC-1 checkout returns 500 at payment
    INC-2 search results load slowly
    TXT
    $ alert='Card charges fail with 500 at checkout'
    $ set -o pipefail
    $ thinkthen find --none "Which incident covers: $alert" --lines \
        < incidents-open.txt \
        | thinkthen decide "Is this the same failure as: $alert"
    thinkthen: evidence is text, not white space
    $ echo $?
    2

`find` behaves per `specification/find.md`: nothing printed, exit 3. The `decide` stage then runs on an empty stream and refuses it. The empty-stream message is its own issue, `2026-09-21-the-empty-evidence-refusal-names-the-rule-backwards.md`.

## Where the fix belongs

The tool behaves per its specification. The published flow and its page turn the designed outcome into an error, so the fix lands in the marketing use cases. A guard that reads the exit code of `find` before asking `decide`, or a flow shape that keeps the stages apart, holds the page's promise.

## How bad it is for a user

Major for the published flow. A user copies a composed pipeline from the deck and it fails on the outcome the page itself describes as normal.

Found by experiment 218, wave 1, areas 4 and 12.
