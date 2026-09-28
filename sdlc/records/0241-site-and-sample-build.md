# Ticket 0241: site and sample build record

This is the first implementation slice. The documentation issue inventory is
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
