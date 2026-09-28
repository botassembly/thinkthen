# Ticket 0241: site and sample build record

This record covers the candidate through main `565a6090`. The documentation issue inventory is
`sdlc/records/0241-documentation-issue-inventory.json`; mixed runtime items
stay open until their runtime criteria land. The batch ticket carries the final
closure map.

## Source and changes

- The default model pin changed request identities. I checked the replay miss
  against the exact request body, model, batch, and recorded response before
  running the guarded `sdlc/scripts/rekey-model` on the site's recording folder.
  It rekeyed 247 entries; the tool log is under
  `target/codex-builds/0241/rekey-model.log`. I copied the current recognize
  and relate recordings from demos 44 and 45. Old Beatles recognize/relate
  measurements remain historical evidence and no longer pretend to be
  executable samples. No paid measurements were rerun or rewritten.
- The site now renders ordinary function commands, values, and exit codes on
  the main pages. Function options and extra examples have one shared route;
  full result objects have a dedicated `/reference/details/` route.
- Python, Polars, TypeScript, Ruby, R, Rust, and C ordinary samples read the
  current `Call.value`, `$value`, or `into_value()` contract. Specialized C
  typed calls remain typed. R examples follow the candidate 0237 API; that
  runtime is still in another lane.
- The `BENCH` sample folder became `bench-pin` so a case-insensitive checkout
  can hold it beside `bench`. The pinned measurement SHA and values did not
  change.

## Focused proof

The offline site build, with Node 22 and the lane's isolated release binary,
passed `named-answers`, `check-samples`, `check-slides`, smoke, Astro, Markdown
twins, Settings, and links. It built 88 Astro pages and checked 126 routes.
Smoke matched 91 recorded CLI examples. It deliberately skips host and SQL
samples; it does not prove installed package execution.

I parsed all 13 Python, 10 Ruby, and 10 R samples; TypeScript transpiled all
10 samples with the repository's TypeScript compiler. All 10 C snippets
compiled in a main-function wrapper against the current header. All 11 Rust
snippets compiled in wrappers against the isolated release rlib; this caught
and corrected the retired `RecognizedEntity::name()` accessor to `text()`.
These checks prove syntax and current API names where the compiler can bind
them. Host samples have not been executed against every installed package.

The full build log and wrapper artifacts are under `target/codex-builds/0241/`.
No provider call, deployment, release arming, noindex change, or external Bench
source change was made.

## Settings and release-facing copy slice

The 46-row settings table's mixed Default cells became 50 value-only rows
with a separate Source column. The audit flags now each have their own default
and allowed values. The table names the built-in address, the key's environment
route on hosts and SQL, the band's inclusive zero low end, and deadline units.
The site reads those values from the table. `sdlc/scripts/settings` reports
50 rows, 54 flags, six environment names, and 15 question-file keys with no
failure; its 10 planted failures still pass. The full offline site build passes
its 50-row settings check and all 126 internal links.

The install pages now label release channels as planned until packages are
published and verified. README's first run uses the source checkout's existing
recording; I ran that exact `decide --replay` invocation with the lane binary
and got `true`, exit zero. The site details route imports a saved request and
Jev response body with its reported usage, and links the canonical type,
result, and schema files.

## Four replayed practical pages

The site adds a Bash coprocess that keeps one `choose` process alive for three
steps using demo 21's recorded exchanges; a paragraph split with `awk -v RS=`
and `jq` that judges four paragraphs against existing recordings; a
Claude Code `PreToolUse` guard that maps the three replayed outcomes from demo
19 to `allow`, `ask`, and `deny`; and a raw-line validator that sets a malformed
record and a non-text body aside before judging three valid records. The guard
does not execute any proposed command. Its host mapping was checked on
2026-09-28 against the official hooks reference at
`https://code.claude.com/docs/en/hooks#pretooluse-decision-control`.
`npm run build` now replays 95 CLI examples, builds 92 pages and their Markdown
twins, and checks 130 internal routes. The sample checker still examines
copyable examples; it omits exact recorded inputs and wire bodies that the
smoke run consumes.

