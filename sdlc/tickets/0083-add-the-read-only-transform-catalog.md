---
flow: build
priority: 83
opens: crates/thinkthen/src/cli crates/thinkthen/transforms crates/thinkthen/tests specification spec sdlc/scripts sdlc/ratchet.json sdlc/planning
---

# 0083: Add the read-only transform catalog

Status: ready

## Outcome and authority

Implement the approved read-only catalog as exactly two commands:

```text
thinkthen transform list
thinkthen transform show NAME
```

Ticket 0050 and ADR 0015 approve this surface after the one-crate move. Ticket 0055 completed that move. This ticket fixes the catalog membership, public names, byte identity, package location, refusal, and offline proof that ticket 0050 deliberately left open. Ian can overturn those choices before implementation.

The command lists or prints embedded `jq` source. It does not apply, parse, validate, interpret, or execute a transform. It does not read a key, environment configuration, standard input, a user file, the repository transform tree, or a platform folder. It does not enter the engine, install the interrupt carrier, open a socket, or start a process.

Root help keeps `status` first, then the ten judgment functions in their settled order, then `cache`, then `transform`, then Clap's generated `help` row. The exact root description is `List or print the built-in jq transforms without running them.` The nested introductions are `List the names of the built-in jq transforms.` for `transform list` and `Print one built-in jq transform exactly as shipped.` for `transform show`. Ticket 0082's post-0081 baseline records the pre-catalog command list; ticket 0083 adds only this utility row immediately after `cache` and before generated `help`, and pins every earlier row byte for byte.

## Catalog contract

The catalog contains all ten reviewed transforms that completed slice 10b. Their public names are the existing lowercase folder names. Names are exact, case-sensitive ASCII strings. There are no aliases, path forms, extensions, prefixes, case folding, or partial matches.

`thinkthen transform list` writes exactly these bytes to standard output and exits 0 with empty standard error:

```text
band
calibration
compare
cost
counts
monitor
score
sweep
triage
trials
```

The order is fixed bytewise ascending order over the public names. It does not depend on filesystem enumeration, locale, build host, current directory, or hash-map order. The final `trials` line ends with one newline.

`thinkthen transform show NAME` writes the selected catalog member byte for byte to standard output, adds and removes nothing, flushes it, and exits 0 with empty standard error. Every admitted file currently ends with one newline, and that newline is part of its identity. The command writes bytes rather than normalizing text or line endings.

An unknown name, including a path, extension, case variant, empty value, or extra value, prints no standard output. An unknown value that reaches catalog lookup exits 2 and writes exactly this line to standard error, without echoing the supplied value:

```text
thinkthen: transform: unknown name; run `thinkthen transform list` to see the catalog
```

Clap retains its ordinary fixed diagnostics for missing or surplus command-line arguments. Catalog lookup owns only one syntactically admitted `NAME` that is absent from the closed table.

## Byte and package boundary

The implementation seeds the packaged catalog from the ten current repository sources at main base `b02db985c6729b57b0c9ce46aba938666b7db0f6`:

| Public name | Repository source | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `band` | `transforms/band/band.jq` | 3,425 | `bd4934353f3d07da55f2642b5090a96d77788a778095bdf37607c8c07729b176` |
| `calibration` | `transforms/calibration/calibration.jq` | 2,893 | `23f91a4231f910c567939e6bd0dbeded6bcb3e9be785991d58091365ca5747f5` |
| `compare` | `transforms/compare/compare.jq` | 12,218 | `1909601f3d5325111e6f2293521211c692bf0e2a4015369d89b84830acde110f` |
| `cost` | `transforms/cost/cost.jq` | 2,295 | `4b00fc0c180bd0464a0e84d0a221cca5204e9a456da41383f43f1f3cda7c209e` |
| `counts` | `transforms/counts/counts.jq` | 1,735 | `de58fa6c5646432e3947d404a1355ebd0724f4829e234f76472b4fe8e6e0132f` |
| `monitor` | `transforms/monitor/monitor.jq` | 3,453 | `18e9a22e298a9819c0817672fa8c8a62e99800176e5f66357e5e2a0024906b27` |
| `score` | `transforms/score/score.jq` | 3,495 | `6df93483f3d017c3d6c91e8920c66ba636099fa767019f46ba792eee34a5f8d5` |
| `sweep` | `transforms/sweep/sweep.jq` | 26,692 | `8100a565923890f16bf338dda397360c7ae55a300648ac972116d4c2212f4871` |
| `triage` | `transforms/triage/triage.jq` | 1,788 | `6ec36edd30443f39a2de5f8e24e09e8aba9cc33446128381f4f5830856744736` |
| `trials` | `transforms/trials/trials.jq` | 8,334 | `aa361137f146a33bf1547e6f0276c5a312a20d9469178f3c28474932d629e2dd` |

