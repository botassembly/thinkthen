# Prune by model with the alias deletes every entry

Status: Open

Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

## What happens

`thinkthen cache prune DIR --answered-by-other-than jev-latest` deletes every entry. Each entry's request asks for `jev-latest`. Each response names the version that answered, such as `jev-1.13.0`. The selector compares the response's model. No entry's response says `jev-latest`, so every entry is "answered by another model". The command gives no warning and exits 0.

A typo does the same. Any name that no entry carries selects every entry.

## Reproduction

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

## What the spec says

- `specification/recording.md`, "Pruning a cache": `--older-than` and `--answered-by-other-than MODEL` select a union. Prune validates that each response names a nonblank model. The page does not say whether MODEL is matched against the requested alias or the answering version.
- `cache prune --help`: "Remove entries answered by any other model". The words are literally true.
- `decide --help` shows `--model <NAME>` with `[default: jev-latest]`. The name a user sees and types is the alias.
- The closed issue `2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder.md` designed the selector to clear "what an older model said". Its example assumes the user knows the answering version.

## Why it matters to a user

The selector exists for one moment: the alias moved to a new version, and the user wants to drop old answers. The natural thing to type is the alias the user always passes. That command empties the cache or the recording. A recording can be a committed test fixture, because `cache prune` accepts any folder. Replays against it then fail, and re-asking costs money. The command reports success.

## Options

1. Refuse a MODEL that no entry's response carries, at exit 2, before deleting anything. The message lists the answering models found. This catches the alias and a typo.
2. Warn and stop when the selector would remove every entry, unless the user adds a confirming flag such as `--all-match-ok`.
3. Match MODEL against either the requested model or the answering model. `jev-latest` would then keep entries that asked for `jev-latest`. This changes the selector's meaning and needs a spec change.
4. Add a `--dry-run` to `cache prune` that prints the count line without deleting.
5. Say in the help that MODEL is the version that answered, such as `jev-1.13.0`, and never the alias.

Options 1, 2, and 4 add a guard. Option 3 changes meaning. Option 5 is wording only and leaves the trap in place.
