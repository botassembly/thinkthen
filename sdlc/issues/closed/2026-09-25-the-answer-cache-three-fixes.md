# The answer cache: three fixes

Status: Closed on 2026-09-25 by ticket 0124. Its Deferred gaps name what it left open: a prune `--dry-run`, a typo guard, and an over-target status line or warning. This issue merges three cache issues filed on 2026-09-25 during the talk's claim check, so the cache work reads in one place:

- `2026-09-25-the-default-cache-binds-to-the-first-address-and-the-refusal-never-names-it`
- `2026-09-25-cache-target-words-promise-a-limit-the-cache-never-applies-alone`
- `2026-09-25-prune-by-model-with-the-alias-deletes-every-entry`

Their text is kept below, one section each. Headings are shifted down one level.

## 1. The default cache binds to the first address, and the refusal never names it


Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

### What happens

The default cache folder binds to the first backend address it meets. A later ordinary command with another `--url` or `THINKTHEN_BASE_URL` stops at exit 5. The message talks about "the recording folder". The user never named a folder. The message does not name the default cache, `--no-cache`, or `THINKTHEN_CACHE`.

A run that sends nothing still binds the folder. In the steps below, the first run stops for a missing key, and the folder is bound anyway.

### Reproduction

Offline, with no key and an empty private cache home. Address 127.0.0.1:9 is never reached.

    $ mkdir -p xdg/cache xdg/config
    $ r() { printf 'Come Together\n' | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL -u THINKTHEN_CACHE \
        XDG_CACHE_HOME=$PWD/xdg/cache XDG_CONFIG_HOME=$PWD/xdg/config \
        thinkthen decide 'It appears on the album Abbey Road.' --lines "$@"; echo "exit $?"; }

    $ r --url http://127.0.0.1:9/v1
    thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent
    thinkthen: stopped at record 1; 0 records finished
    exit 4
    $ ls -A xdg/cache/thinkthen
    .locks  .thinkthen-backend.json

    $ r
    thinkthen: the recording folder belongs to another backend interface or address; restore its backend settings or choose another folder
    thinkthen: stopped at record 1; 0 records finished
    exit 5

    $ r --no-cache
    thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent
    exit 4

The same happens the other way round. A default cache first used against the hosted address refuses `--url http://127.0.0.1:9/v1` at exit 5. Changing only `--model` does not trip it.

`thinkthen status` prints `cache_path` and the entry count. It does not print the address the folder is bound to.

### What the spec says

- `specification/recording.md`: "The first write-capable use of a new or empty folder binds it to the canonical backend interface and resolved endpoint address." A later mismatch fails at exit 5 before the tool reads a key. "The message tells the user to restore the backend settings or choose another folder." The page makes no exception for the default cache.
- The same page: a writing mode creates its private temporary entry before it reads the key. That order explains why a run with no key still binds the folder.
- Ticket 0065 added the binding to stop a silent re-bill after an address change (`closed/2026-09-21-a-cache-resume-under-a-different-address-silently-re-bills-everything.md`). Ticket 0062 and ADR 0033 set up the default cache. None of them considers a user who points one default cache at a second backend.

The binding is ruled. The wording of the refusal for the default cache is not.

### Why it matters to a user

The public story invites a user to try their own backend. `check` passes against that backend, because `check` does not use the cache. The user's first `decide` against it then stops at exit 5. The message tells them to restore settings they meant to change or to choose a folder they never chose. Nothing tells them the default cache exists, where it lives, or how to step around it. A first run that failed for a missing key is enough to lock the folder to an address the user only tried once.

### Options

1. Name the default cache in the refusal when no folder was named. For example: the default cache at PATH is bound to another backend address; use `--no-cache`, set `THINKTHEN_CACHE` to another folder, or clear the folder. The folder-privacy refusal already names `--no-cache`, so this follows an existing pattern.
2. Give the default cache one subfolder per backend address, named by the same digest the marker holds. Each address then gets its own cache, and no refusal fires. Explicit folders keep today's binding. This changes a ruled layout and needs a ruling.
3. Bind a folder only when an entry is written. A run that fails before any send would then leave the folder unbound. This changes the order the spec sets for private temporary entries.
4. Print the bound address in `status` beside `cache_path`, so a user can see the binding before a run fails.

Options 1 and 4 change wording only. Options 2 and 3 change spec behavior and need a ruling.

## 2. Cache "target" words promise a limit the cache never applies alone


Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

### What happens

The default cache grows until someone runs `thinkthen cache prune DIR`. Nothing trims it on its own. That design is ruled, and this issue does not ask to change it. The words around the cache read as a limit the tool keeps:

- `thinkthen status` prints `cache_target_bytes 100000000` beside `cache_bytes`. It prints no warning when the cache is over the target.
- Ticket 0062 is titled "Enable the bounded default cache". ADR 0033 is titled "Bounded default cache and read-only configuration". `specification/README.md` calls the configuration "bounded-cache".
- ADR 0017 calls the 100 MB value "the cache cap".
- `site/src/pages/reference.astro` says "The default is the cache on, with a target of 100,000,000 allocated bytes."