## Reference and site guard follow-up

The SQLite install text now states the complete-request cache rule and names
`thinkthen_batch(1)` for the legacy scalar-after-warm recipe. The DuckDB
recognize sample unnests the scalar relation list, and its relate sample uses
the current rule shape. The reference page matches the command's timeout
range and prune's refusal of a model no reply names. The built HTML check
catches a stray `<code>` tag after a table. I planted that exact fault in a
generated page and observed exit 1 with the page path, then restored the page
and observed the 130-page link check pass. The red log is under
`target/codex-builds/0241/stray-code-red.log`.

## Final candidate proof and limits

After merging main `2f274076`, I rebuilt the isolated release binary and ran
the offline site build again with Node 22. The final log is
`target/codex-builds/0241/site-build-final-corrected.log`: 95 CLI examples replayed,
92 Astro pages and 92 Markdown twins built, the 50-row settings check passed,
and all 130 internal routes linked. `sdlc/scripts/settings --self-test` kept
its 10 planted failures. The HTML guard's planted failure and recovery are
recorded above. The site smoke intentionally does not execute host or SQL
samples.
The repository policy check passed on this merged tree: 189 resolved packages,
with accepted tables, ban lists and dependencies matching. Its log is
`target/codex-builds/0241/policy-latest-main.log`.

`target/codex-builds/0241/sample-provenance.json` lists each changed library
or SQL sample file, its source and proof, and its current
`host_runtime_executed` state. Python parsed 13 examples; Ruby and R parsed 10
each. C compiled 10
snippets against the current header and Rust compiled 11 against the current
release rlib. A TypeScript `noEmit` check over every site `.ts` sample against
the current `index.d.ts` caught and corrected recognized-entity `text`
accessors, optional relations, and tuple types. That is stronger than the
earlier transpile-only check, but no host package or SQL extension was invoked
by the site smoke. The newly landed SQLite `thinkthen_find` sample reads the
selected value from its JSON result. It follows the accepted function shape
in `databases/sqlite/README.md` and `databases/sqlite/tests/test_values.py`,
but this site build did not execute it. All CLI examples use replay, with no
provider call.

The default-backend notices in README and the site link the vendor's published
customer agreement, data processing addendum, and privacy policy. The policy
does not promise a fixed API-input retention period, so the copy gives none.
The six public error kinds are linked to the normative type contract; advice
messages are not presented as stable identifiers. Register 117's runtime
catalog criterion and register 124's external-terms criterion remain open.
The final contract read caught an imprecise draft description of answer kinds
and probability shape on the details route; I corrected it against
`specification/result.md` before the last site build.

## Code review correction

The independent read-only review of `673b61f0` found the relate catalog still
described clashing travel-rule records although its example names gateway and
billing entities. I corrected the lede, primitive, line, input count and
`--jobs` summary against `specification/relate.md`. The new site build log is
`target/codex-builds/0241/site-build-review-correction.log`: 95 CLI examples,
92 pages and Markdown twins, 50 settings rows and 130 linked routes pass.
Inspection of the built function and reference text finds “gateway calls
billing” and “up to 255 entities”, and finds neither “travel rules” nor “255
records”. The host examples and their earlier proof were untouched; CLI
smoke still skips 94 samples, including the host and SQL examples. The original-ID manifest now
distinguishes 12 ticket-fixed, three baseline-fixed, four partial, nine mixed
runtime and ten non-documentation rows. The four partial rows name their exact
unfinished criteria and disposition.

