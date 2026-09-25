# The default cache binds to the first address, and the refusal never names it

Status: Open

Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

## What happens

The default cache folder binds to the first backend address it meets. A later ordinary command with another `--url` or `THINKTHEN_BASE_URL` stops at exit 5. The message talks about "the recording folder". The user never named a folder. The message does not name the default cache, `--no-cache`, or `THINKTHEN_CACHE`.

A run that sends nothing still binds the folder. In the steps below, the first run stops for a missing key, and the folder is bound anyway.

## Reproduction

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

## What the spec says

- `specification/recording.md`: "The first write-capable use of a new or empty folder binds it to the canonical backend interface and resolved endpoint address." A later mismatch fails at exit 5 before the tool reads a key. "The message tells the user to restore the backend settings or choose another folder." The page makes no exception for the default cache.
- The same page: a writing mode creates its private temporary entry before it reads the key. That order explains why a run with no key still binds the folder.
- Ticket 0065 added the binding to stop a silent re-bill after an address change (`2026-09-21-a-cache-resume-under-a-different-address-silently-re-bills-everything.md`). Ticket 0062 and ADR 0033 set up the default cache. None of them considers a user who points one default cache at a second backend.

The binding is ruled. The wording of the refusal for the default cache is not.

## Why it matters to a user

The public story invites a user to try their own backend. `check` passes against that backend, because `check` does not use the cache. The user's first `decide` against it then stops at exit 5. The message tells them to restore settings they meant to change or to choose a folder they never chose. Nothing tells them the default cache exists, where it lives, or how to step around it. A first run that failed for a missing key is enough to lock the folder to an address the user only tried once.

## Options

1. Name the default cache in the refusal when no folder was named. For example: the default cache at PATH is bound to another backend address; use `--no-cache`, set `THINKTHEN_CACHE` to another folder, or clear the folder. The folder-privacy refusal already names `--no-cache`, so this follows an existing pattern.
2. Give the default cache one subfolder per backend address, named by the same digest the marker holds. Each address then gets its own cache, and no refusal fires. Explicit folders keep today's binding. This changes a ruled layout and needs a ruling.
3. Bind a folder only when an entry is written. A run that fails before any send would then leave the folder unbound. This changes the order the spec sets for private temporary entries.
4. Print the bound address in `status` beside `cache_path`, so a user can see the binding before a run fails.

Options 1 and 4 change wording only. Options 2 and 3 change spec behavior and need a ruling.
