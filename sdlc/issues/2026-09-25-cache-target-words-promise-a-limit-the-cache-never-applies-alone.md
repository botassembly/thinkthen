# Cache "target" words promise a limit the cache never applies alone

Status: Open

Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

## What happens

The default cache grows until someone runs `thinkthen cache prune DIR`. Nothing trims it on its own. That design is ruled, and this issue does not ask to change it. The words around the cache read as a limit the tool keeps:

- `thinkthen status` prints `cache_target_bytes 100000000` beside `cache_bytes`. It prints no warning when the cache is over the target.
- Ticket 0062 is titled "Enable the bounded default cache". ADR 0033 is titled "Bounded default cache and read-only configuration". `specification/README.md` calls the configuration "bounded-cache".
- ADR 0017 calls the 100 MB value "the cache cap".
- `site/src/pages/reference.astro` says "The default is the cache on, with a target of 100,000,000 allocated bytes."

No help text says the folder grows until someone prunes it. `cache prune --help` says "Remove selected entries, then the oldest entries above the size target". That line reads as maintenance of a limit that exists elsewhere.

## Reproduction

Offline, with no key. Copy a committed Beatles Bench recording into a private default cache home, and set a 100,000-byte target in the configuration file.

    $ BB=path/to/beatles-bench
    $ mkdir -p x3/cache x3/config/thinkthen
    $ cp -a $BB/examples/11-audit/recording x3/cache/thinkthen; chmod 700 x3/cache/thinkthen
    $ echo '{"schema":"thinkthen.config/1","cache_bytes":100000}' > x3/config/thinkthen/config.json
    $ export XDG_CACHE_HOME=$PWD/x3/cache XDG_CONFIG_HOME=$PWD/x3/config
    $ thinkthen status | grep -E '^cache_(entries|bytes|target)'
    cache_entries 140
    cache_bytes 573440
    cache_target_bytes 100000
    cache_target_source configuration

    $ printf 'Come Together\n' | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
        thinkthen decide 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' --lines
    {"input":"Come Together","value":true}
    $ thinkthen status | grep -E '^cache_(entries|bytes)'
    cache_entries 140
    cache_bytes 573440

The cache sits at more than five times its target. A run succeeds, and nothing trims or warns. Experiment 259 also set a target of 1 byte and added entries through the offline loopback backend. The cache kept every entry.

## What the spec says

- ADR 0017, the rules paragraph: a request never deletes anything. Removal is `thinkthen cache prune DIR`, "by age or by the cap, oldest-written first". "With plain files the cap is applied by prune."
- Ticket 0062: "Pruning remains an explicit command." Its exclusions list "automatic pruning" and "a hard refusal when the cache exceeds its target".
- `specification/recording.md`, "Pruning a cache": `--max-size` sets a target, and without it the configuration target applies. Prune is the only removal surface.

The spec decides the behavior. It does not decide what the words tell a user.

## Why it matters to a user

A user who reads "bounded", "cap", or `cache_target_bytes 100000000` expects the tool to hold the cache near 100 MB. The cache holds the judged text, so an unbounded folder is also a privacy surface the user did not plan for. A long pipeline over customer messages can fill a disk while `status` shows a target the tool never applies. The user learns otherwise only by reading ADR 0017 or the source.

## Options

1. Rename the words. `status` prints `cache_prune_target_bytes`, the tickets and ADR titles drop "bounded", and the site says the value is the size `cache prune` trims to.
2. Keep the names, and add one sentence where the target appears: the cache grows until you run `thinkthen cache prune DIR`. Prune trims it to this size. Candidates are the `status` help, the `cache` help, the site reference, and the first privacy note.
3. Have `status` print a line such as `cache_over_target true` when `cache_bytes` exceeds the target, with no trimming.
4. Have a normal run print one warning when the cache is over its target, with no trimming. This adds stderr output to every run past the target and needs a ruling.

Automatic trimming is out of scope. Ian ruled it out.