After that correction, main landed reviewed 0243 Rust choice descriptions,
0229's read-only unused-entry report, and PostgreSQL ordered find. I merged
them without editing their runtime or test files. The 18-column settings table
retains its Source column, adds 0229's Unused entry report row and carries
PostgreSQL find's None option and Framing cells. With the rebuilt isolated CLI,
`sdlc/scripts/settings --self-test` passed 10/10 and the live table check
reported 51 rows, 56 flags, six environment names, 15 question-file keys and
zero failures. The final site log is
`target/codex-builds/0241/site-build-after-postgresql.log`: 95 replayed CLI
examples, 92 pages and Markdown twins, 51 settings and 130 linked routes pass.
The new PostgreSQL find site sample follows its accepted ordered-array result
shape; it is the 56th changed host/SQL file in the local provenance manifest.
The site check did not execute that SQL extension.
The merged-tree policy check also passed: 189 resolved packages, with accepted
tables, ban lists and dependencies matching. Its log is
`target/codex-builds/0241/policy-after-0229.log`.

## Original C and Python consumer issue correction

The next independent review found that compilation and Python parsing did not
meet the original issues' explicit offline-execution criterion, and the C
examples extracted `value` by string offset. Those two original IDs remained
partial while I corrected the samples. The seven C examples now parse the
complete `Call` JSON with json-c 0.17, require a present `value` and `facts`,
compare typed nested values with `json_object_equal`, and free both parsed
objects and the native reply. The choose example's valid JSON null is distinct
from `thinkthen_call` returning native NULL. The install example requires a
numeric `value` of 2.0. The C surface names `libjson-c-dev` and its pkg-config
link flags. The parser was used only by site examples; no runtime dependency
was added.

I built the current C source at branch base `3b22a2cd` in the lane's isolated
Cargo target. Its `libthinkthen_c.so` SHA-256 is
`a4a301aa43114047c484bd7816d7ee876b5830d9d5ea09148173423bdaee31eb`.
The C headers came from `libjson-c-dev` 0.17-1build1 extracted under
`target/codex-builds/0241`; the linked system runtime is libjson-c5
0.17-1build1. I also built the current editable Python extension from that
source into the lane's Python 3.12 virtual environment with Polars 1.44.2;
`libraries/python/thinkthen/_thinkthen.abi3.so` has SHA-256
`56b98039c502d9789283b02f5c4d30efe2be0ef26e7d93331bef4e8e2dec7c6f`.
The build logs are `c-build-host-samples.log` and
`python-build-host-samples.log` under `target/codex-builds/0241`.

`target/codex-builds/0241/run_host_samples.py` compiled and executed the exact
seven C site snippets, and executed all thirteen exact Python/Polars snippets
without replacing their assertions. Its wrapper supplies `main` and logs a
native call failure; each sample retains its shown requests and value checks.
For Python, `exec(compile(source))` from an isolated work folder avoids a
sample named `polars.py` shadowing the installed package. The only transport
was a loopback server that requires a deep-equal saved request body and returns
its saved response unchanged. `THINKTHEN_BATCH=1` preserved the legacy
singleton request identity. The final run matched 54 saved responses with zero
misses: 20 exact snippets exited zero. Per-file source SHA-256, response paths,
exit logs, and compile wrappers are under
`target/codex-builds/0241/host-sample-proof/`; the combined source inventory
is `target/codex-builds/0241/sample-provenance.json`. No provider call or
recording regeneration occurred. The recognize sample checks the three
recorded entities, and the separate relate sample checks the recorded
`gateway` to `billing` edge. Both have real value assertions.

The final focused site build, `site-build-host-correction.log`, passed under
Node 22 and the lane lock: 95 CLI replay examples, 92 Astro pages and Markdown
twins, 51 settings rows, and 130 linked routes. `check-samples` passed. It
still skips host and SQL samples; the independent host run above proves the
C/Python rows only. The inventory therefore returns to 12 newly fixed
candidate documentation IDs, three fixed on baseline, and four partial. The
remaining partial host/SQL and external Bench criteria stay open.
