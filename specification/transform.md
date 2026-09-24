# Transform catalog

Status: **Settled** by ticket 0083 under ticket 0050 and ADR 0015.

An installed `thinkthen` carries the ten reviewed `jq` transforms from [`../transforms/`](../transforms/) and prints them for `jq -f`. The tool never runs a transform. `jq` does the work.

```sh
thinkthen transform list
thinkthen transform show score > score.jq
jq -n --argjson cut 0.5 -f score.jq run.jsonl
```

## The catalog

The catalog holds exactly these public names: `band`, `calibration`, `compare`, `cost`, `counts`, `monitor`, `score`, `sweep`, `triage`, and `trials`. Each name is the transform's folder name under `transforms/`. A name is an exact, case-sensitive ASCII string. The catalog admits no alias, path, extension, prefix, case variant, or partial match.

The binary embeds the catalog at compile time from `crates/thinkthen/transforms/NAME.jq`. It discovers no member at run time. The lint rung holds each shipped copy byte-identical to `transforms/NAME/NAME.jq`, and the package rung checks the source package.

## `thinkthen transform list`

The command writes the ten names to standard output, one per line, in bytewise ascending order, and exits 0. The last line ends with one newline. Standard error stays empty. Locale, current folder, and build host do not change the order.

## `thinkthen transform show NAME`

The command writes the named transform to standard output byte for byte and exits 0. It adds and removes nothing. The final newline belongs to the file. Standard error stays empty.

A name outside the catalog prints nothing on standard output and exits 2 with this line on standard error:

```text
thinkthen: transform: unknown name; run `thinkthen transform list` to see the catalog
```

The line never repeats the rejected name. A missing or surplus argument gets the ordinary command-line usage error and also exits 2.

## What the catalog never does

Both commands route before any setup. They read no key, environment variable, configuration file, standard input, user file, or repository file. They open no socket, start no process, and write no cache, usage counter, lock, or folder. They never apply, parse, validate, or interpret a transform.