No help text says the folder grows until someone prunes it. `cache prune --help` says "Remove selected entries, then the oldest entries above the size target". That line reads as maintenance of a limit that exists elsewhere.

### Reproduction

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

### What the spec says

- ADR 0017, the rules paragraph: a request never deletes anything. Removal is `thinkthen cache prune DIR`, "by age or by the cap, oldest-written first". "With plain files the cap is applied by prune."
- Ticket 0062: "Pruning remains an explicit command." Its exclusions list "automatic pruning" and "a hard refusal when the cache exceeds its target".
- `specification/recording.md`, "Pruning a cache": `--max-size` sets a target, and without it the configuration target applies. Prune is the only removal surface.

The spec decides the behavior. It does not decide what the words tell a user.

### Why it matters to a user

A user who reads "bounded", "cap", or `cache_target_bytes 100000000` expects the tool to hold the cache near 100 MB. The cache holds the judged text, so an unbounded folder is also a privacy surface the user did not plan for. A long pipeline over customer messages can fill a disk while `status` shows a target the tool never applies. The user learns otherwise only by reading ADR 0017 or the source.

### Options

1. Rename the words. `status` prints `cache_prune_target_bytes`, the tickets and ADR titles drop "bounded", and the site says the value is the size `cache prune` trims to.
2. Keep the names, and add one sentence where the target appears: the cache grows until you run `thinkthen cache prune DIR`. Prune trims it to this size. Candidates are the `status` help, the `cache` help, the site reference, and the first privacy note.
3. Have `status` print a line such as `cache_over_target true` when `cache_bytes` exceeds the target, with no trimming.
4. Have a normal run print one warning when the cache is over its target, with no trimming. This adds stderr output to every run past the target and needs a ruling.

Automatic trimming is out of scope. Ian ruled it out.

## 3. Prune by model with the alias deletes every entry


Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

### What happens

`thinkthen cache prune DIR --answered-by-other-than jev-latest` deletes every entry. Each entry's request asks for `jev-latest`. Each response names the version that answered, such as `jev-1.13.0`. The selector compares the response's model. No entry's response says `jev-latest`, so every entry is "answered by another model". The command gives no warning and exits 0.

A typo does the same. Any name that no entry carries selects every entry.

### Reproduction

Offline, on a copy of a committed Beatles Bench recording.

    $ BB=path/to/beatles-bench
    $ cp -a $BB/examples/11-audit/recording p4
    $ jq -r '[.request.model, .response.model] | @tsv' p4/*.json | sort | uniq -c
        140 jev-latest	jev-1.13.0

    $ thinkthen cache prune p4 --answered-by-other-than jev-latest; echo "exit $?"
    removed 140 entries and 573440 bytes; 0 entries and 0 bytes remain
    exit 0

    $ cp -a $BB/examples/11-audit/recording p5
    $ thinkthen cache prune p5 --answered-by-other-than jev-1.13.0
    removed 0 entries and 0 bytes; 140 entries and 573440 bytes remain
    $ thinkthen cache prune p5 --answered-by-other-than no-such-model; echo "exit $?"
    removed 140 entries and 573440 bytes; 0 entries and 0 bytes remain
    exit 0

### What the spec says

- `specification/recording.md`, "Pruning a cache": `--older-than` and `--answered-by-other-than MODEL` select a union. Prune validates that each response names a nonblank model. The page does not say whether MODEL is matched against the requested alias or the answering version.
- `cache prune --help`: "Remove entries answered by any other model". The words are literally true.
- `decide --help` shows `--model <NAME>` with `[default: jev-latest]`. The name a user sees and types is the alias.
- The closed issue `closed/2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder.md` designed the selector to clear "what an older model said". Its example assumes the user knows the answering version.

### Why it matters to a user

The selector exists for one moment: the alias moved to a new version, and the user wants to drop old answers. The natural thing to type is the alias the user always passes. That command empties the cache or the recording. A recording can be a committed test fixture, because `cache prune` accepts any folder. Replays against it then fail, and re-asking costs money. The command reports success.

### Options

1. Refuse a MODEL that no entry's response carries, at exit 2, before deleting anything. The message lists the answering models found. This catches the alias and a typo.
2. Warn and stop when the selector would remove every entry, unless the user adds a confirming flag such as `--all-match-ok`.
3. Match MODEL against either the requested model or the answering model. `jev-latest` would then keep entries that asked for `jev-latest`. This changes the selector's meaning and needs a spec change.
4. Add a `--dry-run` to `cache prune` that prints the count line without deleting.
5. Say in the help that MODEL is the version that answered, such as `jev-1.13.0`, and never the alias.

Options 1, 2, and 4 add a guard. Option 3 changes meaning. Option 5 is wording only and leaves the trap in place.
