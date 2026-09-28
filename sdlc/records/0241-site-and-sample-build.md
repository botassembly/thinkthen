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

`target/codex-builds/0241/sample-provenance.json` lists each of the 55 changed
library or SQL sample files, the current source example or API document used
to check it, the proof applied, and the explicit `host_runtime_executed: false`
limit. Python parsed 13 examples; Ruby and R parsed 10 each. C compiled 10
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
