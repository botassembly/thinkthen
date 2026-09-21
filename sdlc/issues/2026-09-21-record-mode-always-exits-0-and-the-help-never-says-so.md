# Record mode always exits 0 and the help never says so

Status: Open

A record run exits 0 no matter what its records answer. Three no answers, three unresolved answers, three ties: exit 0 each time. The single-document contract carries the answer in the exit code, and the help of every record-capable verb prints that contract without scoping it to one document.

## Reproduction

Verified against a loopback stand-in, 2026-09-21:

    $ echo '{"mode":"ok","prob":0.0}' > ctl/mode.json
    $ printf 'alpha\nbeta\ngamma\n' | THINKTHEN_API_KEY=canary-qa218 thinkthen decide 'Is it?' --lines --url http://127.0.0.1:8903
    false
    false
    false
    (exit 0)
    $ echo '{"mode":"ok","prob":0.5}' > ctl/mode.json
    $ printf 'alpha\nbeta\ngamma\n' | THINKTHEN_API_KEY=canary-qa218 thinkthen decide 'Is it?' --lines --threshold 0.1:0.9 --url http://127.0.0.1:8903
    null
    null
    null
    (exit 0)

The same setups over one document exit 1 and 3.

`specification/channels.md:63` fixes the rule: "In record mode the exit code reports the run. No record's answer sets it." The help does not carry that sentence. `thinkthen decide --help` opens with "The answer is a bare `true`, `false`, or `null`, and the exit code is 0 for yes, 1 for no, and 3 for unresolved", and no sentence scopes it. The tool knows the scoping: `decide --lines --quiet` is refused with `thinkthen: --quiet carries the answer in the exit code, and no record's answer sets it`.

## The trap

    thinkthen decide 'Is this safe?' --lines < rows && deploy

A file of a hundred falses exits 0, and the second command runs. The values on standard output carry the answers, and a script that reads `$?` never sees them.

## Expected

One sentence in the help of each record-capable verb, the same sentence `channels.md:63` carries: a record run exits 0 when it completes, and only the printed values carry the answers.

## How bad it is for a user

Major. A gate script or a deploy guard reads the documented exit codes and takes the action after every record answered no.

Found by experiment 218, wave 1.5, the blind seat.
