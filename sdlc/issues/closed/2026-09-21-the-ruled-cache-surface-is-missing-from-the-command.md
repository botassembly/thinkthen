# The ruled cache surface is missing from the command

Status: Closed by tickets 0062 and 0063 on 2026-09-22

Ticket 0062 added the default platform folder, `THINKTHEN_CACHE`, `--no-cache`, the closed read-only configuration, the 100,000,000-byte prune target, and `cache prune DIR`. Ticket 0063 adds `status` and persistent numeric counters.

ADR 0017 rules a cache that is on by default, and the binary ships none of it. A user who reads the ruling or the manual expects answers saved on disk, a way to see the cache, a way to clear it, and a cap. The command offers only `--record DIR`, `--replay DIR`, and `--cache DIR`, exactly as `specification/recording.md` settles them. Everything ruled beyond those three is unbuilt.

This is ruled-and-unbuilt work, not a regression. ADR 0017 line 85 says the 100 MB cap and the prune "land with the default location, never after it."

## What is absent

ADR 0017 (`sdlc/planning/adr/0017-libraries-over-one-bound-core.md`, lines 78-89) rules: cache on by default at the XDG cache home, `--cache DIR` or `THINKTHEN_CACHE=DIR` naming a folder, `--no-cache` for one run, a configuration switch and a cache-size setting in a configuration file at `$XDG_CONFIG_HOME/thinkthen`, a 100 MB cap applied by `thinkthen cache prune DIR` oldest-written first, and `thinkthen status` printing both folders, the cache size, and the entry count.

Every one of those is absent. Reproduction, observed 2026-09-21 at main 4791c10:

    $ thinkthen status
    error: unrecognized subcommand 'status'
    exit 2
    $ thinkthen cache prune some-dir
    error: unrecognized subcommand 'cache'
    exit 2
    $ thinkthen decide 'Is this urgent?' --no-cache < /dev/null
    error: unexpected argument '--no-cache' found
    exit 2

`THINKTHEN_CACHE` is ignored. A run with `THINKTHEN_CACHE=$PWD/envcache` writes nothing there, and an identical rerun reaches the backend again. A run with `HOME=$PWD/fakehome XDG_CACHE_HOME=$PWD/xdgcache` and no cache flags writes nothing to either folder. The 100 MB limit, the prune-oldest-first rule, and the warm-pass-larger-than-limit warning have nothing to attach to. The configuration file's shape is undesigned (ADR 0017 line 83 leaves it to the build team), so no configuration read exists either.

## How bad it is for a user

Blocker for the release the experiment brief describes. The release pages and ADR 0017 promise a surface the binary does not carry, and the cache rows of the quality experiment cannot run against it.

Found by experiment 218, wave 1, area 8.