The shipping copies live at `crates/thinkthen/transforms/NAME.jq`, inside the one `thinkthen` crate source package. The binary embeds those files at compile time through a closed static table. It never discovers catalog members at runtime. The existing repository files remain the executable development sources for their examples and tests. A lint-rung check compares every name and every byte in both locations and fails on a missing, extra, renamed, or changed member. A transform change updates both copies in one commit. The packaged copy is the authority for public `show` bytes.

Do not generate the packaged files in `build.rs`, copy them from outside the crate during packaging, or depend on a checkout-relative path. Add no build script and no dependency. `cargo package --locked --offline` must include all ten files, and the existing package rung must inspect the archive rather than assume Cargo included them.

## Command boundary

Route `transform` immediately after argument parsing and version handling, before `Environment::read`, interrupt activation, usage-counter setup, standard-input locking, or any engine construction. The catalog path accepts only the parsed subcommand and the locked standard-output writer. Its closed table contains public names and embedded byte slices only.

Extend mechanical policy to forbid filesystem, environment, network, clock, process, engine, cache, recording, usage, and interrupt owners from the catalog module. Plant representative forbidden references in the policy self-test and prove each is refused. Keep the catalog outside the pure judgment core because it is command distribution data, not a judgment rule.

## Scope and exclusions

Allowed: the nested command grammar and help; one small catalog module; the ten exact packaged `.jq` files; early command routing; one exact unknown-name failure; focused binary, policy, and source-package tests; one Settled specification page and index entry; one executable command page; the package-rung archive check; exact ratchet, queue, ticket, and record updates required when implementation lands.

Excluded: applying, evaluating, parsing, validating, or interpreting transforms; starting `jq` or any other process; accepting standard input or a file path; reading repository or user files at runtime; APIs in Rust, C, language packages, Polars, or databases; transform arguments or metadata; search, descriptions, JSON output, aliases, installation, release archives, installers, workflows, registry publication, site work, credentials, live calls, paid calls, and changes to transform behavior or bytes beyond making the exact packaged copies.

Release artifacts and publication remain later authorized work. This ticket proves a Cargo source package and a binary built from it. It does not create, upload, install, or publish an artifact.

## Budgets

Production Rust changes may touch at most four existing files and add one catalog module. They may add at most 180 nonblank production Rust lines. Total Rust additions, including tests, may not exceed 500 nonblank lines. The package adds exactly ten `.jq` files totaling 66,328 bytes and 1,418 newline-terminated lines, each byte-identical to the table above. Add no dependency and no build script. Keep every Rust source and test file within the repository's 500-nonblank-line ceiling.

Search the existing nested `cache` command grammar, early `--version` path, locked writer, failure mapping, policy scanner, package rung, and compiled-command test helpers before raising the exact ratchet. The implementation record names every production file touched, the net Rust increase, and where duplication was removed or avoided. Stop and re-score before exceeding any file, line, byte, dependency, or surface bound.

## Acceptance

- Observe focused red tests before implementation. Pin the exact list bytes, final newline, empty standard error, exit 0, and identical output across repeated runs, changed locales, unrelated current directories, and deliberately shuffled internal fixture input.
- Pin root help with `status` first, the ten judgment functions unchanged, `cache` immediately after them, `transform` immediately after `cache`, and generated `help` last. Pin the exact root, `list`, and `show` descriptions above in short and long help. No earlier command row or introduction changes from the post-0082 baseline.
- Pin all ten `show` results against byte fixtures and the byte counts and SHA-256 values above. Prove each result has no added prefix, suffix, newline, UTF-8 rewrite, or line-ending rewrite.
- Pin exact-name lookup. Refuse case variants, `.jq` suffixes, `./` paths, traversal strings, prefixes, empty values, and unknown Unicode names. The lookup refusal has empty standard output, exit 2, and the exact fixed sentence above without the rejected value.
- Prove the command path reads no key or configuration by setting canary values for every recognized environment variable and making the normal platform/configuration locations inaccessible. Both catalog commands still return the exact bytes and create or change no file, cache, counter, lock, or folder.
- Prove zero network requests with a counting loopback listener named by `THINKTHEN_BASE_URL`. Set a canary `THINKTHEN_API_KEY`; run every successful catalog command and the unknown-name path; observe zero accepted requests and zero canary occurrences across standard output, standard error, Debug output, and temporary files.
- Prove zero process execution with a trap `jq` and trap helper binaries first on `PATH`; every catalog path leaves their marker absent. The mechanical policy separately refuses process APIs, absolute-path execution code, and engine entry from the catalog owner.
- Prove no standard-input or user-file access. Run with standard input held open and with decoy files named after every transform in the current directory; the commands complete from embedded bytes and leave access markers unchanged. The command grammar admits no input option or path, and policy forbids filesystem APIs in the catalog owner.
- Run `cargo package --locked --offline`, inspect the generated `.crate`, and prove exactly the ten named package files and their bytes are present. Unpack it into a temporary directory outside the checkout, build it offline, remove the unpacked source tree after copying the binary, and run every `list` and `show` check from an empty directory. The isolated run must use no path back into the checkout.
- Prove the package check can fail by changing one copied byte, deleting one packaged member, and adding one unlisted `.jq` member in isolated fixtures. Each planted fault must fail the rung for the stated reason.
- Publish a Settled specification page for the two commands and an executable page that pins the list, one representative exact `show`, and the fixed unknown refusal. Update command help and the transform index without claiming that the tool runs a transform.
- Run focused tests, the policy self-test, package rung, exact ratchet, formatting, Clippy, `git diff --check`, then all four local gates sequentially with key and base-address variables unset. An independent reviewer checks public names, exact bytes, early routing, forbidden-capability proof, package independence, exclusions, and budgets. No live or paid call runs.

## Dependencies and canonical status

Ticket 0050 supplies the approved catalog ruling. Ticket 0055 supplies the landed one-crate package boundary. Under the launch-first queue, implementation waits for tickets 0080, 0081, and 0082 to land on main; 0082 is the immediate prerequisite because both tickets complete nested read-only command surfaces and can overlap the CLI grammar, early routing, help, package checks, and command tests. Drafting and independent design review may proceed before those tickets land. Code starts from their integrated main and rechecks the budgets against that tree.

This ticket is the sole ready implementation authority for the transform catalog. Ticket 0050 remains the landed decision record and does not authorize a second implementation. The quality plan's release-checklist row 15 points here. The canonical ticket status is `ready`; only implementation changes it to `in progress`, and only a reviewed landed record changes it to `landed`.

## Complexity

Contract 1; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 5. Final level: 2. Reasons: the runtime is a closed static lookup with no state, while exact public bytes, early exclusion of all effectful setup, and proof from an unpacked source package cross the CLI, policy, and package rung. Luna Extra High owns implementation design, case analysis, code, and remediation under the three-ticket trial. Independent Sol High sessions recheck this amended ticket before code and review the final diff. Stop and re-score if implementation adds runtime discovery, transform interpretation, a dependency or build script, any API surface, any effectful setup before catalog dispatch, changed transform bytes, release artifacts, or more than the stated file, line, or byte budget.
